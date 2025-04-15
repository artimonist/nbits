#![cfg(test)]
use hex::FromHex;
use nbits::*;

#[test]
fn test_bit_chunks() {
    for (i, &entropy) in ENTROPY_LIST.iter().enumerate() {
        let data = Vec::from_hex(entropy).expect("entropy");
        let mut rem = 0;
        let indices: Vec<u16> = data.bit_chunks_rem(11, &mut rem).collect();
        assert_eq!(indices, INDICES_LIST[i]);
        assert_eq!(rem, indices.len() / 3);
        assert_eq!(indices.bit_conjoin_rem(11, rem), data);
    }
}

/// # Reference
///   <https://iancoleman.io/bip39/>
const ENTROPY_LIST: &[&str] = &[
    "0d5403ae28c460ddfefc967af18962b44d7c6fb23dcfc79a0f288a62895080f8",
    "d88958cc02f09994dc0816411cc0b19195aaf987adada5ab44e19fe5b8c4c48b",
    "99c8a1870ef86dd6137f86a0fb3b688c57525ae8c9f9e0afdbc98d65",
    "cf9b4e9b73f62f92d0b802931c01fae73eeeab725bd214c5",
    "5174bb1dddfc6e2fef4e47df6fcc046a48d195b9",
    "a594006fa887195dd1726fc870787c1f",
    "97da2890de2af07b87f23769",
    "35a5d1cc27c2233c",
    "33448f2a",
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
        // 10011001110 01000101000 01100001110 00011101111 10000110110 11101011000 01001101111 11110000110
        // 10100000111 11011001110 11011010001 00011000101 01110101001 00101101011 10100011001 00111111001
        // 11100000101 01111110110 11110010011 00011010110 0101
        1230, 552, 782, 239, 1078, 1880, 623, 1926, //..
        1287, 1742, 1745, 197, 937, 363, 1305, 505, //..
        1797, 1014, 1939, 214, 640, // 0b010_1000_0000
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
    &[
        // 10100101100 10100000000 00011011111 01010001000 01110001100 10101110111 01000101110 01001101111
        // 11001000011 10000011110 00011111000 0011111
        1324, 1280, 223, 648, 908, 1399, 558, 623, //..
        1603, 1054, 248, 496, // 0b001_1111_0000
    ],
    &[
        // 10010111110 11010001010 00100100001 10111100010 10101111000 00111101110 00011111110 01000110111
        // 01101001
        1214, 1674, 289, 1506, 1400, 494, 254, 567, //..
        840, // 0b011_0100_1000
    ],
    &[
        // 00110101101 00101110100 01110011000 01001111100 00100010001 100111100
        429, 372, 920, 636, 273, 1264, // 0b100_1111_0000
    ],
    &[
        // 00110011010 00100100011 1100101010
        410, 291, 1620,
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

#[test]
fn test_chunks_debug() {
    assert_eq!(
        [0b1111_1111_u8].bit_chunks(1).collect::<Vec<u8>>(),
        vec![1; 8]
    );
    assert_eq!(
        [0b1111_1111_u8].bit_chunks(4).collect::<Vec<u8>>(),
        vec![0b1111; 2]
    );
}

#[test]
fn test_conjoin_debug() {
    assert_eq!([0b1111, 0b1111].bit_conjoin(4), vec![0b1111_1111]);
    assert_eq!(
        [0xFF, 0xFF].bit_conjoin(10),
        vec![0b0011_1111, 0b1100_1111, 0b1111_0000]
    );
    assert_eq!([1; 8].bit_conjoin(1), vec![0b1111_1111]);
    assert_eq!([1; 8].bit_conjoin(2), vec![0b0101_0101, 0b0101_0101]);
    assert_eq!([1; 16].bit_conjoin(1), vec![0b1111_1111, 0b1111_1111]);
}
