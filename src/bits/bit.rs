use super::Bits;

impl<const N: usize> std::ops::BitAnd for Bits<N> {
    type Output = Self;
    #[inline]
    fn bitand(mut self, other: Self) -> Self::Output {
        self &= other;
        self
    }
}

impl<const N: usize> std::ops::BitAnd<&Bits<N>> for Bits<N> {
    type Output = Self;
    #[inline]
    fn bitand(mut self, rhs: &Self) -> Self::Output {
        self &= rhs;
        self
    }
}

impl<const N: usize> std::ops::BitOr for Bits<N> {
    type Output = Self;
    #[inline]
    fn bitor(mut self, other: Self) -> Self::Output {
        self |= other;
        self
    }
}

impl<const N: usize> std::ops::BitOr<&Bits<N>> for Bits<N> {
    type Output = Self;
    #[inline]
    fn bitor(mut self, rhs: &Self) -> Self::Output {
        self |= rhs;
        self
    }
}

impl<const N: usize> std::ops::BitXor for Bits<N> {
    type Output = Self;
    #[inline]
    fn bitxor(mut self, other: Self) -> Self::Output {
        self ^= other;
        self
    }
}

impl<const N: usize> std::ops::BitXor<&Bits<N>> for Bits<N> {
    type Output = Self;
    #[inline]
    fn bitxor(mut self, rhs: &Self) -> Self::Output {
        self ^= rhs;
        self
    }
}

impl<const N: usize> std::ops::BitAndAssign for Bits<N> {
    fn bitand_assign(&mut self, other: Self) {
        (0..N).for_each(|i| self.0[i] &= other.0[i]);
    }
}

impl<const N: usize> std::ops::BitAndAssign<&Bits<N>> for Bits<N> {
    fn bitand_assign(&mut self, other: &Self) {
        (0..N).for_each(|i| self.0[i] &= other.0[i]);
    }
}

impl<const N: usize> std::ops::BitOrAssign for Bits<N> {
    fn bitor_assign(&mut self, other: Self) {
        (0..N).for_each(|i| self.0[i] |= other.0[i]);
    }
}

impl<const N: usize> std::ops::BitOrAssign<&Bits<N>> for Bits<N> {
    fn bitor_assign(&mut self, other: &Self) {
        (0..N).for_each(|i| self.0[i] |= other.0[i]);
    }
}

impl<const N: usize> std::ops::BitXorAssign for Bits<N> {
    fn bitxor_assign(&mut self, other: Self) {
        (0..N).for_each(|i| self.0[i] ^= other.0[i]);
    }
}

impl<const N: usize> std::ops::BitXorAssign<&Bits<N>> for Bits<N> {
    fn bitxor_assign(&mut self, other: &Self) {
        (0..N).for_each(|i| self.0[i] ^= other.0[i]);
    }
}

impl<const N: usize> std::ops::Not for Bits<N> {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        self.0.iter_mut().for_each(|v| {
            *v = !*v;
        });
        self
    }
}
