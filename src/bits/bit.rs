use super::Bits;

impl<const N: usize> std::ops::BitAnd for Bits<N> {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        let mut result = [0u8; N];
        for i in 0..N {
            result[i] = self.0[i] & other.0[i];
        }
        Bits(result)
    }
}

impl<const N: usize> std::ops::BitOr for Bits<N> {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        let mut result = [0u8; N];
        for i in 0..N {
            result[i] = self.0[i] | other.0[i];
        }
        Bits(result)
    }
}

impl<const N: usize> std::ops::BitXor for Bits<N> {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        let mut result = [0u8; N];
        for i in 0..N {
            result[i] = self.0[i] ^ other.0[i];
        }
        Bits(result)
    }
}

impl<const N: usize> std::ops::Not for Bits<N> {
    type Output = Self;

    fn not(self) -> Self::Output {
        let mut result = [0u8; N];
        for i in 0..N {
            result[i] = !self.0[i];
        }
        Bits(result)
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
