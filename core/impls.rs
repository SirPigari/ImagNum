use crate::compat::{
    float_is_zero, float_kind, float_to_parts, int_is_infinite, int_is_nan, int_to_parts,
    int_to_string, make_float_from_parts, make_int_from_parts,
};
use crate::foundation::{Float, FloatKind, Int};
use crate::functions::{create_float, create_int};
use crate::math::{
    ERR_DIV_BY_ZERO, ERR_INFINITE_RESULT, ERR_INVALID_FORMAT, ERR_NEGATIVE_RESULT,
    ERR_NEGATIVE_SQRT, ERR_UNIMPLEMENTED, add_float, add_strings, ceil_float, ceil_int, cos_float,
    cos_int, div_float, div_strings, exp_float, exp_int, floor_float, floor_int, is_string_odd,
    ln_float, ln_int, log10_float, mod_float, mod_strings, mul_float, mul_strings, pow_strings,
    sin_float, sin_int, sqrt_float, sqrt_int, sub_float, sub_strings, tan_float, tan_int,
};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::fmt::{Binary, LowerHex, Octal};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

impl Int {
    pub fn is_negative(&self) -> bool {
        let (_d, neg, _k) = int_to_parts(self);
        neg
    }

    pub fn to_float(&self) -> Result<Float, i16> {
        // Convert integer into Float::Big
        match self {
            Int::Big(bi) => {
                // Construct BigDecimal from BigInt directly to avoid string parsing
                let bd = BigDecimal::from(bi.clone());
                Ok(Float::Big(bd))
            }
            Int::Small(_) => {
                // Use string conversion for small ints
                let s = int_to_string(self);
                match BigDecimal::from_str(&s) {
                    Ok(bd) => Ok(Float::Big(bd)),
                    Err(_) => Err(ERR_INVALID_FORMAT),
                }
            }
        }
    }
    pub fn _add(&self, other: &Self) -> Result<Self, i16> {
        let (_sd, sneg, _sk) = int_to_parts(self);
        let (_od, oneg, _ok) = int_to_parts(other);
        match (sneg, oneg) {
            (false, false) => {
                let (digits, _) = add_strings(&int_to_string(self), &int_to_string(other))?;
                let digits = normalize_int_digits(&digits);
                match BigInt::from_str(&digits) {
                    Ok(bi) => Ok(Int::Big(bi)),
                    Err(_) => Ok(Int::new()),
                }
            }
            (true, true) => {
                let (digits, _) = add_strings(&int_to_string(self), &int_to_string(other))?;
                let digits = normalize_int_digits(&digits);
                match BigInt::from_str(&digits) {
                    Ok(mut bi) => {
                        bi = -bi;
                        Ok(Int::Big(bi))
                    }
                    Err(_) => Ok(Int::new()),
                }
            }
            (false, true) => {
                let (odigits, oneg, _k) = int_to_parts(other);
                let other_int = match BigInt::from_str(&odigits) {
                    Ok(bi) => {
                        if oneg {
                            Int::Big(-bi)
                        } else {
                            Int::Big(bi)
                        }
                    }
                    Err(_) => Int::new(),
                };
                self._sub(&other_int)
            }
            (true, false) => {
                // (-A) + B = B - A
                let (sd, _sneg, _k) = int_to_parts(self);
                let self_pos = make_int_from_parts(sd.clone(), false, FloatKind::Finite);
                let res = other._sub(&self_pos)?;
                Ok(res)
            }
        }
    }
    pub fn _sub(&self, other: &Self) -> Result<Self, i16> {
        let (sd, sneg, _sk) = int_to_parts(self);
        let (od, oneg, _ok) = int_to_parts(other);
        match (sneg, oneg) {
            (false, false) => {
                let (digits, sign_flipped) = sub_strings(&sd, &od)?;
                let digits = normalize_int_digits(&digits);
                let negative = if digits == "0" { false } else { sign_flipped };
                Ok(make_int_from_parts(digits, negative, FloatKind::Finite))
            }
            (true, true) => {
                // -(A) - -(B) = B - A
                let left = make_int_from_parts(od.clone(), false, FloatKind::Finite);
                let right = make_int_from_parts(sd.clone(), false, FloatKind::Finite);
                let res = left._sub(&right)?;
                // result sign already correct
                Ok(res)
            }
            (false, true) => {
                let other_pos = make_int_from_parts(od.clone(), false, FloatKind::Finite);
                self._add(&other_pos)
            }
            (true, false) => {
                let self_pos = make_int_from_parts(sd.clone(), false, FloatKind::Finite);
                let res = self_pos._add(other)?;
                // flip sign
                Ok(-res)
            }
        }
    }
    pub fn _mul(&self, other: &Self) -> Result<Self, i16> {
        let (digits, sign_flipped) = mul_strings(&int_to_string(self), &int_to_string(other))?;
        let digits = normalize_int_digits(&digits);
        let (_d, sneg, _k) = int_to_parts(self);
        let (_od, oneg, _k2) = int_to_parts(other);
        let negative = sneg ^ oneg ^ sign_flipped;
        match BigInt::from_str(&digits) {
            Ok(mut bi) => {
                if negative {
                    bi = -bi
                };
                Ok(Int::Big(bi))
            }
            Err(_) => Ok(Int::new()),
        }
    }
    pub fn _div(&self, other: &Self) -> Result<Self, i16> {
        let (digits, sign_flipped) = div_strings(&int_to_string(self), &int_to_string(other))?;
        let digits = normalize_int_digits(&digits);
        let (_d, sneg, _k) = int_to_parts(self);
        let (_od, oneg, _k2) = int_to_parts(other);
        let negative = sneg ^ oneg ^ sign_flipped;
        match BigInt::from_str(&digits) {
            Ok(mut bi) => {
                if negative {
                    bi = -bi
                };
                Ok(Int::Big(bi))
            }
            Err(_) => Ok(Int::new()),
        }
    }
    pub fn _modulo(&self, other: &Self) -> Result<Self, i16> {
        let (digits, sign_flipped) = mod_strings(&int_to_string(self), &int_to_string(other))?;
        let digits = normalize_int_digits(&digits);
        let (_d, sneg, _k) = int_to_parts(self);
        let negative = sneg ^ sign_flipped;
        match BigInt::from_str(&digits) {
            Ok(mut bi) => {
                if negative {
                    bi = -bi
                };
                Ok(Int::Big(bi))
            }
            Err(_) => Ok(Int::new()),
        }
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
        // For negative integers, return imaginary complex
        // Convert to BigDecimal and take sqrt via Float::Big
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

    // Transcendental wrappers for Int: return Float (may be irrational)
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
        // check for invalid or infinite via compat
        if int_is_nan(self) {
            return Err(ERR_INVALID_FORMAT);
        }
        if int_is_infinite(self) {
            return Err(ERR_INFINITE_RESULT);
        }
        let (digits, negative, _k) = int_to_parts(self);
        if negative || digits.is_empty() || digits == "0" {
            return Err(ERR_NEGATIVE_RESULT);
        }

        let value: usize = digits.parse().map_err(|_| ERR_INVALID_FORMAT)?;
        Ok(value)
    }
    pub fn to_i64(&self) -> Result<i64, i16> {
        if int_is_nan(self) {
            return Err(ERR_INVALID_FORMAT);
        }
        if int_is_infinite(self) {
            return Err(ERR_INFINITE_RESULT);
        }

        let (digits, negative, _k) = int_to_parts(self);
        if digits.is_empty() || digits == "0" {
            return Ok(0 as i64);
        }

        let value = if negative {
            -digits.parse::<i64>().map_err(|_| ERR_INVALID_FORMAT)?
        } else {
            digits.parse::<i64>().map_err(|_| ERR_INVALID_FORMAT)?
        };
        Ok(value)
    }
    pub fn to_i128(&self) -> Result<i128, i16> {
        if int_is_nan(self) {
            return Err(ERR_INVALID_FORMAT);
        }
        if int_is_infinite(self) {
            return Err(ERR_INFINITE_RESULT);
        }

        let (digits, negative, _k) = int_to_parts(self);
        if digits.is_empty() || digits == "0" {
            return Ok(0 as i128);
        }

        let value = if negative {
            -digits.parse::<i128>().map_err(|_| ERR_INVALID_FORMAT)?
        } else {
            digits.parse::<i128>().map_err(|_| ERR_INVALID_FORMAT)?
        };
        Ok(value)
    }
    pub fn from_i64(value: i64) -> Self {
        if value < 0 {
            make_int_from_parts(value.abs().to_string(), true, FloatKind::Finite)
        } else {
            make_int_from_parts(value.to_string(), false, FloatKind::Finite)
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
    /// Plain string representation for Int (same as Display but returns String).
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

    pub fn to_f64(&self) -> Result<f64, i16> {
        // Fast path: if we already have a BigDecimal (or small float convertible), use it directly.
        if let Some(bd) = crate::compat::float_to_bigdecimal(self) {
            return bd.to_f64().ok_or(ERR_INVALID_FORMAT);
        }
        // Handle NaN/Infinity explicitly
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
        // Use math helper to get truncated, possibly irrational result
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
        // If either operand was recurring, keep the result as recurring so display shows the repeating cycle
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
        Ok(make_float_from_parts(
            mantissa,
            exponent,
            negative,
            result_kind,
        ))
    }
    pub fn _div(&self, other: &Self) -> Result<Self, i16> {
        if float_kind(self) == FloatKind::NaN || float_kind(other) == FloatKind::NaN {
            return Err(ERR_INVALID_FORMAT);
        }
        if float_is_zero(other) {
            return Err(ERR_DIV_BY_ZERO);
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

        // If both operands are integer-like (no fractional part), attempt exact rational detection
        let self_is_int_like =
            e1 >= 0 || (e1 < 0 && (-(e1) as usize) <= m1.len() && m1.chars().all(|c| c == '0'));
        let other_is_int_like =
            e2 >= 0 || (e2 < 0 && (-(e2) as usize) <= m2.len() && m2.chars().all(|c| c == '0'));

        if self_is_int_like && other_is_int_like {
            // Build BigInt numerator and denominator from mantissas and exponents
            // numerator = m1 * 10^{max(0, -e1)}; denominator = m2 * 10^{max(0, -e2)}
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
            // Reduce
            let g = num.clone().abs().gcd(&den.clone().abs());
            if !g.is_zero() {
                num = num / &g;
                den = den / &g;
            }
            // remove factors 2 and 5 from denominator to determine recurring
            let mut d = den.clone().abs();
            while (&d % BigInt::from(2u32)).is_zero() {
                d = d / BigInt::from(2u32);
            }
            while (&d % BigInt::from(5u32)).is_zero() {
                d = d / BigInt::from(5u32);
            }
            let recurring = !d.is_one();

            // produce BigDecimal approximation at a safe scale and return recurring/finite accordingly
            let bd = BigDecimal::new(num.clone(), 0) / BigDecimal::new(den.clone(), 0);
            if recurring {
                return Ok(Float::Recurring(bd));
            } else {
                return Ok(Float::Big(bd));
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

        // Convert operands to f64 using the provided conversion helper. This handles
        // BigDecimal-backed floats and avoids brittle string-length checks that
        // rejected long fractional exponents.
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
        let (_m, _e, _, k) = float_to_parts(self);
        make_float_from_parts(_m, _e, false, k)
    }

    // Transcendental wrappers for Float
    pub fn sin(&self) -> Result<Self, i16> {
        let (m, e, neg, _k) = float_to_parts(self);
        let (rm, re, rneg, is_irr) = sin_float(m, e, neg)?;
        if is_irr {
            Ok(make_float_from_parts(rm, re, rneg, FloatKind::Irrational))
        } else {
            Ok(make_float_from_parts(rm, re, rneg, FloatKind::Finite))
        }
    }
    pub fn cos(&self) -> Result<Self, i16> {
        let (m, e, neg, _k) = float_to_parts(self);
        let (rm, re, rneg, is_irr) = cos_float(m, e, neg)?;
        if is_irr {
            Ok(make_float_from_parts(rm, re, rneg, FloatKind::Irrational))
        } else {
            Ok(make_float_from_parts(rm, re, rneg, FloatKind::Finite))
        }
    }
    pub fn tan(&self) -> Result<Self, i16> {
        let (m, e, neg, _k) = float_to_parts(self);
        let (rm, re, rneg, is_irr) = tan_float(m, e, neg)?;
        if is_irr {
            Ok(make_float_from_parts(rm, re, rneg, FloatKind::Irrational))
        } else {
            Ok(make_float_from_parts(rm, re, rneg, FloatKind::Finite))
        }
    }
    pub fn ln(&self) -> Result<Self, i16> {
        let (m, e, neg, _k) = float_to_parts(self);
        let (rm, re, rneg, is_irr) = ln_float(m, e, neg)?;
        if is_irr {
            Ok(make_float_from_parts(rm, re, rneg, FloatKind::Irrational))
        } else {
            Ok(make_float_from_parts(rm, re, rneg, FloatKind::Finite))
        }
    }
    pub fn exp(&self) -> Result<Self, i16> {
        let (m, e, neg, _k) = float_to_parts(self);
        let (rm, re, rneg, is_irr) = exp_float(m, e, neg)?;
        if is_irr {
            Ok(make_float_from_parts(rm, re, rneg, FloatKind::Irrational))
        } else {
            Ok(make_float_from_parts(rm, re, rneg, FloatKind::Finite))
        }
    }
    pub fn log(&self) -> Result<Self, i16> {
        let (m, e, neg, _k) = float_to_parts(self);
        let (rm, re, rneg, is_irr) = log10_float(m, e, neg)?;
        if is_irr {
            Ok(make_float_from_parts(rm, re, rneg, FloatKind::Irrational))
        } else {
            Ok(make_float_from_parts(rm, re, rneg, FloatKind::Finite))
        }
    }
    pub fn floor(&self) -> Result<Self, i16> {
        let (m, e, neg, _k) = float_to_parts(self);
        let (rm, re, rneg) = floor_float(m, e, neg)?;
        Ok(make_float_from_parts(rm, re, rneg, FloatKind::Finite))
    }
    pub fn ceil(&self) -> Result<Self, i16> {
        let (m, e, neg, _k) = float_to_parts(self);
        let (rm, re, rneg) = ceil_float(m, e, neg)?;
        Ok(make_float_from_parts(rm, re, rneg, FloatKind::Finite))
    }

    pub fn from_int(int: &Int) -> Result<Self, i16> {
        if int_is_nan(int) {
            return Err(ERR_INVALID_FORMAT);
        }
        if int_is_infinite(int) {
            // compat: map to float infinity
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
    /// Plain string representation for Float.
    /// - Recurring values are expanded to 10 fractional decimal places (no rounding).
    /// - Irrational values return their numeric form without the trailing `...` used by Display.
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
                // If the recurring BigDecimal is exactly an integer (e.g. 0.(9) == 1),
                // return the integer string instead of a fixed fractional expansion.
                let n = bd.normalized();
                let int_candidate = n.with_scale(0);
                if n == int_candidate {
                    return int_candidate.normalized().to_string();
                }

                // If normalized form is a short finite decimal (e.g. 0.5), return it.
                let s_norm = n.normalized().to_string();
                if s_norm.contains('.') {
                    let parts: Vec<&str> = s_norm.split('.').collect();
                    let frac = parts[1];
                    // If fractional length is reasonably small, prefer the exact normalized string
                    if frac.len() <= 20 {
                        return s_norm;
                    }
                } else {
                    // integer-like already handled, but if no '.' present just return
                    return s_norm;
                }

                // Otherwise create a truncated BigDecimal with scale = 10 (exactly 10 fractional digits)
                let t = bd.with_scale(10);
                // to_string will produce a non-scientific form when scale is set
                let s = t.to_string();
                if s.contains('.') {
                    // ensure exactly 10 digits after decimal point
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

        // For Finite/Irrational/Big cases, try to use BigDecimal directly to avoid string roundtrips
        if let Some(bd) = crate::compat::float_to_bigdecimal(self) {
            // For irrational, BigDecimal already contains the truncated value (no trailing dots)
            return bd.normalized().to_string();
        }
        // Fallback to Display
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
