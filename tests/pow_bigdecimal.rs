use bigdecimal::BigDecimal;
use imagnum::math;
use num_bigint::BigInt;
use std::str::FromStr;

#[test]
fn test_cube_root_of_27() {
    let base = BigDecimal::from_str("27.0").unwrap();
    let num = BigInt::from(1u32);
    let den = BigInt::from(3u32);
    let (res, exact) = math::pow_bigdecimal_rational(&base, &num, &den, 50).unwrap();
    // exact cube root of 27 is 3
    assert!(exact || res.to_string().starts_with("3"));
}
