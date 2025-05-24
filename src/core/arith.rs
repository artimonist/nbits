use super::BitIterator;
use super::Bitwise;

/**
 * Arithmetic operations implementation for `[u8]`
 */
pub trait BitArith {
    type Other: ?Sized;

    /// Comparison for big-endian
    /// # Examples
    /// ```
    /// # use nbits::core::BitArith;
    /// # use std::cmp::Ordering;
    /// assert_eq!([0b0011_0011, 0b0011_0011].bit_be_cmp(&[0b1111_1111]), Ordering::Greater);
    /// assert_eq!([0b0000_0000, 0b0011_0011].bit_be_cmp(&[0b1111_1111]), Ordering::Less);
    /// assert_eq!([0b0011_0011, 0b0011_0011].bit_be_cmp(&[0b0000_0000, 0b1111_1111]), Ordering::Greater);
    /// assert_eq!([0b0011_0011, 0b0011_0011].bit_be_cmp(&[0b1111_1111, 0b0000_0000]), Ordering::Less);
    /// ```
    fn bit_be_cmp(&self, other: &Self) -> std::cmp::Ordering;

    /// Bit arithmetic operator `+=` for big-endian
    /// # Example
    /// ```
    /// # use nbits::core::BitArith;
    /// let (mut a, b) = ([0b1100_1100, 0b1000_0001], [0b1000_0001]);
    /// assert_eq!(a.as_mut().bit_be_add(&b), false);
    /// assert_eq!(a, [0b1100_1101, 0b0000_0010]);
    /// ```
    fn bit_be_add(&mut self, other: &Self::Other) -> bool;

    /// Bit arithmetic operator `-=` for big-endian
    /// # Example
    /// ```
    /// # use nbits::core::BitArith;
    /// let (mut a, b) = ([0b1100_1100, 0b1000_0001], [0b1000_0001]);
    /// assert_eq!(a.as_mut().bit_be_sub(&b), false);
    /// assert_eq!(a, [0b1100_1100, 0b0000_0000]);
    /// ```
    fn bit_be_sub(&mut self, other: &Self::Other) -> bool;

    /// Bit arithmetic operator `*=` for big-endian
    /// # Example
    /// ```
    /// # use nbits::core::BitArith;
    /// let (mut a, b) = ([0b0011_0000, 0b1000_0001], [0b0000_0010]);
    /// assert_eq!(a.as_mut().bit_be_mul(&b), false);
    /// assert_eq!(a, [0b0110_0001, 0b0000_0010]);
    /// ```
    fn bit_be_mul(&mut self, other: &Self::Other) -> bool;

    /// Bit arithmetic operator `/=` for big-endian
    /// # Example
    /// ```
    /// # use nbits::core::BitArith;
    /// let (a, b) = ([0b1100_0011, 0b0000_0001], [0b1000_0001]);
    /// let mut x = a.clone();
    /// x.as_mut().bit_be_div(&b);
    /// assert_eq!(x, (u16::from_be_bytes(a) / u16::from_be_bytes([0, b[0]])).to_be_bytes());
    /// ```
    fn bit_be_div(&mut self, other: &Self::Other) -> bool;

    /// Bit arithmetic operator `%=` for big-endian
    /// # Example
    /// ```
    /// # use nbits::core::BitArith;
    /// let (a, b) = ([0b1100_0011, 0b0000_0001], [0b0000_0001, 0b1000_0001]);
    /// let mut x = a.clone();
    /// x.as_mut().bit_be_rem(&b);
    /// // assert_eq!(x, (u16::from_be_bytes(a) % u16::from_be_bytes(b)).to_be_bytes());
    /// ```
    fn bit_be_rem(&mut self, other: &Self::Other) -> bool;
}

impl BitArith for [u8] {
    type Other = Self;

    fn bit_be_cmp(&self, other: &Self) -> std::cmp::Ordering {
        let max_len = std::cmp::max(self.len(), other.len());
        self.extend_be_iter(max_len)
            .cmp(other.extend_be_iter(max_len))
    }

    fn bit_be_add(&mut self, other: &Self) -> bool {
        self.iter_mut()
            .rev()
            .zip(other.iter().rev().chain(std::iter::repeat(&0)))
            .fold(false, |mut carry, (a, b)| {
                match (carry, *b) {
                    (true, 0xff) => carry = true,
                    (true, _) => (*a, carry) = a.overflowing_add(b + 1),
                    (false, _) => (*a, carry) = a.overflowing_add(*b),
                };
                carry
            })
    }

    fn bit_be_sub(&mut self, other: &Self) -> bool {
        self.iter_mut()
            .rev()
            .zip(other.iter().rev().chain(std::iter::repeat(&0)))
            .fold(false, |mut borrow, (a, b)| {
                match (borrow, *b) {
                    (true, 0xff) => borrow = true,
                    (true, _) => (*a, borrow) = a.overflowing_sub(b + 1),
                    (false, _) => (*a, borrow) = a.overflowing_sub(*b),
                };
                borrow
            })
    }

    fn bit_be_mul(&mut self, other: &Self) -> bool {
        let mut result = vec![0; self.len()];
        let mut overflow = false;
        for (i, bit) in other.bit_iter().rev().enumerate() {
            if bit {
                let mut multiple = self.to_vec();
                overflow |= multiple.bit_shl(i);
                overflow |= result.bit_be_add(&multiple);
            }
        }
        self.copy_from_slice(&result);
        overflow
    }

    fn bit_be_div(&mut self, other: &Self) -> bool {
        if other.iter().all(|&b| b == 0) {
            return true; // Division by zero, return overflow
        }

        // Ignore leading zeros
        let bits_a = self.len() * 8 - self.bit_leading_zeros(); // effective bits length
        let bits_b = other.len() * 8 - other.bit_leading_zeros(); // effective bits length
        if bits_a < bits_b {
            self.fill(0);
            return false;
        }

        let mut other = other.extend_be(self.len()); // extend to the same length
        {
            // Remove common trailing zeros
            let common_divisor_bits = self.bit_trailing_zeros().min(other.bit_trailing_zeros());
            self.bit_shr(common_divisor_bits);
            other.bit_shr(common_divisor_bits);
        }

        // Perform division
        let n = self.len();
        let mut result = vec![0; n];
        let diff = bits_a - bits_b;
        other.bit_shl(diff);
        for i in (0..=diff).rev() {
            if self.bit_be_cmp(&other) != std::cmp::Ordering::Less {
                self.bit_be_sub(&other);
                result[n - 1 - i / 8] |= 1 << (i % 8);
            }
            other.bit_shr(1);
        }

        self.copy_from_slice(&result);
        false
    }

    fn bit_be_rem(&mut self, other: &Self) -> bool {
        if other.iter().all(|&b| b == 0) {
            return true; // Division by zero, return overflow
        }

        // Ignore leading zeros
        let bits_a = self.len() * 8 - self.bit_leading_zeros(); // effective bits length
        let bits_b = other.len() * 8 - other.bit_leading_zeros(); // effective bits length
        if bits_a < bits_b {
            return false;
        }

        let mut other = other.extend_be(self.len()); // extend to the same length
        {
            // Remove common trailing zeros
            let common_divisor_bits = self.bit_trailing_zeros().min(other.bit_trailing_zeros());
            self.bit_shr(common_divisor_bits);
            other.bit_shr(common_divisor_bits);
        }

        // Perform division
        let n = self.len();
        let mut result = vec![0; n];
        let diff = bits_a - bits_b;
        other.bit_shl(diff);
        for i in (0..=diff).rev() {
            if self.bit_be_cmp(&other) != std::cmp::Ordering::Less {
                self.bit_be_sub(&other);
                result[n - 1 - i / 8] |= 1 << (i % 8);
            }
            other.bit_shr(1);
        }
        false
    }
}

trait ByteExtend {
    fn extend_be_iter(&self, n: usize) -> impl DoubleEndedIterator<Item = &u8>;

    fn extend_be(&self, n: usize) -> Vec<u8>;
}

impl ByteExtend for [u8] {
    #[inline(always)]
    fn extend_be_iter(&self, n: usize) -> impl DoubleEndedIterator<Item = &u8> {
        std::iter::repeat_n(&0, n - self.len()).chain(self.iter())
    }

    #[inline(always)]
    fn extend_be(&self, n: usize) -> Vec<u8> {
        let mut data = vec![0; n];
        if self.len() < n {
            data[n - self.len()..].copy_from_slice(self);
        } else {
            assert!(self[..self.len() - n].iter().all(|&b| b == 0));
            data.copy_from_slice(&self[self.len() - n..]);
        }
        data
    }
}

#[cfg(test)]
mod test_arith {
    use super::*;

    #[test]
    fn test_bits_add() {
        let mut a = [0b1111_1111, 0b1111_1111];
        assert_eq!(a.bit_be_add(&[0b0000_0001]), true);
        assert_eq!(a, [0b0000_0000, 0b0000_0000]);

        let mut a = [0b0000_0000, 0b0000_0001];
        assert_eq!(a.bit_be_add(&[0b1111_1111]), false);
        assert_eq!(a, [0b0000_0001, 0b0000_0000]);
    }

    #[test]
    fn test_bits_sub() {
        let mut a = [0b0000_0000, 0b0000_0001];
        assert_eq!(a.bit_be_sub(&[0b1111_1111]), true);
        assert_eq!(a, [0b1111_1111, 0b0000_0010]);

        let mut a = [0b1111_1111, 0b0000_0000];
        assert_eq!(a.bit_be_sub(&[0b0000_0001]), false);
        assert_eq!(a, [0b1111_1110, 0b1111_1111]);
    }

    #[test]
    fn test_bits_mul() {
        let mut a = [0xff, 0xff];
        assert_eq!(a.bit_be_mul(&[0b0000_0010]), true);
        assert_eq!(a, [0b1111_1111, 0b1111_1110]);

        let mut a = [0b0000_0001, 0b0000_0001];
        assert_eq!(a.bit_be_mul(&[0b1111_1111]), false);
        assert_eq!(a, [0b1111_1111, 0b1111_1111]);
    }

    pub trait BeValue {
        fn value(&self) -> u64;
    }

    impl BeValue for [u8] {
        fn value(&self) -> u64 {
            let mut bytes = [0; 8];
            bytes[8 - self.len()..].copy_from_slice(self);
            u64::from_be_bytes(bytes)
        }
    }

    #[test]
    fn test_bits_div() {
        const TDATA: &[(&[u8], &[u8], &[u8])] = &[
            (&[0b0000_1100], &[0b0000_0011], &[0b0000_0100]),
            (&[0b0011_0000, 0], &[0b0000_1100], &[0b0000_0100, 0]),
            (&[0b1100_1100], &[0, 0b0000_0011], &[0b0100_0100]),
        ];
        for (a, b, c) in TDATA {
            assert_eq!(a.value() / b.value(), c.value());
            let mut a = a.to_vec();
            assert_eq!(a.bit_be_div(b), false);
            assert_eq!(&a, c);
        }
    }
}
