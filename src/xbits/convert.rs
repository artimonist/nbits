use super::Bitwise;

pub trait TryConvert<U> {
    type Error;

    fn bits_value(&self) -> Result<U, Self::Error>;
}

impl<U> TryConvert<U> for [u8]
where
    U: Into<u128>,
{
    type Error = &'static str;

    fn bits_value(&self) -> Result<U, Self::Error> {
        let size = std::mem::size_of::<U>();
        match self.len() {
            n if n <= size => {
                let mut bytes = vec![0; size];
                bytes[size - n..].copy_from_slice(self);
                Ok(U::from_be_bytes(bytes.try_into().unwrap()))
            }
            _ => {
                assert!(
                    self[..self.len() - size].bits_all_zero(),
                    "Bits value overflow"
                );
                Ok(U::from_be_bytes(
                    self[self.len() - size..].try_into().unwrap(),
                ))
            }
        }
    }
}

/**
 * Converting bit representations to various integer types.
 */
pub trait Convert {
    fn bits_to_u8(&self) -> u8;
    fn bits_to_u16(&self) -> u16;
    fn bits_to_u32(&self) -> u32;
    fn bits_to_u64(&self) -> u64;
    fn bits_to_u128(&self) -> u128;
}

impl Convert for [u8] {
    fn bits_to_u8(&self) -> u8 {
        match self.len() {
            0 => panic!("Bits buffer is empty"),
            1 => self[0],
            n @ _ => {
                assert!(self[..n - 1].bits_all_zero(), "Bits value overflow");
                self[n - 1]
            }
        }
    }

    fn bits_to_u16(&self) -> u16 {
        match self.len() {
            0 => panic!("Bits buffer is empty"),
            1 => self[0] as u16,
            2 => u16::from_be_bytes([self[0], self[1]]),
            n @ _ => {
                assert!(self[..n - 2].bits_all_zero(), "Bits value overflow");
                u16::from_be_bytes([self[n - 2], self[n - 1]])
            }
        }
    }

    fn bits_to_u32(&self) -> u32 {
        match self.len() {
            0 => panic!("Bits buffer is empty"),
            1 => self[0] as u32,
            2 => u32::from_be_bytes([0, 0, self[0], self[1]]),
            3 => u32::from_be_bytes([0, self[0], self[1], self[2]]),
            4 => u32::from_be_bytes([self[0], self[1], self[2], self[3]]),
            n @ _ => {
                assert!(self[..n - 4].bits_all_zero(), "Bits value overflow");
                u32::from_be_bytes([self[n - 4], self[n - 3], self[n - 2], self[n - 1]])
            }
        }
    }

    fn bits_to_u64(&self) -> u64 {
        match self.len() {
            0 => panic!("Bits buffer is empty"),
            n @ ..=8 => {
                let mut bytes = [0; 8];
                bytes[8 - n..].copy_from_slice(self);
                u64::from_be_bytes(bytes)
            }
            n => {
                assert!(self[..n - 8].bits_all_zero(), "Bits value overflow");
                u64::from_be_bytes(self[n - 8..].try_into().unwrap())
            }
        }
    }

    fn bits_to_u128(&self) -> u128 {
        match self.len() {
            0 => panic!("Bits buffer is empty"),
            n @ ..=16 => {
                let mut bytes = [0; 16];
                bytes[16 - n..].copy_from_slice(self);
                u128::from_be_bytes(bytes)
            }
            n => {
                assert!(self[..n - 16].bits_all_zero(), "Bits value overflow");
                u128::from_be_bytes(self[n - 16..].try_into().unwrap())
            }
        }
    }
}
