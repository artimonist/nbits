/// Enumerate the bits in the buffer  
///
/// # Examples
/// ```
/// # use nbits::BitIterator;
/// assert_eq!(
///     [0b1111_0000_u8].bit_iter().collect::<Vec<bool>>(),
///     vec![true, true, true, true, false, false, false, false]
/// );
/// ```
pub trait BitIterator {
    /// iterator of bit values
    fn bit_iter(self) -> impl DoubleEndedIterator<Item = bool>;
}

impl BitIterator for &[u8] {
    fn bit_iter(self) -> impl DoubleEndedIterator<Item = bool> {
        self.iter()
            .flat_map(|&v| (0_u8..8).rev().map(move |n| (v & (1 << n)) != 0))
    }
}

/// Converting enumerated bool values to buffer  
///
/// # Examples
/// ```
/// # use nbits::ToBits;
/// assert_eq!(
///     vec![true, true, true, true, false, false, false, false].iter().to_bits(),
///     [0b1111_0000]
/// );
/// ```
pub trait ToBits<T> {
    fn to_bits(self) -> Vec<u8>;
}

impl<U> ToBits<bool> for U
where
    U: Iterator<Item = bool>,
{
    fn to_bits(self) -> Vec<u8> {
        let mut v = 0_u8;
        self.chain([false; 7])
            .enumerate()
            .filter_map(|(i, bit)| {
                let n = 7 - (i % 8);
                if bit {
                    v |= 1 << n;
                }
                match n {
                    0 => Some(std::mem::take(&mut v)),
                    _ => None,
                }
            })
            .collect()
    }
}

impl<'a, U> ToBits<&bool> for U
where
    U: Iterator<Item = &'a bool>,
{
    fn to_bits(self) -> Vec<u8> {
        self.copied().to_bits()
    }
}
