/**
 * Bits iterator on [u8]
 */
pub trait BitIterator {
    /// Iterator bits
    /// # Examples
    /// ```
    /// # use nbits::Iterator;
    /// assert_eq!(
    ///   [0b1111_0000_u8].bits_iter().collect::<Vec<bool>>(),
    ///   vec![true, true, true, true, false, false, false, false]
    /// );
    /// ```
    fn bits_iter(&self) -> impl DoubleEndedIterator<Item = bool>;

    /// Convert enumerated bool values to buffer
    /// # Examples
    /// ```
    /// # use nbits::Iterator;
    /// let mut data = [0u8; 2];
    /// data.bits_from_iter([true, true, true, true, false, false, false, false].into_iter());
    /// assert_eq!(data, [0b0000_0000, 0b1111_0000]);
    /// ```
    fn bits_from_iter(&mut self, iter: impl DoubleEndedIterator<Item = bool>);
}

impl BitIterator for [u8] {
    fn bits_iter(&self) -> impl DoubleEndedIterator<Item = bool> {
        self.iter()
            .flat_map(|&v| (0_u8..8).rev().map(move |n| (v & (1 << n)) != 0))
    }

    fn bits_from_iter(&mut self, iter: impl DoubleEndedIterator<Item = bool>) {
        iter.rev()
            .chain(std::iter::repeat(false))
            .take(self.len() * 8)
            .enumerate()
            .for_each(|(i, bit)| {
                let (n, m) = (i / 8, i % 8);
                match bit {
                    true => self[n] |= 1 << m,
                    false => self[n] &= !(1 << m),
                }
            });
        self.reverse();
    }
}
