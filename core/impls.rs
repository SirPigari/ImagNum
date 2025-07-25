use crate::foundation::{Float, Int, NumberKind};
#[allow(unused_imports)]
use crate::math::{
    add_strings, sub_strings, mul_strings, div_strings, mod_strings, pow_strings, sqrt_string, is_string_odd,
    add_float, sub_float, mul_float, div_float, mod_float,
    ERR_UNIMPLEMENTED, ERR_INVALID_FORMAT, ERR_DIV_BY_ZERO, ERR_NEGATIVE_RESULT, ERR_NEGATIVE_SQRT, ERR_NUMBER_TOO_LARGE, ERR_INFINITE_RESULT
};
use crate::functions::{create_float, create_int, create_imaginary};
use std::fmt::{Binary, Octal, LowerHex};
use std::hash::{Hash, Hasher};

impl Int {
    pub fn to_float(&self) -> Result<Float, i16> {
        if self.kind == NumberKind::NaN {
            return Err(ERR_INVALID_FORMAT);
        }
        if self.kind == NumberKind::Infinity {
            return Ok(Float::new("Infinity".to_string(), 0, false, NumberKind::Infinity));
        }
        if self.kind == NumberKind::NegInfinity {
            return Ok(Float::new("Infinity".to_string(), 0, true, NumberKind::NegInfinity));
        }
        if self.digits.is_empty() || self.digits == "0" {
            return Ok(Float::new("0".to_string(), 0, false, NumberKind::Finite));
        }

        let mantissa = self.digits.clone();
        let exponent = 0;

        Ok(Float::new(mantissa, exponent, self.negative, NumberKind::Finite))
    }
    pub fn _add(&self, other: &Self) -> Result<Self, i16> {
        match (self.negative, other.negative) {
            (false, false) => {
                let (digits, _) = add_strings(&self.digits, &other.digits)?;
                let digits = normalize_int_digits(&digits);
                Ok(Int::new(digits, false, NumberKind::Finite))
            }
            (true, true) => {
                let (digits, _) = add_strings(&self.digits, &other.digits)?;
                let digits = normalize_int_digits(&digits);
                Ok(Int::new(digits, true, NumberKind::Finite))
            }
            (false, true) => {
                self._sub(&Int::new(other.digits.clone(), false, other.kind))
            }
            (true, false) => {
                let mut res = other._sub(&Int::new(self.digits.clone(), true, self.kind))?;
                res.negative = !res.negative;
                Ok(res)
            }            
        }
    }
    pub fn _sub(&self, other: &Self) -> Result<Self, i16> {
        match (self.negative, other.negative) {
            (false, false) => {
                let (digits, sign_flipped) = sub_strings(&self.digits, &other.digits)?;
                let digits = normalize_int_digits(&digits);
                let negative = if digits == "0" {
                    false
                } else {
                    sign_flipped
                };
                Ok(Int::new(digits, negative, NumberKind::Finite))
            }
            (true, true) => {
                let res = Int::new(other.digits.clone(), false, other.kind)._sub(&Int::new(self.digits.clone(), false, self.kind))?;
                Ok(Int {
                    digits: res.digits,
                    negative: res.negative,
                    kind: NumberKind::Finite,
                })
            }
            (false, true) => {
                self._add(&Int::new(other.digits.clone(), false, other.kind))
            }
            (true, false) => {
                let mut res = Int::new(self.digits.clone(), false, self.kind)._add(other)?;
                res.negative = true;
                Ok(res)
            }
        }
    }
    pub fn _mul(&self, other: &Self) -> Result<Self, i16> {
        let (digits, sign_flipped) = mul_strings(&self.digits, &other.digits)?;
        let digits = normalize_int_digits(&digits);
        let negative = self.negative ^ other.negative ^ sign_flipped;
        Ok(Int::new(digits, negative, NumberKind::Finite))
    }
    pub fn _div(&self, other: &Self) -> Result<Self, i16> {
        let (digits, sign_flipped) = div_strings(&self.digits, &other.digits)?;
        let digits = normalize_int_digits(&digits);
        let negative = self.negative ^ other.negative ^ sign_flipped;
        Ok(Int::new(digits, negative, NumberKind::Finite))
    }
    pub fn _modulo(&self, other: &Self) -> Result<Self, i16> {
        let (digits, sign_flipped) = mod_strings(&self.digits, &other.digits)?;
        let digits = normalize_int_digits(&digits);
        let negative = self.negative ^ sign_flipped;
        Ok(Int::new(digits, negative, NumberKind::Finite))
    }
    pub fn pow(&self, exponent: &Self) -> Result<Self, i16> {
        if exponent.negative {
            return Err(ERR_INVALID_FORMAT);
        }
        let (digits, sign_flipped) = pow_strings(&self.digits, &exponent.digits)?;
        let digits = normalize_int_digits(&digits);
        let negative = if self.negative && is_string_odd(&exponent.digits) {
            true ^ sign_flipped
        } else {
            sign_flipped
        };
        Ok(Int::new(digits, negative, NumberKind::Finite))
    }
    pub fn sqrt(&self) -> Result<Float, i16> {
        if self.negative {
            return Ok(create_imaginary());
        }
        if self.digits.is_empty() || self.digits == "0" {
            return Ok(Float::new("0".to_string(), 0, false, NumberKind::Finite));
        }
        if self.digits == "1" {
            return Ok(Float::new("1".to_string(), 0, false, NumberKind::Finite));
        }
        let self_float = self.to_float()?;
        self_float.sqrt()
    }
    pub fn abs(&self) -> Self {
        Int::new(self.digits.clone(), false, self.kind)
    }

    pub fn is_zero(&self) -> bool {
        self.digits.is_empty() || self.digits == "0"
    }
    pub fn to_usize(&self) -> Result<usize, i16> {
        if self.kind == NumberKind::NaN {
            return Err(ERR_INVALID_FORMAT);
        }
        if self.kind == NumberKind::Infinity || self.kind == NumberKind::NegInfinity {
            return Err(ERR_INFINITE_RESULT);
        }
        if self.negative || self.digits.is_empty() || self.digits == "0" {
            return Err(ERR_NEGATIVE_RESULT);
        }

        let value: usize = self.digits.parse().map_err(|_| ERR_INVALID_FORMAT)?;
        Ok(value)
    }
    pub fn to_i64(&self) -> Result<i64, i16> {
        if self.kind == NumberKind::NaN {
            return Err(ERR_INVALID_FORMAT);
        }
        if self.kind == NumberKind::Infinity || self.kind == NumberKind::NegInfinity {
            return Err(ERR_INFINITE_RESULT);
        }

        if self.digits.is_empty() || self.digits == "0" {
            return Ok(0 as i64);
        }

        let value = if self.negative {
            -self.digits.parse::<i64>().map_err(|_| ERR_INVALID_FORMAT)?
        } else {
            self.digits.parse::<i64>().map_err(|_| ERR_INVALID_FORMAT)?
        };
        Ok(value)
    }
    pub fn from_i64(value: i64) -> Self {
        if value < 0 {
            Int::new(value.abs().to_string(), true, NumberKind::Finite)
        } else {
            Int::new(value.to_string(), false, NumberKind::Finite)
        }
    }
    pub fn from_str(value: &str) -> Result<Self, i16> {
        if value.is_empty() {
            return Err(ERR_INVALID_FORMAT);
        }
        let int = create_int(value);
        if int.kind == NumberKind::NaN || int.kind == NumberKind::Infinity || int.kind == NumberKind::NegInfinity {
            return Err(ERR_INVALID_FORMAT);
        }
        Ok(int)
    }
    pub fn is_nan(&self) -> bool {
        self.kind == NumberKind::NaN
    }
    pub fn is_infinity(&self) -> bool {
        self.kind == NumberKind::Infinity
    }
}

impl Float {
    pub fn to_f64(&self) -> Result<f64, i16> {
        if self.kind == NumberKind::NaN {
            return Err(ERR_INVALID_FORMAT);
        }
        if self.kind == NumberKind::Infinity {
            return Ok(f64::INFINITY);
        }
        if self.kind == NumberKind::NegInfinity {
            return Ok(f64::NEG_INFINITY);
        }

        let mut mantissa = self.mantissa.clone();
        if self.negative {
            mantissa.insert(0, '-');
        }

        let exponent = self.exponent;
        let value: f64 = match exponent {
            0 => mantissa.parse().map_err(|_| ERR_INVALID_FORMAT)?,
            _ => {
                let base: f64 = mantissa.parse().map_err(|_| ERR_INVALID_FORMAT)?;
                base * 10f64.powi(exponent)
            }
        };
        
        Ok(value)
    }
    pub fn sqrt(&self) -> Result<Self, i16> {
        if self.kind == NumberKind::NaN {
            return Err(ERR_INVALID_FORMAT);
        }
        if self.kind == NumberKind::Infinity {
            return Ok(Float::new("Infinity".to_string(), 0, false, NumberKind::Infinity));
        }
        if self.kind == NumberKind::NegInfinity || self.negative {
            return Err(ERR_NEGATIVE_SQRT);
        }
        let self_f64 = self.to_f64()?;
        if self_f64 < 0.0 {
            return Ok(create_imaginary());
        }
        if self_f64 == 0.0 {
            return Ok(Float::new("0".to_string(), 0, false, NumberKind::Finite));
        }
        let sqrt_value = self_f64.sqrt();
        let mut mantissa = sqrt_value.to_string();
        let exponent = if mantissa.contains('.') {
            let parts: Vec<&str> = mantissa.split('.').collect();
            let integer_part = parts[0];
            let fractional_part = parts[1];
            let exponent = -(fractional_part.len() as i32);
            mantissa = integer_part.to_string() + fractional_part;
            exponent
        } else {
            0
        };
        mantissa = normalize_int_digits(&mantissa);
        Ok(Float::new(mantissa, exponent, false, NumberKind::Finite))
    }
    pub fn _add(&self, other: &Self) -> Result<Self, i16> {
        if self.kind == NumberKind::NaN || other.kind == NumberKind::NaN {
            return Err(ERR_INVALID_FORMAT);
        }
        if self.kind == NumberKind::Infinity && other.kind == NumberKind::Infinity {
            return Ok(Float::new("Infinity".to_string(), 0, false, NumberKind::Infinity));
        }
        if self.kind == NumberKind::NegInfinity && other.kind == NumberKind::NegInfinity {
            return Ok(Float::new("Infinity".to_string(), 0, true, NumberKind::NegInfinity));
        }
        if (self.kind == NumberKind::Infinity && other.kind == NumberKind::NegInfinity) ||
           (self.kind == NumberKind::NegInfinity && other.kind == NumberKind::Infinity) {
            return Err(ERR_INFINITE_RESULT);
        }
    
        let (mantissa, exponent, negative) = add_float(
            self.mantissa.clone(), self.exponent, self.negative,
            other.mantissa.clone(), other.exponent, other.negative,
        )?;
    
        Ok(Float::new(mantissa, exponent, negative, NumberKind::Finite))
    }
    pub fn _sub(&self, other: &Self) -> Result<Self, i16> {
        if self.kind == NumberKind::NaN || other.kind == NumberKind::NaN {
            return Err(ERR_INVALID_FORMAT);
        }
        if self.kind == NumberKind::Infinity && other.kind == NumberKind::Infinity {
            return Ok(Float::new("0".to_string(), 0, false, NumberKind::Finite));
        }
        if self.kind == NumberKind::NegInfinity && other.kind == NumberKind::NegInfinity {
            return Ok(Float::new("0".to_string(), 0, true, NumberKind::Finite));
        }
        if (self.kind == NumberKind::Infinity && other.kind == NumberKind::NegInfinity) ||
           (self.kind == NumberKind::NegInfinity && other.kind == NumberKind::Infinity) {
            return Err(ERR_INFINITE_RESULT);
        }
    
        let (mantissa, exponent, negative) = sub_float(
            self.mantissa.clone(), self.exponent, self.negative,
            other.mantissa.clone(), other.exponent, other.negative,
        )?;
    
        Ok(Float::new(mantissa, exponent, negative, NumberKind::Finite))
    }
    pub fn _mul(&self, other: &Self) -> Result<Self, i16> {
        if self.kind == NumberKind::NaN || other.kind == NumberKind::NaN {
            return Err(ERR_INVALID_FORMAT);
        }
        if self.kind == NumberKind::Infinity || other.kind == NumberKind::Infinity {
            return Ok(Float::new("Infinity".to_string(), 0, self.negative ^ other.negative, NumberKind::Infinity));
        }
        if self.kind == NumberKind::NegInfinity || other.kind == NumberKind::NegInfinity {
            return Ok(Float::new("Infinity".to_string(), 0, !(self.negative ^ other.negative), NumberKind::NegInfinity));
        }

        let (mantissa, exponent, negative) = mul_float(
            self.mantissa.clone(), self.exponent, self.negative,
            other.mantissa.clone(), other.exponent, other.negative,
        )?;

        Ok(Float::new(mantissa, exponent, negative, NumberKind::Finite))
    }
    pub fn _div(&self, other: &Self) -> Result<Self, i16> {
        if self.kind == NumberKind::NaN || other.kind == NumberKind::NaN {
            return Err(ERR_INVALID_FORMAT);
        }
        if other.mantissa.is_empty() || other.mantissa == "0" && other.exponent == 0 && self.exponent == 0 {
            return Err(ERR_DIV_BY_ZERO);
        }
        if self.kind == NumberKind::Infinity && other.kind == NumberKind::Infinity {
            return Ok(Float::new("NaN".to_string(), 0, false, NumberKind::NaN));
        }
        if self.kind == NumberKind::NegInfinity && other.kind == NumberKind::NegInfinity {
            return Ok(Float::new("NaN".to_string(), 0, false, NumberKind::NaN));
        }
        if (self.kind == NumberKind::Infinity && other.kind == NumberKind::NegInfinity) ||
           (self.kind == NumberKind::NegInfinity && other.kind == NumberKind::Infinity) {
            return Ok(Float::new("0".to_string(), 0, self.negative ^ other.negative, NumberKind::Finite));
        }

        let (mantissa, exponent, negative) = div_float(
            self.mantissa.clone(), self.exponent, self.negative,
            other.mantissa.clone(), other.exponent, other.negative,
        )?;

        Ok(Float::new(mantissa, exponent, negative, NumberKind::Finite))
    }
    pub fn _modulo(&self, other: &Self) -> Result<Self, i16> {
        if self.kind == NumberKind::NaN || other.kind == NumberKind::NaN {
            return Err(ERR_INVALID_FORMAT);
        }
        if other.mantissa.is_empty() || other.mantissa == "0" && other.exponent == 0 && self.exponent == 0 {
            return Err(ERR_DIV_BY_ZERO);
        }
        if self.kind == NumberKind::Infinity || self.kind == NumberKind::NegInfinity {
            return Ok(Float::new("NaN".to_string(), 0, false, NumberKind::NaN));
        }

        let (mantissa, exponent, negative) = mod_float(
            self.mantissa.clone(), self.exponent, self.negative,
            other.mantissa.clone(), other.exponent, other.negative,
        )?;

        Ok(Float::new(mantissa, exponent, negative, NumberKind::Finite))
    }
    pub fn _pow(&self, exponent: &Self) -> Result<Self, i16> {
        if self.kind == NumberKind::NaN || exponent.kind == NumberKind::NaN {
            return Err(ERR_INVALID_FORMAT);
        }
        if self.kind == NumberKind::Infinity && exponent.mantissa == "0" {
            return Ok(Float::new("1".to_string(), 0, false, NumberKind::Finite));
        }
        if self.kind == NumberKind::NegInfinity && exponent.mantissa == "0" {
            return Ok(Float::new("1".to_string(), 0, true, NumberKind::Finite));
        }
        if self.kind == NumberKind::Infinity || self.kind == NumberKind::NegInfinity {
            return Ok(Float::new(
                "Infinity".to_string(),
                0,
                self.negative ^ exponent.negative,
                NumberKind::Infinity,
            ));
        }
    
        if self.mantissa.len() > 17 || self.exponent > 308 || self.exponent < -308 {
            return Err(ERR_NUMBER_TOO_LARGE);
        }
        if exponent.mantissa.len() > 17 || exponent.exponent > 308 || exponent.exponent < -308 {
            return Err(ERR_NUMBER_TOO_LARGE);
        }
    
        let base_sign = if self.negative { -1.0 } else { 1.0 };
        let base_val: f64 = self.mantissa.parse::<f64>().unwrap_or(0.0);
        let base_f64 = base_sign * base_val * 10f64.powi(self.exponent);
    
        let exp_sign = if exponent.negative { -1.0 } else { 1.0 };
        let exp_val: f64 = exponent.mantissa.parse::<f64>().unwrap_or(0.0);
        let exponent_f64 = exp_sign * exp_val * 10f64.powi(exponent.exponent);
    
        let pow_res = base_f64.powf(exponent_f64);
    
        if pow_res.is_nan() {
            return Err(ERR_INVALID_FORMAT);
        }
        if pow_res.is_infinite() {
            return Ok(Float::new(
                "Infinity".to_string(),
                0,
                pow_res.is_sign_negative(),
                NumberKind::Infinity,
            ));
        }
    
        let negative = pow_res.is_sign_negative();
        let abs_res = pow_res.abs();
    
        if abs_res == 0.0 {
            return Ok(Float::new("0".to_string(), 0, false, NumberKind::Finite));
        }
    
        let exp = abs_res.log10().floor() as i32;
        let mant = abs_res / 10f64.powi(exp);
    
        let digits = 15;
        let scaled_mant = (mant * 10f64.powi(digits)).round() as u64;
        let mut mantissa_str = scaled_mant.to_string();
        let mut final_exp = exp - digits;
    
        while mantissa_str.ends_with('0') && mantissa_str.len() > 1 {
            mantissa_str.pop();
            final_exp += 1;
        }
    
        Ok(Float::new(mantissa_str, final_exp, negative, NumberKind::Finite))
    }
    
    pub fn pow(&self, exponent: &Self) -> Result<Self, i16> {
        self._pow(exponent).or_else(|_| {
            if self.kind == NumberKind::NaN || exponent.kind == NumberKind::NaN {
                Err(ERR_INVALID_FORMAT)
            } else if self.kind == NumberKind::Infinity || self.kind == NumberKind::NegInfinity {
                Err(ERR_INFINITE_RESULT)
            } else {
                Err(ERR_UNIMPLEMENTED)
            }
        })
    }
    pub fn abs(&self) -> Self {
        Float::new(self.mantissa.clone(), self.exponent, false, self.kind)
    }
    
    pub fn from_int(int: &Int) -> Result<Self, i16> {
        if int.kind == NumberKind::NaN {
            return Err(ERR_INVALID_FORMAT);
        }
        if int.kind == NumberKind::Infinity {
            return Ok(Float::new("Infinity".to_string(), 0, false, NumberKind::Infinity));
        }
        if int.kind == NumberKind::NegInfinity {
            return Ok(Float::new("Infinity".to_string(), 0, true, NumberKind::NegInfinity));
        }
        if int.digits.is_empty() || int.digits == "0" {
            return Ok(Float::new("0".to_string(), 0, false, NumberKind::Finite));
        }

        let mantissa = int.digits.clone();
        let exponent = 0;

        Ok(Float::new(mantissa, exponent, int.negative, NumberKind::Finite))
    }
    pub fn is_zero(&self) -> bool {
        self.mantissa.is_empty() || self.mantissa == "0" && self.exponent == 0
    }
    pub fn round(&self, precision: usize) -> Self {
        if self.kind == NumberKind::NaN || self.kind == NumberKind::Infinity || self.kind == NumberKind::NegInfinity {
            return self.clone();
        }
        if self.is_zero() {
            return Float::new("0".to_string(), 0, false, NumberKind::Finite);
        }
    
        let old_len = self.mantissa.len();
    
        let mut mantissa = self.mantissa.clone();
        let mut exponent = self.exponent;
    
        let mantissa_len = old_len as i32;
        let point_pos = mantissa_len + exponent;
    
        let digits_to_keep = if point_pos > 0 {
            (point_pos as usize) + precision
        } else {
            precision
        };
    
        if mantissa.len() > digits_to_keep {
            let round_digit = mantissa.chars().nth(digits_to_keep).unwrap_or('0').to_digit(10).unwrap_or(0);
            mantissa.truncate(digits_to_keep);
    
            if round_digit >= 5 {
                let mut digits: Vec<u8> = mantissa.bytes().map(|b| b - b'0').collect();
    
                let mut carry = 1;
                for d in digits.iter_mut().rev() {
                    let sum = *d + carry;
                    *d = sum % 10;
                    carry = sum / 10;
                    if carry == 0 {
                        break;
                    }
                }
                if carry > 0 {
                    digits.insert(0, carry);
                    exponent += 1;
                }
                mantissa = digits.into_iter().map(|d| (d + b'0') as char).collect();
            }
    
            exponent += old_len as i32 - mantissa.len() as i32;
        }
    
        while mantissa.len() > 1 && mantissa.starts_with('0') {
            mantissa.remove(0);
            exponent -= 1;
        }
        if mantissa.is_empty() {
            mantissa = "0".to_string();
            exponent = 0;
        }
    
        Float::new(mantissa, exponent, self.negative, NumberKind::Finite)
    }
    
    pub fn truncate(&self, decimal_places: usize) -> Self {
        if self.kind == NumberKind::NaN || self.kind == NumberKind::Infinity || self.kind == NumberKind::NegInfinity {
            return self.clone();
        }
        if self.is_zero() {
            return Float::new("0".to_string(), 0, false, NumberKind::Finite);
        }
    
        let mantissa_len = self.mantissa.len() as i32;
        let point_pos = mantissa_len + self.exponent;
    
        let digits_to_keep = if point_pos > 0 {
            (point_pos as usize) + decimal_places
        } else {
            decimal_places
        };
    
        let mut mantissa = self.mantissa.clone();
        if mantissa.len() > digits_to_keep {
            mantissa.truncate(digits_to_keep);
        }
    
        let mut exponent = self.exponent + (mantissa_len - mantissa.len() as i32);
    
        while mantissa.len() > 1 && mantissa.starts_with('0') {
            mantissa.remove(0);
            exponent -= 1;
        }
        if mantissa.is_empty() {
            mantissa = "0".to_string();
            exponent = 0;
        }
    
        Float::new(mantissa, exponent, self.negative, NumberKind::Finite)
    } 
    pub fn from_f64(value: f64) -> Self {
        create_float(&value.to_string())
    }
    pub fn from_str(value: &str) -> Result<Self, i16> {
        if value.is_empty() {
            return Err(ERR_INVALID_FORMAT);
        }
        let float = create_float(value);
        if float.kind == NumberKind::NaN || float.kind == NumberKind::Infinity || float.kind == NumberKind::NegInfinity {
            return Err(ERR_INVALID_FORMAT);
        }
        Ok(float)
    }
    pub fn is_integer_like(&self) -> bool {
        if self.kind == NumberKind::NaN || self.kind == NumberKind::Infinity || self.kind == NumberKind::NegInfinity {
            return false;
        }
    
        if (self.mantissa.is_empty() && !(self.exponent >= 0)) || !self.mantissa.chars().all(|c| c.is_digit(10)) {
            return false;
        }
    
        if self.exponent >= 0 {
            true
        } else {
            let frac_len = (-self.exponent) as usize;
            if frac_len > self.mantissa.len() {
                return self.mantissa.chars().all(|c| c == '0');
            }
    
            let int_part_len = self.mantissa.len() - frac_len;
            let frac_part = &self.mantissa[int_part_len..];
    
            frac_part.chars().all(|c| c == '0')
        }
    }
    
    pub fn to_int(&self) -> Result<Int, i16> {
        if self.kind == NumberKind::NaN {
            return Err(ERR_INVALID_FORMAT);
        }
        if self.kind == NumberKind::Infinity || self.kind == NumberKind::NegInfinity {
            return Err(ERR_INFINITE_RESULT);
        }
        if self.is_zero() {
            return Ok(Int::new("0".to_string(), false, NumberKind::Finite));
        }
        if !self.is_integer_like() {
            return Err(ERR_INVALID_FORMAT);
        }
    
        let mut digits = self.mantissa.clone();
    
        if self.exponent < 0 {
            let exp = (-self.exponent) as usize;
            if exp >= digits.len() {
                digits = "0".to_string();
            } else {
                digits.truncate(digits.len() - exp);
            }
        } else if self.exponent > 0 {
            digits.push_str(&"0".repeat(self.exponent as usize));
        }
    
        let digits = normalize_int_digits(&digits);
    
        Ok(Int::new(digits, self.negative, NumberKind::Finite))
    }
    
    pub fn is_nan(&self) -> bool {
        self.kind == NumberKind::NaN
    }
    pub fn is_infinity(&self) -> bool {
        self.kind == NumberKind::Infinity
    }
    pub fn make_irrational(&mut self) -> Self {
        if self.kind == NumberKind::NaN || self.kind == NumberKind::Infinity || self.kind == NumberKind::NegInfinity {
            return self.clone()
        }
        self.kind = NumberKind::Irrational;
        self.clone()
    }

    pub fn normalize(&mut self) -> &mut Self{
        let trimmed = self.mantissa.trim_start_matches('0');
        let trimmed_len = trimmed.len();

        if trimmed_len == 0 {
            self.mantissa = "0".to_string();
            self.exponent = 0;
            self.negative = false;
        } else {
            let zeros_removed = self.mantissa.len() - trimmed_len;
            self.mantissa = trimmed.to_string();
            self.exponent += zeros_removed as i32;
        }
        if self.mantissa.is_empty() {
            self.mantissa = "0".to_string();
            self.exponent = 0;
            self.negative = false;
        }
        if self.mantissa == "0" {
            self.exponent = 0;
            self.negative = false;
            self.kind = NumberKind::Finite;
        }
        self
    }
}

fn normalize_int_digits(digits: &str) -> String {
    if digits.is_empty() || digits == "0" {
        return "0".to_string();
    }
    let normalized = digits.trim_start_matches('0');
    if normalized.is_empty() {
        "0".to_string()
    } else {
        normalized.to_string()
    }
}

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        create_float(&value.to_string())
    }
}

impl From<i64> for Int {
    fn from(value: i64) -> Self {
        create_int(&value.to_string())
    }
}

impl Binary for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = if self.negative { "-" } else { "" };
        match self.kind {
            NumberKind::Finite => {
                if let Ok(num) = self.digits.parse::<i128>() {
                    write!(f, "{}{:b}", prefix, num)
                } else {
                    Err(std::fmt::Error)
                }
            }
            _ => Err(std::fmt::Error),
        }
    }
}

impl Octal for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = if self.negative { "-" } else { "" };
        match self.kind {
            NumberKind::Finite => {
                if let Ok(num) = self.digits.parse::<i128>() {
                    write!(f, "{}{:o}", prefix, num)
                } else {
                    Err(std::fmt::Error)
                }
            }
            _ => Err(std::fmt::Error),
        }
    }
}

impl LowerHex for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = if self.negative { "-" } else { "" };
        match self.kind {
            NumberKind::Finite => {
                if let Ok(num) = self.digits.parse::<i128>() {
                    write!(f, "{}{:x}", prefix, num)
                } else {
                    Err(std::fmt::Error)
                }
            }
            _ => Err(std::fmt::Error),
        }
    }
}

// Formatting traits for Float: Not very meaningful to implement binary/oct/lowerhex on floats,
// so let's just fallback to error for now
impl Binary for Float {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Err(std::fmt::Error)
    }
}
impl Octal for Float {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Err(std::fmt::Error)
    }
}
impl LowerHex for Float {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Err(std::fmt::Error)
    }
}

impl Hash for Int {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.digits.hash(state);
        self.negative.hash(state);
        self.kind.hash(state);
    }
}

impl Hash for Float {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.mantissa.hash(state);
        self.exponent.hash(state);
        self.negative.hash(state);
        self.kind.hash(state);
    }
}


impl From<usize> for Int {
    fn from(value: usize) -> Self {
        create_int(&value.to_string())
    }
}

impl From<isize> for Int {
    fn from(value: isize) -> Self {
        create_int(&value.to_string())
    }
}

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        create_float(&value.to_string())
    }
}

impl From<i32> for Int {
    fn from(value: i32) -> Self {
        create_int(&value.to_string())
    }
}