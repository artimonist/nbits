use super::Bits;

#[inline(always)]
fn bits_and<const N: usize>(x: &mut [u8; N], y: &[u8; N]) {
    (0..N).for_each(|i| x[i] &= y[i]);
}

#[inline(always)]
fn bits_or<const N: usize>(x: &mut [u8; N], y: &[u8; N]) {
    (0..N).for_each(|i| x[i] |= y[i]);
}

#[inline(always)]
fn bits_xor<const N: usize>(x: &mut [u8; N], y: &[u8; N]) {
    (0..N).for_each(|i| x[i] ^= y[i]);
}

#[inline(always)]
fn bits_not<const N: usize>(x: &mut [u8; N]) {
    (0..N).for_each(|i| x[i] = !x[i]);
}

macro_rules! impl_bitwise {
    ($op:ident, $fn:ident, $impl:ident) => {
        impl<const N: usize> std::ops::$op for Bits<N> {
            type Output = Self;
            fn $fn(mut self, other: Self) -> Self::Output {
                $impl(&mut self.0, &other.0);
                self
            }
        }

        impl<const N: usize> std::ops::$op<&Bits<N>> for Bits<N> {
            type Output = Self;
            fn $fn(mut self, rhs: &Self) -> Self::Output {
                $impl(&mut self.0, &rhs.0);
                self
            }
        }
    };
}

macro_rules! impl_bitwise_assign {
    ($op:ident, $fn:ident, $impl:ident) => {
        impl<const N: usize> std::ops::$op for Bits<N> {
            fn $fn(&mut self, other: Self) {
                $impl(&mut self.0, &other.0);
            }
        }

        impl<const N: usize> std::ops::$op<&Bits<N>> for Bits<N> {
            fn $fn(&mut self, other: &Self) {
                $impl(&mut self.0, &other.0);
            }
        }
    };
}

/**
 * Bitwise operator `&` for Bits
 */
impl_bitwise!(BitAnd, bitand, bits_and);
impl_bitwise_assign!(BitAndAssign, bitand_assign, bits_and);

/**
 * Bitwise operator `|` for Bits
 */
impl_bitwise!(BitOr, bitor, bits_or);
impl_bitwise_assign!(BitOrAssign, bitor_assign, bits_or);

/**
 * Bitwise operator `^` for Bits
 */
impl_bitwise!(BitXor, bitxor, bits_xor);
impl_bitwise_assign!(BitXorAssign, bitxor_assign, bits_xor);

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
