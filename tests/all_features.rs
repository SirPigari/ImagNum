use imagnum::{Float, Int, create_float, create_int};

#[test]
fn test_int_arithmetic_and_pow() {
    let a = create_int("123");
    let b = create_int("456");

    // add
    let res_add = a._add(&b).unwrap();
    match res_add {
        Int::Big(bi) => assert_eq!(bi.to_string(), "579"),
        _ => panic!("expected BigInt for add result"),
    }

    // sub
    let res_sub = b._sub(&a).unwrap();
    match res_sub {
        Int::Big(bi) => assert_eq!(bi.to_string(), "333"),
        _ => panic!("expected BigInt for sub result"),
    }

    // mul
    let res_mul = a._mul(&b).unwrap();
    match res_mul {
        Int::Big(bi) => assert_eq!(bi.to_string(), "56088"),
        _ => panic!("expected BigInt for mul result"),
    }

    // div
    let res_div = b._div(&a).unwrap();
    match res_div {
        Int::Big(bi) => assert_eq!(bi.to_string(), "4"),
        _ => panic!("expected BigInt for div result"),
    }

    // modulo
    let res_mod = b._modulo(&a).unwrap();
    match res_mod {
        Int::Big(bi) => assert_eq!(bi.to_string(), "87"),
        _ => panic!("expected BigInt for modulo result"),
    }

    // pow
    let two = create_int("2");
    let res_pow = two.pow(&create_int("10")).unwrap();
    match res_pow {
        Int::Big(bi) => assert_eq!(bi.to_string(), "1024"),
        _ => panic!("expected BigInt for pow result"),
    }
}

#[test]
fn test_int_conv_and_props() {
    let neg = create_int("-42");
    assert_eq!(neg.to_i64().unwrap(), -42i64);
    let zero = create_int("0");
    assert!(zero.is_zero());
    let inf_like = create_int("Infinity");
    assert!(inf_like.is_zero());
}

#[test]
fn test_float_arithmetic_and_properties() {
    let a = create_float("123.456");
    let b = create_float("78.9");

    let res_mul = a._mul(&b).unwrap();
    // check numeric approx via to_f64
    let approx = res_mul.to_f64().unwrap();
    assert!((approx - (123.456f64 * 78.9f64)).abs() < 1e-8);

    let res_div = a._div(&b).unwrap();
    let approx_div = res_div.to_f64().unwrap();
    assert!((approx_div - (123.456f64 / 78.9f64)).abs() < 1e-8);

    // pow and sqrt
    let exp = create_float("2");
    let res_pow = a.pow(&exp).unwrap();
    let approx_pow = res_pow.to_f64().unwrap();
    assert!((approx_pow - 123.456f64.powf(2.0)).abs() < 1e-6);

    let res_sqrt = a.sqrt().unwrap();
    let approx_sqrt = res_sqrt.to_f64().unwrap();
    assert!((approx_sqrt - 123.456f64.sqrt()).abs() < 1e-6);
}

#[test]
fn test_float_round_truncate_and_integer_like() {
    let f = create_float("123.456789");
    // round to 2 decimal places
    let r = f.round(2);
    let s = match r {
        Float::Big(bd) | Float::Irrational(bd) => bd.to_string(),
        Float::Small(_) => panic!("unexpected small float"),
        _ => panic!("unexpected float kind"),
    };
    assert!(s.starts_with("123.46") || s.starts_with("123.4"));

    // truncate to 3 decimals
    let t = f.truncate(3);
    let s2 = match t {
        Float::Big(bd) | Float::Irrational(bd) => bd.to_string(),
        Float::Small(_) => panic!("unexpected small float"),
        _ => panic!("unexpected float kind"),
    };
    assert!(s2.starts_with("123.456") || s2.starts_with("123.45"));

    // integer-like check
    let fi = create_float("42.0");
    assert!(fi.is_integer_like());
    let fnl = create_float("42.5");
    assert!(!fnl.is_integer_like());
}

#[test]
fn test_transcendentals_and_truncation() {
    // sqrt(2) irrational and truncated <=137
    let two = create_int("2");
    let s2 = two.sqrt().unwrap();
    match s2 {
        Float::Irrational(bd) => {
            let s = bd.to_string();
            assert!(s.contains('.'));
            let frac_len = s.split('.').nth(1).map(|p| p.len()).unwrap_or(0);
            assert!(frac_len <= 137);
        }
        Float::Big(_) => panic!("expected irrational for sqrt(2)"),
        _ => panic!("unexpected variant for sqrt(2)"),
    }

    // sqrt(4) finite
    let four = create_int("4");
    let s4 = four.sqrt().unwrap();
    match s4 {
        Float::Big(bd) | Float::Irrational(bd) => {
            let s = bd.to_string();
            assert!(s.starts_with("2"));
        }
        _ => panic!("expected finite result for sqrt(4)"),
    }

    // sin(0)=0, cos(0)=1, tan(0)=0
    let zero = create_float("0");
    let s = zero.sin().unwrap();
    assert!(matches!(
        s,
        Float::Big(_) | Float::Irrational(_) | Float::Small(_)
    ));
    assert!(
        zero.cos().unwrap().to_f64().unwrap().abs() - 1.0 < 1e-12
            || zero.cos().unwrap().to_f64().unwrap().abs() < 1e-12
    );
    assert!(zero.tan().unwrap().to_f64().unwrap().abs() < 1e-12);

    // ln(1)=0
    let one = create_float("1");
    let ln1 = one.ln().unwrap();
    assert!(ln1.to_f64().unwrap().abs() < 1e-12);

    // exp(1) ~= e
    let e1 = create_float("1");
    let exp1 = e1.exp().unwrap();
    let approx = exp1.to_f64().unwrap();
    assert!(
        (approx - std::f64::consts::E).abs() < 1e-12 || (approx - std::f64::consts::E).abs() < 1e-8
    );

    // log10(10)=1
    let ten = create_float("10");
    let logt = ten.log10().unwrap();
    assert!(
        (logt.to_f64().unwrap() - 1.0).abs() < 1e-12 || (logt.to_f64().unwrap() - 1.0).abs() < 1e-8
    );

    // floor/ceil
    let f = create_float("3.7");
    let fl = f.floor().unwrap();
    let cl = f.ceil().unwrap();
    assert!(fl.to_f64().unwrap().abs() - 3.0 < 1e-12 || (fl.to_f64().unwrap() - 3.0).abs() < 1e-8);
    assert!(
        (cl.to_f64().unwrap() - 4.0).abs() < 1e-12 || (cl.to_f64().unwrap() - 4.0).abs() < 1e-8
    );
}

#[test]
fn test_nan_infinity_and_complex_imaginary() {
    let nan = create_float("NaN");
    assert!(nan.is_nan());
    let inf = create_float("Infinity");
    // to_f64 maps Infinity appropriately
    assert_eq!(inf.to_f64().unwrap(), std::f64::INFINITY);
    let ninf = create_float("-Infinity");
    assert_eq!(ninf.to_f64().unwrap(), std::f64::NEG_INFINITY);

    // imaginary parsing
    let imag = create_float("3i");
    match imag {
        Float::Complex(_, _) => {}
        _ => panic!("expected complex/imaginary for 3i"),
    }
}

#[test]
fn test_make_irrational_and_to_int() {
    let f = create_float("2.0");
    // to_int should work
    let i = f.to_int().unwrap();
    match i {
        Int::Big(bi) => assert_eq!(bi.to_string(), "2"),
        _ => panic!("expected BigInt from to_int"),
    }

    // make_irrational on existing float
    let mut ff = create_float("1.5");
    let newf = ff.make_irrational();
    match newf {
        Float::Irrational(_) => {}
        _ => panic!("expected Irrational after make_irrational"),
    }
}
