use imagnum::{create_float, create_int};

#[test]
fn int_ref_ops() {
    let a = create_int("10");
    let b = create_int("3");

    let add = (&a + &b).unwrap();
    assert_eq!(format!("{}", add), "13");

    let sub = (&a - &b).unwrap();
    assert_eq!(format!("{}", sub), "7");

    let mul = (&a * &b).unwrap();
    assert_eq!(format!("{}", mul), "30");

    let div = (&a / &b).unwrap();
    // integer division returns quotient
    assert_eq!(format!("{}", div), "3");

    let rem = (&a % &b).unwrap();
    assert_eq!(format!("{}", rem), "1");
}

#[test]
fn float_ref_ops() {
    let x = create_float("6");
    let y = create_float("4");

    let add = (&x + &y).unwrap();
    assert_eq!(format!("{}", add), "10.0");

    let sub = (&x - &y).unwrap();
    assert_eq!(format!("{}", sub), "2.0");

    let mul = (&x * &y).unwrap();
    assert_eq!(format!("{}", mul), "24.0");

    let div = (&x / &y).unwrap();
    assert_eq!(format!("{}", div), "1.5");

    let rem = (&x % &y).unwrap();
    assert_eq!(format!("{}", rem), "2.0");
}
