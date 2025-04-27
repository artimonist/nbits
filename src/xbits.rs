use super::core::{BitArith, BitIterator, Bitwise};

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
    pub fn leading_zeros(&self) -> usize {
        self.0.bit_leading_zeros()
    }

    #[inline(always)]
    pub fn trailing_zeros(&self) -> usize {
        self.0.bit_trailing_zeros()
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

    #[allow(clippy::should_implement_trait)]
    #[inline(always)]
    pub fn cmp(&self, other: &BitsRef) -> std::cmp::Ordering {
        self.0.bit_be_cmp(other.0)
    }
}

impl BitsMut<'_> {
    pub fn as_ref(&self) -> BitsRef {
        BitsRef(self.0)
    }

    #[inline(always)]
    pub fn shl_overflow(&mut self, n: usize) -> bool {
        self.0.bit_shl(n)
    }

    #[inline(always)]
    pub fn shr_overflow(&mut self, n: usize) -> bool {
        self.0.bit_shr(n)
    }

    #[inline(always)]
    pub fn shl(&mut self, n: usize) -> &mut Self {
        self.0.bit_shl(n);
        self
    }

    #[inline(always)]
    pub fn shr(&mut self, n: usize) -> &mut Self {
        self.0.bit_shr(n);
        self
    }

    #[inline(always)]
    pub fn or(&mut self, other: &BitsRef) -> &mut Self {
        self.0.bit_be_or(other.0);
        self
    }

    #[inline(always)]
    pub fn and(&mut self, other: &BitsRef) -> &mut Self {
        self.0.bit_be_and(other.0);
        self
    }

    #[inline(always)]
    pub fn xor(&mut self, other: &BitsRef) -> &mut Self {
        self.0.bit_be_xor(other.0);
        self
    }

    #[inline(always)]
    pub fn not(&mut self) -> &mut Self {
        self.0.bit_not();
        self
    }

    #[inline(always)]
    pub fn reverse(&mut self) -> &mut Self {
        self.0.bit_reverse();
        self
    }

    #[inline(always)]
    pub fn add(&mut self, other: &BitsRef) -> &mut Self {
        self.0.bit_be_add(other.0);
        self
    }

    #[inline(always)]
    pub fn sub(&mut self, other: &BitsRef) -> &mut Self {
        self.0.bit_be_sub(other.0);
        self
    }

    #[inline(always)]
    pub fn mul(&mut self, other: &BitsRef) -> &mut Self {
        self.0.bit_be_mul(other.0);
        self
    }

    #[inline(always)]
    pub fn div(&mut self, other: &BitsRef) -> &mut Self {
        self.0.bit_be_div(other.0);
        self
    }

    #[inline(always)]
    pub fn rem(&mut self, other: &BitsRef) -> &mut Self {
        self.0.bit_be_rem(other.0);
        self
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
