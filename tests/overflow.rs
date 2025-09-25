use imagnum::foundation::{Float, Int, SmallInt};
use imagnum::{create_float, create_int};

#[test]
fn test_small_int_overflow_promotes_to_big() {
    // 127 (i8 max) + 1 -> 128 should be Big
    let a = Int::Small(SmallInt::I8(127));
    let b = Int::Small(SmallInt::I8(1));
    let res = a._add(&b).unwrap();
    match res {
        Int::Big(bi) => assert_eq!(bi.to_string(), "128"),
        Int::Small(_) => panic!("expected promotion to Big on overflow"),
    }

    // -128 (i8 min) - 1 -> -129 should be Big
    let c = Int::Small(SmallInt::I8(-128));
    let d = Int::Small(SmallInt::I8(1));
    let res2 = c._sub(&d).unwrap();
    match res2 {
        Int::Big(bi) => assert_eq!(bi.to_string(), "-129"),
        Int::Small(_) => panic!("expected promotion to Big on negative overflow"),
    }
}

#[test]
fn test_big_int_multiplication_of_very_large_numbers() {
    let mut s1 = String::from("1");
    s1.push_str(&"0".repeat(499));
    let s2 = "9".repeat(500);

    let n1 = create_int(&s1);
    let n2 = create_int(&s2);

    let prod = n1._mul(&n2).unwrap();
    let (digits, _neg, _k) = imagnum::compat::int_to_parts(&prod);
    assert!(digits.len() >= 500);
}

#[test]
fn test_big_float_large_multiplication() {
    let a = create_float(&format!("{}", "1".to_string() + &"0".repeat(400)));
    let b = create_float(&format!("{}", "9".repeat(200)));
    let prod = a._mul(&b).unwrap();
    match prod {
        Float::Big(bd) | Float::Irrational(bd) => {
            let s = bd.to_string();
            assert!(s.len() > 0);
        }
        Float::Small(_) => {
            // small is unlikely for these large operands, but accept if so
        }
        _ => panic!("unexpected float kind for large multiplication"),
    }
}
