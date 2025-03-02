#![allow(unused)]
/*! Bit Operations
 *
 * # Examples
 * [u8] chunks to mnemonic indices by 11 bits.
 * [u8] chunks to base64 indices by 6 bits.
 */

use std::slice::Windows;

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

pub trait BitFrom {
    /// compound bits to u8 vector
    fn bit_from(iter: impl Iterator<Item = bool>) -> Vec<u8>;
}

impl BitFrom for Vec<u8> {
    fn bit_from(iter: impl Iterator<Item = bool>) -> Vec<u8> {
        let mut v = 0_u8;
        iter.chain([false; 7])
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

pub trait ToBits {
    fn to_bits(self) -> Vec<u8>;
}

impl<'a, U: Iterator<Item = &'a bool>> ToBits for U {
    fn to_bits(self) -> Vec<u8> {
        let mut v = 0_u8;
        self.chain([&false; 7])
            .enumerate()
            .filter_map(|(i, bit)| {
                let n = 7 - (i % 8);
                if *bit {
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
    fn bit_chunks_32(&self, n: usize) -> impl Iterator<Item = u32>;
}

impl BitChunks for [u8] {
    fn bit_chunks_32(&self, n: usize) -> impl Iterator<Item = u32> {
        const WINDOW_BYTES: usize = std::mem::size_of::<u64>();
        assert!(n <= 32);

        // enumerate bytes window of 64 bits width, split item values from those windows
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

#[cfg(test)]
mod bit_operation_test {
    use super::*;

    #[test]
    fn test_bit_chunks() {
        use bitcoin::{hashes::Hash, hex::FromHex};
        {
            /// 01010001011 10100101110 11000111011 10111011111 11000110111 00010111111 10111101001 11001000111 11011111011 01111110011 00000001000 11010100100 10001101000 11001010110 111001
            const ENTROPY_15: &str = "5174bb1dddfc6e2fef4e47df6fcc046a48d195b9";
            const INDICES_15: &[u32] = &[
                651, 1326, 1595, 1503, 1591, 191, 1513, 1607, 1787, 1011, 8, 1700, 1128, 1622, 1842,
            ];
            let mut data = Vec::from_hex(ENTROPY_15).expect("ENTROPY_15");
            let check = bitcoin::hashes::sha256::Hash::hash(&data).as_byte_array()[0];
            data.extend([check]);
            let indices = data.bit_chunks_32(11).collect::<Vec<_>>();
            assert_eq!(&indices[..15], INDICES_15);
        }
        {
            /// 11011000100 01001010110 00110011000 00000101111 00001001100 11001010011 01110000001 00000010110 01000001000 11100110000 00101100011 00100011001 01011010101 01111100110 00011110101 10110101101 10100101101 01011010001 00111000011 00111111110 01011011100 01100010011 00010010001 011
            const ENTROPY_24: &str =
                "d88958cc02f09994dc0816411cc0b19195aaf987adada5ab44e19fe5b8c4c48b";
            const INDICES_24: &[u32] = &[
                1732, 598, 408, 47, 76, 1619, 897, 22, 520, 1840, 355, 281, 725, 998, 245, 1453,
                1325, 721, 451, 510, 732, 787, 145, 844,
            ];
            let data = Vec::from_hex(ENTROPY_24).expect("ENTROPY_24");
            let check = bitcoin::hashes::sha256::Hash::hash(&data).as_byte_array()[0];
            let entropy = [data, vec![check]].concat();
            let indices = entropy.bit_chunks_32(11).collect::<Vec<_>>();
            assert_eq!(&indices, INDICES_24);
        }
    }

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
        assert_eq!(Vec::bit_from(vs), INDICES);

        let bits = [true, true, true, false, false, true].iter().to_bits();
        assert_eq!(bits, [0b1110_0100]);
        let bits = [true, true, true, false, false, true, true, true, true]
            .iter()
            .to_bits();
        assert_eq!(bits, [0b1110_0111, 0b1000_0000]);
    }
}
