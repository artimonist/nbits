use super::Bits;

impl<const N: usize> std::ops::Add for Bits<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> std::ops::Sub for Bits<N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> std::ops::Mul for Bits<N> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> std::ops::Div for Bits<N> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> std::ops::Rem for Bits<N> {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> std::ops::AddAssign for Bits<N> {
    fn add_assign(&mut self, other: Self) {
        todo!()
    }
}

impl<const N: usize> std::ops::SubAssign for Bits<N> {
    fn sub_assign(&mut self, other: Self) {
        todo!()
    }
}

impl<const N: usize> std::ops::MulAssign for Bits<N> {
    fn mul_assign(&mut self, other: Self) {
        todo!()
    }
}

impl<const N: usize> std::ops::DivAssign for Bits<N> {
    fn div_assign(&mut self, other: Self) {
        todo!()
    }
}

impl<const N: usize> std::ops::RemAssign for Bits<N> {
    fn rem_assign(&mut self, other: Self) {
        todo!()
    }
}
