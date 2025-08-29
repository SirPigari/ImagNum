use crate::foundation::{Float, Int};
use crate::math::{
    ERR_DIV_BY_ZERO, ERR_INFINITE_RESULT, ERR_INVALID_FORMAT, ERR_NEGATIVE_RESULT,
    ERR_NEGATIVE_SQRT, ERR_NUMBER_TOO_LARGE, ERR_UNIMPLEMENTED,
};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use std::str::FromStr;

pub fn create_int(int: &str) -> Int {
    let s = int.trim();
    if s.is_empty() {
        return Int::new();
    }

    // Integers shouldn't accept NaN/Infinity — those are floats. Return 0 for those inputs.
    let low = s.to_ascii_lowercase();
    if low == "nan" || low == "inf" || low == "infinity" || low == "-inf" || low == "-infinity" {
        return Int::new();
    }

    // Reject floats passed as ints (contain '.') -> return zero Int
    if s.contains('.') {
        return Int::new();
    }

    // Parse into BigInt; try small optimizations later
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

    // Imaginary numbers ending with i -> treat as complex (0 + 1i * value)
    if lower.ends_with('i') {
        let without_i = &s[..s.len() - 1];
        // parse the coefficient; default 1
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

    // Otherwise parse as BigDecimal
    // Support recurring decimal notation like 0.(9) or -1.2(34)
    if let Some(lp) = s.find('(') {
        if s.ends_with(')') {
            // split into before '(' and repeating digits
            let repeat = &s[lp + 1..s.len() - 1];
            let before = &s[..lp];
            // before should contain optional sign, integer part and optional non-repeating fractional part
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

            // validate digits
            if !int_part_str.chars().all(|c| c.is_ascii_digit())
                || !nonrep_str.chars().all(|c| c.is_ascii_digit())
                || !repeat.chars().all(|c| c.is_ascii_digit())
            {
                return Float::NaN;
            }

            // parse BigInt components
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

            // lengths
            let len_nonrep = nonrep_str.len() as u32;
            let len_rep = repeat.len() as u32;

            // denom = 10^{len_nonrep} * (10^{len_rep} - 1)
            let ten = BigInt::from(10u32);
            let pow_nr = ten.pow(len_nonrep);
            let pow_r = ten.pow(len_rep);
            let denom = &pow_nr * (&pow_r - BigInt::from(1u32));

            // numer fractional part = nonrep * (10^{len_rep} - 1) + rep
            let numer_frac = &nonrep * (&pow_r - BigInt::from(1u32)) + &rep;

            // total numerator = ip * denom + numer_frac
            let mut total_num = &ip * &denom + numer_frac;
            if sign < 0 {
                total_num = -total_num;
            }

            // Construct BigDecimal as exact rational and mark Recurring
            let bd = BigDecimal::new(total_num, 0) / BigDecimal::new(denom, 0);
            return Float::Recurring(bd);
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
