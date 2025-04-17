use super::Bits;

#[inline]
pub fn bits_and<const N: usize>(x: &mut [u8; N], y: &[u8; N]) {
    (0..N).for_each(|i| x[i] &= y[i]);
}

#[inline]
pub fn bits_or<const N: usize>(x: &mut [u8; N], y: &[u8; N]) {
    (0..N).for_each(|i| x[i] |= y[i]);
}

#[inline]
pub fn bits_xor<const N: usize>(x: &mut [u8; N], y: &[u8; N]) {
    (0..N).for_each(|i| x[i] ^= y[i]);
}

#[inline]
pub fn bits_not<const N: usize>(x: &mut [u8; N]) {
    (0..N).for_each(|i| x[i] = !x[i]);
}

macro_rules! impl_bitwise {
    ($op:ident, $op_fn:ident, $assign:ident, $assign_fn:ident, $impl:ident) => {
        impl<const N: usize> std::ops::$op for Bits<N> {
            type Output = Self;
            fn $op_fn(mut self, other: Self) -> Self::Output {
                $impl(&mut self.0, &other.0);
                self
            }
        }

        impl<const N: usize> std::ops::$op<&Bits<N>> for Bits<N> {
            type Output = Self;
            fn $op_fn(mut self, rhs: &Self) -> Self::Output {
                $impl(&mut self.0, &rhs.0);
                self
            }
        }

        impl<const N: usize> std::ops::$assign for Bits<N> {
            fn $assign_fn(&mut self, other: Self) {
                $impl(&mut self.0, &other.0);
            }
        }

        impl<const N: usize> std::ops::$assign<&Bits<N>> for Bits<N> {
            fn $assign_fn(&mut self, other: &Self) {
                $impl(&mut self.0, &other.0);
            }
        }
    };
}

/**
 * Bitwise operator `&` for Bits
 */
impl_bitwise!(BitAnd, bitand, BitAndAssign, bitand_assign, bits_and);

/**
 * Bitwise operator `|` for Bits
 */
impl_bitwise!(BitOr, bitor, BitOrAssign, bitor_assign, bits_or);

/**
 * Bitwise operator `^` for Bits
 */
impl_bitwise!(BitXor, bitxor, BitXorAssign, bitxor_assign, bits_xor);

/**
 * Bitwise operator `!` for Bits
 */
impl<const N: usize> std::ops::Not for Bits<N> {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        bits_not(&mut self.0);
        self
    }
}
