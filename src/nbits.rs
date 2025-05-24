use super::core::{BitArith, Bitwise};
use crate::{BitsMut, BitsRef, XBits};

/**
 *  `NBits` is a wrapper around an array of bytes that provides
 *  arithmetic and bitwise operations on the bits represented
 *  by the bytes. The operations are performed in big-endian
 *  order.
 */
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NBits<const N: usize>(pub [u8; N]);

impl<const N: usize> NBits<N> {
    /// Creates a new `NBits` instance with all bits set to 0.
    #[inline(always)]
    pub fn new() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> Default for NBits<N> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> std::fmt::Display for NBits<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for byte in self.0.iter() {
            s.push_str(&format!("{:08b} ", byte));
        }
        write!(f, "{}", s)
    }
}

impl<const N: usize> std::ops::Add<&NBits<N>> for NBits<N> {
    type Output = Self;

    #[inline(always)]
    fn add(mut self, other: &Self) -> Self::Output {
        let overflow = self.0.bit_be_add(&other.0);
        assert!(!overflow, "[nbits] Overflow in `add`");
        self
    }
}

impl<const N: usize> std::ops::Sub<&NBits<N>> for NBits<N> {
    type Output = Self;

    #[inline(always)]
    fn sub(mut self, other: &Self) -> Self::Output {
        let overflow = self.0.bit_be_sub(&other.0);
        assert!(!overflow, "[nbits] Overflow in `sub`");
        self
    }
}

impl<const N: usize> std::ops::Mul<&NBits<N>> for NBits<N> {
    type Output = Self;

    #[inline(always)]
    fn mul(mut self, other: &Self) -> Self::Output {
        let overflow = self.0.bit_be_mul(&other.0);
        assert!(!overflow, "[nbits] Overflow in `mul`");
        self
    }
}

impl<const N: usize> std::ops::Div<&NBits<N>> for NBits<N> {
    type Output = Self;

    #[inline(always)]
    fn div(mut self, other: &Self) -> Self::Output {
        let overflow = self.0.bit_be_div(&other.0);
        assert!(!overflow, "[nbits] Overflow in `div`");
        self
    }
}

impl<const N: usize> std::ops::Rem<&NBits<N>> for NBits<N> {
    type Output = Self;

    #[inline(always)]
    fn rem(mut self, other: &Self) -> Self::Output {
        let overflow = self.0.bit_be_rem(&other.0);
        assert!(!overflow, "[nbits] Overflow in `rem`");
        self
    }
}

impl<const N: usize> std::ops::AddAssign<&NBits<N>> for NBits<N> {
    #[inline(always)]
    fn add_assign(&mut self, other: &Self) {
        let overflow = self.0.bit_be_add(&other.0);
        assert!(!overflow, "[nbits] Overflow in `add_assign`");
    }
}

impl<const N: usize> std::ops::SubAssign<&NBits<N>> for NBits<N> {
    #[inline(always)]
    fn sub_assign(&mut self, other: &Self) {
        let overflow = self.0.bit_be_sub(&other.0);
        assert!(!overflow, "[nbits] Overflow in `sub_assign`");
    }
}

impl<const N: usize> std::ops::MulAssign<&NBits<N>> for NBits<N> {
    #[inline(always)]
    fn mul_assign(&mut self, other: &Self) {
        let overflow = self.0.bit_be_mul(&other.0);
        assert!(!overflow, "[nbits] Overflow in `mul_assign`");
    }
}

impl<const N: usize> std::ops::DivAssign<&NBits<N>> for NBits<N> {
    #[inline(always)]
    fn div_assign(&mut self, other: &Self) {
        let overflow = self.0.bit_be_div(&other.0);
        assert!(!overflow, "[nbits] Overflow in `div_assign`");
    }
}

impl<const N: usize> std::ops::RemAssign<&NBits<N>> for NBits<N> {
    #[inline(always)]
    fn rem_assign(&mut self, other: &Self) {
        let overflow = self.0.bit_be_rem(&other.0);
        assert!(!overflow, "[nbits] Overflow in `rem_assign`");
    }
}

impl<const N: usize> std::ops::BitAnd<&NBits<N>> for NBits<N> {
    type Output = Self;

    #[inline(always)]
    fn bitand(mut self, other: &Self) -> Self::Output {
        self.0.bit_be_and(&other.0);
        self
    }
}

impl<const N: usize> std::ops::BitOr<&NBits<N>> for NBits<N> {
    type Output = Self;

    #[inline(always)]
    fn bitor(mut self, other: &Self) -> Self::Output {
        self.0.bit_be_or(&other.0);
        self
    }
}

impl<const N: usize> std::ops::BitXor<&NBits<N>> for NBits<N> {
    type Output = Self;

    #[inline(always)]
    fn bitxor(mut self, other: &Self) -> Self::Output {
        self.0.bit_be_xor(&other.0);
        self
    }
}

impl<const N: usize> std::ops::Shl<usize> for NBits<N> {
    type Output = Self;

    #[inline(always)]
    fn shl(mut self, rhs: usize) -> Self::Output {
        self.0.bit_shl(rhs);
        self
    }
}

impl<const N: usize> std::ops::Shr<usize> for NBits<N> {
    type Output = Self;

    #[inline(always)]
    fn shr(mut self, rhs: usize) -> Self::Output {
        self.0.bit_shr(rhs);
        self
    }
}

impl<const N: usize> std::ops::BitAndAssign<&NBits<N>> for NBits<N> {
    #[inline(always)]
    fn bitand_assign(&mut self, other: &Self) {
        self.0.bit_be_and(&other.0);
    }
}

impl<const N: usize> std::ops::BitOrAssign<&NBits<N>> for NBits<N> {
    #[inline(always)]
    fn bitor_assign(&mut self, other: &Self) {
        self.0.bit_be_or(&other.0);
    }
}

impl<const N: usize> std::ops::BitXorAssign<&NBits<N>> for NBits<N> {
    #[inline(always)]
    fn bitxor_assign(&mut self, other: &Self) {
        self.0.bit_be_xor(&other.0);
    }
}

impl<const N: usize> std::ops::Not for NBits<N> {
    type Output = Self;

    #[inline(always)]
    fn not(mut self) -> Self::Output {
        self.0.bit_not();
        self
    }
}

impl<const N: usize> std::ops::ShlAssign<usize> for NBits<N> {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: usize) {
        self.0.bit_shl(rhs);
    }
}

impl<const N: usize> std::ops::ShrAssign<usize> for NBits<N> {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: usize) {
        self.0.bit_shr(rhs);
    }
}

impl<const N: usize, U: Into<u64>> std::ops::Add<U> for NBits<N> {
    type Output = Self;

    #[inline(always)]
    fn add(mut self, other: U) -> Self::Output {
        let overflow = self.0.bit_be_add(&other.into().to_be_bytes());
        assert!(!overflow, "[nbits] Overflow in `add`");
        self
    }
}

impl<const N: usize, U: Into<u64>> std::ops::Sub<U> for NBits<N> {
    type Output = Self;

    #[inline(always)]
    fn sub(mut self, other: U) -> Self::Output {
        let overflow = self.0.bit_be_sub(&other.into().to_be_bytes());
        assert!(!overflow, "[nbits] Overflow in `sub`");
        self
    }
}

impl<const N: usize, U: Into<u64>> std::ops::Mul<U> for NBits<N> {
    type Output = Self;

    #[inline(always)]
    fn mul(mut self, other: U) -> Self::Output {
        let overflow = self.0.bit_be_mul(&other.into().to_be_bytes());
        assert!(!overflow, "[nbits] Overflow in `mul`");
        self
    }
}

impl<const N: usize, U: Into<u64>> std::ops::Div<U> for NBits<N> {
    type Output = Self;

    #[inline(always)]
    fn div(mut self, other: U) -> Self::Output {
        let overflow = self.0.bit_be_div(&other.into().to_be_bytes());
        assert!(!overflow, "[nbits] Overflow in `div`");
        self
    }
}

impl<const N: usize, U: Into<u64>> std::ops::Rem<U> for NBits<N> {
    type Output = Self;

    #[inline(always)]
    fn rem(mut self, other: U) -> Self::Output {
        let overflow = self.0.bit_be_rem(&other.into().to_be_bytes());
        assert!(!overflow, "[nbits] Overflow in `rem`");
        self
    }
}

impl<const N: usize, U: Into<u64>> std::ops::AddAssign<U> for NBits<N> {
    #[inline(always)]
    fn add_assign(&mut self, other: U) {
        let overflow = self.0.bit_be_add(&other.into().to_be_bytes());
        assert!(!overflow, "[nbits] Overflow in `add_assign`");
    }
}

impl<const N: usize, U: Into<u64>> std::ops::SubAssign<U> for NBits<N> {
    #[inline(always)]
    fn sub_assign(&mut self, other: U) {
        let overflow = self.0.bit_be_sub(&other.into().to_be_bytes());
        assert!(!overflow, "[nbits] Overflow in `sub_assign`");
    }
}

impl<const N: usize, U: Into<u64>> std::ops::MulAssign<U> for NBits<N> {
    #[inline(always)]
    fn mul_assign(&mut self, other: U) {
        let overflow = self.0.bit_be_mul(&other.into().to_be_bytes());
        assert!(!overflow, "[nbits] Overflow in `mul_assign`");
    }
}

impl<const N: usize, U: Into<u64>> std::ops::DivAssign<U> for NBits<N> {
    #[inline(always)]
    fn div_assign(&mut self, other: U) {
        let overflow = self.0.bit_be_div(&other.into().to_be_bytes());
        assert!(!overflow, "[nbits] Overflow in `div_assign`");
    }
}

impl<const N: usize, U: Into<u64>> std::ops::RemAssign<U> for NBits<N> {
    #[inline(always)]
    fn rem_assign(&mut self, other: U) {
        let overflow = self.0.bit_be_rem(&other.into().to_be_bytes());
        assert!(!overflow, "[nbits] Overflow in `rem_assign`");
    }
}

impl<const N: usize> NBits<N> {
    #[inline(always)]
    pub fn bit(&self, index: usize) -> bool {
        assert!(index < N * 8, "[nbits] Index out of bounds");
        let byte_index = index / 8;
        let bit_index = index % 8;
        ((self.0[byte_index] >> (7 - bit_index)) & 1) == 1
    }
}

impl<const N: usize> XBits for NBits<N> {
    fn bits(&self) -> BitsRef {
        BitsRef(&self.0)
    }

    fn bits_mut(&mut self) -> BitsMut {
        BitsMut(&mut self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nbits() {
        let mut nbits = NBits::<4>::new();
        nbits.0[0] = 0b10101010;
        assert_eq!(nbits.bit(0), true);
        assert_eq!(nbits.bit(1), false);
        assert_eq!(nbits.bit(2), true);
        assert_eq!(nbits.bit(3), false);
        assert_eq!(nbits.bit(4), true);
        assert_eq!(nbits.bit(5), false);
        assert_eq!(nbits.bit(6), true);
        assert_eq!(nbits.bit(7), false);
    }
}
