use imagnum::{create_float, create_int};

#[test]
fn recurring_nine_equals_one() {
    let a = create_float("0.(9)");
    let b = create_float("1");
    assert_eq!(a, b, "0.(9) should compare equal to 1");
}

#[test]
fn recurring_nine_equals_int_one() {
    let a = create_float("0.(9)");
    let i = create_int("1");
    assert_eq!(a, i, "Float 0.(9) should equal Int 1");
}

#[test]
fn big_decimal_recurring_normalization() {
    // additional sanity: a recurring 9 BigDecimal like 0.999... represented
    // as Recurring should equal 1 when compared via Float equality
    let a = create_float("0.(9)");
    let b = create_float("0.9999999999");
    // ensure 0.(9) equals 1
    assert_eq!(a, create_float("1"));
    assert_eq!(a, create_int("1"));
    assert_ne!(a, b);

    assert_eq!(a.to_str(), "1")
}

#[test]
fn recurring_49_equals_05() {
    let a = create_float("0.4(9)");
    let b = create_float("0.5");
    assert_eq!(a, b, "0.4(9) should compare equal to 0.5");
    assert_eq!(a.to_str(), "0.5", "to_str should print normalized 0.5");
}
