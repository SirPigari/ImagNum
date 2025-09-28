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

#[test]
fn test_7div2_equal_3dot5() {
    let a = (imagnum::create_float("7") / imagnum::create_float("2")).expect("7/2 failed");
    let b = imagnum::create_float("3.5");
    assert_eq!(a, b, "7/2 should equal 3.5");

    let c = imagnum::create_int("137").to_float().expect("int to float failed");
    let res1 = (&c / &a).expect("137/(7/2) failed");
    let res2 = (&c / &b).expect("137/3.5 failed");

    assert_eq!(res1, res2, "137/(7/2) should equal 137/3.5");
}
