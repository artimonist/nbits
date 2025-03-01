#![allow(unused)]
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

/// # Parameters
///   T: value type
///   U: window type
pub trait BitChunks<T> {
    fn bit_chunks(&self, n: usize) -> impl Iterator<Item = T>;
}

const TAIL_PADDING: [u8; 8] = [0; 8];

impl BitChunks<u16> for [u8] {
    fn bit_chunks(&self, n: usize) -> impl Iterator<Item = u16> {
        type T = u16; // value type
        type U = u64; // window type
        const VALUE_SIZE: usize = std::mem::size_of::<T>();
        const WINDOW_SIZE: usize = std::mem::size_of::<U>();
        assert!(n <= VALUE_SIZE);
        let bit_mask = (0..n).fold(0, |acc, v| acc | (1 << v));

        let mut start = 0;
        let mut end_max = match self.len() * 8 % n {
            0 => self.len() * 8,
            remainder => self.len() * 8 + (n - remainder),
        };
        self.windows(WINDOW_SIZE)
            .chain(TAIL_PADDING.windows(WINDOW_SIZE))
            .enumerate()
            .flat_map(move |(i, window)| {
                let end = end_max.min(i * 8 + WINDOW_SIZE); // current bit window end
                assert!(i * 8 <= start && start < end); // current bit window start
                let ref window_value: U = unsafe { window.align_to().1[0] }; // window value

                let vs: Vec<_> = (0..WINDOW_SIZE)
                    .take_while(|j| start + j * n < end)
                    .map(|j| (window_value >> (end - start) & bit_mask) as T)
                    .collect();
                start += vs.len() * n;

                // let mut vs = vec![];
                // while (start + n) < end {
                //     start += n;
                //     let v = (window_value >> (end - start) & bit_mask) as T;
                //     vs.push(v);
                // }
                vs
            })
        // (0..self.len()).flat_map(move |i| {
        //     assert!(i * 8 <= start && start <= i * 8 + WINDOW_SIZE);
        //     let mut vs = vec![];
        //     let end = i * 8 + WINDOW_SIZE; // bit window end
        //     while (start + n) <= end && end < (self.len() * 8 + n) {
        //         start += n;
        //         let v: U = self.bit_value(i);
        //         let value = (v >> (end - start) & bit_mask) as T;
        //         vs.push(value);
        //     }
        //     vs
        // })
    }
}

pub trait BitConjoin {
    fn bit_conjoin(self, n: usize) -> Vec<u8>;
}

impl<'a, U: Iterator<Item = &'a u16>> BitConjoin for U {
    fn bit_conjoin(self, n: usize) -> Vec<u8> {
        const SIZE: usize = std::mem::size_of::<u8>();
        assert!(n < SIZE);
        self.filter_map(|_| Some(0_u8)).collect()
    }
}

// impl BitConjoin for &[u16] {
//     fn bit_conjoin(&self, n: usize) -> Vec<u8> {
//         const SIZE: usize = std::mem::size_of::<u16>();
//         assert!(n < SIZE);
//         todo!()
//     }
// }

// impl BitConjoin for &[u32] {
//     fn bit_conjoin(&self, n: usize) -> Vec<u8> {
//         todo!()
//     }
// }

// impl BitChunks<u32> for [u8] {
//     fn bits_chunks(&self, n: usize) -> impl Iterator<Item = u32> {
//         type T = u32;
//         type U = u64;
//         const VALUE_SIZE: usize = std::mem::size_of::<T>();
//         const WINDOW_SIZE: usize = std::mem::size_of::<U>();
//         assert!(n <= VALUE_SIZE);
//         let bit_mask = (0..n).fold(0, |acc, v| acc | (1 << v));

//         let mut start = 0; // bit window start
//         (0..self.len()).flat_map(move |i| {
//             assert!(i * 8 <= start && start <= i * 8 + WINDOW_SIZE);
//             let mut vs = vec![];
//             let end = i * 8 + WINDOW_SIZE; // bit window end
//             while (start + n) <= end && end < (self.len() * 8 + n) {
//                 start += n;
//                 let v: U = self.bit_value(i);
//                 let value = (v >> (end - start) & bit_mask) as T;
//                 vs.push(value);
//             }
//             vs
//         })
//     }
// }

// pub trait BitJoin {
//     /// join bits to buffer
//     fn bit_join(&self, n: usize) -> Vec<u8>;
// }

// impl BitJoin for [u8] {
//     fn bit_join(&self, n: usize) -> Vec<u8> {
//         assert!(n < 8);
//         let mut result = Vec::new();
//         let mut current_byte = 0_u8;
//         let mut bits_filled = 0;

//         for chunk in self.bit_chunks(n) {
//             let chunk = chunk as u8;
//             let remaining_bits = 8 - bits_filled;

//             if n <= remaining_bits {
//                 current_byte |= chunk << (remaining_bits - n);
//                 bits_filled += n;
//             } else {
//                 current_byte |= chunk >> (n - remaining_bits);
//                 result.push(current_byte);
//                 current_byte = chunk << (8 - (n - remaining_bits));
//                 bits_filled = n - remaining_bits;
//             }

//             if bits_filled == 8 {
//                 result.push(current_byte);
//                 current_byte = 0;
//                 bits_filled = 0;
//             }
//         }

//         if bits_filled > 0 {
//             result.push(current_byte);
//         }

//         result
//     }
// }
#[cfg(test)]
mod bit_operation_test {
    use super::*;

    // #[test]
    // fn test_bit_chunks() {
    //     use bitcoin::{hashes::Hash, hex::FromHex};
    //     {
    //         /// 01010001011 10100101110 11000111011 10111011111 11000110111 00010111111 10111101001 11001000111 11011111011 01111110011 00000001000 11010100100 10001101000 11001010110 111001
    //         const ENTROPY_15: &str = "5174bb1dddfc6e2fef4e47df6fcc046a48d195b9";
    //         const INDICES_15: &[u16] = &[
    //             651, 1326, 1595, 1503, 1591, 191, 1513, 1607, 1787, 1011, 8, 1700, 1128, 1622, 1842,
    //         ];
    //         let mut data = Vec::from_hex(ENTROPY_15).expect("ENTROPY_15");
    //         let check = bitcoin::hashes::sha256::Hash::hash(&data).as_byte_array()[0];
    //         data.extend([check]);
    //         let indices = data.bit_chunks(11).collect::<Vec<_>>();
    //         assert_eq!(&indices[..15], INDICES_15);
    //     }
    //     {
    //         /// 11011000100 01001010110 00110011000 00000101111 00001001100 11001010011 01110000001 00000010110 01000001000 11100110000 00101100011 00100011001 01011010101 01111100110 00011110101 10110101101 10100101101 01011010001 00111000011 00111111110 01011011100 01100010011 00010010001 011
    //         const ENTROPY_24: &str =
    //             "d88958cc02f09994dc0816411cc0b19195aaf987adada5ab44e19fe5b8c4c48b";
    //         const INDICES_24: &[u16] = &[
    //             1732, 598, 408, 47, 76, 1619, 897, 22, 520, 1840, 355, 281, 725, 998, 245, 1453,
    //             1325, 721, 451, 510, 732, 787, 145, 844,
    //         ];
    //         let data = Vec::from_hex(ENTROPY_24).expect("ENTROPY_24");
    //         let check = bitcoin::hashes::sha256::Hash::hash(&data).as_byte_array()[0];
    //         let entropy = [data, vec![check]].concat();
    //         let indices = entropy.bit_chunks(11).collect::<Vec<_>>();
    //         assert_eq!(&indices, INDICES_24);
    //     }
    // }

    // #[test]
    // fn test_to_bits() {
    //     const MATRIX: &[[u8; 8]] = &[
    //         [1, 1, 1, 1, 0, 0, 0, 0],
    //         [0, 0, 0, 0, 1, 1, 1, 1],
    //         [1, 1, 0, 0, 0, 0, 1, 1],
    //         [0, 0, 1, 1, 1, 1, 0, 0],
    //     ];
    //     const INDICES: &[u8] = &[0b1111_0000, 0b0000_1111, 0b1100_0011, 0b0011_1100];
    //     let vs = (0..MATRIX.len())
    //         .flat_map(|row| (0..8).map(move |col| MATRIX[row][col] == 1))
    //         .to_bits();
    //     assert_eq!(vs, INDICES);

    //     let bits = [true, true, true, false, false, true].into_iter().to_bits();
    //     assert_eq!(bits, [0b1110_0100]);
    //     let bits = [true, true, true, false, false, true, true, true, true]
    //         .into_iter()
    //         .to_bits();
    //     assert_eq!(bits, [0b1110_0111, 0b1000_0000]);
    // }
}
