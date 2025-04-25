pub struct BitsRef<'a>(pub &'a [u8]);
pub struct BitsMut<'a>(pub &'a mut [u8]);

#[allow(unused)]
pub trait XBits {
    fn bits(&self) -> BitsRef;
    fn bits_mut(&mut self) -> BitsMut;
}

impl<'a> XBits for [u8] {
    fn bits(&self) -> BitsRef {
        BitsRef(self)
    }
    fn bits_mut(&mut self) -> BitsMut {
        BitsMut(self)
    }
}

impl BitsRef<'_> {
    #[inline(always)]
    pub fn all_one(&self) -> bool {
        self.0.bit_all_one()
    }
    #[inline(always)]
    pub fn all_zero(&self) -> bool {
        self.0.bit_all_zero()
    }
    #[inline(always)]
    pub fn iter<'a>(&'a self) -> impl DoubleEndedIterator<Item = bool> + 'a {
        self.0.bit_iter()
    }
}

impl BitsMut<'_> {
    pub fn as_ref(&self) -> BitsRef {
        BitsRef(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::XBits;

    #[test]
    fn test_bits() {
        let mut bits = [0b00000001_u8, 0b00000010, 0b00000100];
        let _xbits = bits.bits();
        let _xbits = bits.bits_mut();
    }
}
