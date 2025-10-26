use imagnum::{Float, Int};
use num_bigint::BigInt;
use bigdecimal::BigDecimal;
use imagnum::errors::{NEGATIVE_RESULT, NUMBER_TOO_LARGE};

#[test]
fn test_int_bitand() {
    let a = Int::from(0b1010);
    let b = Int::from(0b1100);
    // ref
    assert_eq!(&a & &b, Ok(Int::from(0b1000)));
    // owned
    assert_eq!(a & b, Ok(Int::from(0b1000)));
}

#[test]
fn test_int_big_bitand() {
    let a = Int::Big(BigInt::from(0b1010));
    let b = Int::Big(BigInt::from(0b1100));
    assert_eq!(a & b, Ok(Int::Big(BigInt::from(0b1000))));
}

#[test]
fn test_int_bitor() {
    let a = Int::from(0b1010);
    let b = Int::from(0b1100);
    // ref
    assert_eq!(&a | &b, Ok(Int::from(0b1110)));
    // owned
    assert_eq!(a | b, Ok(Int::from(0b1110)));
}

#[test]
fn test_int_bitxor() {
    let a = Int::from(0b1010);
    let b = Int::from(0b1100);
    // ref
    assert_eq!(&a ^ &b, Ok(Int::from(0b0110)));
    // owned
    assert_eq!(a ^ b, Ok(Int::from(0b0110)));
}

#[test]
fn test_int_xnor() {
    let a = Int::from(0b1010);
    let b = Int::from(0b1100);
    let xor = (&a ^ &b).unwrap();
    let expected = !xor;
    assert_eq!(a.xnor(&b), Ok(expected));
}

#[test]
fn test_int_shl() {
    let a = Int::from(1);
    let shift = Int::from(2);
    // ref
    assert_eq!(&a << &shift, Ok(Int::from(4)));
    // owned
    assert_eq!(a << shift, Ok(Int::from(4)));
}

#[test]
fn test_int_shr() {
    let a = Int::from(4);
    let shift = Int::from(2);
    // ref
    assert_eq!(&a >> &shift, Ok(Int::from(1)));
    // owned
    assert_eq!(a >> shift, Ok(Int::from(1)));
}

#[test]
fn test_int_not() {
    let a = Int::from(0b1010);
    let expected = Int::Big(!BigInt::from(0b1010));
    assert_eq!(!&a, expected.clone());
    assert_eq!(!a, expected);
}

#[test]
fn test_int_negative_shift() {
    let a = Int::from(1);
    let shift = Int::from(-1);
    assert_eq!(a << shift, Err(NEGATIVE_RESULT));
}

#[test]
fn test_int_large_shift() {
    let a = Int::from(1);
    let shift = Int::Big(BigInt::from(usize::MAX as u128 + 1));
    assert_eq!(a << shift, Err(NUMBER_TOO_LARGE));
}

#[test]
fn test_int_big_negative() {
    let a = Int::Big(BigInt::from(-10));
    let b = Int::Big(BigInt::from(5));
    assert_eq!(a & b, Ok(Int::Big(BigInt::from(-10) & BigInt::from(5))));
}

// For Float
#[test]
fn test_float_bitand() {
    let a = Float::from(1.0f32);
    let b = Float::from(2.0f32);
    // 1.0 f32 bits: 0x3f800000
    // 2.0 f32 bits: 0x40000000
    // & : 0x00000000 -> 0.0
    // ref
    assert_eq!(&a & &b, Ok(Float::from(0.0f32)));
    // owned
    assert_eq!(a & b, Ok(Float::from(0.0f32)));
}

#[test]
fn test_float_bitor() {
    let a = Float::from(1.0f32);
    let b = Float::from(2.0f32);
    // | : 0x7f800000 -> inf
    // ref
    assert_eq!(&a | &b, Ok(Float::Infinity));
    // owned
    assert_eq!(a | b, Ok(Float::Infinity));
}

#[test]
fn test_float_bitxor() {
    let a = Float::from(1.0f32);
    let b = Float::from(2.0f32);
    // ^ : 0x7f800000 -> inf
    // ref
    assert_eq!(&a ^ &b, Ok(Float::Infinity));
    // owned
    assert_eq!(a ^ b, Ok(Float::Infinity));
}

#[test]
fn test_float_xnor() {
    let a = Float::from(1.0f32);
    let b = Float::from(2.0f32);
    let xor = (&a ^ &b).unwrap();
    let expected = !xor;
    assert_eq!(a.xnor(&b), Ok(expected));
}

#[test]
fn test_float_shl() {
    let a = Float::from(1.0f32);
    let shift = Int::from(1);
    // Shifted bits
    // ref
    let res_ref = (&a << &shift).unwrap();
    assert_ne!(res_ref, Float::from(1.0f32));
    // owned
    let res_owned = (a << shift).unwrap();
    assert_ne!(res_owned, Float::from(1.0f32));
}

#[test]
fn test_float_shr() {
    let a = Float::from(1.0f32);
    let shift = Int::from(1);
    // Shifted bits
    // ref
    let res_ref = (&a >> &shift).unwrap();
    assert_ne!(res_ref, Float::from(1.0f32));
    // owned
    let res_owned = (a >> shift).unwrap();
    assert_ne!(res_owned, Float::from(1.0f32));
}

#[test]
fn test_float_not() {
    let a = Float::from(1.0f32);
    let res_ref = !&a;
    assert_ne!(res_ref, Float::from(1.0f32));
    let res_owned = !a;
    assert_ne!(res_owned, Float::from(1.0f32));
}

#[test]
fn test_float_big_supported() {
    let a = Float::Big(BigDecimal::from(1));
    let b = Float::Big(BigDecimal::from(2));
    assert!((a & b).is_ok());
}

#[test]
fn test_float_big_too_large() {
    let large = BigDecimal::new(BigInt::from(1) << 1024, 0);
    let a = Float::Big(large.clone());
    let b = Float::Big(BigDecimal::from(2));
    assert!((a & b).is_ok());
}

// Test large big int
#[test]
fn test_int_big_large() {
    let a = Int::Big(BigInt::from(1) << 100);
    let b = Int::Big(BigInt::from(1) << 100);
    assert_eq!(a & b, Ok(Int::Big(BigInt::from(1) << 100)));
}

#[test]
fn test_int_big_large_xor() {
    let a = Int::Big(BigInt::from(1) << 100);
    let b = Int::Big(BigInt::from(0));
    assert_eq!(a ^ b, Ok(Int::Big(BigInt::from(1) << 100)));
}
