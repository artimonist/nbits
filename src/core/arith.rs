use super::Bitwise;
use super::Iterator;

/**
 * Arithmetic operations for [u8]
 */
pub trait Arithmetic {
    type Other: ?Sized;

    /// Bit arithmetic operator `+=` for big-endian
    /// # Example
    /// ```
    /// # use nbits::Arithmetic;
    /// let (mut a, b) = ([0b1100_1100, 0b1000_0001], [0b1000_0001]);
    /// assert_eq!(a.as_mut().bit_be_add(&b), false);
    /// assert_eq!(a, [0b1100_1101, 0b0000_0010]);
    /// ```
    fn bit_be_add(&mut self, other: &Self::Other) -> bool;

    /// Bit arithmetic operator `+=` for little-endian
    /// # Example
    /// ```
    /// # use nbits::Arithmetic;
    /// let (mut a, b) = ([ 0b1000_0001, 0b1100_1100], [0b1000_0001]);
    /// assert_eq!(a.as_mut().bit_le_add(&b), false);
    /// assert_eq!(a, [0b0000_0010, 0b1100_1101]);
    /// ```
    fn bit_le_add(&mut self, other: &Self::Other) -> bool;

    /// Bit arithmetic operator `-=` for big-endian
    /// # Example
    /// ```
    /// # use nbits::Arithmetic;
    /// let (mut a, b) = ([0b1100_1100, 0b1000_0001], [0b1000_0001]);
    /// assert_eq!(a.as_mut().bit_be_sub(&b), false);
    /// assert_eq!(a, [0b1100_1100, 0b0000_0000]);
    /// ```
    fn bit_be_sub(&mut self, other: &Self::Other) -> bool;

    /// Bit arithmetic operator `-=` for little-endian
    /// # Example
    /// ```
    /// # use nbits::Arithmetic;
    /// let (mut a, b) = ([0b1000_0001, 0b0000_0001], [0b1000_0010]);
    /// assert_eq!(a.as_mut().bit_le_sub(&b), false);
    /// assert_eq!(a, [0b1111_1111, 0b0000_0000]);
    /// ```
    fn bit_le_sub(&mut self, other: &Self::Other) -> bool;

    /// Bit arithmetic operator `*=` for big-endian
    /// # Example
    /// ```
    /// # use nbits::Arithmetic;
    /// let (mut a, b) = ([0b0011_0000, 0b1000_0001], [0b0000_0010]);
    /// assert_eq!(a.as_mut().bit_be_mul(&b), false);
    /// assert_eq!(a, [0b0110_0001, 0b0000_0010]);
    /// ```
    fn bit_be_mul(&mut self, other: &Self::Other) -> bool;

    /// Bit arithmetic operator `*=` for little-endian
    /// # Example
    /// ```
    /// # use nbits::Arithmetic;
    /// let (mut a, b) = ([0b0000_1100, 0b0011_0000], [0b0000_0010, 0b0000_0000]);
    /// assert_eq!(a.as_mut().bit_le_mul(&b), false);
    /// assert_eq!(a, [0b0001_1000, 0b0110_0000]);
    /// ```
    fn bit_le_mul(&mut self, other: &Self::Other) -> bool;

    /// Bit arithmetic operator `/=` for big-endian
    /// # Example
    /// ```
    /// # use nbits::Arithmetic;
    /// let (a, b) = ([0b1100_0011, 0b0000_0001], [0b1000_0001]);
    /// let mut x = a.clone();
    /// x.as_mut().bit_be_div(&b);
    /// // assert_eq!(x, (u16::from_be_bytes(a) / u16::from_be_bytes([0, b[0]])).to_be_bytes());
    /// ```
    fn bit_be_div(&mut self, other: &Self::Other) -> bool;

    /// Bit arithmetic operator `/=` for little-endian
    /// # Example
    /// ```
    /// # use nbits::Arithmetic;
    /// let (a, b) = ([0b1100_0011, 0b0000_0001], [0b1000_0001, 0b0000_0000]);
    /// println!("a: {}, b: {}", u16::from_le_bytes(a), u16::from_le_bytes([b[0], 0]));
    /// let mut x = a.clone();
    /// x.as_mut().bit_le_div(&b);
    /// // assert_eq!(x, (u16::from_le_bytes(a) / u16::from_le_bytes([b[0], 0])).to_le_bytes());
    /// ```
    fn bit_le_div(&mut self, other: &Self::Other) -> bool;

    /// Bit arithmetic operator `%=`
    /// # Example
    /// ```
    /// # use nbits::Arithmetic;
    /// let (a, b) = ([0b1100_0011, 0b0000_0001], [0b0000_0001, 0b1000_0001]);
    /// let mut x = a.clone();
    /// x.as_mut().bit_be_rem(&b);
    /// // assert_eq!(x, (u16::from_be_bytes(a) % u16::from_be_bytes(b)).to_be_bytes());
    /// ```
    fn bit_be_rem(&mut self, other: &Self::Other) -> bool;
    fn bit_le_rem(&mut self, other: &Self::Other) -> bool;

    /// Comparison for big-endian
    /// # Examples
    /// ```
    /// # use nbits::Arithmetic;
    /// # use std::cmp::Ordering;
    /// assert_eq!([0b0011_0011, 0b0011_0011].bit_be_cmp(&[0b1111_1111]), Ordering::Greater);
    /// assert_eq!([0b0000_0000, 0b0011_0011].bit_be_cmp(&[0b1111_1111]), Ordering::Less);
    /// assert_eq!([0b0011_0011, 0b0011_0011].bit_be_cmp(&[0b0000_0000, 0b1111_1111]), Ordering::Greater);
    /// assert_eq!([0b0011_0011, 0b0011_0011].bit_be_cmp(&[0b1111_1111, 0b0000_0000]), Ordering::Less);
    /// ```
    fn bit_be_cmp(&self, other: &Self) -> std::cmp::Ordering;

    /// Comparison for little-endian
    /// # Examples
    /// ```
    /// # use nbits::Arithmetic;
    /// # use std::cmp::Ordering;
    /// assert_eq!([0b0011_0011, 0b0011_0011].bit_le_cmp(&[0b1111_1111]), Ordering::Greater);
    /// assert_eq!([0b0011_0011, 0b0000_0000].bit_le_cmp(&[0b1111_1111]), Ordering::Less);
    /// assert_eq!([0b0011_0011, 0b0011_0011].bit_le_cmp(&[0b0000_0000, 0b1111_1111]), Ordering::Less);
    /// assert_eq!([0b0011_0011, 0b0011_0011].bit_le_cmp(&[0b1111_1111, 0b0000_0000]), Ordering::Greater);
    /// ```
    fn bit_le_cmp(&self, other: &Self) -> std::cmp::Ordering;
}

impl Arithmetic for [u8] {
    type Other = Self;

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

    fn bit_le_add(&mut self, other: &Self::Other) -> bool {
        self.iter_mut()
            .zip(other.iter().chain(std::iter::repeat(&0)))
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

    fn bit_le_sub(&mut self, other: &Self::Other) -> bool {
        self.iter_mut()
            .zip(other.iter().chain(std::iter::repeat(&0)))
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

    fn bit_le_mul(&mut self, other: &Self::Other) -> bool {
        let mut other = other.to_vec();
        other.reverse();

        let mut result = vec![0; self.len()];
        let mut overflow = false;
        for (i, bit) in other.bit_iter().rev().enumerate() {
            if bit {
                let mut multiple = self.to_vec();
                overflow |= multiple.bit_shl(i);
                overflow |= result.bit_le_add(&multiple);
            }
        }
        self.copy_from_slice(&result);
        overflow
    }

    fn bit_be_div(&mut self, other: &Self) -> bool {
        if other.iter().all(|&b| b == 0) {
            return true; // Division by zero, return overflow
        }

        let mut result = vec![0; self.len()];
        let mut rem = vec![0; self.len()];

        for bit in self.bit_iter().rev() {
            rem.bit_shl(1);
            if bit {
                rem[0] |= 1;
            }
            if rem.as_slice() >= other {
                rem.bit_be_sub(other);
                result[0] |= 1;
            }
            result.bit_shl(1);
        }
        result.bit_shr(1); // Adjust for the extra shift
        self.copy_from_slice(&result);
        false
    }

    fn bit_le_div(&mut self, other: &Self::Other) -> bool {
        if other.iter().all(|&b| b == 0) {
            return true; // Division by zero, return overflow
        }
        if self.bit_be_cmp(other) == std::cmp::Ordering::Greater {
            self.fill(0);
            return false;
        }
        let mut result = vec![0; self.len()];
        while !self.bit_be_sub(other) {
            result.bit_be_add(&[1]);
        }
        self.copy_from_slice(&result);
        false
    }

    fn bit_be_rem(&mut self, other: &Self) -> bool {
        let mut rem = vec![0; self.len()];
        for bit in self.bit_iter() {
            rem.bit_shl(1);
            if bit {
                let len = rem.len();
                rem[len - 1] |= 1;
            }
            if rem.as_slice() >= other {
                rem.bit_be_sub(other);
            }
        }
        self.copy_from_slice(&rem);
        false
    }

    fn bit_le_rem(&mut self, other: &Self::Other) -> bool {
        if other.iter().all(|&b| b == 0) {
            return true; // Division by zero, return overflow
        }
        if self.len() < other.len() {
            self.fill(0);
            return false;
        }
        let mut result = vec![0; self.len()];
        while !self.bit_be_sub(other) {
            result.bit_be_add(&[1]);
        }
        false
    }

    fn bit_be_cmp(&self, other: &Self) -> std::cmp::Ordering {
        let max_len = std::cmp::max(self.len(), other.len());
        self.be_iter_n(max_len).cmp(other.be_iter_n(max_len))
    }

    fn bit_le_cmp(&self, other: &Self) -> std::cmp::Ordering {
        let max_len = std::cmp::max(self.len(), other.len());
        self.le_iter_n(max_len)
            .rev()
            .cmp(other.le_iter_n(max_len).rev())
    }
}

trait ByteIter {
    fn be_iter_n(&self, n: usize) -> impl DoubleEndedIterator<Item = &u8>;
    fn le_iter_n(&self, n: usize) -> impl DoubleEndedIterator<Item = &u8>;
}

impl ByteIter for [u8] {
    #[inline(always)]
    fn be_iter_n(&self, n: usize) -> impl DoubleEndedIterator<Item = &u8> {
        std::iter::repeat_n(&0, n - self.len()).chain(self.iter())
    }

    #[inline(always)]
    fn le_iter_n(&self, n: usize) -> impl DoubleEndedIterator<Item = &u8> {
        self.iter().chain(std::iter::repeat_n(&0, n - self.len()))
    }
}

/**
 * Arithmetic operations for Vec<u8>
 * Auto adjust the length of Vec<u8>.
 */
// impl Arithmetic for Vec<u8> {}

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

    #[test]
    fn test_bits_div() {
        assert!([0b0000_0001_u8, 0b0001_0000].as_ref() > [0, 0b1111_1111_u8].as_ref());
        // let (a, b, c) = ([0, 0b010_0000], [0, 0b0011], [0, 0b0000_1000]);
        // let mut r = a.clone();
        // assert_eq!(r.bits_div_overflow(&b), false);
        // let (x, y, z) = (
        //     u16::from_be_bytes([0, 0b0001_0000]),
        //     u16::from_be_bytes(b),
        //     u16::from_be_bytes(c),
        // );
        // // 5 == 0b0101, 7 == 0b0111
        // println!("x: {}, y: {}, z: {}", x, y, z);
        // assert_eq!(x / y, z);
    }

    #[test]
    fn test_bit_cmp() {
        // assert_eq!(
        //     [0b0000_0000, 0b0000_0000].bit_be_cmp(&[0b1111_1111]),
        //     std::cmp::Ordering::Less
        // );

        const A: [u8; 2] = [0b0011_0011, 0b0011_0011];
        const B: [u8; 2] = [0b1111_1111, 0b0000_0000];
        const C: [u8; 2] = [0b0000_0000, 0b1111_1111];

        let (a, b, c) = (
            u16::from_le_bytes(A),
            u16::from_le_bytes(B),
            u16::from_le_bytes(C),
        );
        // println!("a: {a:016b} = {a}, b: {b:016b} = {b}, c: {c:016b} = {c}");
        assert_eq!(a.cmp(&b), A.as_ref().bit_le_cmp(&B));
        assert_eq!(a.cmp(&c), A.as_ref().bit_le_cmp(&C));

        let (a, b, c) = (
            u16::from_be_bytes(A),
            u16::from_be_bytes(B),
            u16::from_be_bytes(C),
        );
        // println!("a: {a:016b} = {a}, b: {b:016b} = {b}, c: {c:016b} = {c}");
        assert_eq!(a.cmp(&b), A.as_ref().bit_be_cmp(&B));
        assert_eq!(a.cmp(&c), A.as_ref().bit_be_cmp(&C));
    }
}
