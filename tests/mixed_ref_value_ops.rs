use imagnum::{create_float, create_int};

#[test]
fn int_mixed_ref_value_ops() {
    let a = create_int("10");
    let b = create_int("3");

    // a + &b
    let add1 = (a.clone() + &b).unwrap();
    assert_eq!(format!("{}", add1), "13");

    // &a + b
    let add2 = (&a + b.clone()).unwrap();
    assert_eq!(format!("{}", add2), "13");

    // a - &b
    let sub1 = (a.clone() - &b).unwrap();
    assert_eq!(format!("{}", sub1), "7");

    // &a - b
    let sub2 = (&a - b.clone()).unwrap();
    assert_eq!(format!("{}", sub2), "7");

    // a * &b
    let mul1 = (a.clone() * &b).unwrap();
    assert_eq!(format!("{}", mul1), "30");

    // &a * b
    let mul2 = (&a * b.clone()).unwrap();
    assert_eq!(format!("{}", mul2), "30");

    // a / &b
    let div1 = (a.clone() / &b).unwrap();
    assert_eq!(format!("{}", div1), "3");

    // &a / b
    let div2 = (&a / b.clone()).unwrap();
    assert_eq!(format!("{}", div2), "3");

    // a % &b
    let rem1 = (a.clone() % &b).unwrap();
    assert_eq!(format!("{}", rem1), "1");

    // &a % b
    let rem2 = (&a % b.clone()).unwrap();
    assert_eq!(format!("{}", rem2), "1");
}

#[test]
fn float_mixed_ref_value_ops() {
    let x = create_float("6");
    let y = create_float("4");

    // a + &b
    let add1 = (x.clone() + &y).unwrap();
    assert_eq!(format!("{}", add1), "10.0");

    // &a + b
    let add2 = (&x + y.clone()).unwrap();
    assert_eq!(format!("{}", add2), "10.0");

    // a - &b
    let sub1 = (x.clone() - &y).unwrap();
    assert_eq!(format!("{}", sub1), "2.0");

    // &a - b
    let sub2 = (&x - y.clone()).unwrap();
    assert_eq!(format!("{}", sub2), "2.0");

    // a * &b
    let mul1 = (x.clone() * &y).unwrap();
    assert_eq!(format!("{}", mul1), "24.0");

    // &a * b
    let mul2 = (&x * y.clone()).unwrap();
    assert_eq!(format!("{}", mul2), "24.0");

    // a / &b
    let div1 = (x.clone() / &y).unwrap();
    assert_eq!(format!("{}", div1), "1.5");

    // &a / b
    let div2 = (&x / y.clone()).unwrap();
    assert_eq!(format!("{}", div2), "1.5");

    // a % &b
    let rem1 = (x.clone() % &y).unwrap();
    assert_eq!(format!("{}", rem1), "2.0");

    // &a % b
    let rem2 = (&x % y.clone()).unwrap();
    assert_eq!(format!("{}", rem2), "2.0");
}
