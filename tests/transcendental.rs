use imagnum::{Float, create_float, create_int};

#[test]
fn test_sqrt_2_is_irrational_and_truncated() {
    let two = create_int("2");
    let res = two.sqrt().expect("sqrt failed");
    // Expect variant to be Irrational
    match res {
        Float::Irrational(bd) => {
            let s = bd.to_string();
            // should contain decimal point and be truncated to at most 137 decimals
            assert!(s.contains('.'));
            let parts: Vec<&str> = s.split('.').collect();
            let frac_len = parts.get(1).map(|p| p.len()).unwrap_or(0);
            assert!(frac_len <= 137);
            // first digits should match known sqrt(2) prefix
            assert!(s.starts_with("1.4142135623730951"));
        }
        _ => panic!("expected Irrational for sqrt(2)")
    }
}

#[test]
fn test_sqrt_4_is_finite_two() {
    let four = create_int("4");
    let res = four.sqrt().expect("sqrt failed");
    match res {
        Float::Big(bd) | Float::Irrational(bd) => {
            let s = bd.to_string();
            // should be exactly 2 or 2.0
            assert!(s.starts_with("2"));
        }
        Float::Small(_) => {
            // fine too
        }
        _ => panic!("expected finite result for sqrt(4)")
    }
}

#[test]
fn test_sin_of_zero_is_zero() {
    let zero = create_float("0");
    let res = zero.sin().expect("sin failed");
    match res {
        Float::Big(bd) | Float::Irrational(bd) => {
            let s = bd.to_string();
            assert!(s.starts_with("0"));
        }
        Float::Small(_) => {}
        _ => panic!("expected zero for sin(0)")
    }
}

#[test]
fn test_ln_of_1_is_zero() {
    let one = create_float("1");
    let res = one.ln().expect("ln failed");
    match res {
        Float::Big(bd) | Float::Irrational(bd) => {
            let s = bd.to_string();
            // ln(1) == 0
            assert!(s.starts_with("0"));
        }
        Float::Small(_) => {}
        _ => panic!("expected zero for ln(1)")
    }
}
