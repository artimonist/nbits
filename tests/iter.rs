#![cfg(test)]

use bits::BitConjoin;
use bits::*;
use hex::FromHex;

#[test]
fn test_bit_iter() {
    for (i, &data) in DATA_LIST.iter().enumerate() {
        assert_eq!(data.bit_iter().collect::<Vec<_>>(), BITS_LIST[i]);
        assert_eq!(BITS_LIST[i].iter().to_bits(), *data);
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
fn test_bit_chunks() {
    for (i, &entropy) in ENTROPY_LIST.iter().enumerate() {
        let data = Vec::from_hex(entropy).expect("entropy");
        let indices: Vec<u16> = data.bit_chunks(11).collect();
        assert_eq!(indices, INDICES_LIST[i]);
        assert_eq!(indices.into_iter().bit_conjoin(11)[..data.len()], data);
    }
}

/// # Reference
///   <https://iancoleman.io/bip39/>
const ENTROPY_LIST: &[&str] = &[
    "0d5403ae28c460ddfefc967af18962b44d7c6fb23dcfc79a0f288a62895080f8",
    "d88958cc02f09994dc0816411cc0b19195aaf987adada5ab44e19fe5b8c4c48b",
    "cf9b4e9b73f62f92d0b802931c01fae73eeeab725bd214c5",
    "5174bb1dddfc6e2fef4e47df6fcc046a48d195b9",
];
const INDICES_LIST: &[&[u16]] = &[
    &[
        // 00001101010 10100000000 11101011100 01010001100 01000110000 01101110111 11111011111 10010010110
        // 01111010111 10001100010 01011000101 01101000100 11010111110 00110111110 11001000111 10111001111
        // 11000111100 11010000011 11001010001 00010100110 00101000100 10101000010 00000011111 000
        106, 1280, 1884, 652, 560, 887, 2015, 1174, //..
        983, 1122, 709, 836, 1726, 446, 1607, 1487, //..
        1596, 1667, 1617, 166, 324, 1346, 31, 0,
    ],
    &[
        // 11011000100 01001010110 00110011000 00000101111 00001001100 11001010011 01110000001 00000010110
        // 01000001000 11100110000 00101100011 00100011001 01011010101 01111100110 00011110101 10110101101
        // 10100101101 01011010001 00111000011 00111111110 01011011100 01100010011 00010010001 011
        1732, 598, 408, 47, 76, 1619, 897, 22, //..
        520, 1840, 355, 281, 725, 998, 245, 1453, //..
        1325, 721, 451, 510, 732, 787, 145, 768, // 0b011_0000_0000,
    ],
    &[
        // 11001111100 11011010011 10100110110 11100111111 01100010111 11001001011 01000010111 00000000010
        // 10010011000 11100000000 01111110101 11001110011 11101110111 01010101101 11001001011 01111010010
        // 00010100110 00101
        1660, 1747, 1334, 1855, 791, 1611, 535, 2, //..
        1176, 1792, 1013, 1651, 1911, 685, 1611, 978, //..
        166, 320, // 0b001_0100_0000
    ],
    &[
        // 01010001011 10100101110 11000111011 10111011111 11000110111 00010111111 10111101001 11001000111
        // 11011111011 01111110011 00000001000 11010100100 10001101000 11001010110 111001
        651, 1326, 1595, 1503, 1591, 191, 1513, 1607, //..
        1787, 1011, 8, 1700, 1128, 1622, 1824, // 0b111_0010_0000
    ],
];

#[test]
#[should_panic]
fn test_chunks_overflow() {
    let _: Vec<u8> = [8_u8; 8].bit_chunks(11).collect::<Vec<_>>();
}

#[test]
#[should_panic]
fn test_conjoin_overflow() {
    let _ = [222_u32].into_iter().bit_conjoin(0);
}
