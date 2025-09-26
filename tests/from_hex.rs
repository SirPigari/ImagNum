use imagnum::Int;
use num_bigint::BigInt;

#[test]
fn test_from_hex_basic() {
    let i = Int::from_hex("1A").unwrap();
    let expected = BigInt::parse_bytes(b"1A", 16).unwrap();
    assert_eq!(i.to_str(), expected.to_string());
}

#[test]
fn test_from_hex_prefix_and_sign() {
    let i = Int::from_hex("-0xFF").unwrap();
    let expected = BigInt::parse_bytes(b"FF", 16).unwrap();
    assert_eq!(i.to_str(), format!("-{}", expected.to_string()));
}

#[test]
fn test_from_hex_underscores_and_case() {
    let i = Int::from_hex("0xDE_AD_be_ef").unwrap();
    let expected = BigInt::parse_bytes(b"DEADBEEF", 16).unwrap();
    assert_eq!(i.to_str(), expected.to_string());
}

#[test]
fn test_from_hex_very_large() {
    // 128 hex digits (~512 bits)
    let hex = "f".repeat(128);
    let s = hex.as_str();
    let i = Int::from_hex(s).unwrap();
    let expected = BigInt::parse_bytes(s.as_bytes(), 16).unwrap();
    assert_eq!(i.to_str(), expected.to_string());
}

#[test]
fn test_from_hex_invalid() {
    let res = Int::from_hex("0xGHI");
    assert!(res.is_err());
}

#[test]
fn raywhite_test_from_hex() {
    let raywhite_hex = "0xF5F5F5FF";
    let i = Int::from_hex(raywhite_hex).unwrap();
    let expected = "4126537215";
    assert_eq!(i.to_str(), expected);
}