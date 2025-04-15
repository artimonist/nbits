use super::Bits;

impl<const N: usize> std::ops::BitAnd for Bits<N> {
    type Output = Self;

    fn bitand(mut self, other: Self) -> Self::Output {
        self.0.iter_mut().enumerate().for_each(|(i, v)| {
            *v &= other.0[i];
        });
        self
    }
}

impl<const N: usize> std::ops::BitOr for Bits<N> {
    type Output = Self;

    fn bitor(mut self, other: Self) -> Self::Output {
        self.0.iter_mut().enumerate().for_each(|(i, v)| {
            *v |= other.0[i];
        });
        self
    }
}

impl<const N: usize> std::ops::BitXor for Bits<N> {
    type Output = Self;

    fn bitxor(mut self, other: Self) -> Self::Output {
        self.0.iter_mut().enumerate().for_each(|(i, v)| {
            *v ^= other.0[i];
        });
        self
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

impl<const N: usize> std::ops::BitAndAssign for Bits<N> {
    fn bitand_assign(&mut self, other: Self) {
        (0..N).for_each(|i| self.0[i] &= other.0[i]);
    }
}

impl<const N: usize> std::ops::BitOrAssign for Bits<N> {
    fn bitor_assign(&mut self, other: Self) {
        (0..N).for_each(|i| self.0[i] |= other.0[i]);
    }
}

impl<const N: usize> std::ops::BitXorAssign for Bits<N> {
    fn bitxor_assign(&mut self, other: Self) {
        (0..N).for_each(|i| self.0[i] ^= other.0[i]);
    }
}
