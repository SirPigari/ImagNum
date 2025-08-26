use crate::foundation::{Float, Int};
use num_bigint::BigInt;
use bigdecimal::BigDecimal;
use std::str::FromStr;
use crate::math::{
    ERR_INVALID_FORMAT, ERR_DIV_BY_ZERO,
    ERR_NEGATIVE_RESULT, ERR_NUMBER_TOO_LARGE,
    ERR_INFINITE_RESULT, ERR_UNIMPLEMENTED,
    ERR_NEGATIVE_SQRT,
};

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
        let without_i = &s[..s.len()-1];
        // parse the coefficient; default 1
        let coeff = if without_i.is_empty() || without_i == "+" { "1" } else if without_i == "-" { "-1" } else { without_i };
        let bd = BigDecimal::from_str(coeff).unwrap_or_else(|_| BigDecimal::from(0));
        let zero = Float::Big(BigDecimal::from(0));
        let imag = Float::Big(bd);
        return Float::Complex(Box::new(zero), Box::new(imag));
    }

    // Otherwise parse as BigDecimal
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