use super::core::{BitIterator, Bitwise};

/**
 * `XBits` trait provides a way to work with bit-level operations on byte arrays.
 * It allows you to get a reference to the bits in a byte array and perform operations
 * such as checking if all bits are one or zero, and iterating over the bits.
 */
#[allow(unused)]
pub trait XBits {
    fn bits(&self) -> BitsRef;
    fn bits_mut(&mut self) -> BitsMut;
}

impl XBits for [u8] {
    fn bits(&self) -> BitsRef {
        BitsRef(self)
    }
    fn bits_mut(&mut self) -> BitsMut {
        BitsMut(self)
    }
}

pub struct BitsRef<'a>(&'a [u8]);

pub struct BitsMut<'a>(&'a mut [u8]);

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
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = bool> + '_ {
        self.0.bit_iter()
    }

    #[inline(always)]
    pub fn trunks<T>(&self, n: usize) -> impl std::iter::Iterator<Item = T> + '_
    where
        T: TryFrom<u64> + Default + 'static,
    {
        self.0.bit_chunks(n)
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
