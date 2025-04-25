use super::Bitwise;

/**
 * Converting bit representations to various integer types.
 */
pub trait Convert {
    fn bit_be_u8(&self) -> u8;
    fn bit_be_u16(&self) -> u16;
    fn bit_be_u32(&self) -> u32;
    fn bit_be_u64(&self) -> u64;
    fn bit_be_u128(&self) -> u128;

    fn bit_le_u8(&self) -> u8;
    fn bit_le_u16(&self) -> u16;
    fn bit_le_u32(&self) -> u32;
    fn bit_le_u64(&self) -> u64;
    fn bit_le_u128(&self) -> u128;
}

impl Convert for [u8] {
    fn bit_be_u8(&self) -> u8 {
        if self.len() > 1 && self[..self.len() - 1].iter().any(|&v| v != 0) {
            panic!("Bits value overflow");
        }
        self.last().copied().unwrap_or_default()
        // match self.len() {
        //     0 => panic!("Bits buffer is empty"),
        //     1 => self[0],
        //     n @ _ => {
        //         assert!(self[..n - 1].bit_all_zero(), "Bits value overflow");
        //         self[n - 1]
        //     }
        // }
    }

    fn bit_be_u16(&self) -> u16 {
        match self.len() {
            0 => panic!("Bits buffer is empty"),
            1 => self[0] as u16,
            2 => u16::from_be_bytes([self[0], self[1]]),
            n @ _ => {
                assert!(self[..n - 2].bit_all_zero(), "Bits value overflow");
                u16::from_be_bytes([self[n - 2], self[n - 1]])
            }
        }
    }

    fn bit_be_u32(&self) -> u32 {
        match self.len() {
            0 => panic!("Bits buffer is empty"),
            1 => self[0] as u32,
            2 => u32::from_be_bytes([0, 0, self[0], self[1]]),
            3 => u32::from_be_bytes([0, self[0], self[1], self[2]]),
            4 => u32::from_be_bytes([self[0], self[1], self[2], self[3]]),
            n @ _ => {
                assert!(self[..n - 4].bit_all_zero(), "Bits value overflow");
                u32::from_be_bytes([self[n - 4], self[n - 3], self[n - 2], self[n - 1]])
            }
        }
    }

    fn bit_be_u64(&self) -> u64 {
        let mut bytes = [0; 8];
        if self.len() > 8 {
            assert!(
                self[..self.len() - 8].iter().all(|&v| v == 0),
                "Bits value overflow"
            );
        }
        match self.len() {
            0 => panic!("Bits buffer is empty"),
            1 => bytes[7..].copy_from_slice(&self[0..]),
            2 => bytes[6..].copy_from_slice(&self[0..]),
            3 => bytes[5..].copy_from_slice(&self[0..]),
            4 => bytes[4..].copy_from_slice(&self[0..]),
            5 => bytes[3..].copy_from_slice(&self[0..]),
            6 => bytes[2..].copy_from_slice(&self[0..]),
            7 => bytes[1..].copy_from_slice(&self[0..]),
            8 => bytes[0..].copy_from_slice(&self[0..]),
            n @ _ => bytes.copy_from_slice(&self[n - 8..]),
        }
        match self.len() {
            0 => {}
            n @ ..8 => bytes[8 - n..].copy_from_slice(self),
            n @ _ => bytes.copy_from_slice(&self[n - 8..]),
        }
        u64::from_be_bytes(bytes)
    }

    fn bit_be_u128(&self) -> u128 {
        match self.len() {
            0 => panic!("Bits buffer is empty"),
            n @ ..=16 => {
                let mut bytes = [0; 16];
                bytes[16 - n..].copy_from_slice(self);
                u128::from_be_bytes(bytes)
            }
            n => {
                assert!(self[..n - 16].bit_all_zero(), "Bits value overflow");
                u128::from_be_bytes(self[n - 16..].try_into().unwrap())
            }
        }
    }

    fn bit_le_u8(&self) -> u8 {
        todo!()
    }

    fn bit_le_u16(&self) -> u16 {
        todo!()
    }

    fn bit_le_u32(&self) -> u32 {
        todo!()
    }

    fn bit_le_u64(&self) -> u64 {
        todo!()
    }

    fn bit_le_u128(&self) -> u128 {
        todo!()
    }
}

// pub trait TryConvert<U> {
//     type Error;

//     fn bits_value(&self) -> Result<U, Self::Error>;
// }

// impl<U> TryConvert<U> for [u8]
// where
//     U: Into<u128>,
// {
//     type Error = &'static str;

//     fn bits_value(&self) -> Result<U, Self::Error> {
//         let size = std::mem::size_of::<U>();
//         match self.len() {
//             n if n <= size => {
//                 let mut bytes = vec![0; size];
//                 bytes[size - n..].copy_from_slice(self);
//                 Ok(U::from_be_bytes(bytes.try_into().unwrap()))
//             }
//             _ => {
//                 assert!(
//                     self[..self.len() - size].bit_all_zero(),
//                     "Bits value overflow"
//                 );
//                 Ok(U::from_be_bytes(
//                     self[self.len() - size..].try_into().unwrap(),
//                 ))
//             }
//         }
//     }
// }
