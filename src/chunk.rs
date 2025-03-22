use crate::assert_overflow;

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
/// # use nbits::BitChunks;
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
    /// Returns the bits in the buffer grouped by n
    /// # Parameters
    /// - `T`: the type to contains the grouped bits
    /// - `n`: the number of bits to group
    /// - 1 <= n <= size_of::<T>() * 8 <= 32
    fn bit_chunks<T>(&self, n: usize) -> impl Iterator<Item = T>
    where
        T: TryFrom<u64> + Default;

    /// Returns the bits in the buffer grouped by n, and the remaining bits count
    /// # Parameters
    /// - `T`: the type to contains the grouped bits
    /// - `n`: the number of bits to group
    /// - 1 <= n <= size_of::<T>() * 8 <= 32
    /// - `remain`: the remaining bits count = n - (buffer.len() * 8 % n)
    fn bit_chunks_rem<T>(&self, n: usize, remain: &mut usize) -> impl Iterator<Item = T>
    where
        T: TryFrom<u64> + Default;
}

impl BitChunks for [u8] {
    fn bit_chunks<T>(&self, n: usize) -> impl Iterator<Item = T>
    where
        T: TryFrom<u64> + Default,
    {
        let valid_size = (std::mem::size_of::<T>() * 8).min(32);
        assert_overflow!(n, 1, valid_size, "Chunks");

        // enumerate bytes window of 64 bits width, split item values from those windows
        const WINDOW_BITS: usize = std::mem::size_of::<u64>() * 8;
        let bit_mask: u64 = (0..n).fold(0, |acc, v| acc | (1 << v));
        let mut bit_pos = 0;
        (0..self.len()).flat_map(move |i| {
            let window_value = self.byte_window_64(i);
            let window_end = i * 8 + WINDOW_BITS; // current bit window end
            debug_assert!(matches!(window_end - bit_pos, 1..=64)); // current bit window size

            let mut vs = vec![];
            while (bit_pos + n) <= window_end && (bit_pos + n) < self.len() * 8 + n {
                bit_pos += n;
                let value = (window_value >> (window_end - bit_pos)) & bit_mask;
                vs.push(value.try_into().unwrap_or_default());
            }
            vs
        })
    }

    fn bit_chunks_rem<T>(&self, n: usize, remain: &mut usize) -> impl Iterator<Item = T>
    where
        T: TryFrom<u64> + Default,
    {
        *remain = n - (self.len() * 8 % n);
        self.bit_chunks(n)
    }
}

/// 0~7 bits data
#[derive(Debug)]
struct Bits {
    data: u8,
    len: usize,
}

impl Bits {
    pub fn new(data: u8, len: usize) -> Self {
        Bits { data, len }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn value(&self) -> Vec<u8> {
        match self.len > 0 {
            true => vec![self.data],
            false => vec![],
        }
    }
    /// insert bits as value prefix
    pub fn prefix_to(&self, value: &mut u64) {
        if self.len > 0 {
            *value >>= self.len;
            *value |= (self.data as u64) << (64 - 8); // u64 left byte
        }
    }
}

/// Conjoin the lowest n bits of each value  
///
/// # Examples
/// ```
/// # use nbits::BitConjoin;
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
    /// Conjoin the lowest n bits of each value
    /// # Parameters
    /// - `n`: the number of bits to conjoin
    /// - 1 <= n <= 32
    /// - if n > size_of::<T>() * 8, left padding zero bits
    fn bit_conjoin(self, n: usize) -> Vec<u8>;

    /// Conjoin the lowest n bits of each value, and the remaining bits count
    /// # Parameters
    /// - `n`: the number of bits to conjoin
    /// - 1 <= n <= 32
    /// - if n > size_of::<T>() * 8, left padding zero bits
    /// - `remain`: the remaining bits count to ignore
    fn bit_conjoin_rem(self, n: usize, remain: usize) -> Vec<u8>
    where
        Self: Sized,
    {
        assert_overflow!(remain, 0, n - 1, "Conjoin");
        let mut vs = self.bit_conjoin(n);
        let tail_ignore_bytes = (vs.len() * 8 % n + remain) / 8;
        vs.truncate(vs.len() - tail_ignore_bytes);
        vs
    }
}

impl<T, U> BitConjoin<U> for U
where
    T: TryInto<u64>,
    U: Iterator<Item = T>,
{
    fn bit_conjoin(self, n: usize) -> Vec<u8> {
        assert_overflow!(n, 1, 32, "Conjoin");
        const WINDOW_BITS: usize = 64;
        let bit_mask: u64 = (0..n).fold(0, |acc, v| acc | (1 << v));

        let mut rem = Bits::new(0, 0);
        let mut vs: Vec<u8> = self
            .map(|v| v.try_into().unwrap_or_default())
            .flat_map(|mut value: u64| {
                value &= bit_mask;
                value <<= WINDOW_BITS - n;
                rem.prefix_to(&mut value);

                let partial = (n + rem.len()) / 8;
                let bytes = value.to_be_bytes();
                rem = Bits::new(bytes[partial], (n + rem.len()) % 8);
                bytes[..partial].to_vec()
            })
            .collect();
        vs.extend_from_slice(&rem.value());
        vs
    }
}

impl<'a, T, U> BitConjoin<&U> for U
where
    T: 'a + TryInto<u64> + Copy,
    U: Iterator<Item = &'a T>,
{
    #[inline]
    fn bit_conjoin(self, n: usize) -> Vec<u8> {
        self.copied().bit_conjoin(n)
    }
}

impl<T> BitConjoin<T> for &[T]
where
    T: TryInto<u64> + Copy,
{
    #[inline]
    fn bit_conjoin(self, n: usize) -> Vec<u8> {
        self.iter().copied().bit_conjoin(n)
    }
}
