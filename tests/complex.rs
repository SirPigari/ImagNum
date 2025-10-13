use imagnum::*;

#[test]
fn test_create_complex() {
    let z = create_complex("3", "4");
    assert_eq!(z.to_string(), "3.0 + 4.0i");
}

#[test]
fn test_complex_addition() {
    // (3 + 4i) + (1 + 2i) = 4 + 6i
    let z1 = create_complex("3", "4");
    let z2 = create_complex("1", "2");
    let result = (z1 + z2).unwrap();
    assert_eq!(result.to_string(), "4.0 + 6.0i");
}

#[test]
fn test_complex_subtraction() {
    // (5 + 7i) - (2 + 3i) = 3 + 4i
    let z1 = create_complex("5", "7");
    let z2 = create_complex("2", "3");
    let result = (z1 - z2).unwrap();
    assert_eq!(result.to_string(), "3.0 + 4.0i");
}

#[test]
fn test_complex_multiplication() {
    // (3 + 2i) * (1 + 4i) = 3 + 12i + 2i + 8i² = 3 + 14i - 8 = -5 + 14i
    let z1 = create_complex("3", "2");
    let z2 = create_complex("1", "4");
    let result = (z1 * z2).unwrap();
    assert_eq!(result.to_string(), "-5.0 + 14.0i");
}

#[test]
fn test_complex_division() {
    // (4 + 2i) / (3 - 1i) = [(4*3 + 2*(-1)) + (2*3 - 4*(-1))i] / (9 + 1)
    //                     = [12 - 2 + 6i + 4i] / 10
    //                     = [10 + 10i] / 10
    //                     = 1 + 1i
    let z1 = create_complex("4", "2");
    let z2 = create_complex("3", "-1");
    let result = (z1 / z2).unwrap();
    // Note: The display shows "1.0 + i" when imaginary part is exactly 1.0
    assert_eq!(result.to_string(), "1.0 + i");
}

#[test]
fn test_pure_imaginary() {
    // 0 + 5i
    let z = create_complex("0", "5");
    assert_eq!(z.to_string(), "5.0i");
}

#[test]
fn test_pure_real() {
    // 7 + 0i should display as just 7
    let z = create_complex("7", "0");
    assert_eq!(z.to_string(), "7.0");
}

#[test]
fn test_imaginary_unit() {
    // 0 + 1i
    let i = create_imaginary();
    assert_eq!(i.to_string(), "i");
}

#[test]
fn test_negative_imaginary() {
    // 0 - 1i
    let z = create_complex("0", "-1");
    assert_eq!(z.to_string(), "-i");
}

#[test]
fn test_complex_with_negative_imaginary() {
    // 3 - 4i
    let z = create_complex("3", "-4");
    assert_eq!(z.to_string(), "3.0 - 4.0i");
}

#[test]
fn test_complex_real_addition() {
    // (3 + 4i) + 2 = 5 + 4i
    let z = create_complex("3", "4");
    let r = create_float("2");
    let result = (z + r).unwrap();
    assert_eq!(result.to_string(), "5.0 + 4.0i");
}

#[test]
fn test_real_complex_addition() {
    // 2 + (3 + 4i) = 5 + 4i
    let r = create_float("2");
    let z = create_complex("3", "4");
    let result = (r + z).unwrap();
    assert_eq!(result.to_string(), "5.0 + 4.0i");
}

#[test]
fn test_complex_real_multiplication() {
    // (3 + 4i) * 2 = 6 + 8i
    let z = create_complex("3", "4");
    let r = create_float("2");
    let result = (z * r).unwrap();
    assert_eq!(result.to_string(), "6.0 + 8.0i");
}

#[test]
fn test_complex_real_division() {
    // (6 + 8i) / 2 = 3 + 4i
    let z = create_complex("6", "8");
    let r = create_float("2");
    let result = (z / r).unwrap();
    assert_eq!(result.to_string(), "3.0 + 4.0i");
}

#[test]
fn test_real_complex_division() {
    // 10 / (3 + 4i) = 10(3 - 4i) / 25 = (30 - 40i) / 25 = 1.2 - 1.6i
    let r = create_float("10");
    let z = create_complex("3", "4");
    let result = (r / z).unwrap();
    assert_eq!(result.to_string(), "1.2 - 1.6i");
}

#[test]
fn test_i_squared() {
    // i * i = -1
    let i = create_imaginary();
    let result = (i.clone() * i).unwrap();
    assert_eq!(result.to_string(), "-1.0");
}

#[test]
fn test_complex_conjugate_multiplication() {
    // (3 + 4i) * (3 - 4i) = 9 - 16i² = 9 + 16 = 25
    let z1 = create_complex("3", "4");
    let z2 = create_complex("3", "-4");
    let result = (z1 * z2).unwrap();
    assert_eq!(result.to_string(), "25.0");
}

#[test]
fn test_complex_zero_addition() {
    // (3 + 4i) + (0 + 0i) = 3 + 4i
    let z1 = create_complex("3", "4");
    let z2 = create_complex("0", "0");
    let result = (z1 + z2).unwrap();
    assert_eq!(result.to_string(), "3.0 + 4.0i");
}

#[test]
fn test_complex_division_by_real() {
    // (8 + 6i) / 2 = 4 + 3i
    let z = create_complex("8", "6");
    let r = create_float("2");
    let result = (z / r).unwrap();
    assert_eq!(result.to_string(), "4.0 + 3.0i");
}

#[test]
fn test_complex_abs() {
    // |3 + 4i| = sqrt(9 + 16) = sqrt(25) = 5
    let z = create_complex("3", "4");
    let result = z.abs();
    assert_eq!(result.to_string(), "5.0");
}

#[test]
fn test_complex_sqrt() {
    // sqrt(3 + 4i)
    let z = create_complex("3", "4");
    let result = z.sqrt().unwrap();
    // Result should be approximately 2 + i
    assert!(result.is_complex());
    let s = result.to_string();
    assert!(s.contains("2.0") && s.contains("i"));
}

#[test]
fn test_complex_exp() {
    // exp(i*pi) = -1 (Euler's identity)
    // We'll test exp(πi) ≈ -1
    let pi = create_float("3.141592653589793");
    let i = create_imaginary();
    let pi_i = (pi * i).unwrap();
    let result = pi_i.exp().unwrap();
    // Result should be complex, close to -1 + 0i
    assert!(result.is_complex());
    // Check that it's approximately -1
    let s = result.to_string();
    assert!(s.contains("-1") || s.contains("-0.9"));
}

#[test]
fn test_complex_sin() {
    // sin(i) = i*sinh(1) (purely imaginary result)
    let i = create_imaginary();
    let result = i.sin().unwrap();
    assert!(result.is_complex());
    // sinh(1) ≈ 1.175, so result should be approximately 0 + 1.175i
    let s = result.to_string();
    assert!(s.contains("i"));
}

#[test]
fn test_complex_cos() {
    // cos(i) = cosh(1) ≈ 1.543 (real result)
    let i = create_imaginary();
    let result = i.cos().unwrap();
    // cosh(1) ≈ 1.543, result should be approximately 1.543 + 0i
    assert!(result.is_complex());
    let s = result.to_string();
    assert!(s.contains("1.5") || s.contains("1.6"));
}

#[test]
fn test_complex_tan() {
    // tan(1 + i)
    let z = create_complex("1", "1");
    let result = z.tan().unwrap();
    assert!(result.is_complex());
}

#[test]
fn test_complex_ln() {
    // ln(e) = 1, ln(e + 0i) = 1 + 0i
    let e = create_float("2.718281828459045");
    let result = e.ln().unwrap();
    let val = result.to_f64().unwrap();
    assert!((val - 1.0).abs() < 0.0001);
    
    // ln(i) = ln(|i|) + i*arg(i) = 0 + i*π/2
    let i = create_imaginary();
    let result_i = i.ln().unwrap();
    assert!(result_i.is_complex());
}

#[test]
fn test_complex_log() {
    // log_2(8) = 3
    let eight = create_float("8");
    let two = create_float("2");
    let result = eight.log(&two).unwrap();
    let val = result.to_f64().unwrap();
    assert!((val - 3.0).abs() < 0.0001);
    
    // log with complex numbers
    let z = create_complex("2", "3");
    let base = create_float("10");
    let result = z.log(&base).unwrap();
    assert!(result.is_complex());
}

#[test]
fn test_complex_log10() {
    // log10(100) = 2
    let hundred = create_float("100");
    let result = hundred.log10().unwrap();
    let val = result.to_f64().unwrap();
    assert!((val - 2.0).abs() < 0.0001);
    
    // log10 of complex number
    let z = create_complex("1", "1");
    let result = z.log10().unwrap();
    assert!(result.is_complex());
}

#[test]
fn test_complex_pow() {
    // i^2 = -1
    let i = create_imaginary();
    let two = create_float("2");
    let result = i.pow(&two).unwrap();
    assert!(result.is_complex());
    // Result should be approximately -1
    let s = result.to_string();
    assert!(s.contains("-1"));
    
    // (1 + i)^2 = 1 + 2i + i^2 = 1 + 2i - 1 = 2i
    let z = create_complex("1", "1");
    let result2 = z.pow(&two).unwrap();
    assert!(result2.is_complex());
    // Should be approximately 0 + 2i
    let s = result2.to_string();
    assert!(s.contains("2") && s.contains("i"));
}

#[test]
fn test_complex_round() {
    // (3.456 + 7.891i).round(2) = (3.46 + 7.89i)
    let z = create_complex("3.456", "7.891");
    let rounded = z.round(2);
    assert!(rounded.is_complex());
    let s = rounded.to_string();
    assert!(s.contains("3.46") && s.contains("7.89"));
}

#[test]
fn test_complex_is_complex() {
    let z = create_complex("1", "2");
    assert!(z.is_complex());
    
    let r = create_float("5");
    assert!(!r.is_complex());
}

#[test]
fn test_complex_floor_ceil_error() {
    // floor and ceil should return error for complex numbers
    let z = create_complex("3", "4");
    assert!(z.floor().is_err());
    assert!(z.ceil().is_err());
}

#[test]
fn test_complex_modulo_error() {
    // modulo should return error for complex numbers
    let z1 = create_complex("3", "4");
    let z2 = create_complex("1", "2");
    let result = z1 % z2;
    assert!(result.is_err());
}
