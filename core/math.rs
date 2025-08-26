// math.rs — BigInt/BigDecimal backed math utilities
// Replaces slow string-based routines with efficient big-number operations.
use bigdecimal::{BigDecimal, Zero};
use bigdecimal::num_bigint::BigInt;
use bigdecimal::num_bigint::ToBigInt;
use num_traits::{ToPrimitive, FromPrimitive, Signed};
use std::str::FromStr;

pub const ERR_UNIMPLEMENTED: i16 = -1;
pub const ERR_INVALID_FORMAT: i16 = 1;
pub const ERR_DIV_BY_ZERO: i16 = 2;
pub const ERR_NEGATIVE_RESULT: i16 = 3;
pub const ERR_NEGATIVE_SQRT: i16 = 4;
pub const ERR_NUMBER_TOO_LARGE: i16 = 5;
pub const ERR_INFINITE_RESULT: i16 = 6;

type IntResult<T> = std::result::Result<(T, bool), i16>;
type FloatResult<T> = std::result::Result<(T, i32, bool), i16>;

fn parse_positive_digits(s: &str) -> Result<BigInt, i16> {
    if s.is_empty() { return Err(ERR_INVALID_FORMAT); }
    if !s.chars().all(|c| c.is_ascii_digit()) { return Err(ERR_INVALID_FORMAT); }
    match BigInt::parse_bytes(s.as_bytes(), 10) {
        Some(bi) => Ok(bi),
        None => Err(ERR_INVALID_FORMAT),
    }
}

pub fn is_string_odd(s: &str) -> bool {
    s.chars().rev().next().map_or(false, |c| c.to_digit(10).unwrap_or(0) % 2 == 1)
}

// Integer ops using BigInt
pub fn add_strings(a: &str, b: &str) -> IntResult<String> {
    let a = parse_positive_digits(a)?;
    let b = parse_positive_digits(b)?;
    let sum = a + b;
    Ok((sum.to_string(), false))
}

pub fn sub_strings(a: &str, b: &str) -> IntResult<String> {
    let a = parse_positive_digits(a)?;
    let b = parse_positive_digits(b)?;
    let diff = a - b;
    if diff.is_negative() {
        Ok((diff.abs().to_string(), true))
    } else {
        Ok((diff.to_string(), false))
    }
}

pub fn mul_strings(a: &str, b: &str) -> IntResult<String> {
    let a = parse_positive_digits(a)?;
    let b = parse_positive_digits(b)?;
    let prod = a * b;
    Ok((prod.to_string(), false))
}

pub fn div_strings(a: &str, b: &str) -> IntResult<String> {
    let a = parse_positive_digits(a)?;
    let b = parse_positive_digits(b)?;
    if b.is_zero() { return Err(ERR_DIV_BY_ZERO); }
    let q = a / b;
    Ok((q.to_string(), false))
}

pub fn rem_strings(a: &str, b: &str) -> IntResult<String> {
    let a = parse_positive_digits(a)?;
    let b = parse_positive_digits(b)?;
    if b.is_zero() { return Err(ERR_DIV_BY_ZERO); }
    let r = a % b;
    Ok((r.to_string(), false))
}

pub fn mod_strings(a: &str, b: &str) -> IntResult<String> { rem_strings(a,b) }

pub fn pow_strings(base: &str, exponent: &str) -> IntResult<String> {
    // exponent must be a non-negative integer and reasonably sized
    let a = parse_positive_digits(base)?;
    let exp_bi = parse_positive_digits(exponent)?;
    let exp_u64 = match exp_bi.to_u64() {
        Some(v) => v,
        None => return Err(ERR_NUMBER_TOO_LARGE),
    };
    if exp_u64 > 1_000_000 { return Err(ERR_NUMBER_TOO_LARGE); }
    // fast pow via repeated squaring using BigInt::pow
    let result = a.pow(exp_u64.try_into().unwrap_or(0u32));
    Ok((result.to_string(), false))
}

pub fn sqrt_string(a: &str) -> IntResult<String> {
    let a = parse_positive_digits(a)?;
    if a.is_zero() { return Ok(("0".to_string(), false)); }
    // integer sqrt via binary search
    let mut low = BigInt::from(0);
    let mut high = a.clone();
    while &low < &high {
        let mid = (&low + &high + 1u32) >> 1u32; // (low+high+1)/2
        let sq = &mid * &mid;
        if sq <= a { low = mid; } else { high = mid - 1u32; }
    }
    Ok((low.to_string(), false))
}

// Helpers for floats
fn to_bigdecimal(mant: &str, exp: i32, neg: bool) -> BigDecimal {
    let mant_len = mant.len() as i32;
    let decimal_pos = mant_len + exp;
    let s = if decimal_pos <= 0 {
        let zeros = "0".repeat((-decimal_pos) as usize);
        format!("0.{}{}", zeros, mant)
    } else if decimal_pos >= mant_len {
        let zeros = "0".repeat((decimal_pos - mant_len) as usize);
        format!("{}{}", mant, zeros)
    } else {
        let (int_part, frac_part) = mant.split_at(decimal_pos as usize);
        format!("{}.{}", int_part, frac_part)
    };
    let bd = BigDecimal::from_str(&s).unwrap_or_else(|_| BigDecimal::zero());
    if neg { -bd } else { bd }
}

fn from_bigdecimal(bd: &BigDecimal) -> (String, i32, bool) {
    let s = bd.normalized().to_string();
    let neg = s.starts_with('-');
    let s = s.trim_start_matches('-');
    if s == "0" || s.is_empty() { return ("0".to_string(), 0, false); }
    let parts: Vec<&str> = s.split('E').collect();
    let (base, exp_part) = if parts.len() == 2 { (parts[0], parts[1]) } else { (s, "0") };
    let exp_from_e: i32 = exp_part.parse().unwrap_or(0);
    let (mant, exp) = if let Some(dot) = base.find('.') {
        let mantissa = base[..dot].to_string() + &base[dot + 1..];
        let exp_decimal = -((base.len() - dot - 1) as i32);
        (mantissa.trim_start_matches('0').to_string(), exp_decimal)
    } else {
        (base.trim_start_matches('0').to_string(), 0)
    };
    let final_exp = exp + exp_from_e;
    (mant, final_exp, neg)
}

fn truncate_bd_to_decimals(bd: &BigDecimal, decimals: usize) -> BigDecimal {
    // Set scale to `decimals` (number of fractional digits) without rounding
    bd.with_scale(decimals as i64)
}

// Float arithmetic (keep old signatures)
pub fn add_float(
    mant1: String,
    exp1: i32,
    neg1: bool,
    mant2: String,
    exp2: i32,
    neg2: bool,
) -> FloatResult<String> {
    let a = to_bigdecimal(&mant1, exp1, neg1);
    let b = to_bigdecimal(&mant2, exp2, neg2);
    let sum = a + b;
    Ok(from_bigdecimal(&sum))
}

pub fn sub_float(
    mant1: String,
    exp1: i32,
    neg1: bool,
    mant2: String,
    exp2: i32,
    neg2: bool,
) -> FloatResult<String> {
    let a = to_bigdecimal(&mant1, exp1, neg1);
    let b = to_bigdecimal(&mant2, exp2, neg2);
    let diff = a - b;
    Ok(from_bigdecimal(&diff))
}

pub fn mul_float(
    mant1: String,
    exp1: i32,
    neg1: bool,
    mant2: String,
    exp2: i32,
    neg2: bool,
) -> FloatResult<String> {
    let a = to_bigdecimal(&mant1, exp1, neg1);
    let b = to_bigdecimal(&mant2, exp2, neg2);
    let prod = a * b;
    Ok(from_bigdecimal(&prod))
}

pub fn div_float(
    mant1: String,
    exp1: i32,
    neg1: bool,
    mant2: String,
    exp2: i32,
    neg2: bool,
) -> FloatResult<String> {
    let a = to_bigdecimal(&mant1, exp1, neg1);
    let b = to_bigdecimal(&mant2, exp2, neg2);
    if b.is_zero() { return Err(ERR_DIV_BY_ZERO); }
    let scale = ((mant1.len() + mant2.len()) as i64 + 20).max(50);
    let quotient = (a / b).with_scale(scale);
    Ok(from_bigdecimal(&quotient))
}

pub fn mod_float(
    mant1: String,
    exp1: i32,
    neg1: bool,
    mant2: String,
    exp2: i32,
    neg2: bool,
) -> FloatResult<String> {
    let a = to_bigdecimal(&mant1, exp1, neg1);
    let b = to_bigdecimal(&mant2, exp2, neg2);
    if b.is_zero() { return Err(ERR_DIV_BY_ZERO); }
    let div_floor = BigDecimal::from(a.with_scale(0).to_bigint().unwrap() / b.with_scale(0).to_bigint().unwrap_or(BigInt::from(1u32)));
    let res = a - b * div_floor;
    Ok(from_bigdecimal(&res))
}

// Transcendental functions — fast f64-based approximations. Mark result irrational and truncate to 137 decimals.
#[allow(dead_code)]
fn float_from_f64_to_parts(mut v: f64) -> (String, i32, bool) {
    if v.is_nan() { return ("".to_string(), 0, false); }
    if v.is_infinite() { return ("".to_string(), 0, false); }
    let neg = v.is_sign_negative();
    if neg { v = v.abs(); }
    let s = format!("{:.50e}", v); // capture enough digits
    // parse scientific notation like 1.2345e+03
    if let Some((base, exp_part)) = s.split_once('e') {
        let exp_i: i32 = exp_part.parse().unwrap_or(0);
        let base = base.replace('.', "").trim_start_matches('0').to_string();
    let computed_exp = exp_i - (base.len() as i32 - 1);
    if base.is_empty() { return ("0".to_string(), 0, false); }
    (base, computed_exp, neg)
    } else {
        // fallback
        let bd = BigDecimal::from_f64(v).unwrap_or_else(|| BigDecimal::zero());
        from_bigdecimal(&bd)
    }
}

pub fn sin_float(mant: String, exp: i32, neg: bool) -> Result<(String, i32, bool, bool), i16> {
    let bd = to_bigdecimal(&mant, exp, neg);
    let f = bd.to_f64().ok_or(ERR_INVALID_FORMAT)?;
    let res = f.sin();
    if res.is_nan() { return Err(ERR_INVALID_FORMAT); }
    if res.is_infinite() { return Err(ERR_INFINITE_RESULT); }
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m,e,neg2) = from_bigdecimal(&trunc);
    Ok((m,e,neg2,true))
}

pub fn sqrt_float(mant: String, exp: i32, neg: bool) -> Result<(String, i32, bool, bool), i16> {
    let bd = to_bigdecimal(&mant, exp, neg);
    if bd.is_negative() { return Err(ERR_NEGATIVE_SQRT); }
    let f = bd.to_f64().ok_or(ERR_INVALID_FORMAT)?;
    let res = f.sqrt();
    if res.is_nan() { return Err(ERR_INVALID_FORMAT); }
    if res.is_infinite() { return Err(ERR_INFINITE_RESULT); }
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m2,e2,neg2) = from_bigdecimal(&trunc);
    // consider result irrational if truncated result has fractional digits (exp < 0)
    let is_irrational = e2 < 0;
    Ok((m2,e2,neg2,is_irrational))
}

pub fn cos_float(mant: String, exp: i32, neg: bool) -> Result<(String, i32, bool, bool), i16> {
    let bd = to_bigdecimal(&mant, exp, neg);
    let f = bd.to_f64().ok_or(ERR_INVALID_FORMAT)?;
    let res = f.cos();
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m,e,neg2) = from_bigdecimal(&trunc);
    Ok((m,e,neg2,true))
}

pub fn tan_float(mant: String, exp: i32, neg: bool) -> Result<(String, i32, bool, bool), i16> {
    let bd = to_bigdecimal(&mant, exp, neg);
    let f = bd.to_f64().ok_or(ERR_INVALID_FORMAT)?;
    let res = f.tan();
    if res.is_nan() { return Err(ERR_INVALID_FORMAT); }
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m,e,neg2) = from_bigdecimal(&trunc);
    Ok((m,e,neg2,true))
}

pub fn ln_float(mant: String, exp: i32, neg: bool) -> Result<(String,i32,bool,bool), i16> {
    let bd = to_bigdecimal(&mant, exp, neg);
    if bd.is_negative() || bd.is_zero() { return Err(ERR_INVALID_FORMAT); }
    let f = bd.to_f64().ok_or(ERR_INVALID_FORMAT)?;
    let res = f.ln();
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m,e,neg2) = from_bigdecimal(&trunc);
    Ok((m,e,neg2,true))
}

pub fn exp_float(mant: String, exp: i32, neg: bool) -> Result<(String,i32,bool,bool), i16> {
    let bd = to_bigdecimal(&mant, exp, neg);
    let f = bd.to_f64().ok_or(ERR_INVALID_FORMAT)?;
    let res = f.exp();
    if res.is_infinite() { return Err(ERR_INFINITE_RESULT); }
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m,e,neg2) = from_bigdecimal(&trunc);
    Ok((m,e,neg2,true))
}

pub fn log10_float(mant: String, exp: i32, neg: bool) -> Result<(String,i32,bool,bool), i16> {
    let bd = to_bigdecimal(&mant, exp, neg);
    if bd.is_negative() || bd.is_zero() { return Err(ERR_INVALID_FORMAT); }
    let f = bd.to_f64().ok_or(ERR_INVALID_FORMAT)?;
    let res = f.log10();
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m,e,neg2) = from_bigdecimal(&trunc);
    Ok((m,e,neg2,true))
}

pub fn floor_float(mant: String, exp: i32, neg: bool) -> Result<(String,i32,bool), i16> {
    let bd = to_bigdecimal(&mant, exp, neg);
    let bi = bd.with_scale(0).to_bigint().unwrap_or(BigInt::from(0));
    let bd_floor = BigDecimal::from(bi.clone());
    Ok(from_bigdecimal(&bd_floor))
}

pub fn ceil_float(mant: String, exp: i32, neg: bool) -> Result<(String,i32,bool), i16> {
    let bd = to_bigdecimal(&mant, exp, neg);
    let bi = bd.with_scale(0).to_bigint().unwrap_or(BigInt::from(0));
    let bd_floor = BigDecimal::from(bi.clone());
    if bd - bd_floor.clone() > BigDecimal::zero() {
        let one = BigDecimal::from(1);
        let bd_ceil = bd_floor + one;
        Ok(from_bigdecimal(&bd_ceil))
    } else {
        Ok(from_bigdecimal(&bd_floor))
    }
}

pub fn abs_float(mant: String, exp: i32, _neg: bool) -> Result<(String,i32,bool), i16> {
    let bd = to_bigdecimal(&mant, exp, false);
    Ok(from_bigdecimal(&bd.abs()))
}

// Integer wrappers for transcendental functions — convert to BigDecimal and call float impls
pub fn sin_int(digits: String, negative: bool) -> Result<(String,i32,bool,bool), i16> {
    let bd = to_bigdecimal(&digits, 0, negative);
    let f = bd.to_f64().ok_or(ERR_INVALID_FORMAT)?;
    let res = f.sin();
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m,e,neg2) = from_bigdecimal(&trunc);
    Ok((m,e,neg2,true))
}

pub fn sqrt_int(digits: String, negative: bool) -> Result<(String,i32,bool,bool), i16> {
    // For integers, if negative -> imaginary handled by caller
    let bd = to_bigdecimal(&digits, 0, negative);
    if bd.is_negative() { return Err(ERR_NEGATIVE_SQRT); }
    let f = bd.to_f64().ok_or(ERR_INVALID_FORMAT)?;
    let res = f.sqrt();
    if res.is_nan() { return Err(ERR_INVALID_FORMAT); }
    if res.is_infinite() { return Err(ERR_INFINITE_RESULT); }
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m2,e2,neg2) = from_bigdecimal(&trunc);
    let is_irrational = e2 < 0;
    Ok((m2,e2,neg2,is_irrational))
}

pub fn cos_int(digits: String, negative: bool) -> Result<(String,i32,bool,bool), i16> {
    let bd = to_bigdecimal(&digits, 0, negative);
    let f = bd.to_f64().ok_or(ERR_INVALID_FORMAT)?;
    let res = f.cos();
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m,e,neg2) = from_bigdecimal(&trunc);
    Ok((m,e,neg2,true))
}

pub fn tan_int(digits: String, negative: bool) -> Result<(String,i32,bool,bool), i16> {
    let bd = to_bigdecimal(&digits, 0, negative);
    let f = bd.to_f64().ok_or(ERR_INVALID_FORMAT)?;
    let res = f.tan();
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m,e,neg2) = from_bigdecimal(&trunc);
    Ok((m,e,neg2,true))
}

pub fn ln_int(digits: String, negative: bool) -> Result<(String,i32,bool,bool), i16> {
    let bd = to_bigdecimal(&digits, 0, negative);
    if bd.is_negative() || bd.is_zero() { return Err(ERR_INVALID_FORMAT); }
    let f = bd.to_f64().ok_or(ERR_INVALID_FORMAT)?;
    let res = f.ln();
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m,e,neg2) = from_bigdecimal(&trunc);
    Ok((m,e,neg2,true))
}

pub fn exp_int(digits: String, negative: bool) -> Result<(String,i32,bool,bool), i16> {
    let bd = to_bigdecimal(&digits, 0, negative);
    let f = bd.to_f64().ok_or(ERR_INVALID_FORMAT)?;
    let res = f.exp();
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m,e,neg2) = from_bigdecimal(&trunc);
    Ok((m,e,neg2,true))
}

// floor/ceil/abs for ints: identity or abs
pub fn floor_int(digits: String, negative: bool) -> Result<(String,bool), i16> {
    Ok((digits, negative))
}

pub fn ceil_int(digits: String, negative: bool) -> Result<(String,bool), i16> {
    Ok((digits, negative))
}

pub fn abs_int(digits: String, _negative: bool) -> Result<(String,bool), i16> {
    Ok((digits, false))
}
