use imagnum::{create_int, create_float};
use imagnum::foundation::Int;

#[test]
fn int_ord_big_vs_big() {
    let a = create_int("123456789012345678901234567890");
    let b = create_int("123456789012345678901234567891");
    assert!(a < b, "big vs big: a < b");
}

#[test]
fn int_ord_small_vs_small() {
    let a = Int::new_small(42i32);
    let b = Int::new_small(100i32);
    assert!(a < b, "small vs small: 42 < 100");
}

#[test]
fn int_ord_mixed_small_big() {
    let a = Int::new_small(42i32);
    let b = create_int("12345678901234567890");
    assert!(a < b, "mixed small/big: 42 < big");
}

#[test]
fn float_ord_mixed_small_big_example() {
    // verify the earlier REPL example stays true
    let a = create_float("4.3902176");
    let b = create_float("3.0");
    assert!(a >= b, "4.3902176 >= 3.0 should be true");
}
