/// Enumerate the bits in the buffer  
///
/// # Examples
/// ```
/// # use bits::BitIterator;
/// assert_eq!(
///     [0b1111_0000_u8].bit_iter().collect::<Vec<bool>>(),
///     vec![true, true, true, true, false, false, false, false]
/// );
/// ```
pub trait BitIterator {
    /// iterator of bit values
    fn bit_iter(self) -> impl Iterator<Item = bool>;
}

impl BitIterator for &[u8] {
    fn bit_iter(self) -> impl Iterator<Item = bool> {
        self.iter()
            .flat_map(|&v| (0_u8..8).rev().map(move |n| (v & (1 << n)) != 0))
    }
}

/// Converting enumerated bool values to buffer  
///
/// # Examples
/// ```
/// # use bits::ToBits;
/// assert_eq!(
///     vec![true, true, true, true, false, false, false, false].iter().to_bits(),
///     [0b1111_0000]
/// );
/// ```
pub trait ToBits<T> {
    fn to_bits(self) -> Vec<u8>;
}

impl<U> ToBits<bool> for U
where
    U: Iterator<Item = bool>,
{
    fn to_bits(self) -> Vec<u8> {
        let mut v = 0_u8;
        self.chain([false; 7])
            .enumerate()
            .filter_map(|(i, bit)| {
                let n = 7 - (i % 8);
                if bit {
                    v |= 1 << n;
                }
                match n {
                    0 => Some(std::mem::take(&mut v)),
                    _ => None,
                }
            })
            .collect()
    }
}

impl<'a, U> ToBits<&bool> for U
where
    U: Iterator<Item = &'a bool>,
{
    fn to_bits(self) -> Vec<u8> {
        self.copied().to_bits()
    }
}

trait ByteWindow {
    // get window value from bytes. If insufficient, tail padding zero
    fn byte_window_64(&self, byte_index: usize) -> u64;
}

impl ByteWindow for [u8] {
    fn byte_window_64(&self, i: usize) -> u64 {
        const WINDOW_BYTES: usize = std::mem::size_of::<u64>();
        let mut bytes = [0; WINDOW_BYTES];
        match self.len() - i {
            WINDOW_BYTES.. => bytes.copy_from_slice(&self[i..i + WINDOW_BYTES]),
            n @ 1..WINDOW_BYTES => bytes[..n].copy_from_slice(&self[i..i + n]),
            _ => {}
        }
        u64::from_be_bytes(bytes)
    }
}

/// Returns the bits in the buffer grouped by n  
///
/// # Examples
/// ```
/// # use bits::BitChunks;
/// assert_eq!(
///     vec![0b1111_1111, 0b1111_1111].bit_chunks(6).collect::<Vec<u8>>(),
///     vec![0b11_1111, 0b11_1111, 0b11_1100]
/// );
/// assert_eq!(
///     vec![0b1111_1111; 3].bit_chunks(11).collect::<Vec<u16>>(),
///     vec![0b111_1111_1111, 0b111_1111_1111, 0b110_0000_0000]
/// );
/// ```
pub trait BitChunks {
    fn bit_chunks<T>(&self, n: usize) -> impl Iterator<Item = T>
    where
        T: TryFrom<u64> + Default;
}

impl BitChunks for [u8] {
    fn bit_chunks<T>(&self, n: usize) -> impl Iterator<Item = T>
    where
        T: TryFrom<u64> + Default,
    {
        let valid_size = (std::mem::size_of::<T>() * 8).min(32);
        assert!(
            0 < n && n <= valid_size,
            "[bits] Chunk size {n} overflow of: 1..={valid_size}",
        );
        // enumerate bytes window of 64 bits width, split item values from those windows
        const WINDOW_BYTES: usize = std::mem::size_of::<u64>();
        let bit_mask: u64 = (0..n).fold(0, |acc, v| acc | (1 << v));
        let mut bit_pos = 0;
        (0..self.len()).flat_map(move |i| {
            let window_value = self.byte_window_64(i);
            let window_end = i * 8 + WINDOW_BYTES * 8; // current bit window end
            debug_assert!((i * 8..window_end).contains(&bit_pos)); // current bit window start

            let mut vs = vec![];
            while (bit_pos + n) <= window_end && (bit_pos + n) < self.len() * 8 + n {
                bit_pos += n;
                let value = (window_value >> (window_end - bit_pos)) & bit_mask;
                vs.push(value.try_into().unwrap_or_default());
            }
            vs
        })
    }
}

#[derive(Debug)]
struct PartialBits {
    data: u8,
    len: usize,
}

impl PartialBits {
    pub fn new(data: u8, len: usize) -> Self {
        PartialBits { data, len }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn insert_prefix(&self, value: &mut u64) {
        if self.len > 0 {
            *value >>= self.len;
            *value |= (self.data as u64) << (64 - 8); // u64 left byte
        }
    }
    pub fn value(&self) -> Vec<u8> {
        match self.len > 0 {
            true => vec![self.data],
            false => vec![],
        }
    }
}

/// Conjoin the lowest n bits of each value  
///
/// # Examples
/// ```
/// # use bits::BitConjoin;
/// assert_eq!(
///     vec![0b11_1111_u8, 0b11_1111, 0b11_1111].bit_conjoin(6),
///     vec![0b1111_1111, 0b1111_1111, 0b1100_0000]
/// );
/// assert_eq!(
///     vec![0b1111_u16, 0b1111, 0b1111].bit_conjoin(6),
///     vec![0b001111_00, 0b1111_0011, 0b1100_0000]
/// );
/// ```
pub trait BitConjoin<T> {
    fn bit_conjoin(self, n: usize) -> Vec<u8>;
}

impl<T, U> BitConjoin<U> for U
where
    T: Into<u64>,
    U: Iterator<Item = T>,
{
    fn bit_conjoin(self, n: usize) -> Vec<u8> {
        assert!(
            matches!(n, 1..=32),
            "[bits] Conjoin size {n} overflow of: 1..=32"
        );
        const WINDOW_WIDTH: usize = 64;
        let bit_mask = (0..n).fold(0, |acc, v| acc | (1 << v));

        let mut rem = PartialBits::new(0, 0);
        let mut vs: Vec<u8> = self
            .map(|v| v.into())
            .flat_map(|mut value| {
                value &= bit_mask;
                value <<= WINDOW_WIDTH - n;
                rem.insert_prefix(&mut value);

                let partial = (n + rem.len()) / 8;
                let bytes = value.to_be_bytes();
                rem = PartialBits::new(bytes[partial], (n + rem.len()) % 8);
                bytes[..partial].to_vec()
            })
            .collect();
        vs.extend_from_slice(&rem.value());
        vs
    }
}

impl<'a, T, U> BitConjoin<&U> for U
where
    T: 'a + Into<u64> + Copy,
    U: Iterator<Item = &'a T>,
{
    fn bit_conjoin(self, n: usize) -> Vec<u8> {
        self.copied().bit_conjoin(n)
    }
}
impl<T> BitConjoin<T> for &[T]
where
    T: Into<u64> + Copy,
{
    #[inline]
    fn bit_conjoin(self, n: usize) -> Vec<u8> {
        self.iter().copied().bit_conjoin(n)
    }
}
