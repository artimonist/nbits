#![cfg(test)]

use nbits::core::{BitIterator, FromBits};

#[test]
fn test_bit_iter() {
    for (i, &data) in DATA_LIST.iter().enumerate() {
        assert_eq!(data.bit_iter().collect::<Vec<_>>(), BITS_LIST[i]);
        assert_eq!(Vec::from_bits(BITS_LIST[i].iter().copied()), *data);
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

#[test]
fn test_doc() {
    assert_eq!(
        vec![0b1111_1111].bit_chunks(6).collect::<Vec<u8>>(),
        vec![0b11_1111, 0b11_0000]
    );
    assert_eq!(
        vec![0b1111_1111, 0b1111_1111]
            .bit_chunks(6)
            .collect::<Vec<u8>>(),
        vec![0b11_1111, 0b11_1111, 0b11_1100]
    );
    assert_eq!(
        vec![0b1111_1111; 3].bit_chunks(11).collect::<Vec<u16>>(),
        vec![0b111_1111_1111, 0b111_1111_1111, 0b110_0000_0000]
    );

    assert_eq!(
        Vec::from_bits_chunk([0b11_1111_u8, 0b11_1111, 0b11_1111].into_iter(), 6),
        vec![0b1111_1111, 0b1111_1111, 0b1100_0000]
    );
    assert_eq!(
        Vec::from_bits_chunk([0b1111_u16, 0b1111, 0b1111].into_iter(), 6),
        vec![0b001111_00, 0b1111_0011, 0b1100_0000]
    );

    assert_eq!(
        [0b1111_0000_u8].bit_iter().collect::<Vec<bool>>(),
        vec![true, true, true, true, false, false, false, false]
    );
    assert_eq!(
        Vec::from_bits(
            [true, true, true, true, false, false, false, false]
                .iter()
                .copied()
        ),
        [0b1111_0000]
    );
}
