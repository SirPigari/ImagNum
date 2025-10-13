use imagnum::ApproxEq;
use imagnum::functions::{create_int, create_float, create_complex};

#[test]
fn test_int_approx_eq_exact() {
    let a = create_int("100");
    let b = create_int("100");
    assert!(a.approx_eq(&b, 0.0));
}

#[test]
fn test_int_approx_eq_within_epsilon() {
    let a = create_int("100");
    let b = create_int("101");
    assert!(a.approx_eq(&b, 1.5));
    assert!(!a.approx_eq(&b, 0.5));
}

#[test]
fn test_int_approx_eq_outside_epsilon() {
    let a = create_int("100");
    let b = create_int("110");
    assert!(!a.approx_eq(&b, 5.0));
    assert!(a.approx_eq(&b, 10.0));
}

#[test]
fn test_float_approx_eq_exact() {
    let a = create_float("3.14159");
    let b = create_float("3.14159");
    assert!(a.approx_eq(&b, 0.0));
}

#[test]
fn test_float_approx_eq_within_epsilon() {
    let a = create_float("3.14159");
    let b = create_float("3.14160");
    assert!(a.approx_eq(&b, 0.001));
    
    let c = create_float("3.14159");
    let d = create_float("3.15");
    assert!(!c.approx_eq(&d, 0.001));
}

#[test]
fn test_float_approx_eq_nan() {
    use imagnum::foundation::NAN;
    let a = create_float("3.14");
    assert!(!a.approx_eq(&NAN, 1.0));
    assert!(!NAN.approx_eq(&NAN, 1.0));
}

#[test]
fn test_float_approx_eq_infinity() {
    use imagnum::foundation::{INFINITY, NEG_INFINITY};
    let a = create_float("3.14");
    
    assert!(INFINITY.approx_eq(&INFINITY, 1.0));
    assert!(NEG_INFINITY.approx_eq(&NEG_INFINITY, 1.0));
    assert!(!INFINITY.approx_eq(&NEG_INFINITY, 1.0));
    assert!(!a.approx_eq(&INFINITY, 1.0));
}

#[test]
fn test_complex_approx_eq() {
    let a = create_complex("1", "2");
    let b = create_complex("1.001", "2.001");
    
    // Small difference within epsilon
    assert!(a.approx_eq(&b, 0.01));
    
    // Larger difference outside epsilon
    let c = create_complex("1", "2");
    let d = create_complex("5", "8");
    assert!(!c.approx_eq(&d, 1.0));
    assert!(c.approx_eq(&d, 10.0));
}

#[test]
fn test_complex_vs_real_approx_eq() {
    let a = create_complex("1", "2");
    let b = create_float("1");
    
    // Complex and real should never be approximately equal
    assert!(!a.approx_eq(&b, 10.0));
}

#[test]
fn test_float_approx_eq_precision() {
    let a = create_float("0.1");
    let b = create_float("0.10001");
    
    assert!(a.approx_eq(&b, 0.001));
    
    let c = create_float("0.1");
    let d = create_float("0.2");
    assert!(!c.approx_eq(&d, 0.001));
}
