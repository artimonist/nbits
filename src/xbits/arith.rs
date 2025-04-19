/**
 * Arithmetic operations for [u8]
 */
pub trait Arithmetic {
    fn bits_add_overflow(&mut self, other: &Self) -> bool;
    fn bits_sub_overflow(&mut self, other: &Self) -> bool;
    fn bits_mul_overflow(&mut self, other: &Self) -> bool;
    fn bits_div_overflow(&mut self, other: &Self) -> bool;
    fn bits_rem_overflow(&mut self, other: &Self) -> bool;
}

impl Arithmetic for [u8] {
    fn bits_add_overflow(&mut self, other: &Self) -> bool {
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

    fn bits_sub_overflow(&mut self, other: &Self) -> bool {
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

    fn bits_mul_overflow(&mut self, _other: &Self) -> bool {
        // Implement multiplication overflow logic here
        false
    }

    fn bits_div_overflow(&mut self, _other: &Self) -> bool {
        // Implement division overflow logic here
        false
    }

    fn bits_rem_overflow(&mut self, _other: &Self) -> bool {
        // Implement remainder overflow logic here
        false
    }
}

#[cfg(test)]
mod test_arith {
    use super::*;

    #[test]
    fn test_bits_add() {
        let mut a = [0b1111_1111, 0b1111_1111];
        assert_eq!(a.bits_add_overflow(&[0b0000_0001]), true);
        assert_eq!(a, [0b0000_0000, 0b0000_0000]);

        let mut a = [0b0000_0000, 0b0000_0001];
        assert_eq!(a.bits_add_overflow(&[0b1111_1111]), false);
        assert_eq!(a, [0b0000_0001, 0b0000_0000]);
    }

    #[test]
    fn test_bits_sub() {
        let mut a = [0b0000_0000, 0b0000_0001];
        assert_eq!(a.bits_sub_overflow(&[0b1111_1111]), true);
        assert_eq!(a, [0b1111_1111, 0b0000_0010]);

        let mut a = [0b1111_1111, 0b0000_0000];
        assert_eq!(a.bits_sub_overflow(&[0b0000_0001]), false);
        assert_eq!(a, [0b1111_1110, 0b1111_1111]);
    }
}
