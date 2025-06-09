// foundation.rs
// This file contains the basic numeric types and their implementations for the ImagNum library.

use std::fmt;
use std::ops::{Not, BitAnd, BitOr};
use std::str::FromStr;
use std::fmt::Display;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumberKind {
    NaN,
    Infinity,
    NegInfinity,
    Irrational,
    Rational,
    Finite,
    Imaginary,
    Complex,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Int {
    pub digits: String,
    pub negative: bool,
    pub kind: NumberKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Float {
    pub mantissa: String,
    pub exponent: i64,
    pub negative: bool,
    pub kind: NumberKind,
}

impl Int {
    pub fn new(digits: String, negative: bool, kind: NumberKind) -> Self {
        Int { digits, negative, kind }
    }

    pub fn new_from_i64(value: i64) -> Self {
        let negative = value < 0;
        let digits = value.abs().to_string();
        Int {
            digits,
            negative,
            kind: NumberKind::Finite,
        }
    }
    
    pub fn new_from_str(value: &str) -> Self {
        let trimmed = value.trim();
        let (negative, digits_str) = if let Some(stripped) = trimmed.strip_prefix('-') {
            (true, stripped)
        } else {
            (false, trimmed)
        };

        if !digits_str.chars().all(|c| c.is_ascii_digit()) || digits_str.is_empty() {
            return Int {
                digits: String::new(),
                negative,
                kind: NumberKind::NaN,
            };
        }

        Int {
            digits: digits_str.to_string(),
            negative,
            kind: NumberKind::Finite,
        }
    }
}

impl Float {
    pub fn new(mantissa: String, exponent: i64, negative: bool, kind: NumberKind) -> Self {
        Float { mantissa, exponent, negative, kind }
    }

    pub fn new_from_f64(value: f64) -> Self {
        if value.is_nan() {
            return Float {
                mantissa: String::new(),
                exponent: 0,
                negative: false,
                kind: NumberKind::NaN,
            };
        }
        if value.is_infinite() {
            return Float {
                mantissa: String::new(),
                exponent: 0,
                negative: value.is_sign_negative(),
                kind: if value.is_sign_positive() { NumberKind::Infinity } else { NumberKind::NegInfinity },
            };
        }

        let negative = value.is_sign_negative();
        let abs_val = value.abs();

        let sci_str = format!("{:e}", abs_val);
        let parts: Vec<&str> = sci_str.split('e').collect();

        let mantissa_raw = parts[0].replace('.', "");
        let exponent = parts.get(1).and_then(|e| e.parse::<i64>().ok()).unwrap_or(0);

        Float {
            mantissa: mantissa_raw,
            exponent,
            negative,
            kind: NumberKind::Finite,
        }
    }

    pub fn new_from_str(value: &str) -> Self {
        let trimmed = value.trim();

        if trimmed.eq_ignore_ascii_case("nan") {
            return Float {
                mantissa: String::new(),
                exponent: 0,
                negative: false,
                kind: NumberKind::NaN,
            };
        }

        if trimmed.eq_ignore_ascii_case("inf") || trimmed.eq_ignore_ascii_case("infinity") {
            return Float {
                mantissa: String::new(),
                exponent: 0,
                negative: false,
                kind: NumberKind::Infinity,
            };
        }

        if trimmed.eq_ignore_ascii_case("-inf") || trimmed.eq_ignore_ascii_case("-infinity") {
            return Float {
                mantissa: String::new(),
                exponent: 0,
                negative: true,
                kind: NumberKind::NegInfinity,
            };
        }

        if let Ok(f_val) = trimmed.parse::<f64>() {
            return Self::new_from_f64(f_val);
        }

        Float {
            mantissa: String::new(),
            exponent: 0,
            negative: false,
            kind: NumberKind::NaN,
        }
    }
}
