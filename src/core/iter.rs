use crate::assert_overflow;

/**
 * Bits iterator implementation on `[u8]`
 */
pub trait BitIterator {
    /// Iterator bits
    /// # Examples
    /// ```
    /// # use nbits::core::BitIterator;
    /// assert_eq!(
    ///   [0b1111_0000_u8].bit_iter().collect::<Vec<bool>>(),
    ///   vec![true, true, true, true, false, false, false, false]
    /// );
    /// ```
    fn bit_iter(&self) -> impl DoubleEndedIterator<Item = bool>;

    /// Returns the bits in the buffer grouped by n  
    ///
    /// # Parameters  
    /// - `T`: the type to contains the grouped bits  
    /// - `n`: the number of bits to group  
    /// - 1 <= n <= T::BITS <= 32
    ///   
    /// # Examples  
    /// ```
    /// # use nbits::core::BitIterator;
    /// assert_eq!(
    ///     vec![0b1111_1111, 0b1111_1111].bit_chunks(6).collect::<Vec<u8>>(),
    ///     vec![0b11_1111, 0b11_1111, 0b11_1100]
    /// );
    /// assert_eq!(
    ///     vec![0b1111_1111; 3].bit_chunks(11).collect::<Vec<u16>>(),
    ///     vec![0b111_1111_1111, 0b111_1111_1111, 0b110_0000_0000]
    /// );
    /// ```
    fn bit_chunks<T>(&self, n: usize) -> impl Iterator<Item = T>
    where
        T: TryFrom<u64> + Default;
}

impl BitIterator for [u8] {
    fn bit_iter(&self) -> impl DoubleEndedIterator<Item = bool> {
        self.iter()
            .flat_map(|&v| (0_u8..8).rev().map(move |n| (v & (1 << n)) != 0))
    }

    fn bit_chunks<T>(&self, n: usize) -> impl Iterator<Item = T>
    where
        T: TryFrom<u64> + Default,
    {
        let valid_size = (std::mem::size_of::<T>() * 8).min(32);
        assert_overflow!(n, 1, valid_size, "bit_chunks");

        // enumerate bytes window of 64 bits width, split item values from those windows
        let bit_mask: u64 = (0..n).fold(0, |acc, v| acc | (1 << v));
        let mut bit_pos = 0;
        (0..self.len()).flat_map(move |i| {
            let window_value = self.byte_window_64(i);
            let window_end = i * 8 + u64::BITS as usize; // current bit window end
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
}

pub trait FromBits {
    /// Convert enumerated bool values to buffer
    /// # Examples
    /// ```
    /// # use nbits::FromBits;
    /// let bits = vec![false, false, false, false, true, true, true, true, false];
    /// let data = Vec::from_bits(bits.into_iter());
    /// assert_eq!(data, [0b0000_1111, 0b0000_0000]);
    /// ```
    fn from_bits<U>(bits: U) -> Self
    where
        U: Iterator<Item = bool>;

    /// Conjoin the lowest n bits of each value  
    ///
    /// # Parameters
    /// - `n`: the number of bits to conjoin
    /// - 1 <= n <= 32
    /// - if n > T::BITS, left padding zero bits
    ///
    /// # Examples
    /// ```
    /// # use nbits::FromBits;
    /// assert_eq!(
    ///     Vec::from_bits_chunk([0b11_1111_u8, 0b11_1111, 0b11_1111].into_iter(),6),
    ///     vec![0b1111_1111, 0b1111_1111, 0b1100_0000]
    /// );
    /// assert_eq!(
    ///     Vec::from_bits_chunk([0b1111_u16, 0b1111, 0b1111].into_iter(), 6),
    ///     vec![0b001111_00, 0b1111_0011, 0b1100_0000]
    /// );
    /// ```
    fn from_bits_chunk<T, U>(chunks: U, n: usize) -> Self
    where
        T: TryInto<u64>,
        U: Iterator<Item = T>;
}

impl FromBits for Vec<u8> {
    fn from_bits<U>(bits: U) -> Self
    where
        U: Iterator<Item = bool>,
    {
        let mut v = 0_u8;
        bits.chain([false; 7])
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

    fn from_bits_chunk<T, U>(chunks: U, n: usize) -> Self
    where
        T: TryInto<u64>,
        U: Iterator<Item = T>,
    {
        assert_overflow!(n, 1, 32, "from_chunks");
        let bit_mask: u64 = (0..n).fold(0, |acc, v| acc | (1 << v));

        let mut rem = TinyBits::new(0, 0);
        let mut vs: Vec<u8> = chunks
            .map(|v| v.try_into().unwrap_or_default())
            .flat_map(|mut value: u64| {
                value &= bit_mask;
                value <<= u64::BITS as usize - n;
                rem.prefix_to(&mut value);

                let partial = (n + rem.len()) / 8;
                let bytes = value.to_be_bytes();
                rem = TinyBits::new(bytes[partial], (n + rem.len()) % 8);
                bytes[..partial].to_vec()
            })
            .collect();
        vs.extend_from_slice(&rem.value());
        vs
    }
}

trait ByteWindow {
    // get window value from bytes. If insufficient, tail padding zero
    fn byte_window_64(&self, byte_index: usize) -> u64;
}

impl ByteWindow for [u8] {
    fn byte_window_64(&self, i: usize) -> u64 {
        let bytes = self[i..]
            .iter()
            .copied()
            .chain(std::iter::repeat(0))
            .take(8)
            .collect::<Vec<u8>>();
        u64::from_be_bytes(bytes.try_into().unwrap())
    }
}

/// 0~7 bits data
#[derive(Debug)]
struct TinyBits {
    data: u8,
    len: usize,
}

impl TinyBits {
    pub fn new(data: u8, len: usize) -> Self {
        TinyBits { data, len }
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
