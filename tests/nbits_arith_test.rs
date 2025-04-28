#![cfg(test)]

use nbits::NBits as Bits;

#[test]
fn test_bits_arith() {
    assert_eq!(Bits([0, 0]) + &Bits([0, 0]), Bits([0, 0]));
    assert_eq!(Bits([0, 0]) + &Bits([255, 255]), Bits([255, 255]));
    assert_eq!(Bits([55, 255]) + &Bits([30, 10]), Bits([86, 9]));
    assert_eq!(Bits([155, 255]) + &Bits([30, 100]), Bits([186, 99]));
    assert_eq!(Bits([0, 1, 1]) + &Bits([5, 255, 255]), Bits([6, 1, 0]));

    assert_eq!(Bits([0, 0]) - &Bits([0, 0]), Bits([0, 0]));
    assert_eq!(Bits([255, 255]) - &Bits([0, 0]), Bits([255, 255]));
    assert_eq!(Bits([55, 255]) - &Bits([30, 10]), Bits([25, 245]));
    assert_eq!(Bits([155, 0]) - &Bits([30, 100]), Bits([124, 156]));
    assert_eq!(Bits([155, 100]) - &Bits([30, 255]), Bits([124, 101]));
    assert_eq!(Bits([1, 5, 0]) - &Bits([0, 255, 255]), Bits([0, 5, 1]));

    assert_eq!(Bits([0, 0]) * &Bits([0, 0]), Bits([0, 0]));
    assert_eq!(Bits([0, 0]) * &Bits([255, 255]), Bits([0, 0]));
    assert_eq!(Bits([0, 1]) * &Bits([255, 10]), Bits([255, 10]));
    assert_eq!(Bits([0, 0, 1]) * &Bits([5, 255, 255]), Bits([5, 255, 255]));
    assert_eq!(Bits([1, 1, 1]) * &Bits([0, 0, 255]), Bits([255, 255, 255]));
    assert_eq!(Bits([0, 1, 255]) * &Bits([0, 0, 255]), Bits([1, 253, 1]));
}

#[test]
#[should_panic]
fn test_add_overflow() {
    let _ = Bits([255, 255]) + &Bits([0, 1]);
}

#[test]
#[should_panic]
fn test_sub_overflow() {
    let _ = Bits([0, 0]) - &Bits([0, 1]);
}
