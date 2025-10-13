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
