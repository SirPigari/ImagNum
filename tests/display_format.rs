use imagnum::create_int;

#[test]
fn test_recurring_display_1_over_3() {
    let one = create_int("1");
    let three = create_int("3");
    let f1 = one.to_float().unwrap();
    let f3 = three.to_float().unwrap();
    let res = f1._div(&f3).unwrap();
    let s = format!("{}", res);
    assert_eq!(s, "0.(3)");
}

#[test]
fn test_nonrecurring_display_1_over_8() {
    let one = create_int("1");
    let eight = create_int("8");
    let f1 = one.to_float().unwrap();
    let f8 = eight.to_float().unwrap();
    let res = f1._div(&f8).unwrap();
    let s = format!("{}", res);
    assert_eq!(s, "0.125");
}

#[test]
fn test_recurring_preservation_after_add() {
    let one = create_int("1");
    let three = create_int("3");
    let f1 = one.to_float().unwrap();
    let f3 = three.to_float().unwrap();
    let third = f1._div(&f3).unwrap();
    let sum = third._add(&f1).unwrap();
    let s = format!("{}", sum);
    assert_eq!(s, "1.(3)");
}
