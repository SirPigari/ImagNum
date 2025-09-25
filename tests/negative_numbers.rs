use imagnum::{create_float, create_int};

#[test]
fn negative_int_arithmetic() {
    let a = create_int("-5");
    let b = create_int("2");
    let res = (a * b).unwrap();
    assert_eq!(res.to_string(), "-10");

    let a = create_int("-5");
    let b = create_int("-5");
    let res = (a * b).unwrap();
    assert_eq!(res.to_string(), "25");

    let a = create_int("-3");
    let b = create_int("1");
    let res = (a + b).unwrap();
    assert_eq!(res.to_string(), "-2");
}

#[test]
fn negative_float_arithmetic() {
    let a = create_float("-2.5");
    let b = create_float("4.0");
    let res = (a * b).unwrap();
    assert_eq!(res.to_string(), "-10.0");

    let a = create_float("-2.5");
    let b = create_float("-2.0");
    let res = (a * b).unwrap();
    assert_eq!(res.to_string(), "5.0");
}

#[test]
fn negative_unary_parsing() {
    // create_int should accept leading minus
    let a = create_int("-42");
    assert_eq!(a.to_string(), "-42");

    // create_float should accept leading minus
    let f = create_float("-3.14");
    assert_eq!(f.to_string(), "-3.14");
}

#[test]
fn one_plus_negative() {
    let a = create_int("1");
    let b = create_int("-1");
    let res = (a + b).unwrap();
    assert_eq!(res.to_string(), "0");

    let a = create_float("1.0");
    let b = create_float("-1.0");
    let res = (a + b).unwrap();
    assert_eq!(res.to_string(), "0.0");
}

#[test]
fn negative_multiplication() {
    let a = create_int("-3");
    let b = create_int("4");
    let res = (a * b).unwrap();
    assert_eq!(res.to_string(), "-12");

    let a = create_float("-2.5");
    let b = create_float("4.0");
    let res = (a * b).unwrap();
    assert_eq!(res.to_string(), "-10.0");
}

#[test]
fn negative_division() {
    let a = create_int("-10");
    let b = create_int("2");
    let res = (&a / &b).unwrap();
    assert_eq!(res.to_string(), "-5");

    let c = create_int("-7");
    let d = create_int("-2");
    let res2 = (&c / &d).unwrap();
    let res3 = (&c / &b).unwrap();

    assert_eq!(res2.to_string(), "4");
    assert_eq!(res3.to_string(), "-4");

    let a = create_float("-9.0");
    let b = create_float("3.0");
    let res = (a / b).unwrap();
    assert_eq!(res.to_string(), "-3.0");
}