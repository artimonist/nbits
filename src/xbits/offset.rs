/**
 * This module provides functions to shift bits in a byte array.
 */
pub trait BitOffset {
    /// Shift bits to the left
    /// # Parameters
    /// - `data`: The data to be shifted
    /// - `n`: The number of bits to shift
    /// # Returns
    /// - `true` if the leftmost `1` bits are overflowed
    /// # Examples
    /// ```
    /// use nbits::BitOffset;
    /// let mut data: [u8; 2] = [0b1111_1111, 0b0000_0000];
    /// assert_eq!(data.bits_shl_overflow(4), true);
    /// assert_eq!(data, [0b1111_0000, 0b0000_0000]);
    /// ```
    fn bits_shl_overflow(&mut self, n: usize) -> bool;

    /// Shift bits to the right
    /// # Parameters
    /// - `data`: The data to be shifted
    /// - `n`: The number of bits to shift
    /// # Returns
    /// - `true` if the rightmost `1` bits are overflowed
    /// # Examples
    /// ```
    /// use nbits::BitOffset;
    /// let mut data = [0b1111_1111, 0b0000_0000];
    /// assert_eq!(data.bits_shr_overflow(4), false);
    /// assert_eq!(data, [0b0000_1111, 0b1111_0000]);
    /// ```
    /// # Note
    /// The right shift will fill the leftmost bits with 0
    /// and the rightmost bits with the original value
    fn bits_shr_overflow(&mut self, n: usize) -> bool;
}

impl BitOffset for [u8] {
    fn bits_shl_overflow(&mut self, n: usize) -> bool {
        let len = self.len();
        let data = self;
        if n >= len * 8 {
            data.fill(0);
            return true;
        }

        let (n, m) = (n / 8, n % 8);
        data.copy_within(n.., 0);
        data[len - n..].fill(0);

        if m != 0 {
            let mut carry = 0;
            data.iter_mut().take(len - n).rev().for_each(|v| {
                (*v, carry) = ((*v << m) | carry, *v >> (8 - m));
            });
            return carry != 0;
        }
        false
    }

    fn bits_shr_overflow(&mut self, n: usize) -> bool {
        let len = self.len();
        let data = self;
        if n >= len * 8 {
            data.fill(0);
            return true;
        }

        let (n, m) = (n / 8, n % 8);
        data.copy_within(..len - n, n);
        data[..n].fill(0);

        if m != 0 {
            let mut carry = 0;
            data.iter_mut().skip(n).for_each(|v| {
                (*v, carry) = ((*v >> m) | carry, *v << (8 - m));
            });
            return carry != 0;
        }
        false
    }
}

#[cfg(test)]
mod test_offset {
    use super::BitOffset;

    #[test]
    fn test_bits_shl() {
        let mut data: [u8; 2] = [0b1111_1111, 0b0000_0000];
        assert_eq!(data.bits_shl_overflow(4), true);
        assert_eq!(data, [0b1111_0000, 0b0000_0000]);
    }

    #[test]
    fn test_bits_shr() {
        let mut data: [u8; 2] = [0b1111_1111, 0b0000_0000];
        assert_eq!(data.bits_shr_overflow(4), false);
        assert_eq!(data, [0b0000_1111, 0b1111_0000]);
        assert_eq!([0b1].bits_shr_overflow(1), true);
    }
}
