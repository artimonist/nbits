use super::core::{BitIterator, Bitwise};

/**
 * `XBits` trait provides a way to work with bit-level operations on byte arrays.
 * It allows you to get a reference to the bits in a byte array and perform operations
 * such as checking if all bits are one or zero, and iterating over the bits.
 */
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

/// A reference to a byte array that allows for bit-level operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitsRef<'a>(pub &'a [u8]);

impl BitsRef<'_> {
    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        self.0
    }

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
    pub fn chunks<T>(&self, n: usize) -> impl std::iter::Iterator<Item = T> + '_
    where
        T: TryFrom<u64> + Default + 'static,
    {
        self.0.bit_chunks(n)
    }
}

/// A mutable reference to a byte array that allows for bit-level operations.
#[derive(Debug, PartialEq, Eq)]
pub struct BitsMut<'a>(pub &'a mut [u8]);

impl BitsMut<'_> {
    #[inline(always)]
    pub fn to_ref(&self) -> BitsRef {
        BitsRef(self.0)
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
    pub fn or(&mut self, other: BitsRef) -> &mut Self {
        self.0.bit_be_or(other.0);
        self
    }

    #[inline(always)]
    pub fn and(&mut self, other: BitsRef) -> &mut Self {
        self.0.bit_be_and(other.0);
        self
    }

    #[inline(always)]
    pub fn xor(&mut self, other: BitsRef) -> &mut Self {
        self.0.bit_be_xor(other.0);
        self
    }

    #[inline(always)]
    pub fn be_or<U: Into<u64>>(&mut self, other: U) -> &mut Self {
        self.0.bit_be_or(&other.into().to_be_bytes());
        self
    }

    #[inline(always)]
    pub fn be_and<U: Into<u64>>(&mut self, other: U) -> &mut Self {
        self.0.bit_be_and(&other.into().to_be_bytes());
        self
    }

    #[inline(always)]
    pub fn be_xor<U: Into<u64>>(&mut self, other: U) -> &mut Self {
        self.0.bit_be_xor(&other.into().to_be_bytes());
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
}

impl std::fmt::Display for BitsRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for byte in self.0.iter() {
            s.push_str(&format!("{:08b} ", byte));
        }
        write!(f, "{}", s.trim())
    }
}

impl std::fmt::Display for BitsMut<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for byte in self.0.iter() {
            s.push_str(&format!("{:08b} ", byte));
        }
        write!(f, "{}", s.trim())
    }
}

impl std::ops::Index<usize> for BitsRef<'_> {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.0.len() * 8 {
            panic!("Index out of bounds");
        }
        let byte_index = index / 8;
        let bit_index = index % 8;
        let byte = self.0[byte_index];
        let bit = (byte >> (7 - bit_index)) & 1;
        if bit == 1 {
            &true
        } else {
            &false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::XBits;

    #[test]
    fn test_bits() {
        let mut buf = [0b00000001_u8, 0b00000010, 0b00000100];
        let _xbits = buf.bits();
        let _xbits = buf[0..2].bits_mut().or(1024_u16.to_be_bytes().bits());

        for i in 0..buf.len() {
            assert_eq!(buf.bits()[i], buf.bits().iter().nth(i).unwrap());
        }

        let mut vs = [0b1111_1111, 0b1100_0000];
        vs.bits_mut().reverse();
        assert_eq!(vs, [0b0000_0011, 0b1111_1111]);
    }
}
