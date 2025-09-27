use imagnum::Int;

#[test]
fn test_radix_binary_and_octal() {
    let i = Int::from_str_radix("1011", 2).unwrap();
    assert_eq!(i.to_str_radix(2).unwrap(), "1011");
    let j = Int::from_str_radix("17", 8).unwrap();
    assert_eq!(j.to_str_radix(8).unwrap(), "17");
}

#[test]
fn test_radix_decimal_and_hex() {
    let i = Int::from_str_radix("255", 10).unwrap();
    assert_eq!(i.to_str_radix(10).unwrap(), "255");
    let j = Int::from_str_radix("FF", 16).unwrap();
    assert_eq!(j.to_str_radix(16).unwrap().to_lowercase(), "ff");
}

#[test]
fn test_radix_large_and_negative() {
    let hex = "f".repeat(64);
    let i = Int::from_str_radix(&hex, 16).unwrap();
    assert_eq!(i.to_str_radix(16).unwrap().to_lowercase(), hex);
    let n = Int::from_str_radix("-1234", 10).unwrap();
    assert_eq!(n.to_str_radix(10).unwrap(), "-1234");
}

#[test]
fn test_radix_invalid() {
    assert!(Int::from_str_radix("1Z", 10).is_err());
    assert!(Int::from_str_radix("123", 1).is_err());
    assert!(Int::from_str_radix("", 10).is_err());
}

#[test]
fn test_radix_with_underscores() {
    let i = Int::from_str_radix("DE_AD_BE_EF", 16).unwrap();
    assert_eq!(i.to_str_radix(16).unwrap().to_lowercase(), "deadbeef");
}
