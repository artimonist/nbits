# Bit operations on buffer  

### Usage  
  [u16] chunks to mnemonic indices by 11 bits.    
  [u8] chunks to base64 indices by 6 bits.  

### Examples  
```
use bits::{BitChunks, BitConjoin, BitIterator, ToBits};

assert_eq!(
    vec![0b1111_1111, 0b1111_1111].bit_chunks(6).collect::<Vec<u8>>(),
    vec![0b11_1111, 0b11_1111, 0b11_1100]
);
assert_eq!(
    vec![0b1111_1111; 3].bit_chunks(11).collect::<Vec<u16>>(),
    vec![0b111_1111_1111, 0b111_1111_1111, 0b110_0000_0000]
);
assert_eq!(
    vec![0b11_1111_u8, 0b11_1111, 0b11_1111].bit_conjoin(6),
    vec![0b1111_1111, 0b1111_1111, 0b1100_0000]
);
assert_eq!(
    vec![0b1111_u16, 0b1111, 0b1111].bit_conjoin(6),
    vec![0b001111_00, 0b1111_0011, 0b1100_0000]
);
assert_eq!(
    [0b1111_0000_u8].bit_iter().collect::<Vec<bool>>(),
    vec![true, true, true, true, false, false, false, false]
);
assert_eq!(
    vec![true, true, true, true, false, false, false, false].iter().to_bits(),
    [0b1111_0000]
);
```