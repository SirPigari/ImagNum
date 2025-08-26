// math.rs — BigInt/BigDecimal backed math utilities
// Replaces slow string-based routines with efficient big-number operations.
use bigdecimal::num_bigint::BigInt;
use bigdecimal::num_bigint::ToBigInt;
use bigdecimal::{BigDecimal, Zero};
use num_traits::{FromPrimitive, Signed, ToPrimitive};
use std::str::FromStr;

use num_integer::Integer;

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
    if s.is_empty() {
        return Err(ERR_INVALID_FORMAT);
    }
    if !s.chars().all(|c| c.is_ascii_digit()) {
        return Err(ERR_INVALID_FORMAT);
    }
    match BigInt::parse_bytes(s.as_bytes(), 10) {
        Some(bi) => Ok(bi),
        None => Err(ERR_INVALID_FORMAT),
    }
}

pub fn is_string_odd(s: &str) -> bool {
    s.chars()
        .rev()
        .next()
        .map_or(false, |c| c.to_digit(10).unwrap_or(0) % 2 == 1)
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
    if b.is_zero() {
        return Err(ERR_DIV_BY_ZERO);
    }
    let q = a / b;
    Ok((q.to_string(), false))
}

pub fn rem_strings(a: &str, b: &str) -> IntResult<String> {
    let a = parse_positive_digits(a)?;
    let b = parse_positive_digits(b)?;
    if b.is_zero() {
        return Err(ERR_DIV_BY_ZERO);
    }
    let r = a % b;
    Ok((r.to_string(), false))
}

pub fn mod_strings(a: &str, b: &str) -> IntResult<String> {
    rem_strings(a, b)
}

pub fn pow_strings(base: &str, exponent: &str) -> IntResult<String> {
    // exponent must be a non-negative integer and reasonably sized
    let a = parse_positive_digits(base)?;
    let exp_bi = parse_positive_digits(exponent)?;
    let exp_u64 = match exp_bi.to_u64() {
        Some(v) => v,
        None => return Err(ERR_NUMBER_TOO_LARGE),
    };
    if exp_u64 > 1_000_000 {
        return Err(ERR_NUMBER_TOO_LARGE);
    }
    // fast pow via repeated squaring using BigInt::pow
    let result = a.pow(exp_u64.try_into().unwrap_or(0u32));
    Ok((result.to_string(), false))
}

pub fn sqrt_string(a: &str) -> IntResult<String> {
    let a = parse_positive_digits(a)?;
    if a.is_zero() {
        return Ok(("0".to_string(), false));
    }
    // integer sqrt via binary search
    let mut low = BigInt::from(0);
    let mut high = a.clone();
    while &low < &high {
        let mid = (&low + &high + 1u32) >> 1u32; // (low+high+1)/2
        let sq = &mid * &mid;
        if sq <= a {
            low = mid;
        } else {
            high = mid - 1u32;
        }
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

pub fn from_bigdecimal(bd: &BigDecimal) -> (String, i32, bool) {
    let s = bd.normalized().to_string();
    let neg = s.starts_with('-');
    let s = s.trim_start_matches('-');
    if s == "0" || s.is_empty() {
        return ("0".to_string(), 0, false);
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
    (mant, final_exp, neg)
}

fn truncate_bd_to_decimals(bd: &BigDecimal, decimals: usize) -> BigDecimal {
    // Set scale to `decimals` (number of fractional digits) without rounding
    bd.with_scale(decimals as i64)
}

// Parse a BigDecimal (string form) into a rational numerator/denominator pair
// by interpreting the decimal representation: e.g. "1.25" -> (125, 100)
#[allow(dead_code)]
pub fn bigdecimal_to_fraction(bd: &BigDecimal) -> (BigInt, BigInt) {
    // Use normalized string to avoid scientific notation surprises
    let s = bd.normalized().to_string();
    let mut lower = s;
    let neg = lower.starts_with('-');
    if neg {
        lower = lower.trim_start_matches('-').to_string();
    }
    let parts: Vec<&str> = lower.split('E').collect();
    let (base, exp_part) = if parts.len() == 2 {
        (parts[0], parts[1])
    } else {
        (lower.as_str(), "0")
    };
    let exp_from_e: i32 = exp_part.parse().unwrap_or(0);

    if let Some(dot) = base.find('.') {
        let int_part = &base[..dot];
        let frac_part = &base[dot + 1..];
        let numerator_str = format!("{}{}", int_part, frac_part);
        let mut numerator =
            BigInt::parse_bytes(numerator_str.as_bytes(), 10).unwrap_or_else(|| BigInt::from(0));
        let mut denominator = BigInt::from(10u64).pow(frac_part.len() as u32);
        // apply exponent from scientific notation
        if exp_from_e > 0 {
            // multiply numerator by 10^exp_from_e
            numerator *= BigInt::from(10u64).pow(exp_from_e as u32);
        } else if exp_from_e < 0 {
            denominator *= BigInt::from(10u64).pow((-exp_from_e) as u32);
        }
        if neg {
            numerator = -numerator;
        }
        let g = numerator.clone().abs().gcd(&denominator);
        (numerator / &g, denominator / &g)
    } else {
        // integer-like base
        let mut numerator =
            BigInt::parse_bytes(base.as_bytes(), 10).unwrap_or_else(|| BigInt::from(0));
        let mut denominator = BigInt::from(1u64);
        if exp_from_e > 0 {
            numerator *= BigInt::from(10u64).pow(exp_from_e as u32);
        } else if exp_from_e < 0 {
            denominator *= BigInt::from(10u64).pow((-exp_from_e) as u32);
        }
        if neg {
            numerator = -numerator;
        }
        let g = numerator.clone().abs().gcd(&denominator);
        (numerator / &g, denominator / &g)
    }
}

// Integer power for BigDecimal (exponent >= 0), exponent as u64
fn bigdecimal_pow_integer(mut base: BigDecimal, exp: BigInt) -> BigDecimal {
    // exponent may be large; perform exponentiation by squaring using u32 chunks
    if exp.is_zero() {
        return BigDecimal::from(1);
    }
    let mut result = BigDecimal::from(1);
    // Convert exp to positive BigInt
    let mut e = exp.clone();
    if e < BigInt::from(0) {
        // negative handled by caller
        e = -e;
    }
    // Repeated squaring with BigInt bits
    while !e.is_zero() {
        if (&e & BigInt::from(1u32)) == BigInt::from(1u32) {
            result = result * base.clone();
        }
        e = e >> 1u32;
        if !e.is_zero() {
            base = base.clone() * base.clone();
        }
    }
    result
}

// Compute n-th root of a positive BigDecimal using Newton's method.
// Returns (root, exact) where exact indicates whether root^n == a exactly.
fn bigdecimal_nth_root(
    a: &BigDecimal,
    n: u64,
    precision: usize,
) -> Result<(BigDecimal, bool), i16> {
    if *a == BigDecimal::zero() {
        return Ok((BigDecimal::zero(), true));
    }
    if n == 0 {
        return Err(ERR_INVALID_FORMAT);
    }
    if a.is_negative() {
        // odd roots of negative numbers are allowed; caller should handle sign
    }

    // desired scale (extra guard digits)
    let guard = 10usize;
    let scale = (precision + guard) as i64;

    // initial guess: use 1 or magnitude heuristic
    let mut x = a.with_scale(scale) / BigDecimal::from(n as i64);
    if x == BigDecimal::zero() {
        x = BigDecimal::from(1);
    }

    // convergence tolerance not explicitly used; rely on scale-based checks below
    // Newton iteration: x_{k+1} = (1/n) * ((n-1)*x_k + a / x_k^{n-1})
    for _ in 0..200 {
        // compute x^{n-1}
        let mut x_pow = BigDecimal::from(1);
        for _ in 0..(n - 1) {
            x_pow = x_pow * x.clone();
        }
        if x_pow == BigDecimal::zero() {
            return Err(ERR_INVALID_FORMAT);
        }
        let a_div = (a.with_scale(scale)) / x_pow;
        let numerator = (x.clone() * BigDecimal::from((n - 1) as i64)) + a_div;
        let x_next = numerator / BigDecimal::from(n as i64);

        // convergence check: |x_next - x| < 10^{-precision}
        let diff = if x_next.clone() > x.clone() {
            x_next.clone() - x.clone()
        } else {
            x.clone() - x_next.clone()
        };
        // compare diff to tol (both are BigDecimal with scales)
        if diff.with_scale(0).is_zero() {
            x = x_next;
            break;
        }
        // use string comparison magnitude: convert to scientific string and compare length
        // simpler: stop if diff < 10^{-precision}
        let cmp = diff.with_scale(precision as i64);
        if cmp == BigDecimal::zero() {
            x = x_next;
            break;
        }
        x = x_next;
    }

    // Check exactness: compute x^n and compare to a at high precision
    let mut x_pow_n = BigDecimal::from(1);
    for _ in 0..n {
        x_pow_n = x_pow_n * x.clone();
    }
    // normalize scales for comparison
    let diff = if x_pow_n.clone() > a.clone() {
        x_pow_n.clone() - a.clone()
    } else {
        a.clone() - x_pow_n.clone()
    };
    let approx_zero = diff.with_scale(precision as i64);
    let exact = approx_zero == BigDecimal::zero();
    Ok((x.with_scale(precision as i64), exact))
}

// Compute base^(num/den) where num and den are integers. Returns (bd_result, is_exact)
pub fn pow_bigdecimal_rational(
    base: &BigDecimal,
    num: &BigInt,
    den: &BigInt,
    precision: usize,
) -> Result<(BigDecimal, bool), i16> {
    // Handle negative exponent sign
    let mut numerator = num.clone();
    let denominator = den.clone();
    let neg_exp = numerator.is_negative();
    if neg_exp {
        numerator = -numerator;
    }
    // If denominator == 1 -> integer power
    if denominator == BigInt::from(1u32) {
        let res = bigdecimal_pow_integer(base.clone(), numerator);
        if neg_exp {
            return Ok((BigDecimal::from(1) / res, true));
        }
        return Ok((res, true));
    }

    // Compute base^{numerator} first
    let mut base_pow = BigDecimal::from(1);
    // exponent numerator may be large; but assume reasonable
    let mut n = numerator.clone();
    while n > BigInt::from(0) {
        // multiply by base once; this is naive but acceptable for small numerators
        base_pow = base_pow * base.clone();
        n = n - BigInt::from(1u32);
    }

    // Now take denominator-th root of base_pow
    let den_u64 = denominator.to_u64().unwrap_or(0);
    if den_u64 == 0 {
        return Err(ERR_INVALID_FORMAT);
    }
    let (root, exact) = bigdecimal_nth_root(&base_pow, den_u64, precision)?;
    if neg_exp {
        Ok((
            (BigDecimal::from(1) / root).with_scale(precision as i64),
            exact,
        ))
    } else {
        Ok((root.with_scale(precision as i64), exact))
    }
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
    if b.is_zero() {
        return Err(ERR_DIV_BY_ZERO);
    }
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
    if b.is_zero() {
        return Err(ERR_DIV_BY_ZERO);
    }
    let div_floor = BigDecimal::from(
        a.with_scale(0).to_bigint().unwrap()
            / b.with_scale(0).to_bigint().unwrap_or(BigInt::from(1u32)),
    );
    let res = a - b * div_floor;
    Ok(from_bigdecimal(&res))
}

// Transcendental functions — fast f64-based approximations. Mark result irrational and truncate to 137 decimals.
#[allow(dead_code)]
fn float_from_f64_to_parts(mut v: f64) -> (String, i32, bool) {
    if v.is_nan() {
        return ("".to_string(), 0, false);
    }
    if v.is_infinite() {
        return ("".to_string(), 0, false);
    }
    let neg = v.is_sign_negative();
    if neg {
        v = v.abs();
    }
    let s = format!("{:.50e}", v); // capture enough digits
    // parse scientific notation like 1.2345e+03
    if let Some((base, exp_part)) = s.split_once('e') {
        let exp_i: i32 = exp_part.parse().unwrap_or(0);
        let base = base.replace('.', "").trim_start_matches('0').to_string();
        let computed_exp = exp_i - (base.len() as i32 - 1);
        if base.is_empty() {
            return ("0".to_string(), 0, false);
        }
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
    if res.is_nan() {
        return Err(ERR_INVALID_FORMAT);
    }
    if res.is_infinite() {
        return Err(ERR_INFINITE_RESULT);
    }
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m, e, neg2) = from_bigdecimal(&trunc);
    Ok((m, e, neg2, true))
}

pub fn sqrt_float(mant: String, exp: i32, neg: bool) -> Result<(String, i32, bool, bool), i16> {
    let bd = to_bigdecimal(&mant, exp, neg);
    if bd.is_negative() {
        return Err(ERR_NEGATIVE_SQRT);
    }
    let f = bd.to_f64().ok_or(ERR_INVALID_FORMAT)?;
    let res = f.sqrt();
    if res.is_nan() {
        return Err(ERR_INVALID_FORMAT);
    }
    if res.is_infinite() {
        return Err(ERR_INFINITE_RESULT);
    }
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m2, e2, neg2) = from_bigdecimal(&trunc);
    // consider result irrational if truncated result has fractional digits (exp < 0)
    let is_irrational = e2 < 0;
    Ok((m2, e2, neg2, is_irrational))
}

pub fn cos_float(mant: String, exp: i32, neg: bool) -> Result<(String, i32, bool, bool), i16> {
    let bd = to_bigdecimal(&mant, exp, neg);
    let f = bd.to_f64().ok_or(ERR_INVALID_FORMAT)?;
    let res = f.cos();
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m, e, neg2) = from_bigdecimal(&trunc);
    Ok((m, e, neg2, true))
}

pub fn tan_float(mant: String, exp: i32, neg: bool) -> Result<(String, i32, bool, bool), i16> {
    let bd = to_bigdecimal(&mant, exp, neg);
    let f = bd.to_f64().ok_or(ERR_INVALID_FORMAT)?;
    let res = f.tan();
    if res.is_nan() {
        return Err(ERR_INVALID_FORMAT);
    }
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m, e, neg2) = from_bigdecimal(&trunc);
    Ok((m, e, neg2, true))
}

pub fn ln_float(mant: String, exp: i32, neg: bool) -> Result<(String, i32, bool, bool), i16> {
    let bd = to_bigdecimal(&mant, exp, neg);
    if bd.is_negative() || bd.is_zero() {
        return Err(ERR_INVALID_FORMAT);
    }
    let f = bd.to_f64().ok_or(ERR_INVALID_FORMAT)?;
    let res = f.ln();
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m, e, neg2) = from_bigdecimal(&trunc);
    Ok((m, e, neg2, true))
}

pub fn exp_float(mant: String, exp: i32, neg: bool) -> Result<(String, i32, bool, bool), i16> {
    let bd = to_bigdecimal(&mant, exp, neg);
    let f = bd.to_f64().ok_or(ERR_INVALID_FORMAT)?;
    let res = f.exp();
    if res.is_infinite() {
        return Err(ERR_INFINITE_RESULT);
    }
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m, e, neg2) = from_bigdecimal(&trunc);
    Ok((m, e, neg2, true))
}

pub fn log10_float(mant: String, exp: i32, neg: bool) -> Result<(String, i32, bool, bool), i16> {
    let bd = to_bigdecimal(&mant, exp, neg);
    if bd.is_negative() || bd.is_zero() {
        return Err(ERR_INVALID_FORMAT);
    }
    let f = bd.to_f64().ok_or(ERR_INVALID_FORMAT)?;
    let res = f.log10();
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m, e, neg2) = from_bigdecimal(&trunc);
    Ok((m, e, neg2, true))
}

pub fn floor_float(mant: String, exp: i32, neg: bool) -> Result<(String, i32, bool), i16> {
    let bd = to_bigdecimal(&mant, exp, neg);
    let bi = bd.with_scale(0).to_bigint().unwrap_or(BigInt::from(0));
    let bd_floor = BigDecimal::from(bi.clone());
    Ok(from_bigdecimal(&bd_floor))
}

pub fn ceil_float(mant: String, exp: i32, neg: bool) -> Result<(String, i32, bool), i16> {
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

pub fn abs_float(mant: String, exp: i32, _neg: bool) -> Result<(String, i32, bool), i16> {
    let bd = to_bigdecimal(&mant, exp, false);
    Ok(from_bigdecimal(&bd.abs()))
}

// Integer wrappers for transcendental functions — convert to BigDecimal and call float impls
pub fn sin_int(digits: String, negative: bool) -> Result<(String, i32, bool, bool), i16> {
    let bd = to_bigdecimal(&digits, 0, negative);
    let f = bd.to_f64().ok_or(ERR_INVALID_FORMAT)?;
    let res = f.sin();
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m, e, neg2) = from_bigdecimal(&trunc);
    Ok((m, e, neg2, true))
}

pub fn sqrt_int(digits: String, negative: bool) -> Result<(String, i32, bool, bool), i16> {
    // For integers, if negative -> imaginary handled by caller
    let bd = to_bigdecimal(&digits, 0, negative);
    if bd.is_negative() {
        return Err(ERR_NEGATIVE_SQRT);
    }
    let f = bd.to_f64().ok_or(ERR_INVALID_FORMAT)?;
    let res = f.sqrt();
    if res.is_nan() {
        return Err(ERR_INVALID_FORMAT);
    }
    if res.is_infinite() {
        return Err(ERR_INFINITE_RESULT);
    }
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m2, e2, neg2) = from_bigdecimal(&trunc);
    let is_irrational = e2 < 0;
    Ok((m2, e2, neg2, is_irrational))
}

pub fn cos_int(digits: String, negative: bool) -> Result<(String, i32, bool, bool), i16> {
    let bd = to_bigdecimal(&digits, 0, negative);
    let f = bd.to_f64().ok_or(ERR_INVALID_FORMAT)?;
    let res = f.cos();
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m, e, neg2) = from_bigdecimal(&trunc);
    Ok((m, e, neg2, true))
}

pub fn tan_int(digits: String, negative: bool) -> Result<(String, i32, bool, bool), i16> {
    let bd = to_bigdecimal(&digits, 0, negative);
    let f = bd.to_f64().ok_or(ERR_INVALID_FORMAT)?;
    let res = f.tan();
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m, e, neg2) = from_bigdecimal(&trunc);
    Ok((m, e, neg2, true))
}

pub fn ln_int(digits: String, negative: bool) -> Result<(String, i32, bool, bool), i16> {
    let bd = to_bigdecimal(&digits, 0, negative);
    if bd.is_negative() || bd.is_zero() {
        return Err(ERR_INVALID_FORMAT);
    }
    let f = bd.to_f64().ok_or(ERR_INVALID_FORMAT)?;
    let res = f.ln();
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m, e, neg2) = from_bigdecimal(&trunc);
    Ok((m, e, neg2, true))
}

pub fn exp_int(digits: String, negative: bool) -> Result<(String, i32, bool, bool), i16> {
    let bd = to_bigdecimal(&digits, 0, negative);
    let f = bd.to_f64().ok_or(ERR_INVALID_FORMAT)?;
    let res = f.exp();
    let bdres = BigDecimal::from_f64(res).unwrap_or_else(|| BigDecimal::zero());
    let trunc = truncate_bd_to_decimals(&bdres, 137);
    let (m, e, neg2) = from_bigdecimal(&trunc);
    Ok((m, e, neg2, true))
}

// floor/ceil/abs for ints: identity or abs
pub fn floor_int(digits: String, negative: bool) -> Result<(String, bool), i16> {
    Ok((digits, negative))
}

pub fn ceil_int(digits: String, negative: bool) -> Result<(String, bool), i16> {
    Ok((digits, negative))
}

pub fn abs_int(digits: String, _negative: bool) -> Result<(String, bool), i16> {
    Ok((digits, false))
}
