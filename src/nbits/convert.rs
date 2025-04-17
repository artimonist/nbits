use super::Bits;

impl<const N: usize> From<[u8; N]> for Bits<N> {
    fn from(array: [u8; N]) -> Self {
        Self(array)
    }
}

impl<const N: usize> From<Bits<N>> for [u8; N] {
    fn from(bits: Bits<N>) -> Self {
        bits.0
    }
}

impl<const N: usize> TryFrom<Vec<u8>> for Bits<N> {
    type Error = ();

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        value.try_into()
    }
}
