use imag_num;
use imag_num::foundation::{Int, Float, NumberKind};

fn test_basic() {
    let int1 = Int::new_from_i64(42);
    assert_eq!(int1.digits, "42");
    assert!(!int1.negative);
    assert_eq!(int1.kind, NumberKind::Finite);

    let int2 = Int::new_from_str("-12345");
    assert_eq!(int2.digits, "12345");
    assert!(int2.negative);
    assert_eq!(int2.kind, NumberKind::Finite);

    let float1 = Float::new_from_f64(3.14);
    assert_eq!(float1.mantissa, "314");
    assert_eq!(float1.exponent, 0);
    assert!(!float1.negative);
}

fn main() {
    // Test basic functionality
    test_basic();

    // You can add more tests here for other functionalities
    println!("All tests passed!");
}
