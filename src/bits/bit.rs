use super::Bits;

impl<const N: usize> std::ops::BitAnd for Bits<N> {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        (0..N)
            .map(|i| self.0[i] & other.0[i])
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap()
    }
}

impl<const N: usize> std::ops::BitOr for Bits<N> {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        (0..N)
            .map(|i| self.0[i] | other.0[i])
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap()
    }
}

impl<const N: usize> std::ops::BitXor for Bits<N> {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        (0..N)
            .map(|i| self.0[i] ^ other.0[i])
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap()
    }
}

impl<const N: usize> std::ops::Not for Bits<N> {
    type Output = Self;

    fn not(self) -> Self::Output {
        (0..N)
            .map(|i| !self.0[i])
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap()
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
