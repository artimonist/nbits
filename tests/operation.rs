#![cfg(test)]

use bits::*;

#[test]
fn test_bit_iterator() {
    for (i, &data) in DATA_LIST.iter().enumerate() {
        assert_eq!(data.bit_iter().collect::<Vec<_>>(), BITS_LIST[i]);
        assert_eq!(BITS_LIST[i].iter().to_bits(), *data);
        assert_eq!(Vec::bit_from(BITS_LIST[i].iter().map(|&v| v)), *data);
    }
}

const DATA_LIST: &[&[u8]] = &[
    &[],
    &[0; 1],
    &[0; 2],
    &[0; 4],
    &[0; 8],
    &[u8::MAX; 1],
    &[u8::MAX; 2],
    &[u8::MAX; 4],
    &[u8::MAX; 8],
    &[0b0000_0001],
    &[0b1000_0000],
    &[0b1111_1110],
    &[0b0111_1111],
    &[0b0101_0101],
    &[0b1010_1010],
    &[0b0000_0000, 0b1111_1111],
    &[0b1111_1111, 0b0000_0000],
    &[0b1100_0011, 0b0011_1100],
];

const BITS_LIST: &[&[bool]] = &[
    &[],
    &[false; 8],
    &[false; 16],
    &[false; 32],
    &[false; 64],
    &[true; 8],
    &[true; 16],
    &[true; 32],
    &[true; 64],
    &[false, false, false, false, false, false, false, true],
    &[true, false, false, false, false, false, false, false],
    &[true, true, true, true, true, true, true, false],
    &[false, true, true, true, true, true, true, true],
    &[false, true, false, true, false, true, false, true],
    &[true, false, true, false, true, false, true, false],
    &[
        false, false, false, false, false, false, false, false, //..
        true, true, true, true, true, true, true, true,
    ],
    &[
        true, true, true, true, true, true, true, true, false, //..
        false, false, false, false, false, false, false,
    ],
    &[
        true, true, false, false, false, false, true, true, //..
        false, false, true, true, true, true, false, false,
    ],
];
