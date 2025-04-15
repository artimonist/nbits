use std::io::Read;

use super::Bits;
use crate::assert_overflow;

impl<const N: usize> std::ops::Shl<usize> for Bits<N> {
    type Output = Self;

    fn shl(mut self, n: usize) -> Self::Output {
        assert_overflow!(n, 1, N * 8 - 1, "<<");

        let ref mut data = self.0;
        data.copy_within(n / 8.., 0);
        data[N - n / 8..].fill(0);

        if n % 8 != 0 {
            let m = n % 8;
            let mut carry = 0;
            data.iter_mut().take(N - n / 8).rev().for_each(|v| {
                (*v, carry) = ((*v << m) | carry, *v >> (8 - m));
            });
        }
        self
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
