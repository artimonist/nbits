use std::ops::Add;

use crate::assert_overflow;

use super::Bits;

/**
 * Arithmetic operator `+` for Bits
 */
impl<const N: usize> std::ops::Add for Bits<N> {
    type Output = Self;
    #[inline(always)]
    fn add(mut self, other: Self) -> Self::Output {
        self += &other;
        self
    }
}

impl<const N: usize> std::ops::Add<&Bits<N>> for Bits<N> {
    type Output = Self;
    #[inline(always)]
    fn add(mut self, other: &Self) -> Self::Output {
        self += other;
        self
    }
}

impl<const N: usize> std::ops::AddAssign for Bits<N> {
    #[inline(always)]
    fn add_assign(&mut self, other: Self) {
        *self += &other;
    }
}

impl<const N: usize> std::ops::AddAssign<&Bits<N>> for Bits<N> {
    fn add_assign(&mut self, other: &Self) {
        let mut carry = false;
        self.0
            .iter_mut()
            .zip(other.0)
            .rev()
            .for_each(|(a, b)| match (carry, b) {
                (true, 0xff) => carry = true,
                (true, _) => (*a, carry) = a.overflowing_add(b + 1),
                (false, _) => (*a, carry) = a.overflowing_add(b),
            });
        assert!(!carry, "[nbits] Overflow in operator `+`");
    }
}

/**
 * Arithmetic operator `-` for Bits
 */
impl<const N: usize> std::ops::Sub for Bits<N> {
    type Output = Self;
    #[inline(always)]
    fn sub(mut self, other: Self) -> Self::Output {
        self -= &other;
        self
    }
}

impl<const N: usize> std::ops::Sub<&Bits<N>> for Bits<N> {
    type Output = Self;
    #[inline(always)]
    fn sub(mut self, other: &Self) -> Self::Output {
        self -= other;
        self
    }
}

impl<const N: usize> std::ops::SubAssign for Bits<N> {
    #[inline(always)]
    fn sub_assign(&mut self, other: Self) {
        *self -= &other;
    }
}

impl<const N: usize> std::ops::SubAssign<&Bits<N>> for Bits<N> {
    fn sub_assign(&mut self, other: &Self) {
        if *self < *other {
            panic!("[nbits] Overflow in operator `-`");
        }
        let mut borrow = false;
        self.0
            .iter_mut()
            .zip(other.0)
            .rev()
            .for_each(|(a, b)| match (borrow, b) {
                (true, 0xff) => borrow = true,
                (true, _) => (*a, borrow) = a.overflowing_sub(b.add(1)),
                (false, _) => (*a, borrow) = a.overflowing_sub(b),
            });
    }
}

/**
 * Arithmetic operator `*` for Bits
 */
impl<const N: usize> std::ops::Mul for Bits<N> {
    type Output = Self;
    #[inline(always)]
    fn mul(mut self, other: Self) -> Self::Output {
        self *= &other;
        self
    }
}

impl<const N: usize> std::ops::Mul<&Bits<N>> for Bits<N> {
    type Output = Self;
    #[inline(always)]
    fn mul(mut self, other: &Self) -> Self::Output {
        self *= other;
        self
    }
}

impl<const N: usize> std::ops::MulAssign for Bits<N> {
    #[inline(always)]
    fn mul_assign(&mut self, other: Self) {
        *self *= &other;
    }
}

impl<const N: usize> std::ops::MulAssign<&Bits<N>> for Bits<N> {
    fn mul_assign(&mut self, other: &Self) {
        todo!()
    }
}

/**
 * Arithmetic operator `/` for Bits
 */
impl<const N: usize> std::ops::Div for Bits<N> {
    type Output = Self;
    #[inline(always)]
    fn div(mut self, other: Self) -> Self::Output {
        self /= &other;
        self
    }
}

impl<const N: usize> std::ops::Div<&Bits<N>> for Bits<N> {
    type Output = Self;
    #[inline(always)]
    fn div(mut self, other: &Self) -> Self::Output {
        self /= other;
        self
    }
}

impl<const N: usize> std::ops::DivAssign for Bits<N> {
    #[inline(always)]
    fn div_assign(&mut self, other: Self) {
        *self /= &other;
    }
}

impl<const N: usize> std::ops::DivAssign<&Bits<N>> for Bits<N> {
    fn div_assign(&mut self, other: &Self) {
        todo!()
    }
}

/**
 * Arithmetic operator `%` for Bits
 */
impl<const N: usize> std::ops::Rem for Bits<N> {
    type Output = Self;
    #[inline(always)]
    fn rem(mut self, other: Self) -> Self::Output {
        self %= &other;
        self
    }
}

impl<const N: usize> std::ops::Rem<&Bits<N>> for Bits<N> {
    type Output = Self;
    #[inline(always)]
    fn rem(mut self, other: &Self) -> Self::Output {
        self %= other;
        self
    }
}

impl<const N: usize> std::ops::RemAssign for Bits<N> {
    fn rem_assign(&mut self, other: Self) {
        *self %= &other;
    }
}

impl<const N: usize> std::ops::RemAssign<&Bits<N>> for Bits<N> {
    fn rem_assign(&mut self, other: &Self) {
        todo!()
    }
}
