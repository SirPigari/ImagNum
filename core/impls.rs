use crate::compat::{
    float_is_negative, float_is_zero, float_kind, float_to_parts,
    int_is_infinite, int_is_nan, int_to_parts, int_to_string, make_float_from_parts,
    make_int_from_parts,
};
use crate::foundation::{Float, FloatKind, Int, SmallFloat, SmallInt};
use crate::functions::{create_float, create_int};
use crate::math::{
    ERR_DIV_BY_ZERO, ERR_INFINITE_RESULT, ERR_INVALID_FORMAT, ERR_NEGATIVE_RESULT,
    ERR_NEGATIVE_SQRT, ERR_UNIMPLEMENTED, add_float, ceil_float, ceil_int, cos_float,
    cos_int, div_float, exp_float, exp_int, floor_float, floor_int, is_string_odd,
    ln_float, ln_int, log10_float, mod_float, mul_float, pow_strings,
    bigdecimal_pow_integer,
    sin_float, sin_int, sqrt_float, sqrt_int, sub_float, tan_float, tan_int,
    LN_10,
};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::fmt::{Binary, LowerHex, Octal};
use std::str::FromStr;
use std::hash::{Hash, Hasher};

fn normalize_recurring_decimal(float: Float) -> Float {
    if let Float::Recurring(ref bd) = float {
        let n = bd.normalized();
        
        let int_candidate = n.with_scale(0);
        if n == int_candidate {
            return Float::Big(int_candidate);
        }
        
        let s_norm = n.normalized().to_string();
        if s_norm.contains('.') {
            let parts: Vec<&str> = s_norm.split('.').collect();
            if parts.len() == 2 {
                let frac = parts[1];
                if frac.chars().all(|c| c == '9') && !frac.is_empty() {
                    let int_part: i64 = parts[0].parse().unwrap_or(0);
                    return Float::Big(BigDecimal::from(int_part + 1));
                }
                if frac.len() > 1 && frac.chars().skip(1).all(|c| c == '9') {
                    let first_digit = frac.chars().next().unwrap_or('0');
                    if let Some(digit_val) = first_digit.to_digit(10) {
                        let new_digit = digit_val + 1;
                        if new_digit < 10 {
                            let new_frac = format!("{}", new_digit);
                            let new_val = format!("{}.{}", parts[0], new_frac);
                            if let Ok(new_bd) = BigDecimal::from_str(&new_val) {
                                return Float::Big(new_bd);
                            }
                        }
                    }
                }
            }
        }
    }
    float
}

impl Int {
    fn smallint_to_bigint(si: &SmallInt) -> BigInt {
        match si {
            SmallInt::I8(v) => BigInt::from(*v),
            SmallInt::U8(v) => BigInt::from(*v),
            SmallInt::I16(v) => BigInt::from(*v),
            SmallInt::U16(v) => BigInt::from(*v),
            SmallInt::I32(v) => BigInt::from(*v),
            SmallInt::U32(v) => BigInt::from(*v),
            SmallInt::I64(v) => BigInt::from(*v),
            SmallInt::U64(v) => BigInt::from(*v),
            SmallInt::I128(v) => BigInt::from(*v),
            SmallInt::U128(v) => BigInt::from(*v),
            SmallInt::USize(v) => BigInt::from(*v),
            SmallInt::ISize(v) => BigInt::from(*v),
        }
    }

    pub fn is_negative(&self) -> bool {
        let (_d, neg, _k) = int_to_parts(self);
        neg
    }

    pub fn to_float(&self) -> Result<Float, i16> {
        match self {
            Int::Big(bi) => {
                let bd = BigDecimal::from(bi.clone());
                Ok(Float::Big(bd))
            }
            Int::Small(_) => {
                let s = int_to_string(self);
                match BigDecimal::from_str(&s) {
                    Ok(bd) => Ok(Float::Big(bd)),
                    Err(_) => Err(ERR_INVALID_FORMAT),
                }
            }
        }
    }
    pub fn _add(&self, other: &Self) -> Result<Self, i16> {
        let a = match self {
            Int::Big(bi) => bi.clone(),
            Int::Small(si) => Int::smallint_to_bigint(si),
        };
        let b = match other {
            Int::Big(bi) => bi.clone(),
            Int::Small(si) => Int::smallint_to_bigint(si),
        };
        Ok(Int::Big(a + b))
    }
    pub fn _sub(&self, other: &Self) -> Result<Self, i16> {
        let a = match self {
            Int::Big(bi) => bi.clone(),
            Int::Small(si) => Int::smallint_to_bigint(si),
        };
        let b = match other {
            Int::Big(bi) => bi.clone(),
            Int::Small(si) => Int::smallint_to_bigint(si),
        };
        Ok(Int::Big(a - b))
    }
    pub fn _mul(&self, other: &Self) -> Result<Self, i16> {
        let a = match self {
            Int::Big(bi) => bi.clone(),
            Int::Small(si) => Int::smallint_to_bigint(si),
        };
        let b = match other {
            Int::Big(bi) => bi.clone(),
            Int::Small(si) => Int::smallint_to_bigint(si),
        };
        Ok(Int::Big(a * b))
    }
    pub fn _div(&self, other: &Self) -> Result<Self, i16> {
        let a = match self {
            Int::Big(bi) => bi.clone(),
            Int::Small(si) => Int::smallint_to_bigint(si),
        };
        let b = match other {
            Int::Big(bi) => bi.clone(),
            Int::Small(si) => Int::smallint_to_bigint(si),
        };
        if b.is_zero() { return Err(ERR_DIV_BY_ZERO); }
        let (quot, rem) = (a.clone() / b.clone(), a.clone() % b.clone());
        if rem.is_zero() { return Ok(Int::Big(quot)); }
        let two = BigInt::from(2);
        let abs_rem_times_two = rem.abs() * &two;
        let abs_b = b.abs();
        let same_sign = a.is_negative() == b.is_negative();
        let rounded = if abs_rem_times_two >= abs_b {
            if same_sign { quot + BigInt::from(1) } else { quot - BigInt::from(1) }
        } else { quot };
        Ok(Int::Big(rounded))
    }
    pub fn _modulo(&self, other: &Self) -> Result<Self, i16> {
        let a = match self {
            Int::Big(bi) => bi.clone(),
            Int::Small(si) => Int::smallint_to_bigint(si),
        };
        let b = match other {
            Int::Big(bi) => bi.clone(),
            Int::Small(si) => Int::smallint_to_bigint(si),
        };
        if b.is_zero() { return Err(ERR_DIV_BY_ZERO); }
        Ok(Int::Big(a % b))
    }
    pub fn pow(&self, exponent: &Self) -> Result<Self, i16> {
        let (ed, eneg, _ek) = int_to_parts(exponent);
        if eneg {
            return Err(ERR_INVALID_FORMAT);
        }
        let (sd, sneg, _sk) = int_to_parts(self);
        let (digits, sign_flipped) = pow_strings(&sd, &ed)?;
        let digits = normalize_int_digits(&digits);
        let negative = if sneg && is_string_odd(&ed) {
            true ^ sign_flipped
        } else {
            sign_flipped
        };
        Ok(make_int_from_parts(digits, negative, FloatKind::Finite))
    }
    pub fn sqrt(&self) -> Result<Float, i16> {
        let (mant, neg, _k) = int_to_parts(self);
        let (m2, e2, neg2, is_irr) = sqrt_int(mant, neg)?;
        if is_irr {
            Ok(make_float_from_parts(m2, e2, neg2, FloatKind::Irrational))
        } else {
            Ok(make_float_from_parts(m2, e2, neg2, FloatKind::Finite))
        }
    }
    pub fn abs(&self) -> Self {
        let (digits, _neg, _k) = int_to_parts(self);
        make_int_from_parts(digits, false, FloatKind::Finite)
    }

    pub fn sin(&self) -> Result<Float, i16> {
        let (digits, neg, _k) = int_to_parts(self);
        let (m, e, neg2, is_irr) = sin_int(digits, neg)?;
        if is_irr {
            Ok(make_float_from_parts(m, e, neg2, FloatKind::Irrational))
        } else {
            Ok(make_float_from_parts(m, e, neg2, FloatKind::Finite))
        }
    }
    pub fn cos(&self) -> Result<Float, i16> {
        let (digits, neg, _k) = int_to_parts(self);
        let (m, e, neg2, is_irr) = cos_int(digits, neg)?;
        if is_irr {
            Ok(make_float_from_parts(m, e, neg2, FloatKind::Irrational))
        } else {
            Ok(make_float_from_parts(m, e, neg2, FloatKind::Finite))
        }
    }
    pub fn tan(&self) -> Result<Float, i16> {
        let (digits, neg, _k) = int_to_parts(self);
        let (m, e, neg2, is_irr) = tan_int(digits, neg)?;
        if is_irr {
            Ok(make_float_from_parts(m, e, neg2, FloatKind::Irrational))
        } else {
            Ok(make_float_from_parts(m, e, neg2, FloatKind::Finite))
        }
    }
    pub fn ln(&self) -> Result<Float, i16> {
        let (digits, neg, _k) = int_to_parts(self);
        let (m, e, neg2, is_irr) = ln_int(digits, neg)?;
        if is_irr {
            Ok(make_float_from_parts(m, e, neg2, FloatKind::Irrational))
        } else {
            Ok(make_float_from_parts(m, e, neg2, FloatKind::Finite))
        }
    }
    pub fn exp(&self) -> Result<Float, i16> {
        let (digits, neg, _k) = int_to_parts(self);
        let (m, e, neg2, is_irr) = exp_int(digits, neg)?;
        if is_irr {
            Ok(make_float_from_parts(m, e, neg2, FloatKind::Irrational))
        } else {
            Ok(make_float_from_parts(m, e, neg2, FloatKind::Finite))
        }
    }
    pub fn floor(&self) -> Result<Self, i16> {
        let (digits, neg, _k) = int_to_parts(self);
        let (d, n) = floor_int(digits, neg)?;
        Ok(make_int_from_parts(d, n, FloatKind::Finite))
    }
    pub fn ceil(&self) -> Result<Self, i16> {
        let (digits, neg, _k) = int_to_parts(self);
        let (d, n) = ceil_int(digits, neg)?;
        Ok(make_int_from_parts(d, n, FloatKind::Finite))
    }

    pub fn is_zero(&self) -> bool {
        let (digits, _neg, _k) = int_to_parts(self);
        digits.is_empty() || digits == "0"
    }
    pub fn to_usize(&self) -> Result<usize, i16> {
        if int_is_nan(self) {
            return Err(ERR_INVALID_FORMAT);
        }
        if int_is_infinite(self) {
            return Err(ERR_INFINITE_RESULT);
        }
        match self {
            Int::Big(bi) => {
                if bi.is_negative() {
                    return Err(ERR_NEGATIVE_RESULT);
                }
                bi.to_usize().ok_or(ERR_INVALID_FORMAT)
            }
            Int::Small(si) => {
                match si {
                    SmallInt::USize(v) => Ok(*v),
                    SmallInt::U64(v) => Ok(*v as usize),
                    SmallInt::U32(v) => Ok(*v as usize),
                    SmallInt::U16(v) => Ok(*v as usize),
                    SmallInt::U8(v) => Ok(*v as usize),
                    SmallInt::ISize(v) if *v >= 0 => Ok(*v as usize),
                    SmallInt::I64(v) if *v >= 0 => Ok(*v as usize),
                    SmallInt::I32(v) if *v >= 0 => Ok(*v as usize),
                    SmallInt::I16(v) if *v >= 0 => Ok(*v as usize),
                    SmallInt::I8(v) if *v >= 0 => Ok(*v as usize),
                    _ => Err(ERR_NEGATIVE_RESULT),
                }
            }
        }
    }
    pub fn to_i64(&self) -> Result<i64, i16> {
        if int_is_nan(self) {
            return Err(ERR_INVALID_FORMAT);
        }
        if int_is_infinite(self) {
            return Err(ERR_INFINITE_RESULT);
        }
        match self {
            Int::Big(bi) => bi.to_i64().ok_or(ERR_INVALID_FORMAT),
            Int::Small(si) => match si {
                SmallInt::I64(v) => Ok(*v),
                SmallInt::U64(v) => Ok(*v as i64),
                SmallInt::I32(v) => Ok(*v as i64),
                SmallInt::U32(v) => Ok(*v as i64),
                SmallInt::I16(v) => Ok(*v as i64),
                SmallInt::U16(v) => Ok(*v as i64),
                SmallInt::I8(v) => Ok(*v as i64),
                SmallInt::U8(v) => Ok(*v as i64),
                SmallInt::I128(v) => (*v).try_into().map_err(|_| ERR_INVALID_FORMAT),
                SmallInt::U128(v) => (*v).try_into().map_err(|_| ERR_INVALID_FORMAT),
                SmallInt::ISize(v) => Ok(*v as i64),
                SmallInt::USize(v) => Ok(*v as i64),
            },
        }
    }
    pub fn to_i128(&self) -> Result<i128, i16> {
        if int_is_nan(self) {
            return Err(ERR_INVALID_FORMAT);
        }
        if int_is_infinite(self) {
            return Err(ERR_INFINITE_RESULT);
        }
        match self {
            Int::Big(bi) => bi.to_i128().ok_or(ERR_INVALID_FORMAT),
            Int::Small(si) => match si {
                SmallInt::I128(v) => Ok(*v),
                SmallInt::U128(v) => Ok(*v as i128),
                SmallInt::I64(v) => Ok(*v as i128),
                SmallInt::U64(v) => Ok(*v as i128),
                SmallInt::I32(v) => Ok(*v as i128),
                SmallInt::U32(v) => Ok(*v as i128),
                SmallInt::I16(v) => Ok(*v as i128),
                SmallInt::U16(v) => Ok(*v as i128),
                SmallInt::I8(v) => Ok(*v as i128),
                SmallInt::U8(v) => Ok(*v as i128),
                SmallInt::ISize(v) => Ok(*v as i128),
                SmallInt::USize(v) => Ok(*v as i128),
            },
        }
    }
    pub fn from_i64(value: i64) -> Self {
        if value < 0 {
            make_int_from_parts(value.abs().to_string(), true, FloatKind::Finite)
        } else {
            make_int_from_parts(value.to_string(), false, FloatKind::Finite)
        }
    }
    pub fn from_i128(value: i128) -> Self {
        if value < 0 {
            make_int_from_parts(value.abs().to_string(), true, FloatKind::Finite)
        } else {
            make_int_from_parts(value.to_string(), false, FloatKind::Finite)
        }
    }
    pub fn from_hex(value: &str) -> Result<Self, i16> {
        if value.is_empty() {
            return Err(ERR_INVALID_FORMAT);
        }
        let mut s = value.trim();
        let mut negative = false;
        if s.starts_with('-') {
            negative = true;
            s = &s[1..];
        } else if s.starts_with('+') {
            s = &s[1..];
        }
        if s.starts_with("0x") || s.starts_with("0X") {
            s = &s[2..];
        }
        if s.is_empty() {
            return Err(ERR_INVALID_FORMAT);
        }

        let mut acc = BigInt::from(0u32);
        let sixteen = BigInt::from(16u32);
        for c in s.chars() {
            let digit = match c {
                '0'..='9' => (c as u8 - b'0') as u32,
                'a'..='f' => (10 + (c as u8 - b'a')) as u32,
                'A'..='F' => (10 + (c as u8 - b'A')) as u32,
                '_' => continue,
                _ => return Err(ERR_INVALID_FORMAT),
            };
            acc = &acc * &sixteen + BigInt::from(digit);
        }
        if negative {
            acc = -acc;
        }
        Ok(Int::Big(acc))
    }

    pub fn from_str_radix(value: &str, radix: u32) -> Result<Self, i16> {
        if radix < 2 || radix > 36 {
            return Err(ERR_INVALID_FORMAT);
        }
        if value.is_empty() {
            return Err(ERR_INVALID_FORMAT);
        }
        let mut s = value.trim();
        let mut negative = false;
        if s.starts_with('-') {
            negative = true;
            s = &s[1..];
        } else if s.starts_with('+') {
            s = &s[1..];
        }
        if s.is_empty() {
            return Err(ERR_INVALID_FORMAT);
        }
        let mut acc = BigInt::from(0u32);
        let base = BigInt::from(radix);
        for c in s.chars() {
            if c == '_' { continue; }
            let digit = c.to_digit(radix);
            if digit.is_none() {
                return Err(ERR_INVALID_FORMAT);
            }
            acc = &acc * &base + BigInt::from(digit.unwrap());
        }
        if negative { acc = -acc; }
        Ok(Int::Big(acc))
    }

    pub fn to_str_radix(&self, radix: u32) -> Result<String, i16> {
        if radix < 2 || radix > 36 {
            return Err(ERR_INVALID_FORMAT);
        }
        match self {
            Int::Big(bi) => Ok(bi.to_str_radix(radix)),
            Int::Small(si) => {
                let bi = Int::smallint_to_bigint(si);
                Ok(bi.to_str_radix(radix))
            }
        }
    }
    pub fn from_str(value: &str) -> Result<Self, i16> {
        if value.is_empty() {
            return Err(ERR_INVALID_FORMAT);
        }
        let int = create_int(value);
        if int_is_nan(&int) || int_is_infinite(&int) {
            return Err(ERR_INVALID_FORMAT);
        }
        Ok(int)
    }
    pub fn is_nan(&self) -> bool {
        int_is_nan(self)
    }
    pub fn is_infinity(&self) -> bool {
        int_is_infinite(self)
    }
    pub fn to_str(&self) -> String {
        format!("{}", self)
    }
}

impl Float {
    pub fn is_negative(&self) -> bool {
        let (_m, _e, neg, _k) = float_to_parts(self);
        neg
    }

    pub fn is_recurring(&self) -> bool {
        float_kind(self) == FloatKind::Recurring
    }

    pub fn is_irrational(&self) -> bool {
        float_kind(self) == FloatKind::Irrational
    }

    pub fn is_complex(&self) -> bool {
        matches!(self, Float::Complex(_, _))
    }

    pub fn conj(&self) -> Self {
        if let Float::Complex(real, imag) = self {
            let neg_imag = Float::Big(BigDecimal::from(0))._sub(imag).unwrap_or_else(|_| Float::NaN);
            Float::Complex(real.clone(), Box::new(neg_imag))
        } else {
            self.clone()
        }
    }

    pub fn to_f64(&self) -> Result<f64, i16> {
        if let Some(bd) = crate::compat::float_to_bigdecimal(self) {
            return bd.to_f64().ok_or(ERR_INVALID_FORMAT);
        }
        let k = float_kind(self);
        if k == FloatKind::NaN {
            return Err(ERR_INVALID_FORMAT);
        }
        if k == FloatKind::Infinity {
            return Ok(f64::INFINITY);
        }
        if k == FloatKind::NegInfinity {
            return Ok(f64::NEG_INFINITY);
        }
        Err(ERR_INVALID_FORMAT)
    }
    pub fn sqrt(&self) -> Result<Self, i16> {
        // Complex sqrt: sqrt(a + bi) = sqrt(r) * (cos(θ/2) + i*sin(θ/2))
        // where r = |a + bi| and θ = atan2(b, a)
        if let Float::Complex(real, imag) = self {
            // sqrt(a+bi) = ±(sqrt((r+a)/2) + i*sign(b)*sqrt((r-a)/2))
            // where r = sqrt(a² + b²)
            let a_sq = real._mul(real)?;
            let b_sq = imag._mul(imag)?;
            let r_sq = a_sq._add(&b_sq)?;
            let r = r_sq.sqrt()?;
            
            let r_plus_a = r._add(real)?;
            let r_minus_a = r._sub(real)?;
            
            let two = Float::Big(BigDecimal::from(2));
            let half_r_plus_a = r_plus_a._div(&two)?;
            let half_r_minus_a = r_minus_a._div(&two)?;
            
            let new_real = half_r_plus_a.sqrt()?;
            let new_imag_abs = half_r_minus_a.sqrt()?;
            
            // Sign of new imaginary part matches sign of old imaginary part
            let new_imag = if float_is_negative(imag) {
                Float::Big(BigDecimal::from(0))._sub(&new_imag_abs)?
            } else {
                new_imag_abs
            };
            
            return Ok(Float::Complex(Box::new(new_real), Box::new(new_imag)));
        }
        
        let kind = float_kind(self);
        if kind == FloatKind::NaN {
            return Err(ERR_INVALID_FORMAT);
        }
        if kind == FloatKind::Infinity {
            return Ok(Float::Infinity);
        }
        if kind == FloatKind::NegInfinity || self.is_negative() {
            return Err(ERR_NEGATIVE_SQRT);
        }
        let (m, e, neg, _k) = float_to_parts(self);
        let (m, e, neg, is_irr) = sqrt_float(m, e, neg)?;
        if is_irr {
            return Ok(make_float_from_parts(m, e, neg, FloatKind::Irrational));
        }
        Ok(make_float_from_parts(m, e, neg, FloatKind::Finite))
    }
    pub fn _add(&self, other: &Self) -> Result<Self, i16> {
        if float_kind(self) == FloatKind::NaN || float_kind(other) == FloatKind::NaN {
            return Err(ERR_INVALID_FORMAT);
        }

        // (a + bi) + (c + di) = (a+c) + (b+d)i
        match (self, other) {
            (Float::Complex(r1, i1), Float::Complex(r2, i2)) => {
                let real = r1._add(r2)?;
                let imag = i1._add(i2)?;
                return Ok(Float::Complex(Box::new(real), Box::new(imag)));
            }
            (Float::Complex(r, i), other_val) | (other_val, Float::Complex(r, i)) => {
                let real = r._add(other_val)?;
                return Ok(Float::Complex(Box::new(real), Box::new(*i.clone())));
            }
            _ => {}
        }

        let k1 = float_kind(self);
        let k2 = float_kind(other);
        if k1 == FloatKind::Finite && k2 == FloatKind::Finite {
            if let (Some(a_bd), Some(b_bd)) = (
                crate::compat::float_to_bigdecimal(self),
                crate::compat::float_to_bigdecimal(other),
            ) {
                let res = a_bd + b_bd;
                return Ok(Float::Big(res));
            }
        }

        if float_kind(self) == FloatKind::Infinity && float_kind(other) == FloatKind::Infinity {
            return Ok(Float::Infinity);
        }
        if float_kind(self) == FloatKind::NegInfinity && float_kind(other) == FloatKind::NegInfinity
        {
            return Ok(Float::NegInfinity);
        }
        if (float_kind(self) == FloatKind::Infinity && float_kind(other) == FloatKind::NegInfinity)
            || (float_kind(self) == FloatKind::NegInfinity
                && float_kind(other) == FloatKind::Infinity)
        {
            return Err(ERR_INFINITE_RESULT);
        }

        let (m1, e1, n1, _k1) = float_to_parts(self);
        let (m2, e2, n2, _k2) = float_to_parts(other);
        let (mantissa, exponent, negative) = add_float(m1.clone(), e1, n1, m2.clone(), e2, n2)?;
        let result_kind = if float_kind(self) == FloatKind::Recurring
            || float_kind(other) == FloatKind::Recurring
        {
            FloatKind::Recurring
        } else {
            FloatKind::Finite
        };
        Ok(make_float_from_parts(
            mantissa,
            exponent,
            negative,
            result_kind,
        ))
    }
    pub fn _sub(&self, other: &Self) -> Result<Self, i16> {
        if float_kind(self) == FloatKind::NaN || float_kind(other) == FloatKind::NaN {
            return Err(ERR_INVALID_FORMAT);
        }

        // (a + bi) - (c + di) = (a-c) + (b-d)i
        match (self, other) {
            (Float::Complex(r1, i1), Float::Complex(r2, i2)) => {
                let real = r1._sub(r2)?;
                let imag = i1._sub(i2)?;
                return Ok(Float::Complex(Box::new(real), Box::new(imag)));
            }
            (Float::Complex(r, i), other_val) => {
                let real = r._sub(other_val)?;
                return Ok(Float::Complex(Box::new(real), Box::new(*i.clone())));
            }
            (other_val, Float::Complex(r, i)) => {
                let real = other_val._sub(r)?;
                let neg_imag = Float::Big(BigDecimal::from(0))._sub(i)?;
                return Ok(Float::Complex(Box::new(real), Box::new(neg_imag)));
            }
            _ => {}
        }

        let k1 = float_kind(self);
        let k2 = float_kind(other);
        if k1 == FloatKind::Finite && k2 == FloatKind::Finite {
            if let (Some(a_bd), Some(b_bd)) = (
                crate::compat::float_to_bigdecimal(self),
                crate::compat::float_to_bigdecimal(other),
            ) {
                let res = a_bd - b_bd;
                return Ok(Float::Big(res));
            }
        }

        if float_kind(self) == FloatKind::Infinity && float_kind(other) == FloatKind::Infinity {
            return Ok(make_float_from_parts(
                "0".to_string(),
                0,
                false,
                FloatKind::Finite,
            ));
        }
        if float_kind(self) == FloatKind::NegInfinity && float_kind(other) == FloatKind::NegInfinity
        {
            return Ok(make_float_from_parts(
                "0".to_string(),
                0,
                true,
                FloatKind::Finite,
            ));
        }
        if (float_kind(self) == FloatKind::Infinity && float_kind(other) == FloatKind::NegInfinity)
            || (float_kind(self) == FloatKind::NegInfinity
                && float_kind(other) == FloatKind::Infinity)
        {
            return Err(ERR_INFINITE_RESULT);
        }

        let (m1, e1, n1, _k1) = float_to_parts(self);
        let (m2, e2, n2, _k2) = float_to_parts(other);
        let (mantissa, exponent, negative) = sub_float(m1.clone(), e1, n1, m2.clone(), e2, n2)?;
        let result_kind = if float_kind(self) == FloatKind::Recurring
            || float_kind(other) == FloatKind::Recurring
        {
            FloatKind::Recurring
        } else {
            FloatKind::Finite
        };
        Ok(make_float_from_parts(
            mantissa,
            exponent,
            negative,
            result_kind,
        ))
    }
    pub fn _mul(&self, other: &Self) -> Result<Self, i16> {
        if float_kind(self) == FloatKind::NaN || float_kind(other) == FloatKind::NaN {
            return Err(ERR_INVALID_FORMAT);
        }

        // (a + bi)(c + di) = (ac - bd) + (ad + bc)i
        match (self, other) {
            (Float::Complex(a, b), Float::Complex(c, d)) => {
                let ac = a._mul(c)?;
                let bd = b._mul(d)?;
                let ad = a._mul(d)?;
                let bc = b._mul(c)?;
                let real = ac._sub(&bd)?;
                let imag = ad._add(&bc)?;
                return Ok(Float::Complex(Box::new(real), Box::new(imag)));
            }
            (Float::Complex(r, i), other_val) | (other_val, Float::Complex(r, i)) => {
                let real = r._mul(other_val)?;
                let imag = i._mul(other_val)?;
                return Ok(Float::Complex(Box::new(real), Box::new(imag)));
            }
            _ => {}
        }

        let k1 = float_kind(self);
        let k2 = float_kind(other);
        if k1 == FloatKind::Finite && k2 == FloatKind::Finite {
            if let (Some(a_bd), Some(b_bd)) = (
                crate::compat::float_to_bigdecimal(self),
                crate::compat::float_to_bigdecimal(other),
            ) {
                let res = a_bd * b_bd;
                return Ok(Float::Big(res));
            }
        }

        if float_kind(self) == FloatKind::Infinity || float_kind(other) == FloatKind::Infinity {
            let neg = self.is_negative() ^ other.is_negative();
            return Ok(if neg {
                Float::NegInfinity
            } else {
                Float::Infinity
            });
        }
        if float_kind(self) == FloatKind::NegInfinity || float_kind(other) == FloatKind::NegInfinity
        {
            let neg = self.is_negative() ^ other.is_negative();
            return Ok(if neg {
                Float::NegInfinity
            } else {
                Float::Infinity
            });
        }

        let (m1, e1, n1, _k1) = float_to_parts(self);
        let (m2, e2, n2, _k2) = float_to_parts(other);
        let (mantissa, exponent, negative) = mul_float(m1.clone(), e1, n1, m2.clone(), e2, n2)?;
        let result_kind = if float_kind(self) == FloatKind::Recurring
            || float_kind(other) == FloatKind::Recurring
        {
            FloatKind::Recurring
        } else {
            FloatKind::Finite
        };
        let mut result = make_float_from_parts(
            mantissa,
            exponent,
            negative,
            result_kind,
        );
        
        if result_kind == FloatKind::Recurring {
            result = normalize_recurring_decimal(result);
        }
        
        Ok(result)
    }
    pub fn _div(&self, other: &Self) -> Result<Self, i16> {
        if float_kind(self) == FloatKind::NaN || float_kind(other) == FloatKind::NaN {
            return Err(ERR_INVALID_FORMAT);
        }
        if float_is_zero(other) {
            return Err(ERR_DIV_BY_ZERO);
        }

        // (a + bi)/(c + di) = [(ac + bd) + (bc - ad)i] / (c² + d²)
        match (self, other) {
            (Float::Complex(a, b), Float::Complex(c, d)) => {
                let ac = a._mul(c)?;
                let bd = b._mul(d)?;
                let bc = b._mul(c)?;
                let ad = a._mul(d)?;
                let c_sq = c._mul(c)?;
                let d_sq = d._mul(d)?;
                let denom = c_sq._add(&d_sq)?;
                
                if float_is_zero(&denom) {
                    return Err(ERR_DIV_BY_ZERO);
                }
                
                let real_num = ac._add(&bd)?;
                let imag_num = bc._sub(&ad)?;
                let real = real_num._div(&denom)?;
                let imag = imag_num._div(&denom)?;
                return Ok(Float::Complex(Box::new(real), Box::new(imag)));
            }
            (Float::Complex(r, i), other_val) => {
                let real = r._div(other_val)?;
                let imag = i._div(other_val)?;
                return Ok(Float::Complex(Box::new(real), Box::new(imag)));
            }
            (other_val, Float::Complex(c, d)) => {
                // a / (c + di) = [ac - di] / (c² + d²) = [ac/(c²+d²)] + [-ad/(c²+d²)]i
                let c_sq = c._mul(c)?;
                let d_sq = d._mul(d)?;
                let denom = c_sq._add(&d_sq)?;
                
                if float_is_zero(&denom) {
                    return Err(ERR_DIV_BY_ZERO);
                }
                
                let ac = other_val._mul(c)?;
                let ad = other_val._mul(d)?;
                let real = ac._div(&denom)?;
                let neg_ad = Float::Big(BigDecimal::from(0))._sub(&ad)?;
                let imag = neg_ad._div(&denom)?;
                return Ok(Float::Complex(Box::new(real), Box::new(imag)));
            }
            _ => {}
        }

        if float_kind(self) == FloatKind::Infinity && float_kind(other) == FloatKind::Infinity {
            return Ok(Float::NaN);
        }
        if float_kind(self) == FloatKind::NegInfinity && float_kind(other) == FloatKind::NegInfinity
        {
            return Ok(Float::NaN);
        }
        if (float_kind(self) == FloatKind::Infinity && float_kind(other) == FloatKind::NegInfinity)
            || (float_kind(self) == FloatKind::NegInfinity
                && float_kind(other) == FloatKind::Infinity)
        {
            let neg = self.is_negative() ^ other.is_negative();
            return Ok(make_float_from_parts(
                "0".to_string(),
                0,
                neg,
                FloatKind::Finite,
            ));
        }

        let (m1, e1, n1, _) = float_to_parts(self);
        let (m2, e2, n2, _) = float_to_parts(other);

        let self_is_int_like =
            e1 >= 0 || (e1 < 0 && (-(e1) as usize) <= m1.len() && m1.chars().all(|c| c == '0'));
        let other_is_int_like =
            e2 >= 0 || (e2 < 0 && (-(e2) as usize) <= m2.len() && m2.chars().all(|c| c == '0'));

        if self_is_int_like && other_is_int_like {
            let mut num_str = m1.clone();
            if e1 < 0 {
                num_str.push_str(&"0".repeat((-e1) as usize));
            }
            let mut den_str = m2.clone();
            if e2 < 0 {
                den_str.push_str(&"0".repeat((-e2) as usize));
            }
            let mut num = BigInt::from_str(&num_str).unwrap_or_else(|_| BigInt::from(0));
            let mut den = BigInt::from_str(&den_str).unwrap_or_else(|_| BigInt::from(1));
            if den.is_zero() {
                return Err(ERR_DIV_BY_ZERO);
            }
            let g = num.clone().abs().gcd(&den.clone().abs());
            if !g.is_zero() {
                num = num / &g;
                den = den / &g;
            }
            let mut d = den.clone().abs();
            while (&d % BigInt::from(2u32)).is_zero() {
                d = d / BigInt::from(2u32);
            }
            while (&d % BigInt::from(5u32)).is_zero() {
                d = d / BigInt::from(5u32);
            }

            let den_abs = den.clone().abs();
            let num_abs = num.clone().abs();
            let neg = n1 ^ n2;
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
                return Ok(Float::Big(bd));
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
                    return Ok(Float::Recurring(bd));
                }
                Err(_) => {
                    let s_out = if neg { format!("-{}.{}", int_part, frac_str) } else { format!("{}.{}", int_part, frac_str) };
                    let bd = BigDecimal::from_str(&s_out).unwrap_or_else(|_| BigDecimal::from(0));
                    return Ok(Float::Recurring(bd));
                }
            }
        }

        let (mantissa, exponent, negative) = div_float(m1, e1, n1, m2, e2, n2)?;
        Ok(make_float_from_parts(
            mantissa,
            exponent,
            negative,
            FloatKind::Finite,
        ))
    }
    pub fn _modulo(&self, other: &Self) -> Result<Self, i16> {
        if self.is_complex() || other.is_complex() {
            return Err(ERR_INVALID_FORMAT);
        }
        
        if float_kind(self) == FloatKind::NaN || float_kind(other) == FloatKind::NaN {
            return Err(ERR_INVALID_FORMAT);
        }
        if float_is_zero(other) {
            return Err(ERR_DIV_BY_ZERO);
        }
        if float_kind(self) == FloatKind::Infinity || float_kind(self) == FloatKind::NegInfinity {
            return Ok(Float::NaN);
        }

        let (m1, e1, n1, _) = float_to_parts(self);
        let (m2, e2, n2, _) = float_to_parts(other);
        let (mantissa, exponent, negative) = mod_float(m1, e1, n1, m2, e2, n2)?;
        Ok(make_float_from_parts(
            mantissa,
            exponent,
            negative,
            FloatKind::Finite,
        ))
    }
    pub fn _pow(&self, exponent: &Self) -> Result<Self, i16> {
        // Complex power: z^w = exp(w * ln(z))
        if self.is_complex() || exponent.is_complex() {
            let ln_z = self.ln()?;
            let w_ln_z = exponent._mul(&ln_z)?;
            return w_ln_z.exp();
        }
        
        if float_kind(self) == FloatKind::NaN || float_kind(exponent) == FloatKind::NaN {
            return Err(ERR_INVALID_FORMAT);
        }
        if float_is_zero(exponent) {
            // x^0 == 1
            let (_m, _e, _, _) = float_to_parts(self);
            return Ok(make_float_from_parts(
                "1".to_string(),
                0,
                false,
                FloatKind::Finite,
            ));
        }
        if float_kind(self) == FloatKind::Infinity || float_kind(self) == FloatKind::NegInfinity {
            let neg = self.is_negative() ^ exponent.is_negative();
            return Ok(if neg {
                Float::NegInfinity
            } else {
                Float::Infinity
            });
        }

        if let Float::Recurring(exp_bd) = exponent {
            let (num, den) = crate::math::bigdecimal_to_fraction(&exp_bd);
            if den != num_bigint::BigInt::from(1u32) {
                if let Some(den_u64) = den.to_u64() {
                    if den_u64 > 0 && den_u64 <= 200 {
                        if let Some(base_bd) = crate::compat::float_to_bigdecimal(self) {
                            if let Ok((res_bd, _exact)) = crate::math::pow_bigdecimal_rational(&base_bd, &num, &den, 137) {
                                return Ok(Float::Big(res_bd));
                            }
                        }
                    }
                }
            }

            if let Some(exp_f64) = exp_bd.to_f64() {
                if let Some((p_u64, q_u64)) = approx_rational_from_f64(exp_f64, 200) {
                    if q_u64 > 0 && q_u64 <= 200 {
                        if p_u64 == 0 {
                            return Ok(make_float_from_parts("1".to_string(), 0, false, FloatKind::Finite));
                        }
                        if let Some(base_bd) = crate::compat::float_to_bigdecimal(self) {
                            let mut pow_bd = BigDecimal::from(1u32);
                            for _ in 0..p_u64 { pow_bd = pow_bd * base_bd.clone(); }
                            if let Some(root_bd) = bigdecimal_nth_root(pow_bd, q_u64 as u32, 100) {
                                return Ok(Float::Big(root_bd));
                            }
                        }
                    }
                }
            }
        }

        if exponent.is_integer_like() {
            if let Some(exp_bd) = crate::compat::float_to_bigdecimal(exponent) {
                let (mant, exp_i32, neg) = crate::math::from_bigdecimal(&exp_bd);
                let mut digits = mant;
                if exp_i32 > 0 {
                    digits.push_str(&"0".repeat(exp_i32 as usize));
                }
                let digits = digits.trim_start_matches('0').to_string();
                if digits.is_empty() {
                    return Ok(make_float_from_parts("1".to_string(), 0, false, FloatKind::Finite));
                }
                match BigInt::from_str(&digits) {
                    Ok(mut bi) => {
                        if neg { bi = -bi; }
                        if let Some(base_bd) = crate::compat::float_to_bigdecimal(self) {
                            let res_bd = bigdecimal_pow_integer(base_bd.clone(), bi);
                            return Ok(Float::Big(res_bd));
                        }
                    }
                    Err(_) => {}
                }
            }
        }

        if let Some(exp_bd) = crate::compat::float_to_bigdecimal(exponent) {
            let (num, den) = crate::math::bigdecimal_to_fraction(&exp_bd);
            if den != num_bigint::BigInt::from(1u32) {
                if let Some(den_u64) = den.to_u64() {
                    if den_u64 > 0 && den_u64 <= 200 {
                        if let Some(base_bd) = crate::compat::float_to_bigdecimal(self) {
                            if let Ok((res_bd, _exact)) = crate::math::pow_bigdecimal_rational(&base_bd, &num, &den, 137) {
                                return Ok(Float::Big(res_bd));
                            }
                        }
                    }
                }
            }
        }

        let base_f64 = match self.to_f64() {
            Ok(v) => v,
            Err(_) => return Err(ERR_INVALID_FORMAT),
        };
        let exponent_f64 = match exponent.to_f64() {
            Ok(v) => v,
            Err(_) => return Err(ERR_INVALID_FORMAT),
        };

        let pow_res = base_f64.powf(exponent_f64);

        if pow_res.is_nan() {
            return Err(ERR_INVALID_FORMAT);
        }
        if pow_res.is_infinite() {
            return Ok(if pow_res.is_sign_negative() {
                Float::NegInfinity
            } else {
                Float::Infinity
            });
        }

        let negative = pow_res.is_sign_negative();
        let abs_res = pow_res.abs();

        if abs_res == 0.0 {
            return Ok(make_float_from_parts(
                "0".to_string(),
                0,
                false,
                FloatKind::Finite,
            ));
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

        Ok(make_float_from_parts(
            mantissa_str,
            final_exp,
            negative,
            FloatKind::Finite,
        ))
    }

    pub fn pow(&self, exponent: &Self) -> Result<Self, i16> {
        self._pow(exponent).or_else(|_| {
            if float_kind(self) == FloatKind::NaN || float_kind(exponent) == FloatKind::NaN {
                Err(ERR_INVALID_FORMAT)
            } else if float_kind(self) == FloatKind::Infinity
                || float_kind(self) == FloatKind::NegInfinity
            {
                Err(ERR_INFINITE_RESULT)
            } else {
                Err(ERR_UNIMPLEMENTED)
            }
        })
    }
    pub fn abs(&self) -> Self {
        // Complex abs: |a + bi| = sqrt(a² + b²)
        if let Float::Complex(real, imag) = self {
            let a_sq = real._mul(real).unwrap_or_else(|_| Float::NaN);
            let b_sq = imag._mul(imag).unwrap_or_else(|_| Float::NaN);
            let sum = a_sq._add(&b_sq).unwrap_or_else(|_| Float::NaN);
            return sum.sqrt().unwrap_or(Float::NaN);
        }
        
        let (_m, _e, _, k) = float_to_parts(self);
        make_float_from_parts(_m, _e, false, k)
    }

    pub fn sin(&self) -> Result<Self, i16> {
        // Complex sin: sin(a + bi) = sin(a)cosh(b) + i*cos(a)sinh(b)
        if let Float::Complex(real, imag) = self {
            let sin_a = real.sin()?;
            let cos_a = real.cos()?;
            let exp_b = imag.exp()?;
            let neg_b = Float::Big(BigDecimal::from(0))._sub(imag)?;
            let exp_neg_b = neg_b.exp()?;
            
            // cosh(b) = (e^b + e^(-b))/2
            let cosh_b = exp_b._add(&exp_neg_b)?._div(&Float::Big(BigDecimal::from(2)))?;
            // sinh(b) = (e^b - e^(-b))/2
            let sinh_b = exp_b._sub(&exp_neg_b)?._div(&Float::Big(BigDecimal::from(2)))?;
            
            let new_real = sin_a._mul(&cosh_b)?;
            let new_imag = cos_a._mul(&sinh_b)?;
            
            return Ok(Float::Complex(Box::new(new_real), Box::new(new_imag)));
        }
        
        let (m, e, neg, _k) = float_to_parts(self);
        let (rm, re, rneg, is_irr) = sin_float(m, e, neg)?;
        if is_irr {
            Ok(make_float_from_parts(rm, re, rneg, FloatKind::Irrational))
        } else {
            Ok(make_float_from_parts(rm, re, rneg, FloatKind::Finite))
        }
    }
    pub fn cos(&self) -> Result<Self, i16> {
        // Complex cos: cos(a + bi) = cos(a)cosh(b) - i*sin(a)sinh(b)
        if let Float::Complex(real, imag) = self {
            let sin_a = real.sin()?;
            let cos_a = real.cos()?;
            let exp_b = imag.exp()?;
            let neg_b = Float::Big(BigDecimal::from(0))._sub(imag)?;
            let exp_neg_b = neg_b.exp()?;
            
            // cosh(b) = (e^b + e^(-b))/2
            let cosh_b = exp_b._add(&exp_neg_b)?._div(&Float::Big(BigDecimal::from(2)))?;
            // sinh(b) = (e^b - e^(-b))/2
            let sinh_b = exp_b._sub(&exp_neg_b)?._div(&Float::Big(BigDecimal::from(2)))?;
            
            let new_real = cos_a._mul(&cosh_b)?;
            let neg_sin_a = Float::Big(BigDecimal::from(0))._sub(&sin_a)?;
            let new_imag = neg_sin_a._mul(&sinh_b)?;
            
            return Ok(Float::Complex(Box::new(new_real), Box::new(new_imag)));
        }
        
        let (m, e, neg, _k) = float_to_parts(self);
        let (rm, re, rneg, is_irr) = cos_float(m, e, neg)?;
        if is_irr {
            Ok(make_float_from_parts(rm, re, rneg, FloatKind::Irrational))
        } else {
            Ok(make_float_from_parts(rm, re, rneg, FloatKind::Finite))
        }
    }
    pub fn tan(&self) -> Result<Self, i16> {
        // Complex tan: tan(z) = sin(z) / cos(z)
        if let Float::Complex(_, _) = self {
            let sin_z = self.sin()?;
            let cos_z = self.cos()?;
            return sin_z._div(&cos_z);
        }
        
        let (m, e, neg, _k) = float_to_parts(self);
        let (rm, re, rneg, is_irr) = tan_float(m, e, neg)?;
        if is_irr {
            Ok(make_float_from_parts(rm, re, rneg, FloatKind::Irrational))
        } else {
            Ok(make_float_from_parts(rm, re, rneg, FloatKind::Finite))
        }
    }
    pub fn ln(&self) -> Result<Self, i16> {
        // Complex ln: ln(a + bi) = ln(|a + bi|) + i*arg(a + bi)
        // where arg(a + bi) = atan2(b, a)
        if let Float::Complex(real, imag) = self {
            let abs_val = self.abs();
            let ln_abs = abs_val.ln()?;
            
            let arg = if float_is_zero(real) {
                // Pure imaginary
                if float_is_negative(imag) {
                    Float::Big(BigDecimal::from_str("-1.5707963267948966").unwrap()) // -π/2
                } else {
                    Float::Big(BigDecimal::from_str("1.5707963267948966").unwrap()) // π/2
                }
            } else if float_is_zero(imag) {
                if float_is_negative(real) {
                    Float::Big(BigDecimal::from_str("3.1415926535897932").unwrap()) // π
                } else {
                    Float::Big(BigDecimal::from(0))
                }
            } else {
                // General case: use atan(b/a) then adjust for quadrant
                let ratio = imag._div(real)?;
                let atan = if let Ok(f64_val) = ratio.to_f64() {
                    Float::Big(BigDecimal::from_f64(f64_val.atan()).unwrap())
                } else {
                    return Err(ERR_INVALID_FORMAT);
                };
                
                if float_is_negative(real) {
                    if float_is_negative(imag) {
                        atan._sub(&Float::Big(BigDecimal::from_str("3.1415926535897932").unwrap()))?
                    } else {
                        atan._add(&Float::Big(BigDecimal::from_str("3.1415926535897932").unwrap()))?
                    }
                } else {
                    atan
                }
            };
            
            return Ok(Float::Complex(Box::new(ln_abs), Box::new(arg)));
        }
        
        let (m, e, neg, _k) = float_to_parts(self);
        let (rm, re, rneg, is_irr) = ln_float(m, e, neg)?;
        if is_irr {
            Ok(make_float_from_parts(rm, re, rneg, FloatKind::Irrational))
        } else {
            Ok(make_float_from_parts(rm, re, rneg, FloatKind::Finite))
        }
    }
    pub fn exp(&self) -> Result<Self, i16> {
        // Complex exp: exp(a + bi) = e^a * (cos(b) + i*sin(b))
        if let Float::Complex(real, imag) = self {
            let exp_a = real.exp()?;
            let cos_b = imag.cos()?;
            let sin_b = imag.sin()?;
            
            let new_real = exp_a._mul(&cos_b)?;
            let new_imag = exp_a._mul(&sin_b)?;
            
            return Ok(Float::Complex(Box::new(new_real), Box::new(new_imag)));
        }
        
        let (m, e, neg, _k) = float_to_parts(self);
        let (rm, re, rneg, is_irr) = exp_float(m, e, neg)?;
        if is_irr {
            Ok(make_float_from_parts(rm, re, rneg, FloatKind::Irrational))
        } else {
            Ok(make_float_from_parts(rm, re, rneg, FloatKind::Finite))
        }
    }
    pub fn log(&self, base: &Float) -> Result<Self, i16> {
        // Complex log with base: log_base(z) = ln(z) / ln(base)
        if self.is_complex() || base.is_complex() {
            let ln_z = self.ln()?;
            let ln_base = base.ln()?;
            return ln_z._div(&ln_base);
        }
        
        // For real numbers: log_base(x) = ln(x) / ln(base)
        let ln_self = self.ln()?;
        let ln_base = base.ln()?;
        ln_self._div(&ln_base)
    }
    
    pub fn log10(&self) -> Result<Self, i16> {
        // Complex log base 10: log10(z) = ln(z) / ln(10)
        if let Float::Complex(_, _) = self {
            let ln_z = self.ln()?;
            let ln_10_complex = Float::Complex(Box::new(Float::from_str(LN_10).unwrap()), Box::new(Float::Big(BigDecimal::from(0))));
            return ln_z._div(&ln_10_complex);
        }
        
        let (m, e, neg, _k) = float_to_parts(self);
        let (rm, re, rneg, is_irr) = log10_float(m, e, neg)?;
        if is_irr {
            Ok(make_float_from_parts(rm, re, rneg, FloatKind::Irrational))
        } else {
            Ok(make_float_from_parts(rm, re, rneg, FloatKind::Finite))
        }
    }
    pub fn floor(&self) -> Result<Self, i16> {
        if self.is_complex() {
            return Err(ERR_INVALID_FORMAT);
        }
        
        let (m, e, neg, _k) = float_to_parts(self);
        let (rm, re, rneg) = floor_float(m, e, neg)?;
        Ok(make_float_from_parts(rm, re, rneg, FloatKind::Finite))
    }
    pub fn ceil(&self) -> Result<Self, i16> {
        if self.is_complex() {
            return Err(ERR_INVALID_FORMAT);
        }
        
        let (m, e, neg, _k) = float_to_parts(self);
        let (rm, re, rneg) = ceil_float(m, e, neg)?;
        Ok(make_float_from_parts(rm, re, rneg, FloatKind::Finite))
    }

    pub fn from_int(int: &Int) -> Result<Self, i16> {
        if int_is_nan(int) {
            return Err(ERR_INVALID_FORMAT);
        }
        if int_is_infinite(int) {
            let (_d, neg, _k) = int_to_parts(int);
            return Ok(if neg {
                Float::NegInfinity
            } else {
                Float::Infinity
            });
        }
        let (mantissa, neg, _k) = int_to_parts(int);
        if mantissa.is_empty() || mantissa == "0" {
            return Ok(make_float_from_parts(
                "0".to_string(),
                0,
                false,
                FloatKind::Finite,
            ));
        }
        Ok(make_float_from_parts(mantissa, 0, neg, FloatKind::Finite))
    }
    pub fn is_zero(&self) -> bool {
        float_is_zero(self)
    }
    pub fn round(&self, precision: usize) -> Self {
        // Round each component of complex number separately
        if let Float::Complex(real, imag) = self {
            let rounded_real = real.round(precision);
            let rounded_imag = imag.round(precision);
            return Float::Complex(Box::new(rounded_real), Box::new(rounded_imag));
        }
        
        let k = float_kind(self);
        if k == FloatKind::NaN || k == FloatKind::Infinity || k == FloatKind::NegInfinity {
            return self.clone();
        }
        if self.is_zero() {
            return make_float_from_parts("0".to_string(), 0, false, FloatKind::Finite);
        }

        let (mut mantissa, mut exponent, neg, _k) = {
            let (m, e, n, k1) = float_to_parts(self);
            (m, e, n, k1)
        };

        let old_len = mantissa.len();
        let mantissa_len = old_len as i32;
        let point_pos = mantissa_len + exponent;

        let digits_to_keep = if point_pos > 0 {
            (point_pos as usize) + precision
        } else {
            precision
        };

        if mantissa.len() > digits_to_keep {
            let round_digit = mantissa
                .chars()
                .nth(digits_to_keep)
                .unwrap_or('0')
                .to_digit(10)
                .unwrap_or(0);
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

        make_float_from_parts(mantissa, exponent, neg, FloatKind::Finite)
    }

    pub fn truncate(&self, decimal_places: usize) -> Self {
        let k = float_kind(self);
        if k == FloatKind::NaN || k == FloatKind::Infinity || k == FloatKind::NegInfinity {
            return self.clone();
        }
        if self.is_zero() {
            return make_float_from_parts("0".to_string(), 0, false, FloatKind::Finite);
        }

        let (mut mantissa, exponent, neg, _k) = float_to_parts(self);
        let mantissa_len = mantissa.len() as i32;
        let point_pos = mantissa_len + exponent;

        let digits_to_keep = if point_pos > 0 {
            (point_pos as usize) + decimal_places
        } else {
            decimal_places
        };

        if mantissa.len() > digits_to_keep {
            mantissa.truncate(digits_to_keep);
        }
        let mut exponent = exponent + (mantissa_len - mantissa.len() as i32);

        while mantissa.len() > 1 && mantissa.starts_with('0') {
            mantissa.remove(0);
            exponent -= 1;
        }
        if mantissa.is_empty() {
            mantissa = "0".to_string();
            exponent = 0;
        }

        make_float_from_parts(mantissa, exponent, neg, FloatKind::Finite)
    }
    pub fn from_f64(value: f64) -> Self {
        create_float(&value.to_string())
    }
    pub fn from_str(value: &str) -> Result<Self, i16> {
        if value.is_empty() {
            return Err(ERR_INVALID_FORMAT);
        }
        let float = create_float(value);
        let k = float_kind(&float);
        if k == FloatKind::NaN || k == FloatKind::Infinity || k == FloatKind::NegInfinity {
            return Err(ERR_INVALID_FORMAT);
        }
        Ok(float)
    }
    pub fn is_integer_like(&self) -> bool {
        let k = float_kind(self);
        if k == FloatKind::NaN || k == FloatKind::Infinity || k == FloatKind::NegInfinity {
            return false;
        }
        let (mant, exp, _neg, _k) = float_to_parts(self);
        if (mant.is_empty() && !(exp >= 0)) || !mant.chars().all(|c| c.is_digit(10)) {
            return false;
        }
        if exp >= 0 {
            return true;
        }
        let frac_len = (-exp) as usize;
        if frac_len > mant.len() {
            return mant.chars().all(|c| c == '0');
        }
        let int_part_len = mant.len() - frac_len;
        let frac_part = &mant[int_part_len..];
        frac_part.chars().all(|c| c == '0')
    }

    pub fn to_int(&self) -> Result<Int, i16> {
        if float_to_parts(self).3 == FloatKind::NaN {
            return Err(ERR_INVALID_FORMAT);
        }
        let k = float_kind(self);
        if k == FloatKind::Infinity || k == FloatKind::NegInfinity {
            return Err(ERR_INFINITE_RESULT);
        }
        if self.is_zero() {
            return Ok(make_int_from_parts(
                "0".to_string(),
                false,
                FloatKind::Finite,
            ));
        }
        if !self.is_integer_like() {
            return Err(ERR_INVALID_FORMAT);
        }

        let (mut digits, exp, neg, _k) = float_to_parts(self);
        if exp < 0 {
            let e = (-exp) as usize;
            if e >= digits.len() {
                digits = "0".to_string();
            } else {
                digits.truncate(digits.len() - e);
            }
        } else if exp > 0 {
            digits.push_str(&"0".repeat(exp as usize));
        }
        let digits = normalize_int_digits(&digits);
        Ok(make_int_from_parts(digits, neg, FloatKind::Finite))
    }

    pub fn is_nan(&self) -> bool {
        float_to_parts(self).3 == FloatKind::NaN
    }
    pub fn is_infinity(&self) -> bool {
        float_to_parts(self).3 == FloatKind::Infinity
    }
    pub fn to_str(&self) -> String {
        let k = float_kind(self);
        if k == FloatKind::NaN {
            return "NaN".to_string();
        }
        if k == FloatKind::Infinity {
            return "Infinity".to_string();
        }
        if k == FloatKind::NegInfinity {
            return "-Infinity".to_string();
        }

        if k == FloatKind::Recurring {
            if let Float::Recurring(ref bd) = *self {
                let n = bd.normalized();
                let int_candidate = n.with_scale(0);
                if n == int_candidate {
                    return int_candidate.normalized().to_string();
                }

                let s_norm = n.normalized().to_string();
                if s_norm.contains('.') {
                    let parts: Vec<&str> = s_norm.split('.').collect();
                    let frac = parts[1];
                    if frac.len() <= 20 {
                        return s_norm;
                    }
                } else {
                    return s_norm;
                }

                let t = bd.with_scale(10);
                let s = t.to_string();
                if s.contains('.') {
                    let parts: Vec<&str> = s.split('.').collect();
                    let int_part = parts[0];
                    let mut frac = parts[1].to_string();
                    if frac.len() > 10 {
                        frac.truncate(10);
                    } else if frac.len() < 10 {
                        frac.push_str(&"0".repeat(10 - frac.len()));
                    }
                    return format!("{}.{}", int_part, frac);
                } else {
                    return format!("{}.{}", s, "0".repeat(10));
                }
            }
        }

        if let Some(bd) = crate::compat::float_to_bigdecimal(self) {
            return bd.normalized().to_string();
        }
        format!("{}", self)
    }
    pub fn make_irrational(&mut self) -> Self {
        let k = float_kind(self);
        if k == FloatKind::NaN || k == FloatKind::Infinity || k == FloatKind::NegInfinity {
            return self.clone();
        }
        let (m, e, neg, _) = float_to_parts(self);
        let newf = make_float_from_parts(m, e, neg, FloatKind::Irrational);
        *self = newf.clone();
        newf
    }

    pub fn normalize(&mut self) -> &mut Self {
        let (mut mant, mut exp, neg, _k) = float_to_parts(self);
        let trimmed = mant.trim_start_matches('0');
        let trimmed_len = trimmed.len();
        if trimmed_len == 0 {
            mant = "0".to_string();
            exp = 0;
        } else {
            let zeros_removed = mant.len() - trimmed_len;
            mant = trimmed.to_string();
            exp += zeros_removed as i32;
        }
        if mant.is_empty() {
            mant = "0".to_string();
            exp = 0;
        }
        if mant == "0" {
            exp = 0;
        }
        let newf = make_float_from_parts(mant, exp, neg, FloatKind::Finite);
        *self = newf;
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
        let (digits, negative, _k) = int_to_parts(self);
        let prefix = if negative { "-" } else { "" };
        if let Ok(num) = digits.parse::<i128>() {
            write!(f, "{}{:b}", prefix, num)
        } else {
            Err(std::fmt::Error)
        }
    }
}

impl Octal for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (digits, negative, _k) = int_to_parts(self);
        let prefix = if negative { "-" } else { "" };
        if let Ok(num) = digits.parse::<i128>() {
            write!(f, "{}{:o}", prefix, num)
        } else {
            Err(std::fmt::Error)
        }
    }
}

impl LowerHex for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (digits, negative, _k) = int_to_parts(self);
        let prefix = if negative { "-" } else { "" };
        if let Ok(num) = digits.parse::<i128>() {
            write!(f, "{}{:x}", prefix, num)
        } else {
            Err(std::fmt::Error)
        }
    }
}

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
        let (digits, negative, _k) = int_to_parts(self);
        digits.hash(state);
        negative.hash(state);
    }
}

impl Hash for Float {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let (mant, exp, neg, k) = float_to_parts(self);
        mant.hash(state);
        exp.hash(state);
        neg.hash(state);
        k.hash(state);
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

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        use crate::compat::float_to_bigdecimal;

        if let Float::NaN = self {
            return false;
        }
        if let Float::NaN = other {
            return false;
        }

        match (self, other) {
            (Float::Infinity, Float::Infinity) => return true,
            (Float::NegInfinity, Float::NegInfinity) => return true,
            (Float::Infinity, Float::NegInfinity) | (Float::NegInfinity, Float::Infinity) => {
                return false;
            }
            // Handle complex number equality: (a+bi) == (c+di) iff a==c and b==d
            (Float::Complex(r1, i1), Float::Complex(r2, i2)) => {
                return r1.eq(r2) && i1.eq(i2);
            }
            // Complex != Real
            (Float::Complex(_, _), _) | (_, Float::Complex(_, _)) => {
                return false;
            }
            _ => {}
        }

        if let (Some(a), Some(b)) = (float_to_bigdecimal(self), float_to_bigdecimal(other)) {
            return a.normalized() == b.normalized();
        }

        false
    }
}

impl PartialEq<Int> for Float {
    fn eq(&self, other: &Int) -> bool {
        use crate::compat::{float_to_bigdecimal, int_to_parts};
        use std::str::FromStr;

        if let Float::NaN = self {
            return false;
        }

        if let Some(a) = float_to_bigdecimal(self) {
            let (digits, negative, _k) = int_to_parts(other);
            if let Ok(mut bi) = BigInt::from_str(&digits) {
                if negative {
                    bi = -bi;
                }
                let bdec = BigDecimal::new(bi, 0);
                return a.normalized() == bdec.normalized();
            }
        }
        false
    }
}

impl PartialEq<Float> for Int {
    fn eq(&self, other: &Float) -> bool {
        other.eq(self)
    }
}

pub trait IntoSmallInt {
    fn into_small_int(self) -> Int;
}

macro_rules! impl_small_int {
    ($($t:ty => $variant:ident),*) => {
        $(
            impl IntoSmallInt for $t {
                fn into_small_int(self) -> Int {
                    Int::Small(SmallInt::$variant(self))
                }
            }
        )*
    };
}

impl_small_int!(
    i8 => I8,
    u8 => U8,
    i16 => I16,
    u16 => U16,
    i32 => I32,
    u32 => U32,
    i64 => I64,
    u64 => U64,
    i128 => I128,
    u128 => U128,
    isize => ISize,
    usize => USize
);

pub trait IntoSmallFloat {
    fn into_small_float(self) -> Float;
}

macro_rules! impl_small_float {
    ($($t:ty => $variant:ident),*) => {
        $(
            impl IntoSmallFloat for $t {
                fn into_small_float(self) -> Float {
                    Float::Small(SmallFloat::$variant(self))
                }
            }
        )*
    };
}

impl_small_float!(f32 => F32, f64 => F64);

fn approx_rational_from_f64(x: f64, max_den: u64) -> Option<(u64, u64)> {
    if x.is_nan() || x.is_infinite() { return None; }
    let mut a: Vec<u64> = Vec::new();
    let mut r = x;
    for _ in 0..64 {
        let ai = r.floor() as u64;
        a.push(ai);
        let frac = r - (ai as f64);
        if frac.abs() < 1e-15 { break; }
        r = 1.0 / frac;
    }
    let mut num0: i128 = 0; let mut den0: i128 = 1;
    let mut num1: i128 = 1; let mut den1: i128 = 0;
    for ai in a {
        let num2 = ai as i128 * num1 + num0;
        let den2 = ai as i128 * den1 + den0;
        if den2 > (max_den as i128) { break; }
        num0 = num1; den0 = den1;
        num1 = num2; den1 = den2;
    }
    if den1 <= 0 { return None; }
    if num1 < 0 { return None; }
    Some((num1 as u64, den1 as u64))
}

fn bigdecimal_nth_root(a: BigDecimal, n: u32, prec: usize) -> Option<BigDecimal> {
    if n == 0 { return None; }
    if a.is_zero() { return Some(BigDecimal::from(0u32)); }
    if a.is_negative() { return None; }
    use bigdecimal::ToPrimitive;
    let af = a.to_f64().unwrap_or(1.0);
    let mut x = BigDecimal::from_f64(af.powf(1.0 / (n as f64))).unwrap_or_else(|| BigDecimal::from(1u32));
    let scale = prec as i64;
    for _ in 0..(prec + 20) {
        // x_{k+1} = (1/n) * ((n-1)*x_k + a / x_k^{n-1})
        let mut x_pow = BigDecimal::from(1u32);
        for _ in 0..(n-1) { x_pow = &x_pow * &x; }
        if x_pow.is_zero() { return None; }
        let t = &a / x_pow;
        let numer = (&BigDecimal::from((n-1) as i32) * &x) + t;
        let next = numer / BigDecimal::from(n as i32);
        let diff = (&next - &x).abs();
        x = next.with_scale(scale);
        if diff.to_f64().unwrap_or(0.0) < 10f64.powi(-(prec as i32)) { break; }
    }
    Some(x)
}

pub trait ApproxEq {
    fn approx_eq(&self, n: &Self, epsilon: f64) -> bool;
}

impl ApproxEq for Int {
    fn approx_eq(&self, n: &Self, epsilon: f64) -> bool {
        if self == n {
            return true;
        }
        
        let a_bigint = match self {
            Int::Big(b) => b.clone(),
            Int::Small(s) => {
                match s {
                    SmallInt::I8(v) => BigInt::from(*v),
                    SmallInt::U8(v) => BigInt::from(*v),
                    SmallInt::I16(v) => BigInt::from(*v),
                    SmallInt::U16(v) => BigInt::from(*v),
                    SmallInt::I32(v) => BigInt::from(*v),
                    SmallInt::U32(v) => BigInt::from(*v),
                    SmallInt::I64(v) => BigInt::from(*v),
                    SmallInt::U64(v) => BigInt::from(*v),
                    SmallInt::I128(v) => BigInt::from(*v),
                    SmallInt::U128(v) => BigInt::from(*v),
                    SmallInt::ISize(v) => BigInt::from(*v),
                    SmallInt::USize(v) => BigInt::from(*v),
                }
            }
        };
        
        let b_bigint = match n {
            Int::Big(b) => b.clone(),
            Int::Small(s) => {
                match s {
                    SmallInt::I8(v) => BigInt::from(*v),
                    SmallInt::U8(v) => BigInt::from(*v),
                    SmallInt::I16(v) => BigInt::from(*v),
                    SmallInt::U16(v) => BigInt::from(*v),
                    SmallInt::I32(v) => BigInt::from(*v),
                    SmallInt::U32(v) => BigInt::from(*v),
                    SmallInt::I64(v) => BigInt::from(*v),
                    SmallInt::U64(v) => BigInt::from(*v),
                    SmallInt::I128(v) => BigInt::from(*v),
                    SmallInt::U128(v) => BigInt::from(*v),
                    SmallInt::ISize(v) => BigInt::from(*v),
                    SmallInt::USize(v) => BigInt::from(*v),
                }
            }
        };
        
        let diff = (a_bigint - b_bigint).abs();
        
        let epsilon_bigint = BigInt::from(epsilon.abs() as i64);
        
        diff <= epsilon_bigint
    }
}

impl ApproxEq for Float {
    fn approx_eq(&self, n: &Self, epsilon: f64) -> bool {
        match (self, n) {
            (Float::NaN, _) | (_, Float::NaN) => return false,
            (Float::Infinity, Float::Infinity) => return true,
            (Float::NegInfinity, Float::NegInfinity) => return true,
            (Float::Infinity, _) | (_, Float::Infinity) => return false,
            (Float::NegInfinity, _) | (_, Float::NegInfinity) => return false,
            _ => {}
        }
        
        if self.is_complex() && n.is_complex() {
            if let (Float::Complex(r1, i1), Float::Complex(r2, i2)) = (self, n) {
                return r1.approx_eq(r2, epsilon) && i1.approx_eq(i2, epsilon);
            }
        }
        
        if self.is_complex() != n.is_complex() {
            return false;
        }
        
        if let Ok(diff) = self - n {
            let abs_diff = diff.abs();
            
            if let Ok(diff_val) = abs_diff.to_f64() {
                return diff_val.abs() <= epsilon;
            }
        }
        
        false
    }
}
