use crate::foundation::{Float, FloatKind, Int, SmallFloat, SmallInt};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use std::str::FromStr;

pub fn int_to_string(i: &Int) -> String {
    match i {
        Int::Big(b) => b.to_string().trim_start_matches('-').to_string(),
        Int::Small(s) => {
            let s = match s {
                SmallInt::I8(v) => v.to_string(),
                SmallInt::U8(v) => v.to_string(),
                SmallInt::I16(v) => v.to_string(),
                SmallInt::U16(v) => v.to_string(),
                SmallInt::I32(v) => v.to_string(),
                SmallInt::U32(v) => v.to_string(),
                SmallInt::I64(v) => v.to_string(),
                SmallInt::U64(v) => v.to_string(),
                SmallInt::I128(v) => v.to_string(),
                SmallInt::U128(v) => v.to_string(),
                SmallInt::USize(v) => v.to_string(),
                SmallInt::ISize(v) => v.to_string(),
            };
            s.trim_start_matches('-').to_string()
        }
    }
}

pub fn float_to_parts(f: &Float) -> (String, i32, bool, FloatKind) {
    match f {
        Float::Big(bd) => from_bigdecimal(bd),
        Float::Irrational(bd) => {
            let (m, e, neg, _k) = from_bigdecimal(bd);
            (m, e, neg, FloatKind::Irrational)
        }
        Float::Small(s) => match s {
            SmallFloat::F32(v) => {
                let s = v.to_string();
                match BigDecimal::from_str(&s) {
                    Ok(bd) => from_bigdecimal(&bd),
                    Err(_) => (String::new(), 0, v.is_sign_negative(), FloatKind::Finite),
                }
            }
            SmallFloat::F64(v) => {
                let s = v.to_string();
                match BigDecimal::from_str(&s) {
                    Ok(bd) => from_bigdecimal(&bd),
                    Err(_) => (String::new(), 0, v.is_sign_negative(), FloatKind::Finite),
                }
            }
        },
        Float::NaN => (String::new(), 0, false, FloatKind::NaN),
        Float::Infinity => (String::new(), 0, false, FloatKind::Infinity),
        Float::NegInfinity => (String::new(), 0, true, FloatKind::NegInfinity),
        Float::Complex(_, _) => (String::new(), 0, false, FloatKind::Complex),
    }
}

fn from_bigdecimal(bd: &BigDecimal) -> (String, i32, bool, FloatKind) {
    let s = bd.normalized().to_string();
    let neg = s.starts_with('-');
    let s = s.trim_start_matches('-');

    if s == "0" || s.is_empty() {
        return ("0".to_string(), 0, false, FloatKind::Finite);
    }

    let parts: Vec<&str> = s.split('E').collect();
    let (base, exp_part) = if parts.len() == 2 {
        (parts[0], parts[1])
    } else {
        (s, "0")
    };

    let exp_from_e: i32 = exp_part.parse().unwrap_or(0);

    let (mant, exp) = if let Some(dot) = base.find('.') {
        let mantissa = base[..dot].to_string() + &base[dot + 1..];
        let exp_decimal = -((base.len() - dot - 1) as i32);
        (mantissa.trim_start_matches('0').to_string(), exp_decimal)
    } else {
        (base.trim_start_matches('0').to_string(), 0)
    };

    let final_exp = exp + exp_from_e;
    (mant, final_exp, neg, FloatKind::Finite)
}

pub fn float_is_zero(f: &Float) -> bool {
    match f {
        Float::Big(bd) | Float::Irrational(bd) => {
            let (m, _e, _neg, _k) = from_bigdecimal(bd);
            m == "0"
        }
        Float::Small(s) => match s {
            SmallFloat::F32(v) => *v == 0.0,
            SmallFloat::F64(v) => *v == 0.0,
        },
        _ => false,
    }
}

pub fn int_to_parts(i: &Int) -> (String, bool, FloatKind) {
    match i {
        Int::Big(bi) => {
            let s = bi.to_string();
            let neg = s.starts_with('-');
            let digits = s.trim_start_matches('-').to_string();
            (digits, neg, FloatKind::Finite)
        }
        Int::Small(sv) => {
            let s = match sv {
                SmallInt::I8(v) => v.to_string(),
                SmallInt::U8(v) => v.to_string(),
                SmallInt::I16(v) => v.to_string(),
                SmallInt::U16(v) => v.to_string(),
                SmallInt::I32(v) => v.to_string(),
                SmallInt::U32(v) => v.to_string(),
                SmallInt::I64(v) => v.to_string(),
                SmallInt::U64(v) => v.to_string(),
                SmallInt::I128(v) => v.to_string(),
                SmallInt::U128(v) => v.to_string(),
                SmallInt::USize(v) => v.to_string(),
                SmallInt::ISize(v) => v.to_string(),
            };
            let neg = s.starts_with('-');
            let digits = s.trim_start_matches('-').to_string();
            (digits, neg, FloatKind::Finite)
        }
    }
}

pub fn make_int_from_parts(digits: String, negative: bool, _kind: FloatKind) -> Int {
    // Int cannot represent NaN/Infinity in the new foundation; ignore _kind
    match BigDecimal::from_str(&digits) {
        Ok(_) => {
            // parse into BigInt
            match BigInt::from_str(&digits) {
                Ok(mut bi) => {
                    if negative {
                        bi = -bi
                    };
                    Int::Big(bi)
                }
                Err(_) => Int::new(),
            }
        }
        Err(_) => {
            // fallback to BigInt parse attempt
            match BigInt::from_str(&digits) {
                Ok(mut bi) => {
                    if negative {
                        bi = -bi
                    };
                    Int::Big(bi)
                }
                Err(_) => Int::new(),
            }
        }
    }
}

pub fn make_float_from_parts(
    mantissa: String,
    exponent: i32,
    negative: bool,
    kind: FloatKind,
) -> Float {
    match kind {
        FloatKind::NaN => Float::NaN,
        FloatKind::Infinity => {
            if negative {
                Float::NegInfinity
            } else {
                Float::Infinity
            }
        }
        FloatKind::NegInfinity => Float::NegInfinity,
        FloatKind::Irrational | FloatKind::Finite => {
            // Build BigDecimal from mantissa and exponent: mantissa * 10^exponent
            // mantissa is digits without decimal point
            let mut s = mantissa;
            if s.is_empty() {
                s = "0".to_string();
            }
            // insert sign
            if negative && !s.starts_with('-') {
                s = format!("-{}", s);
            }
            // Construct BigDecimal by applying scale = -exponent
            if let Ok(bi) = BigInt::from_str(&s) {
                let scale = -(exponent as i64);
                let bd = BigDecimal::new(bi, scale);
                if kind == FloatKind::Irrational {
                    Float::Irrational(bd)
                } else {
                    Float::Big(bd)
                }
            } else {
                // fallback: try parsing as BigDecimal string
                let s2 = if exponent == 0 {
                    s.clone()
                } else {
                    format!("{}e{}", s, exponent)
                };
                match BigDecimal::from_str(&s2) {
                    Ok(bd) => {
                        if kind == FloatKind::Irrational {
                            Float::Irrational(bd)
                        } else {
                            Float::Big(bd)
                        }
                    }
                    Err(_) => Float::NaN,
                }
            }
        }
        FloatKind::Complex | FloatKind::Imaginary => {
            // Can't reconstruct complex from parts; return NaN
            Float::NaN
        }
    }
}

pub fn int_is_nan(_i: &Int) -> bool {
    false
}
pub fn int_is_infinite(_i: &Int) -> bool {
    false
}
pub fn float_kind(f: &Float) -> FloatKind {
    float_to_parts(f).3
}
pub fn float_is_negative(f: &Float) -> bool {
    float_to_parts(f).2
}
