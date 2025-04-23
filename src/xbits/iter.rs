/**
 * Bits iterator on [u8]
 */
pub trait BitIterator {
    /// Iterator bits
    /// # Examples
    /// ```
    /// # use nbits::Iterator;
    /// assert_eq!(
    ///   [0b1111_0000_u8].bit_iter().collect::<Vec<bool>>(),
    ///   vec![true, true, true, true, false, false, false, false]
    /// );
    /// ```
    fn bit_iter(&self) -> impl DoubleEndedIterator<Item = bool>;

    /// Convert enumerated bool values to buffer
    /// # Examples
    /// ```
    /// # use nbits::Iterator;
    /// let mut data = [0u8; 2];
    /// data.bit_from_iter([true, true, true, true, false, false, false, false].into_iter());
    /// assert_eq!(data, [0b1111_0000, 0b0000_0000]);
    /// ```
    fn bit_from_iter(&mut self, iter: impl Iterator<Item = bool>);

    fn bit_chunks<T>(&self, n: usize) -> impl Iterator<Item = T>
    where
        T: TryFrom<u64> + Default;

    fn bit_from_chunk<T>(&mut self, n: usize, chunks: impl Iterator<Item = T>)
    where
        T: TryInto<u64>;
}

impl BitIterator for [u8] {
    fn bit_iter(&self) -> impl DoubleEndedIterator<Item = bool> {
        self.iter()
            .flat_map(|&v| (0_u8..8).rev().map(move |n| (v & (1 << n)) != 0))
    }

    fn bit_from_iter(&mut self, iter: impl Iterator<Item = bool>) {
        self.fill(0);
        iter.take(self.len() * 8).enumerate().for_each(|(i, bit)| {
            if bit {
                let (n, m) = (i / 8, i % 8);
                self[n] |= 1 << (7 - m);
            }
        });
    }

    fn bit_chunks<T>(&self, _n: usize) -> impl Iterator<Item = T>
    where
        T: TryFrom<u64> + Default,
    {
        self.iter().take(0).map(|_| T::default())
    }

    fn bit_from_chunk<T>(&mut self, _n: usize, _chunks: impl Iterator<Item = T>)
    where
        T: TryInto<u64>,
    {
        todo!()
    }
}

// pub trait FromBits {
//     fn from_bits<U>(bits: U) -> Vec<u8>
//     where
//         U: Iterator<Item = bool>;

//     fn from_chunks<T, U>(chunks: U, n: usize) -> Vec<u8>
//     where
//         T: TryInto<u64>,
//         U: Iterator<Item = T>;
// }

// impl FromBits for Vec<u8> {
//     fn from_bits<U>(_bits: U) -> Vec<u8>
//     where
//         U: Iterator<Item = bool>,
//     {
//         todo!()
//     }

//     fn from_chunks<T, U>(_chunks: U, _n: usize) -> Vec<u8>
//     where
//         T: TryInto<u64>,
//         U: Iterator<Item = T>,
//     {
//         todo!()
//     }
// }

// impl<const N: usize> FromBits for [u8; N] {
//     fn from_bits<U>(_bits: U) -> Vec<u8>
//     where
//         U: Iterator<Item = bool>,
//     {
//         todo!()
//     }

//     fn from_chunks<T, U>(_chunks: U, _n: usize) -> Vec<u8>
//     where
//         T: TryInto<u64>,
//         U: Iterator<Item = T>,
//     {
//         todo!()
//     }
// }

#[cfg(test)]
mod test_iter {
    use super::*;

    #[test]
    fn test_chunks() {
        // Vec::from_chunks(chunks, n)
        //        <[u8; 8]>::from_chunks(vec![0b1111_0000_u8, 0b1111_0000_u8].into_iter(), 8);
    }
}
