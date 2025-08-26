use imagnum::create_int;
use imagnum::foundation::{Float, Int, SmallFloat, SmallInt};

#[test]
fn test_small_int_basic_ops() {
    // create small ints directly
    let a = Int::Small(SmallInt::I32(10));
    let b = Int::Small(SmallInt::I32(3));

    // addition
    let sum = a._add(&b).unwrap();
    let (digits, neg, _k) = imagnum::compat::int_to_parts(&sum);
    assert_eq!(digits, "13");
    assert!(!neg);

    // subtraction
    let diff = a._sub(&b).unwrap();
    let (digits, neg, _k) = imagnum::compat::int_to_parts(&diff);
    assert_eq!(digits, "7");
    assert!(!neg);

    // multiplication
    let prod = a._mul(&b).unwrap();
    let (digits, _, _) = imagnum::compat::int_to_parts(&prod);
    assert_eq!(digits, "30");

    // division
    let quot = a._div(&b).unwrap();
    let (digits, _, _) = imagnum::compat::int_to_parts(&quot);
    assert_eq!(digits, "3");

    // modulo
    let rem = a._modulo(&b).unwrap();
    let (digits, _, _) = imagnum::compat::int_to_parts(&rem);
    assert_eq!(digits, "1");
}

#[test]
fn test_small_int_and_big_interop() {
    let small = Int::Small(SmallInt::I32(100));
    let big = create_int("250");
    let res = small._add(&big).unwrap();
    let (digits, _, _) = imagnum::compat::int_to_parts(&res);
    assert_eq!(digits, "350");
}

#[test]
fn test_small_float_basic_ops_and_conversions() {
    let a = Float::Small(SmallFloat::F64(3.5));
    let b = Float::Small(SmallFloat::F64(1.25));

    // to_f64
    assert!((a.to_f64().unwrap() - 3.5).abs() < 1e-12);
    assert!((b.to_f64().unwrap() - 1.25).abs() < 1e-12);

    // add
    let sum = a._add(&b).unwrap();
    let approx = sum.to_f64().unwrap();
    assert!((approx - 4.75).abs() < 1e-9);

    // mul
    let prod = a._mul(&b).unwrap();
    let approx = prod.to_f64().unwrap();
    assert!((approx - (3.5 * 1.25)).abs() < 1e-9);

    // sqrt
    let s = b.sqrt().unwrap();
    let approx = s.to_f64().unwrap();
    assert!((approx - 1.25f64.sqrt()).abs() < 1e-9);

    // to_int for integer-like float
    let fi = Float::Small(SmallFloat::F64(42.0));
    let i = fi.to_int().unwrap();
    let (digits, neg, _k) = imagnum::compat::int_to_parts(&i);
    assert_eq!(digits, "42");
    assert!(!neg);
}

#[test]
fn test_small_float_transcendentals() {
    let sf = Float::Small(SmallFloat::F64(0.0));
    // sin(0)=0, cos(0)=1
    let s = sf.sin().unwrap();
    assert!(s.to_f64().unwrap().abs() < 1e-12);
    let c = sf.cos().unwrap();
    assert!((c.to_f64().unwrap() - 1.0).abs() < 1e-12);
}
