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
    /// assert_eq!(data.bit_shl(4), true);
    /// assert_eq!(data, [0b1111_0000, 0b0000_0000]);
    /// ```
    fn bit_shl(&mut self, n: usize) -> bool;

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
    /// assert_eq!(data.bit_shr(4), false);
    /// assert_eq!(data, [0b0000_1111, 0b1111_0000]);
    /// ```
    /// # Note
    /// The right shift will fill the leftmost bits with 0
    /// and the rightmost bits with the original value
    fn bit_shr(&mut self, n: usize) -> bool;

    /// Reverse the bits of the data
    /// # Examples
    /// ```
    /// # use nbits::Bitwise;
    /// assert_eq!([0b0000_1111, 0b0000_0011].bit_reverse(), [0b1100_0000, 0b1111_0000]);
    /// ```
    fn bit_reverse(&mut self) -> &mut Self;

    /// Bitwise operator `!`
    /// # Examples
    /// ```
    /// # use nbits::Bitwise;
    /// assert_eq!([0b0000_1111, 0b0000_0011].bit_not(), [0b1111_0000, 0b1111_1100]);
    /// ```
    fn bit_not(&mut self) -> &mut Self;

    /// Bitwise operator `&` for big-endian
    /// # Examples
    /// ```
    /// # use nbits::Bitwise;
    /// assert_eq!([0b0011_0011, 0b0011_0011].bit_be_and(&[0b1111_1111]), [0b0000_0000, 0b0011_0011]);
    /// ```
    fn bit_be_and(&mut self, other: &Self) -> &mut Self;

    /// Bitwise operator `|` for big-endian
    /// # Examples
    /// ```
    /// # use nbits::Bitwise;
    /// assert_eq!([0b0011_0011, 0b0011_0011].bit_be_or(&[0b1111_1111]), [0b0011_0011, 0b1111_1111]);
    /// ```
    fn bit_be_or(&mut self, other: &Self) -> &mut Self;

    /// Bitwise operator `^` for big-endian
    /// # Examples
    /// ```
    /// # use nbits::Bitwise;
    /// assert_eq!([0b0011_0011, 0b0011_0011].bit_be_xor(&[0b1111_1111]), [0b0011_0011, 0b1100_1100]);
    /// ```
    fn bit_be_xor(&mut self, other: &Self) -> &mut Self;

    /// Bitwise operator `&` for little-endian
    /// # Examples
    /// ```
    /// # use nbits::Bitwise;
    /// assert_eq!([0b0011_0011, 0b0011_0011].bit_le_and(&[0b1111_1111]), [0b0011_0011, 0b0000_0000]);
    /// ```
    fn bit_le_and(&mut self, other: &Self) -> &mut Self;

    /// Bitwise operator `|` for little-endian
    /// # Examples
    /// ```
    /// # use nbits::Bitwise;
    /// assert_eq!([0b0011_0011, 0b0011_0011].bit_le_or(&[0b1111_1111]), [0b1111_1111, 0b0011_0011]);
    /// ```
    fn bit_le_or(&mut self, other: &Self) -> &mut Self;

    /// Bitwise operator `^` for little-endian
    /// # Examples
    /// ```
    /// # use nbits::Bitwise;
    /// assert_eq!([0b0011_0011, 0b0011_0011].bit_le_xor(&[0b1111_1111]), [0b1100_1100, 0b0011_0011]);
    /// ```
    fn bit_le_xor(&mut self, other: &Self) -> &mut Self;

    /// Bitwise comparison for big-endian
    /// # Examples
    /// ```
    /// # use nbits::Bitwise;
    /// # use std::cmp::Ordering;
    /// assert_eq!([0b0011_0011, 0b0011_0011].bit_be_cmp(&[0b1111_1111]), Ordering::Greater);
    /// assert_eq!([0b0011_0011, 0b0011_0011].bit_be_cmp(&[0b0000_0000, 0b1111_1111]), Ordering::Greater);
    /// assert_eq!([0b0011_0011, 0b0011_0011].bit_be_cmp(&[0b1111_1111, 0b0000_0000]), Ordering::Less);
    /// ```
    fn bit_be_cmp(&self, other: &Self) -> std::cmp::Ordering;

    /// Bitwise comparison for little-endian
    /// # Examples
    /// ```
    /// # use nbits::Bitwise;
    /// # use std::cmp::Ordering;
    /// assert_eq!([0b0011_0011, 0b0011_0011].bit_le_cmp(&[0b1111_1111]), Ordering::Greater);
    /// assert_eq!([0b0011_0011, 0b0011_0011].bit_le_cmp(&[0b0000_0000, 0b1111_1111]), Ordering::Less);
    /// assert_eq!([0b0011_0011, 0b0011_0011].bit_le_cmp(&[0b1111_1111, 0b0000_0000]), Ordering::Greater);
    /// ```
    fn bit_le_cmp(&self, other: &Self) -> std::cmp::Ordering;

    /// Check if all bits are zero
    fn bit_all_zero(&self) -> bool;

    /// Check if all bits are one
    fn bit_all_one(&self) -> bool;
}

impl Bitwise for [u8] {
    fn bit_shl(&mut self, n: usize) -> bool {
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

    fn bit_shr(&mut self, n: usize) -> bool {
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
    fn bit_reverse(&mut self) -> &mut Self {
        self.reverse();
        self.iter_mut().for_each(|a| *a = a.reverse_bits());
        self
    }

    #[inline]
    fn bit_not(&mut self) -> &mut Self {
        self.iter_mut().for_each(|a| *a = !*a);
        self
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
    fn bit_le_and(&mut self, other: &Self) -> &mut Self {
        self.iter_mut()
            .zip(other.iter().chain(std::iter::repeat(&0)))
            .for_each(|(a, b)| *a &= *b);
        self
    }

    #[inline]
    fn bit_le_or(&mut self, other: &Self) -> &mut Self {
        self.iter_mut()
            .zip(other.iter().chain(std::iter::repeat(&0)))
            .for_each(|(a, b)| *a |= *b);
        self
    }

    #[inline]
    fn bit_le_xor(&mut self, other: &Self) -> &mut Self {
        self.iter_mut()
            .zip(other.iter().chain(std::iter::repeat(&0)))
            .for_each(|(a, b)| *a ^= *b);
        self
    }

    fn bit_be_cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.len().cmp(&other.len()) {
            std::cmp::Ordering::Equal => self.iter().cmp(other.iter()),
            ord => ord,
        }
    }

    #[inline]
    fn bit_le_cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.len().cmp(&other.len()) {
            std::cmp::Ordering::Equal => self.iter().rev().cmp(other.iter().rev()),
            ord => ord,
        }
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
    fn test_bit_shl() {
        let mut data: [u8; 2] = [0b1111_1111, 0b0000_0000];
        assert_eq!(data.bit_shl(4), true);
        assert_eq!(data, [0b1111_0000, 0b0000_0000]);
    }

    #[test]
    fn test_bit_shr() {
        let mut data: [u8; 2] = [0b1111_1111, 0b0000_0000];
        assert_eq!(data.bit_shr(4), false);
        assert_eq!(data, [0b0000_1111, 0b1111_0000]);
        assert_eq!([0b1].bit_shr(1), true);
    }

    #[test]
    fn test_bit_cmp() {
        const A: [u8; 2] = [0b0011_0011, 0b0011_0011];
        const B: [u8; 2] = [0b1111_1111, 0b0000_0000];
        const C: [u8; 2] = [0b0000_0000, 0b1111_1111];

        let (a, b, c) = (
            u16::from_le_bytes(A),
            u16::from_le_bytes(B),
            u16::from_le_bytes(C),
        );
        println!("a: {a:016b} = {a}, b: {b:016b} = {b}, c: {c:016b} = {c}");
        assert_eq!(a.cmp(&b), A.as_ref().bit_le_cmp(&B));
        assert_eq!(a.cmp(&c), A.as_ref().bit_le_cmp(&C));

        let (a, b, c) = (
            u16::from_be_bytes(A),
            u16::from_be_bytes(B),
            u16::from_be_bytes(C),
        );
        println!("a: {a:016b} = {a}, b: {b:016b} = {b}, c: {c:016b} = {c}");
        assert_eq!(a.cmp(&b), A.as_ref().bit_be_cmp(&B));
        assert_eq!(a.cmp(&c), A.as_ref().bit_be_cmp(&C));
    }
}
