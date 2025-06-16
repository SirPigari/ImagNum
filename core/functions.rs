use crate::foundation::{Float, Int, NumberKind};
use crate::math::{
    ERR_INVALID_FORMAT, ERR_DIV_BY_ZERO,
    ERR_NEGATIVE_RESULT, ERR_NUMBER_TOO_LARGE,
    ERR_INFINITE_RESULT, ERR_UNIMPLEMENTED,
    ERR_NEGATIVE_SQRT,
};

pub fn create_int(int: &str) -> Int {
    let negative = int.trim().starts_with('-');
    let digits = if negative { &int[1..] } else { int };

    let kind = match digits.to_ascii_lowercase().as_str() {
        "nan" => NumberKind::NaN,
        "inf" | "infinity" => {
            if negative {
                NumberKind::NegInfinity
            } else {
                NumberKind::Infinity
            }
        }
        _ => NumberKind::Finite,
    };

    let digits = match kind {
        NumberKind::NaN | NumberKind::Infinity | NumberKind::NegInfinity => String::new(),
        _ => digits.to_string(),
    };

    Int::new(digits, negative, kind)
}

pub fn create_float(float: &str) -> Float {
    let negative = float.trim().starts_with('-');
    let trimmed = if negative { &float[1..] } else { float }.to_ascii_lowercase();

    let kind = if trimmed == "nan" {
        NumberKind::NaN
    } else if trimmed == "inf" || trimmed == "infinity" {
        if negative {
            NumberKind::NegInfinity
        } else {
            NumberKind::Infinity
        }
    } else if trimmed.ends_with('i') {
        NumberKind::Imaginary
    } else {
        NumberKind::Finite
    };

    if matches!(kind, NumberKind::NaN | NumberKind::Infinity | NumberKind::NegInfinity) {
        return Float::new(String::new(), 0, negative, kind);
    }

    let (base, exp_part) = if let Some(e_index) = trimmed.find('e') {
        let (b, e) = trimmed.split_at(e_index);
        (b, e[1..].parse::<i32>().unwrap_or(0))
    } else {
        (trimmed.as_str(), 0)
    };

    let mut mantissa = String::new();

    for c in base.chars() {
        if c != '.' {
            mantissa.push(c);
        }
    }

    let decimal_index = base.find('.').unwrap_or(base.len());
    let digits_after_dot = base.len().saturating_sub(decimal_index + 1);
    let exponent = exp_part - digits_after_dot as i32;

    Float::new(mantissa, exponent, negative, kind)
}

pub fn create_imaginary() -> Float {
    Float::new(String::new(), 0, false, NumberKind::Imaginary)
}

pub fn create_imaginary_int() -> Int {
    Int::new(String::new(), false, NumberKind::Imaginary)
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