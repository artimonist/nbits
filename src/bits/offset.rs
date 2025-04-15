use super::Bits;
use crate::assert_overflow;

impl<const N: usize> std::ops::Shl<usize> for Bits<N> {
    type Output = Self;

    fn shl(self, n: usize) -> Self::Output {
        assert_overflow!(n, 1, N * 8 - 1, "<<");
        if n % 8 == 0 {
            let mut data = self.0;
            data[n / 8..].copy_from_slice(&self.0[..N - n / 8]);
            Bits(data)
        } else if n < 8 {
            let mut data = vec![0; N];
            let mut carry = 0;
            for i in 0..N {
                let next_carry = self.0[i] >> (8 - n);
                data[i] = (self.0[i] << n) | carry;
                carry = next_carry;
            }
            Bits(data.try_into().unwrap())
        } else {
            todo!()
        }

        // let mut data = vec![0; N];
        // if n >= 8 {
        //     self.0[..N - n.div_ceil(8)];
        // }
        // let mut data = self.0[..N - n / 8].to_vec();
        // let mut carry = 0;
        // for i in 0..N - n / 8 {
        //     let next_carry = self.0[i] >> (8 - n % 8);
        //     data[i] = (self.0[i] << n % 8) | carry;
        //     carry = next_carry;
        // }
        // Bits(data.try_into().unwrap())
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
