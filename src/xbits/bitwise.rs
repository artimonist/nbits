/**
 * Bitwise operations for [u8]
 */
pub trait Bitwise {
    /// Bitwise operator `<<`
    /// Shift bits to the left
    /// # Parameters
    /// - `data`: The data to be shifted
    /// - `n`: The number of bits to shift
    /// # Returns
    /// - `true` if the leftmost `1` bits are overflowed
    /// # Examples
    /// ```
    /// # use nbits::Bitwise;
    /// let mut data: [u8; 2] = [0b1111_1111, 0b0000_0000];
    /// assert_eq!(data.bit_shl_overflow(4), true);
    /// assert_eq!(data, [0b1111_0000, 0b0000_0000]);
    /// ```
    fn bit_shl_overflow(&mut self, n: usize) -> bool;

    /// Bitwise operator `>>`
    /// Shift bits to the right
    /// # Parameters
    /// - `data`: The data to be shifted
    /// - `n`: The number of bits to shift
    /// # Returns
    /// - `true` if the rightmost `1` bits are overflowed
    /// # Examples
    /// ```
    /// # use nbits::Bitwise;
    /// let mut data = [0b1111_1111, 0b0000_0000];
    /// assert_eq!(data.bit_shr_overflow(4), false);
    /// assert_eq!(data, [0b0000_1111, 0b1111_0000]);
    /// ```
    /// # Note
    /// The right shift will fill the leftmost bits with 0
    /// and the rightmost bits with the original value
    fn bit_shr_overflow(&mut self, n: usize) -> bool;

    /// Bitwise operator `&`
    /// # Examples
    /// ```
    /// # use nbits::Bitwise;
    /// assert_eq!([0b0011_0011, 0b0011_0011].bit_be_and(&[0b1111_1111]), [0b0000_0000, 0b0011_0011]);
    /// ```
    fn bit_be_and(&mut self, other: &Self) -> &mut Self;

    /// Bitwise operator `|`
    /// # Examples
    /// ```
    /// # use nbits::Bitwise;
    /// assert_eq!([0b0011_0011, 0b0011_0011].bit_be_or(&[0b1111_1111]), [0b0011_0011, 0b1111_1111]);
    /// ```
    fn bit_be_or(&mut self, other: &Self) -> &mut Self;

    /// Bitwise operator `^`
    /// # Examples
    /// ```
    /// # use nbits::Bitwise;
    /// assert_eq!([0b0011_0011, 0b0011_0011].bit_be_xor(&[0b1111_1111]), [0b0011_0011, 0b1100_1100]);
    /// ```
    fn bit_be_xor(&mut self, other: &Self) -> &mut Self;

    /// Bitwise operator `!`
    /// # Examples
    /// ```
    /// # use nbits::Bitwise;
    /// assert_eq!([0b0000_1111, 0b0000_0011].bit_not(), [0b1111_0000, 0b1111_1100]);
    /// ```
    fn bit_not(&mut self) -> &mut Self;

    /// Reverse the bits of the data
    /// # Examples
    /// ```
    /// # use nbits::Bitwise;
    /// assert_eq!([0b0000_1111, 0b0000_0011].bit_reverse(), [0b1100_0000, 0b1111_0000]);
    /// ```
    fn bit_reverse(&mut self) -> &mut Self;

    /// Check if all bits are zero
    fn bit_all_zero(&self) -> bool;

    /// Check if all bits are one
    fn bit_all_one(&self) -> bool;
}

impl Bitwise for [u8] {
    fn bit_shl_overflow(&mut self, n: usize) -> bool {
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

    fn bit_shr_overflow(&mut self, n: usize) -> bool {
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

    #[inline]
    fn bit_be_and(&mut self, other: &Self) -> &mut Self {
        self.iter_mut()
            .rev()
            .zip(other.iter().rev().chain(std::iter::repeat(&0)))
            .for_each(|(a, b)| *a &= *b);
        self
    }

    #[inline]
    fn bit_be_or(&mut self, other: &Self) -> &mut Self {
        self.iter_mut()
            .rev()
            .zip(other.iter().rev())
            .for_each(|(a, b)| *a |= *b);
        self
    }

    #[inline]
    fn bit_be_xor(&mut self, other: &Self) -> &mut Self {
        self.iter_mut()
            .rev()
            .zip(other.iter().rev().chain(std::iter::repeat(&0)))
            .for_each(|(a, b)| {
                println!("a: {:08b}, b: {:08b}", a, b);
                println!("v: {:08b}", *a ^ *b);
                *a ^= *b
            });
        self
    }

    #[inline]
    fn bit_not(&mut self) -> &mut Self {
        self.iter_mut().for_each(|a| *a = !*a);
        self
    }

    #[inline]
    fn bit_reverse(&mut self) -> &mut Self {
        self.reverse();
        self.iter_mut().for_each(|a| *a = a.reverse_bits());
        self
    }

    #[inline]
    fn bit_all_zero(&self) -> bool {
        self.iter().all(|&b| b == 0)
    }

    #[inline]
    fn bit_all_one(&self) -> bool {
        self.iter().all(|&b| b == 0xff)
    }
}

#[cfg(test)]
mod test_offset {
    use super::Bitwise;

    #[test]
    fn test_bits_shl() {
        let mut data: [u8; 2] = [0b1111_1111, 0b0000_0000];
        assert_eq!(data.bit_shl_overflow(4), true);
        assert_eq!(data, [0b1111_0000, 0b0000_0000]);
    }

    #[test]
    fn test_bits_shr() {
        let mut data: [u8; 2] = [0b1111_1111, 0b0000_0000];
        assert_eq!(data.bit_shr_overflow(4), false);
        assert_eq!(data, [0b0000_1111, 0b1111_0000]);
        assert_eq!([0b1].bit_shr_overflow(1), true);
    }
}
