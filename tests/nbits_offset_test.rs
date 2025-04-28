#![cfg(test)]

use nbits::NBits as Bits;

#[test]
fn test_bits_offset() {
    let data = [0b0000_0001];
    assert_eq!(Bits(data) << 1, Bits([0b0000_0010]));
    assert_eq!(Bits(data) << 2, Bits([0b0000_0100]));
    assert_eq!(Bits(data) << 3, Bits([0b0000_1000]));
    assert_eq!(Bits(data) << 4, Bits([0b0001_0000]));
    assert_eq!(Bits(data) << 5, Bits([0b0010_0000]));
    assert_eq!(Bits(data) << 6, Bits([0b0100_0000]));
    assert_eq!(Bits(data) << 7, Bits([0b1000_0000]));

    let data = [0b1111_1111, 0b1111_1111];
    assert_eq!(Bits(data) << 1, Bits([0b1111_1111, 0b1111_1110]));
    assert_eq!(Bits(data) << 2, Bits([0b1111_1111, 0b1111_1100]));
    assert_eq!(Bits(data) << 3, Bits([0b1111_1111, 0b1111_1000]));
    assert_eq!(Bits(data) << 4, Bits([0b1111_1111, 0b1111_0000]));
    assert_eq!(Bits(data) << 5, Bits([0b1111_1111, 0b1110_0000]));
    assert_eq!(Bits(data) << 6, Bits([0b1111_1111, 0b1100_0000]));
    assert_eq!(Bits(data) << 7, Bits([0b1111_1111, 0b1000_0000]));
    assert_eq!(Bits(data) << 8, Bits([0b1111_1111, 0b0000_0000]));
    assert_eq!(Bits(data) << 9, Bits([0b1111_1110, 0b0000_0000]));
    assert_eq!(Bits(data) << 10, Bits([0b1111_1100, 0b0000_0000]));
    assert_eq!(Bits(data) << 11, Bits([0b1111_1000, 0b0000_0000]));
    assert_eq!(Bits(data) << 12, Bits([0b1111_0000, 0b0000_0000]));
    assert_eq!(Bits(data) << 13, Bits([0b1110_0000, 0b0000_0000]));
    assert_eq!(Bits(data) << 14, Bits([0b1100_0000, 0b0000_0000]));
    assert_eq!(Bits(data) << 15, Bits([0b1000_0000, 0b0000_0000]));

    let data = [0b1000_0000];
    assert_eq!(Bits(data) >> 7, Bits([0b0000_0001]));
    assert_eq!(Bits(data) >> 6, Bits([0b0000_0010]));
    assert_eq!(Bits(data) >> 5, Bits([0b0000_0100]));
    assert_eq!(Bits(data) >> 4, Bits([0b0000_1000]));
    assert_eq!(Bits(data) >> 3, Bits([0b0001_0000]));
    assert_eq!(Bits(data) >> 2, Bits([0b0010_0000]));
    assert_eq!(Bits(data) >> 1, Bits([0b0100_0000]));

    let data = [0b1111_1111, 0b1111_1111];
    assert_eq!(Bits(data) >> 1, Bits([0b0111_1111, 0b1111_1111]));
    assert_eq!(Bits(data) >> 2, Bits([0b0011_1111, 0b1111_1111]));
    assert_eq!(Bits(data) >> 3, Bits([0b0001_1111, 0b1111_1111]));
    assert_eq!(Bits(data) >> 4, Bits([0b0000_1111, 0b1111_1111]));
    assert_eq!(Bits(data) >> 5, Bits([0b0000_0111, 0b1111_1111]));
    assert_eq!(Bits(data) >> 6, Bits([0b0000_0011, 0b1111_1111]));
    assert_eq!(Bits(data) >> 7, Bits([0b0000_0001, 0b1111_1111]));
    assert_eq!(Bits(data) >> 8, Bits([0b0000_0000, 0b1111_1111]));
    assert_eq!(Bits(data) >> 9, Bits([0b0000_0000, 0b0111_1111]));
    assert_eq!(Bits(data) >> 10, Bits([0b0000_0000, 0b0011_1111]));
    assert_eq!(Bits(data) >> 11, Bits([0b0000_0000, 0b0001_1111]));
    assert_eq!(Bits(data) >> 12, Bits([0b0000_0000, 0b0000_1111]));
    assert_eq!(Bits(data) >> 13, Bits([0b0000_0000, 0b0000_0111]));
    assert_eq!(Bits(data) >> 14, Bits([0b0000_0000, 0b0000_0011]));
    assert_eq!(Bits(data) >> 15, Bits([0b0000_0000, 0b0000_0001]));
}
