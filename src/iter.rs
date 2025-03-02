/*! Bit Operations
 *
 * # Examples
 * [u8] chunks to mnemonic indices by 11 bits.
 * [u8] chunks to base64 indices by 6 bits.
 */

pub trait BitIterator {
    /// iterator of bit values
    fn bit_iter(&self) -> impl Iterator<Item = bool>;
}

impl BitIterator for [u8] {
    fn bit_iter(&self) -> impl Iterator<Item = bool> {
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
    fn bit_chunks_8(&self, n: usize) -> impl Iterator<Item = u8>;
    fn bit_chunks_16(&self, n: usize) -> impl Iterator<Item = u16>;
    fn bit_chunks_32(&self, n: usize) -> impl Iterator<Item = u32>;
}

impl BitChunks for [u8] {
    #[inline]
    fn bit_chunks_8(&self, n: usize) -> impl Iterator<Item = u8> {
        assert!(0 < n && n <= 8, "[bits] Chunk size overflow: 0 < n <= 8");
        self.bit_chunks_32(n).map(|v| v as u8)
    }
    #[inline]
    fn bit_chunks_16(&self, n: usize) -> impl Iterator<Item = u16> {
        assert!(0 < n && n <= 16, "[bits] Chunk size overflow: 0 < n <= 16");
        self.bit_chunks_32(n).map(|v| v as u16)
    }
    fn bit_chunks_32(&self, n: usize) -> impl Iterator<Item = u32> {
        assert!(0 < n && n <= 32, "[bits] Chunk size overflow: 0 < n <= 32");
        // enumerate bytes window of 64 bits width, split item values from those windows
        const WINDOW_BYTES: usize = std::mem::size_of::<u64>();
        let bit_mask = (0..n).fold(0, |acc, v| acc | (1 << v));
        let mut bit_pos = 0;
        (0..self.len())
            .take_while(move |i| i * 8 + WINDOW_BYTES * 8 < self.len() * 8 + n)
            .flat_map(move |i| {
                let window_value = self.byte_window_64(i);
                let window_end = i * 8 + WINDOW_BYTES * 8; // current bit window end
                assert!((i * 8..window_end).contains(&bit_pos)); // current bit window start

                let mut vs = vec![];
                while (bit_pos + n) <= window_end {
                    bit_pos += n;
                    let value = ((window_value >> (window_end - bit_pos)) & bit_mask);
                    vs.push(value as u32);
                }
                vs
            })
    }
}

pub trait BitConjoin<T> {
    fn bit_conjoin(self, n: usize) -> Vec<u8>;
}

impl<U: Iterator<Item = u8>> BitConjoin<u8> for U {
    #[inline]
    fn bit_conjoin(self, n: usize) -> Vec<u8> {
        self.map(|v| v as u64).bit_conjoin(n)
    }
}

impl<U: Iterator<Item = u16>> BitConjoin<u16> for U {
    #[inline]
    fn bit_conjoin(self, n: usize) -> Vec<u8> {
        self.map(|v| v as u64).bit_conjoin(n)
    }
}

impl<U: Iterator<Item = u32>> BitConjoin<u32> for U {
    #[inline]
    fn bit_conjoin(self, n: usize) -> Vec<u8> {
        self.map(|v| v as u64).bit_conjoin(n)
    }
}

impl<U: Iterator<Item = u64>> BitConjoin<u64> for U {
    fn bit_conjoin(mut self, n: usize) -> Vec<u8> {
        assert!(
            0 < n && n <= 32,
            "[bits] Conjoin size overflow: 0 < n <= 32"
        );
        const WINDOW_BITS: usize = 64;
        let bit_mask = (0..n).fold(0, |acc, v| acc | (1 << v));

        let mut vs: Vec<u8> = vec![];
        let mut remainder: u64 = 0;
        let mut remainder_len: usize = 0;
        for mut v in self {
            println!("remainder: {remainder_len}, {remainder:b}");
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

#[cfg(test)]
mod bit_operation_test {
    use super::*;

    #[test]
    fn test_to_bits() {
        const MATRIX: &[[u8; 8]] = &[
            [1, 1, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 1, 1, 1],
            [1, 1, 0, 0, 0, 0, 1, 1],
            [0, 0, 1, 1, 1, 1, 0, 0],
        ];
        const INDICES: &[u8] = &[0b1111_0000, 0b0000_1111, 0b1100_0011, 0b0011_1100];
        let vs = (0..MATRIX.len()).flat_map(|row| (0..8).map(move |col| MATRIX[row][col] == 1));
        assert_eq!(vs.to_bits(), INDICES);

        let bits = [true, true, true, false, false, true].iter().to_bits();
        assert_eq!(bits, [0b1110_0100]);
        let bits = [true, true, true, false, false, true, true, true, true]
            .iter()
            .to_bits();
        assert_eq!(bits, [0b1110_0111, 0b1000_0000]);
    }
}
