// math.rs
// String-based arithmetic operations for the ImagNum library.
// This module provides basic arithmetic operations on strings representing large integers.
use std::collections::VecDeque;
use bigdecimal::{BigDecimal, Zero};
use bigdecimal::num_bigint::ToBigInt;
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

fn parse_num(s: &str) -> IntResult<VecDeque<u8>> {
    if s.is_empty() || !s.chars().all(|c| c.is_ascii_digit()) {
        return Err(ERR_INVALID_FORMAT);
    }
    // digits stored least significant digit first
    Ok((s.chars().rev().map(|c| c as u8 - b'0').collect(), false))
}

fn to_string(mut digits: VecDeque<u8>, _negative: bool) -> String {
    while digits.len() > 1 && *digits.back().unwrap() == 0 {
        digits.pop_back();
    }
    digits.into_iter().rev().map(|d| (b'0' + d) as char).collect()
}

fn is_smaller(a: &VecDeque<u8>, b: &VecDeque<u8>) -> bool {
    if a.len() != b.len() {
        return a.len() < b.len();
    }
    for (x, y) in a.iter().rev().zip(b.iter().rev()) {
        if x != y {
            return x < y;
        }
    }
    false
}

fn is_vec_ge(a: &Vec<u32>, b: &Vec<u32>) -> bool {
    if a.len() != b.len() {
        return a.len() > b.len();
    }
    for (x, y) in a.iter().zip(b.iter()) {
        if x != y {
            return x > y;
        }
    }
    true
}

fn vec_sub(a: &Vec<u32>, b: &Vec<u32>) -> Vec<u32> {
    let mut a = a.clone();
    let mut b = b.clone();
    let mut res = Vec::new();
    let mut borrow = 0;

    a.reverse();
    b.reverse();

    for i in 0..a.len() {
        let x = a[i];
        let y = if i < b.len() { b[i] } else { 0 };
        let sub = if x >= y + borrow {
            x - y - borrow
        } else {
            x + 10 - y - borrow
        };
        borrow = if x < y + borrow { 1 } else { 0 };
        res.push(sub);
    }

    while res.len() > 1 && res.last() == Some(&0) {
        res.pop();
    }

    res.reverse();
    res
}

pub fn is_string_odd(s: &str) -> bool {
    s.chars().rev().next().map_or(false, |c| c.to_digit(10).unwrap_or(0) % 2 == 1)
}

pub fn add_strings(a: &str, b: &str) -> IntResult<String> {
    let (mut a, _) = parse_num(a)?;
    let (mut b, _) = parse_num(b)?;
    let mut res = VecDeque::new();
    let mut carry = 0;

    while !a.is_empty() || !b.is_empty() || carry > 0 {
        let x = a.pop_front().unwrap_or(0);
        let y = b.pop_front().unwrap_or(0);
        let sum = x + y + carry;
        res.push_back(sum % 10);
        carry = sum / 10;
    }

    Ok((to_string(res, false), false))
}

pub fn sub_strings(a: &str, b: &str) -> IntResult<String> {
    let (mut a, _) = parse_num(a)?;
    let (mut b, _) = parse_num(b)?;
    let mut flipped = false;

    if is_smaller(&a, &b) {
        std::mem::swap(&mut a, &mut b);
        flipped = true;
    }

    let mut res = VecDeque::new();
    let mut borrow = 0;

    while !a.is_empty() {
        let x = a.pop_front().unwrap();
        let y = b.pop_front().unwrap_or(0);
        let val = if x >= y + borrow {
            x - y - borrow
        } else {
            x + 10 - y - borrow
        };
        borrow = if x < y + borrow { 1 } else { 0 };
        res.push_back(val);
    }

    Ok((to_string(res, flipped), flipped))
}

pub fn mul_strings(a: &str, b: &str) -> IntResult<String> {
    let (a, _) = parse_num(a)?;
    let (b, _) = parse_num(b)?;

    let mut res = vec![0u16; a.len() + b.len()];

    for (i, &x) in a.iter().enumerate() {
        let mut carry = 0u16;
        for (j, &y) in b.iter().enumerate() {
            let idx = i + j;
            let prod = x as u16 * y as u16 + res[idx] + carry;
            res[idx] = prod % 10;
            carry = prod / 10;
        }
        res[i + b.len()] += carry;
    }

    let mut dq: VecDeque<u8> = res.into_iter().map(|v| v as u8).collect();
    while dq.len() > 1 && *dq.back().unwrap() == 0 {
        dq.pop_back();
    }

    Ok((to_string(dq, false), false))
}

pub fn div_strings(a: &str, b: &str) -> IntResult<String> {
    let (a_digits, _) = parse_num(a)?;
    let (b_digits, _) = parse_num(b)?;

    if b_digits.iter().all(|&d| d == 0) {
        return Err(ERR_DIV_BY_ZERO);
    }

    let dividend = a_digits.iter().rev().map(|&d| d as u32).collect::<Vec<u32>>();
    let divisor = b_digits.iter().rev().map(|&d| d as u32).collect::<Vec<u32>>();

    let mut result = Vec::new();
    let mut current = Vec::new();

    for &digit in &dividend {
        current.push(digit);
        while current.len() > 1 && current[0] == 0 {
            current.remove(0);
        }

        let mut count = 0;
        while is_vec_ge(&current, &divisor) {
            current = vec_sub(&current, &divisor);
            count += 1;
        }
        result.push(count as u8);
    }

    result.reverse();

    let dq = VecDeque::from(result);
    Ok((to_string(dq, false), false))
}

pub fn mod_strings(a: &str, b: &str) -> IntResult<String> {
    let (div, _) = div_strings(a, b)?;
    let (prod, _) = mul_strings(&div, b)?;
    let (rem, flipped) = sub_strings(a, &prod)?;
    Ok((rem, flipped))
}

pub fn rem_strings(a: &str, b: &str) -> IntResult<String> {
    let (div, _) = div_strings(a, b)?;
    let (prod, _) = mul_strings(&div, b)?;
    let (rem, flipped) = sub_strings(a, &prod)?;
    Ok((rem, flipped))
}

pub fn pow_strings(base: &str, exponent: &str) -> IntResult<String> {
    let (mut exp, _) = parse_num(exponent)?;
    if exp.is_empty() || exp.iter().all(|&x| x == 0) {
        return Ok(("1".to_string(), false));
    }

    let (mut result, _) = parse_num("1")?;
    let (mut base_val, _) = parse_num(base)?;

    while !exp.iter().all(|&x| x == 0) {
        if exp[0] % 2 == 1 {
            result = parse_num(&mul_strings(&to_string(result.clone(), false), &to_string(base_val.clone(), false))?.0)?.0;
        }
        exp = parse_num(&div_strings(&to_string(exp.clone(), false), "2")?.0)?.0;
        base_val = parse_num(&mul_strings(&to_string(base_val.clone(), false), &to_string(base_val.clone(), false))?.0)?.0;
    }

    Ok((to_string(result, false), false))
}

pub fn sqrt_string(a: &str) -> IntResult<String> {
    if a.starts_with('-') {
        return Err(ERR_NEGATIVE_SQRT);
    }

    let (mut low, _) = parse_num("0")?;
    let (mut high, _) = parse_num(a)?;
    let mut ans = low.clone();

    while is_smaller(&low, &high) || low == high {
        let mid = parse_num(&div_strings(
            &add_strings(&to_string(low.clone(), false), &to_string(high.clone(), false))?.0,
            "2",
        )?.0)?.0;

        let sq = parse_num(&mul_strings(&to_string(mid.clone(), false), &to_string(mid.clone(), false))?.0)?.0;

        if is_smaller(&sq, &parse_num(a)?.0) || sq == parse_num(a)?.0 {
            ans = mid.clone();
            low = parse_num(&add_strings(&to_string(mid, false), "1")?.0)?.0;
        } else {
            high = parse_num(&sub_strings(&to_string(mid, false), "1")?.0)?.0;
        }
    }

    Ok((to_string(ans, false), false))
}

#[allow(dead_code)]
fn normalize(mut mantissa: String, mut exp: i32, negative: bool) -> FloatResult<String> {
    while mantissa.ends_with('0') && mantissa.len() > 1 {
        mantissa.pop();
        exp += 1;
    }
    if mantissa == "0" {
        return Ok(("0".to_string(), 0, false));
    }
    Ok((mantissa, exp, negative))
}


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

    let (mant, exp) = if let Some(dot) = s.find('.') {
        let mant = s[..dot].to_string() + &s[dot+1..];
        let exp = -((s.len() - dot - 1) as i32);
        (mant.trim_start_matches('0').to_string(), exp)
    } else {
        (s.trim_start_matches('0').to_string(), 0)
    };

    (mant, exp, neg)
}

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

    let scale = (mant1.len() + mant2.len()) as i64 + 5;
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
    if mant2 == "0" {
        return Err(ERR_DIV_BY_ZERO);
    }
    let a = to_bigdecimal(&mant1, exp1, neg1);
    let b = to_bigdecimal(&mant2, exp2, neg2);

    let div = &a / &b;
    let div_floor = BigDecimal::from(div.with_scale(0).to_bigint().unwrap());

    let res = a - b * div_floor;

    Ok(from_bigdecimal(&res))
}