/*! Bit Operations
 *
 * # Examples
 * [u8] chunks to mnemonic indices by 11 bits.
 * [u8] chunks to base64 indices by 6 bits.
 */

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

pub trait ToBits<T> {
    fn to_bits(self) -> Vec<u8>;
}

impl<U: Iterator<Item = bool>> ToBits<bool> for U {
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

impl<'a, U: Iterator<Item = &'a bool>> ToBits<&bool> for U {
    #[inline]
    fn to_bits(self) -> Vec<u8> {
        self.map(|&v| v).to_bits()
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

pub trait BitChunks {
    fn bit_chunks<T>(self, n: usize) -> impl Iterator<Item = T>
    where
        T: TryFrom<u64> + Default;
}

impl BitChunks for &[u8] {
    fn bit_chunks<T>(self, n: usize) -> impl Iterator<Item = T>
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
                let value = ((window_value >> (window_end - bit_pos)) & bit_mask);
                vs.push(value.try_into().unwrap_or_default());
            }
            vs
        })
    }
}

pub trait BitConjoin<T> {
    fn bit_conjoin(self, n: usize) -> Vec<u8>;
}

impl<T, U: Iterator<Item = T>> BitConjoin<U> for U
where
    T: Into<u64>,
{
    fn bit_conjoin(mut self, n: usize) -> Vec<u8> {
        assert!(
            matches!(n, 1..=32),
            "[bits] Conjoin size {n} overflow of: 1..=32"
        );
        const WINDOW_BITS: usize = 64;
        let bit_mask = (0..n).fold(0, |acc, v| acc | (1 << v));

        let mut vs: Vec<u8> = vec![];
        let mut remainder: u64 = 0;
        let mut remainder_len: usize = 0;
        for mut v in self.map(|v| v.into()) {
            v &= bit_mask;
            v <<= WINDOW_BITS - (n + remainder_len);
            if remainder_len != 0 {
                v |= remainder << (WINDOW_BITS - remainder_len);
            }

            let partial = (n + remainder_len) / 8;
            let bytes = v.to_be_bytes()[..=partial].to_vec();
            vs.extend_from_slice(&bytes[..partial]);
            remainder_len = (n + remainder_len) % 8;
            if remainder_len > 0 {
                remainder = (bytes[partial] >> (8 - remainder_len)) as u64;
            } else {
                remainder = 0;
                assert_eq!(bytes[partial], 0);
            }
        }
        vs
    }
}

impl<'a, T, U: Iterator<Item = &'a T>> BitConjoin<&U> for U
where
    T: 'a + Into<u64> + Copy,
{
    #[inline]
    fn bit_conjoin(self, n: usize) -> Vec<u8> {
        self.map(|&v| v).bit_conjoin(n)
    }
}

impl<T> BitConjoin<T> for &[T]
where
    T: Into<u64> + Copy,
{
    #[inline]
    fn bit_conjoin(self, n: usize) -> Vec<u8> {
        self.iter().map(|&v| v).bit_conjoin(n)
    }
}
