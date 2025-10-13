use crate::foundation::{Float, Int};
use crate::math::{
    ERR_DIV_BY_ZERO, ERR_INFINITE_RESULT, ERR_INVALID_FORMAT, ERR_NEGATIVE_RESULT,
    ERR_NEGATIVE_SQRT, ERR_NUMBER_TOO_LARGE, ERR_UNIMPLEMENTED,
};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{Signed, Zero, ToPrimitive};
use std::str::FromStr;

pub fn create_int(int: &str) -> Int {
    let s = int.trim();
    if s.is_empty() {
        return Int::new();
    }

    let low = s.to_ascii_lowercase();
    if low == "nan" || low == "inf" || low == "infinity" || low == "-inf" || low == "-infinity" {
        return Int::new();
    }

    if s.contains('.') {
        return Int::new();
    }

    match BigInt::from_str(s) {
        Ok(b) => Int::Big(b),
        Err(_) => Int::new(),
    }
}

pub fn create_float(float: &str) -> Float {
    let s = float.trim();
    if s.is_empty() {
        return Float::Big(BigDecimal::from(0));
    }

    let lower = s.to_ascii_lowercase();
    if lower == "nan" {
        return Float::NaN;
    }
    if lower == "inf" || lower == "infinity" {
        return Float::Infinity;
    }
    if lower == "-inf" || lower == "-infinity" {
        return Float::NegInfinity;
    }

    if lower.ends_with('i') {
        let without_i = &s[..s.len() - 1];
        let coeff = if without_i.is_empty() || without_i == "+" {
            "1"
        } else if without_i == "-" {
            "-1"
        } else {
            without_i
        };
        let bd = BigDecimal::from_str(coeff).unwrap_or_else(|_| BigDecimal::from(0));
        let zero = Float::Big(BigDecimal::from(0));
        let imag = Float::Big(bd);
        return Float::Complex(Box::new(zero), Box::new(imag));
    }

    if let Some(lp) = s.find('(') {
        if s.ends_with(')') {
            let repeat = &s[lp + 1..s.len() - 1];
            let before = &s[..lp];
            let sign = if before.starts_with('-') { -1 } else { 1 };
            let before_nosign = if before.starts_with('+') || before.starts_with('-') {
                &before[1..]
            } else {
                before
            };
            let (int_part_str, nonrep_str) = if let Some(dot) = before_nosign.find('.') {
                (&before_nosign[..dot], &before_nosign[dot + 1..])
            } else {
                (before_nosign, "")
            };

            if !int_part_str.chars().all(|c| c.is_ascii_digit())
                || !nonrep_str.chars().all(|c| c.is_ascii_digit())
                || !repeat.chars().all(|c| c.is_ascii_digit())
            {
                return Float::NaN;
            }

            let ip = BigInt::from_str(if int_part_str.is_empty() {
                "0"
            } else {
                int_part_str
            })
            .unwrap_or_else(|_| BigInt::from(0));
            let nonrep = if nonrep_str.is_empty() {
                BigInt::from(0)
            } else {
                BigInt::from_str(nonrep_str).unwrap_or_else(|_| BigInt::from(0))
            };
            let rep = BigInt::from_str(repeat).unwrap_or_else(|_| BigInt::from(0));

            let len_nonrep = nonrep_str.len() as u32;
            let len_rep = repeat.len() as u32;

            let ten = BigInt::from(10u32);
            let pow_nr = ten.pow(len_nonrep);
            let pow_r = ten.pow(len_rep);
            let denom = &pow_nr * (&pow_r - BigInt::from(1u32));

            let numer_frac = &nonrep * (&pow_r - BigInt::from(1u32)) + &rep;

            let mut total_num = &ip * &denom + numer_frac;
            if sign < 0 {
                total_num = -total_num;
            }

            use std::collections::HashMap;
            let mut num_abs = total_num.clone();
            let den_abs = denom.clone().abs();
            let neg = total_num.sign() == num_bigint::Sign::Minus;
            if neg { num_abs = -num_abs.clone(); }
            let int_part = (&num_abs / &den_abs).to_string();
            let mut rem = num_abs % &den_abs;
            let mut seen: HashMap<BigInt, usize> = HashMap::new();
            let mut digits: Vec<char> = Vec::new();
            let max_digits = 10000usize;
            while !rem.is_zero() && !seen.contains_key(&rem) && digits.len() < max_digits {
                seen.insert(rem.clone(), digits.len());
                rem = rem * BigInt::from(10u32);
                let q = (&rem / &den_abs).to_i32().unwrap_or(0);
                digits.push(std::char::from_digit(q as u32, 10).unwrap_or('0'));
                rem = rem % &den_abs;
            }

            let mut frac_str = String::new();
            if digits.is_empty() {
                let s_out = if neg { format!("-{}.0", int_part) } else { format!("{}.0", int_part) };
                let bd = BigDecimal::from_str(&s_out).unwrap_or_else(|_| BigDecimal::from(0));
                return Float::Big(bd);
            } else {
                if let Some(start) = seen.get(&rem) {
                    let start = *start;
                    let nonrep: String = digits[..start].iter().collect();
                    let rep: String = digits[start..].iter().collect();
                    let min_repeats = 4usize;
                    let repeat_count = min_repeats;
                    frac_str.push_str(&nonrep);
                    for _ in 0..repeat_count {
                        frac_str.push_str(&rep);
                    }
                } else {
                    for d in digits.iter() { frac_str.push(*d); }
                }
            }

            let digits_concat = format!("{}{}", int_part.trim_start_matches('-'), frac_str);
            match BigInt::from_str(&digits_concat) {
                Ok(mut bi) => {
                    if neg {
                        bi = -bi;
                    }
                    let scale = frac_str.len() as i64;
                    let bd = BigDecimal::new(bi, scale);
                    return Float::Recurring(bd);
                }
                Err(_) => {
                    let s_out = if neg { format!("-{}.{}", int_part, frac_str) } else { format!("{}.{}", int_part, frac_str) };
                    let bd = BigDecimal::from_str(&s_out).unwrap_or_else(|_| BigDecimal::from(0));
                    return Float::Recurring(bd);
                }
            }
        }
    }

    match BigDecimal::from_str(s) {
        Ok(bd) => Float::Big(bd),
        Err(_) => Float::NaN,
    }
}

pub fn create_imaginary() -> Float {
    let zero = BigDecimal::from(0);
    let one = BigDecimal::from(1);
    Float::Complex(Box::new(Float::Big(zero)), Box::new(Float::Big(one)))
}

pub fn create_complex(real: &str, imag: &str) -> Float {
    let real_part = create_float(real);
    let imag_part = create_float(imag);
    Float::Complex(Box::new(real_part), Box::new(imag_part))
}

pub fn get_error_message(code: i16) -> &'static str {
    match code {
        ERR_INVALID_FORMAT => "Invalid format",
        ERR_DIV_BY_ZERO => "Division by zero",
        ERR_NEGATIVE_RESULT => "Negative result",
        ERR_NUMBER_TOO_LARGE => "Number too large",
        ERR_INFINITE_RESULT => "Infinite result",
        ERR_UNIMPLEMENTED => "Operation not implemented",
        ERR_NEGATIVE_SQRT => "Square root of a negative number",
        _ => "Unknown error",
    }
}

pub fn get_error_code(message: &str) -> i16 {
    match message.to_lowercase().trim() {
        "invalid format" => ERR_INVALID_FORMAT,
        "division by zero" => ERR_DIV_BY_ZERO,
        "negative result" => ERR_NEGATIVE_RESULT,
        "number too large" => ERR_NUMBER_TOO_LARGE,
        "infinite result" => ERR_INFINITE_RESULT,
        "operation not implemented" => ERR_UNIMPLEMENTED,
        "square root of a negative number" => ERR_NEGATIVE_SQRT,
        _ => 0, // Unknown error
    }
}

#[macro_export]
macro_rules! int {
    ($val:expr) => {
        create_int($val)
    };
}

#[macro_export]
macro_rules! float {
    ($val:expr) => {
        create_float($val)
    };
}
