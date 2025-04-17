#![allow(unused)]

mod arith;
mod bitwise;
mod convert;
mod offset;

pub(self) use arith::{bits_add_overflow, bits_sub_overflow};
pub(self) use bitwise::{bits_and, bits_not, bits_or, bits_xor};
pub(self) use offset::{bits_shl, bits_shr};

/**
 * A struct representing a fixed-size array of bits.
 */

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bits<const N: usize>(pub [u8; N]);

impl<const N: usize> Bits<N> {
    pub fn new() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> Default for Bits<N> {
    fn default() -> Self {
        Self([0; N])
    }
}
