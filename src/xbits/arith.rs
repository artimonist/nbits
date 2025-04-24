/**
 * Arithmetic operations for [u8]
 */
pub trait Arithmetic {
    type Other: ?Sized;

    fn bit_be_add(&mut self, other: &Self::Other) -> bool;
    fn bit_be_sub(&mut self, other: &Self::Other) -> bool;
    fn bit_be_mul(&mut self, other: &Self::Other) -> bool;
    fn bit_be_div(&mut self, other: &Self::Other) -> bool;
    fn bit_be_rem(&mut self, other: &Self::Other) -> bool;
}

impl Arithmetic for [u8] {
    type Other = Self;

    fn bit_be_add(&mut self, other: &Self) -> bool {
        self.iter_mut()
            .rev()
            .zip(other.iter().rev().chain(std::iter::repeat(&0)))
            .fold(false, |mut carry, (a, b)| {
                match (carry, *b) {
                    (true, 0xff) => carry = true,
                    (true, _) => (*a, carry) = a.overflowing_add(b + 1),
                    (false, _) => (*a, carry) = a.overflowing_add(*b),
                };
                carry
            })
    }

    fn bit_be_sub(&mut self, other: &Self) -> bool {
        self.iter_mut()
            .rev()
            .zip(other.iter().rev().chain(std::iter::repeat(&0)))
            .fold(false, |mut borrow, (a, b)| {
                match (borrow, *b) {
                    (true, 0xff) => borrow = true,
                    (true, _) => (*a, borrow) = a.overflowing_sub(b + 1),
                    (false, _) => (*a, borrow) = a.overflowing_sub(*b),
                };
                borrow
            })
    }

    fn bit_be_mul(&mut self, other: &Self) -> bool {
        use super::Bitwise;
        use super::Iterator;

        let mut result = vec![0; self.len()];
        let mut overflow = false;
        for (i, bit) in other.bit_iter().rev().enumerate() {
            if bit {
                let mut multiple = self.to_vec();
                overflow |= multiple.bit_shl(i);
                overflow |= result.bit_be_add(&multiple);
            }
        }
        self.copy_from_slice(&result);
        overflow
    }

    fn bit_be_div(&mut self, other: &Self) -> bool {
        use super::Bitwise;
        use super::Iterator;

        let ones_a = self.len() * 8 - self.bit_iter().take_while(|&b| !b).count();
        let ones_b = other.len() * 8 - other.bit_iter().take_while(|&b| !b).count();
        if ones_b == 0 {
            return true;
        }
        if ones_a < ones_b {
            self.fill(0);
            return false;
        }
        // if ones_a == ones_b {
        //     match self.bit_sub_overflow(other) {
        //         true => self.fill(0),
        //         false => self.bit_from_iter(std::iter::once(true)),
        //     }
        //     return false;
        // }

        // ones_a >= ones_b && self >= other
        let len = self.len();
        let a = self;
        let mut b = match len > other.len() {
            true => vec![0; len - other.len()],
            false => vec![0; 0],
        };
        b.extend_from_slice(other);

        // let mut r = vec![0; len];

        for i in 0..ones_a - ones_b {
            let mut tmp = b.clone();
            tmp.bit_shl(i);
            if a.bit_be_sub(&tmp) {
                // r[i] = 1;
                // a.bits_add_overflow(&tmp);
            }
        }
        // let mut multiple = 1 << (ones_a - ones_b);
        // divisor.bits_shl_overflow(ones_a - ones_b);

        // let mut result = self[..].to_vec();
        // result.bits_shr_overflow(ones_a - ones_b);

        // self.copy_from_slice(&result);
        false
    }

    fn bit_be_rem(&mut self, _other: &Self) -> bool {
        // Implement remainder overflow logic here
        false
    }
}

/**
 * Arithmetic operations for Vec<u8>
 */
// impl Arithmetic for Vec<u8> {}

#[cfg(test)]
mod test_arith {
    use super::*;

    #[test]
    fn test_bits_add() {
        let mut a = [0b1111_1111, 0b1111_1111];
        assert_eq!(a.bit_be_add(&[0b0000_0001]), true);
        assert_eq!(a, [0b0000_0000, 0b0000_0000]);

        let mut a = [0b0000_0000, 0b0000_0001];
        assert_eq!(a.bit_be_add(&[0b1111_1111]), false);
        assert_eq!(a, [0b0000_0001, 0b0000_0000]);
    }

    #[test]
    fn test_bits_sub() {
        let mut a = [0b0000_0000, 0b0000_0001];
        assert_eq!(a.bit_be_sub(&[0b1111_1111]), true);
        assert_eq!(a, [0b1111_1111, 0b0000_0010]);

        let mut a = [0b1111_1111, 0b0000_0000];
        assert_eq!(a.bit_be_sub(&[0b0000_0001]), false);
        assert_eq!(a, [0b1111_1110, 0b1111_1111]);
    }

    #[test]
    fn test_bits_mul() {
        let mut a = [0xff, 0xff];
        assert_eq!(a.bit_be_mul(&[0b0000_0010]), true);
        assert_eq!(a, [0b1111_1111, 0b1111_1110]);

        let mut a = [0b0000_0001, 0b0000_0001];
        assert_eq!(a.bit_be_mul(&[0b1111_1111]), false);
        assert_eq!(a, [0b1111_1111, 0b1111_1111]);
    }

    #[test]
    fn test_bits_div() {
        assert!([0b0000_0001_u8, 0b0001_0000].as_ref() > [0, 0b1111_1111_u8].as_ref());
        // let (a, b, c) = ([0, 0b010_0000], [0, 0b0011], [0, 0b0000_1000]);
        // let mut r = a.clone();
        // assert_eq!(r.bits_div_overflow(&b), false);
        // let (x, y, z) = (
        //     u16::from_be_bytes([0, 0b0001_0000]),
        //     u16::from_be_bytes(b),
        //     u16::from_be_bytes(c),
        // );
        // // 5 == 0b0101, 7 == 0b0111
        // println!("x: {}, y: {}, z: {}", x, y, z);
        // assert_eq!(x / y, z);
    }
}
