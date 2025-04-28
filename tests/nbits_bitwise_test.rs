#![cfg(test)]

use nbits::NBits as Bits;

#[test]
fn test_bits_bitwise() {
    assert_eq!(Bits([0]), Bits([0b0000_1111]) & &Bits([0b1111_0000]));
    assert_eq!(Bits([0b1100]), Bits([0b0000_1111]) & &Bits([0b1111_1100]));

    assert_eq!(Bits([0xff]), Bits([0b0000_1111]) | &Bits([0b1111_0000]));
    assert_eq!(Bits([0x0f]), Bits([0b0000_1111]) | &Bits([0b0000_0011]));

    assert_eq!(Bits([0xff]), Bits([0b0000_1111]) ^ &Bits([0b1111_0000]));
    assert_eq!(Bits([0x0f]), Bits([0b1111_1111]) ^ &Bits([0b1111_0000]));

    assert_eq!(Bits([0b1111_1111]), !Bits([0b0000_0000]));
    assert_eq!(Bits([0b1111_0000]), !Bits([0b0000_1111]));

    let mut bits = Bits([0b0000_1111]);
    bits &= &Bits([0b1111_0000]);
    assert_eq!(bits, Bits([0b0000_0000]));

    let mut bits = Bits([0b0000_1111]);
    bits |= &Bits([0b1111_0000]);
    assert_eq!(bits, Bits([0b1111_1111]));

    let mut bits = Bits([0b0000_1111]);
    bits ^= &Bits([0b1111_0000]);
    assert_eq!(bits, Bits([0b1111_1111]));
}
