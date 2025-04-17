use crate::nbits::bits_shl;

use super::Bits;
use std::ops::Add;

pub fn bits_add_overflow<const N: usize>(x: &mut [u8; N], y: &[u8; N]) -> bool {
    let mut carry = false;
    x.iter_mut()
        .zip(y)
        .rev()
        .for_each(|(a, b)| match (carry, b) {
            (true, 0xff) => carry = true,
            (true, _) => (*a, carry) = a.overflowing_add(b + 1),
            (false, _) => (*a, carry) = a.overflowing_add(*b),
        });
    carry
}

pub fn bits_sub_overflow<const N: usize>(x: &mut [u8; N], y: &[u8; N]) -> bool {
    let mut borrow = false;
    x.iter_mut()
        .zip(y)
        .rev()
        .for_each(|(a, b)| match (borrow, b) {
            (true, 0xff) => borrow = true,
            (true, _) => (*a, borrow) = a.overflowing_sub(b + 1),
            (false, _) => (*a, borrow) = a.overflowing_sub(*b),
        });
    borrow
}

pub fn bits_mul_overflow<const N: usize>(x: &mut [u8; N], y: &[u8; N]) -> bool {
    use crate::BitIterator;
    let mut overflow = false;
    *x = y.bit_iter().rev().fold([0; N], |mut acc, bit| {
        if bit {
            overflow |= bits_add_overflow(&mut acc, x);
        }
        bits_shl(x, 1);
        acc
    });
    overflow
}

pub fn bits_div_overflow<const N: usize>(x: &mut [u8; N], y: &[u8; N]) -> bool {
    todo!()
}

pub fn bits_rem_overflow<const N: usize>(x: &mut [u8; N], y: &[u8; N]) -> bool {
    todo!()
}

macro_rules! impl_arithmetic {
    ($op:ident, $op_fn:ident, $assign:ident, $assign_fn:ident, $impl:ident) => {
        impl<const N: usize> std::ops::$op for Bits<N> {
            type Output = Self;
            fn $op_fn(mut self, other: Self) -> Self::Output {
                let overflow = $impl(&mut self.0, &other.0);
                assert!(!overflow, "[nbits] Overflow in `{}`", stringify!($op));
                self
            }
        }

        impl<const N: usize> std::ops::$op<&Bits<N>> for Bits<N> {
            type Output = Self;
            fn $op_fn(mut self, other: &Self) -> Self::Output {
                let overflow = $impl(&mut self.0, &other.0);
                assert!(!overflow, "[nbits] Overflow in `{}`", stringify!($op));
                self
            }
        }

        impl<const N: usize> std::ops::$assign for Bits<N> {
            fn $assign_fn(&mut self, other: Self) {
                let overflow = $impl(&mut self.0, &other.0);
                assert!(!overflow, "[nbits] Overflow in `{}`", stringify!($assign));
            }
        }

        impl<const N: usize> std::ops::$assign<&Bits<N>> for Bits<N> {
            fn $assign_fn(&mut self, other: &Self) {
                let overflow = $impl(&mut self.0, &other.0);
                assert!(!overflow, "[nbits] Overflow in `{}`", stringify!($assign));
            }
        }
    };
}

/**
 * Arithmetic operator `+` for Bits
 */
impl_arithmetic!(Add, add, AddAssign, add_assign, bits_add_overflow);

/**
 * Arithmetic operator `-` for Bits
 */
impl_arithmetic!(Sub, sub, SubAssign, sub_assign, bits_sub_overflow);

/**
 * Arithmetic operator `*` for Bits
 */
impl_arithmetic!(Mul, mul, MulAssign, mul_assign, bits_mul_overflow);

/**
 * Arithmetic operator `/` for Bits
 */
impl_arithmetic!(Div, div, DivAssign, div_assign, bits_div_overflow);

/**
 * Arithmetic operator `%` for Bits
 */
impl_arithmetic!(Rem, rem, RemAssign, rem_assign, bits_rem_overflow);

// impl<const N: usize> std::ops::Mul for Bits<N> {
//     type Output = Self;
//     #[inline(always)]
//     fn mul(mut self, other: Self) -> Self::Output {
//         self *= &other;
//         self
//     }
// }

// impl<const N: usize> std::ops::Mul<&Bits<N>> for Bits<N> {
//     type Output = Self;
//     #[inline(always)]
//     fn mul(mut self, other: &Self) -> Self::Output {
//         self *= other;
//         self
//     }
// }

// impl<const N: usize> std::ops::MulAssign for Bits<N> {
//     #[inline(always)]
//     fn mul_assign(&mut self, other: Self) {
//         *self *= &other;
//     }
// }

// impl<const N: usize> std::ops::MulAssign<&Bits<N>> for Bits<N> {
//     fn mul_assign(&mut self, other: &Self) {
//         // (0..N * 8).for_each(|i| {
//         //   // other.0 << i & 1 == 1 {
//         //   //   self.0 = self.0 << i;
//         //   // }
//         // });
//     }
// }

// /**
//  * Arithmetic operator `/` for Bits
//  */
// impl<const N: usize> std::ops::Div for Bits<N> {
//     type Output = Self;
//     #[inline(always)]
//     fn div(mut self, other: Self) -> Self::Output {
//         self /= &other;
//         self
//     }
// }

// impl<const N: usize> std::ops::Div<&Bits<N>> for Bits<N> {
//     type Output = Self;
//     #[inline(always)]
//     fn div(mut self, other: &Self) -> Self::Output {
//         self /= other;
//         self
//     }
// }

// impl<const N: usize> std::ops::DivAssign for Bits<N> {
//     #[inline(always)]
//     fn div_assign(&mut self, other: Self) {
//         *self /= &other;
//     }
// }

// impl<const N: usize> std::ops::DivAssign<&Bits<N>> for Bits<N> {
//     fn div_assign(&mut self, other: &Self) {
//         todo!()
//     }
// }

// /**
//  * Arithmetic operator `%` for Bits
//  */
// impl<const N: usize> std::ops::Rem for Bits<N> {
//     type Output = Self;
//     #[inline(always)]
//     fn rem(mut self, other: Self) -> Self::Output {
//         self %= &other;
//         self
//     }
// }

// impl<const N: usize> std::ops::Rem<&Bits<N>> for Bits<N> {
//     type Output = Self;
//     #[inline(always)]
//     fn rem(mut self, other: &Self) -> Self::Output {
//         self %= other;
//         self
//     }
// }

// impl<const N: usize> std::ops::RemAssign for Bits<N> {
//     fn rem_assign(&mut self, other: Self) {
//         *self %= &other;
//     }
// }

// impl<const N: usize> std::ops::RemAssign<&Bits<N>> for Bits<N> {
//     fn rem_assign(&mut self, other: &Self) {
//         todo!()
//     }
// }
