use super::Bits;
use crate::assert_overflow;

pub fn bits_shl<const N: usize>(data: &mut [u8; N], n: usize) {
    let (n, m) = (n / 8, n % 8);
    data.copy_within(n.., 0);
    data[N - n..].fill(0);

    if m != 0 {
        let mut carry = 0;
        data.iter_mut().take(N - n).rev().for_each(|v| {
            (*v, carry) = ((*v << m) | carry, *v >> (8 - m));
        });
    }
}

pub fn bits_shr<const N: usize>(data: &mut [u8; N], n: usize) {
    let (n, m) = (n / 8, n % 8);
    data.copy_within(..N - n, n);
    data[..n].fill(0);

    if m != 0 {
        let mut carry = 0;
        data.iter_mut().skip(n).for_each(|v| {
            (*v, carry) = ((*v >> m) | carry, *v << (8 - m));
        });
    }
}

/**
 * Offset operator `<<` && `<<=` for Bits
 */
impl<const N: usize> std::ops::Shl<usize> for Bits<N> {
    type Output = Self;
    fn shl(mut self, n: usize) -> Self::Output {
        assert_overflow!(n, 0, N * 8 - 1, "<<");
        bits_shl(&mut self.0, n);
        self
    }
}

impl<const N: usize> std::ops::ShlAssign<usize> for Bits<N> {
    fn shl_assign(&mut self, n: usize) {
        assert_overflow!(n, 0, N * 8 - 1, "<<=");
        bits_shl(&mut self.0, n);
    }
}

/**
 * Offset operator `>>` && `>>=` for Bits
 */
impl<const N: usize> std::ops::Shr<usize> for Bits<N> {
    type Output = Self;
    fn shr(mut self, n: usize) -> Self::Output {
        assert_overflow!(n, 0, N * 8 - 1, ">>");
        bits_shr(&mut self.0, n);
        self
    }
}

impl<const N: usize> std::ops::ShrAssign<usize> for Bits<N> {
    fn shr_assign(&mut self, n: usize) {
        assert_overflow!(n, 0, N * 8 - 1, ">>=");
        bits_shr(&mut self.0, n);
    }
}
