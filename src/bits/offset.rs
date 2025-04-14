use super::Bits;
use crate::assert_overflow;

impl<const N: usize> std::ops::Shl<usize> for Bits<N> {
    type Output = Self;

    fn shl(self, n: usize) -> Self::Output {
        assert_overflow!(n, 1, N * 8 - 1, "<<");
        todo!()
    }
}

impl<const N: usize> std::ops::Shr<usize> for Bits<N> {
    type Output = Self;

    fn shr(self, n: usize) -> Self::Output {
        assert_overflow!(n, 1, N * 8 - 1, ">>");
        todo!()
    }
}

impl<const N: usize> std::ops::ShlAssign<usize> for Bits<N> {
    fn shl_assign(&mut self, n: usize) {
        assert_overflow!(n, 1, N * 8 - 1, "<<=");
        todo!()
    }
}

impl<const N: usize> std::ops::ShrAssign<usize> for Bits<N> {
    fn shr_assign(&mut self, n: usize) {
        assert_overflow!(n, 1, N * 8 - 1, ">>=");
        todo!()
    }
}
