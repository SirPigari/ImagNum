#[cfg(feature = "serde")]
mod test_serde {
    #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
    struct Data {
        int_value: imagnum::Int,
        float_value: imagnum::Float,
    }


    #[test]
    fn test_int_serde() {
        use imagnum::Int;
        use serde_json;

        let original = Int::from_str("123456789012345678901234567890").unwrap();
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: Int = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_float_serde() {
        use imagnum::Float;
        use serde_json;

        let original = Float::from_str("3.1415926535897932384626433832795028841971").unwrap();
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: Float = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_struct_serde() {
        use imagnum::{Int, Float};
        use serde_json;

        let data = Data {
            int_value: Int::from_str("987654321098765432109876543210").unwrap(),
            float_value: Float::from_str("2.7182818284590452353602874713526624977572").unwrap(),
        };

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: Data = serde_json::from_str(&serialized).unwrap();

        assert_eq!(data, deserialized);
    }
}

#[cfg(feature = "random")]
mod test_random {
    use imagnum::*;
    use imagnum::random::*;
    use num_traits::cast::ToPrimitive;

    #[test]
    fn test_randint() {
        let min = Int::from_str("1000").unwrap();
        let max = Int::from_str("2000").unwrap();

        for _ in 0..100 {
            let r = randint(&min, &max);
            assert!(r >= min && r <= max, "randint produced {} outside [{}, {}]", r.to_str(), min.to_str(), max.to_str());
        }
    }

    #[test]
    fn test_randfloat() {
        let min = Float::from_str("1.0").unwrap();
        let max = Float::from_str("2.0").unwrap();

        for _ in 0..100 {
            let r = randfloat(&min, &max);
            if let Float::Big(bd) = r {
                let r_f64 = bd.to_f64().unwrap_or(f64::NAN);
                assert!(r_f64 >= 1.0 && r_f64 <= 2.0);
            }
        }
    }

    #[test]
    fn test_randdecimal() {
        let min = Float::from_str("0.0").unwrap();
        let max = Float::from_str("10.0").unwrap();
        let precision = 50;

        for _ in 0..100 {
            let r = randdecimal(&min, &max, precision);
            if let Float::Big(bd) = r {
                let r_f64 = bd.to_f64().unwrap_or(f64::NAN);
                assert!(r_f64 >= 0.0 && r_f64 <= 10.0);
            }
        }
    }

    #[test]
    fn test_randcomplex() {
        let min = Float::from_str("0.0").unwrap();
        let max = Float::from_str("5.0").unwrap();

        for _ in 0..100 {
            let r = randcomplex(&min, &max);
            if let Float::Complex(real, imag) = r {
                if let Float::Big(bd_real) = *real {
                    let val = bd_real.to_f64().unwrap_or(f64::NAN);
                    assert!(val >= 0.0 && val <= 5.0);
                }
                if let Float::Big(bd_imag) = *imag {
                    let val = bd_imag.to_f64().unwrap_or(f64::NAN);
                    assert!(val >= 0.0 && val <= 5.0);
                }
            }
        }
    }

    #[test]
    fn test_randreal() {
        let min = Float::from_str("0.0").unwrap();
        let max = Float::from_str("100.0").unwrap();

        for _ in 0..100 {
            let r = randreal(&min, &max);
            match r {
                Float::Big(bd) | Float::Recurring(bd) | Float::Irrational(bd) => {
                    let val = bd.to_f64().unwrap_or(f64::NAN);
                    assert!(val >= 0.0 && val <= 100.0);
                }
                _ => {}
            }
        }
    }

    #[test]
    fn test_rand() {
        for _ in 0..100 {
            let r = rand();
            let val = r.to_f64().unwrap_or(f64::NAN);
            assert!(val >= 0.0 && val <= 1.0, "rand() produced {}", val);
        }
    }
}
