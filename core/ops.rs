use crate::foundation::{Int, Float, FloatKind};
use crate::compat::{int_to_string, float_to_parts, int_to_parts, make_float_from_parts, int_is_nan, int_is_infinite, float_kind};
use std::ops::{
    Add, Sub, Mul, Div, Rem, Neg,
    AddAssign, SubAssign, MulAssign, DivAssign, RemAssign,
};
use std::cmp::{Ordering, PartialOrd};
use std::fmt::{Display, Formatter, Result as FmtResult};

impl Add for Int {
    type Output = Result<Self, i16>;

    fn add(self, other: Self) -> Self::Output {
        self._add(&other.clone())
    }
}

impl Sub for Int {
    type Output = Result<Self, i16>;

    fn sub(self, other: Self) -> Self::Output {
        self._sub(&other.clone())
    }
}

impl Mul for Int {
    type Output = Result<Self, i16>;

    fn mul(self, other: Self) -> Self::Output {
        self._mul(&other.clone())
    }
}

impl Div for Int {
    type Output = Result<Self, i16>;

    fn div(self, other: Self) -> Self::Output {
        self._div(&other.clone())
    }
}

impl Rem for Int {
    type Output = Result<Self, i16>;

    fn rem(self, other: Self) -> Self::Output {
        self._modulo(&other.clone())
    }
}

impl Neg for Int {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Int::Big(b) => {
                // Negate BigInt
                Int::Big(-b)
            }
            Int::Small(s) => {
                // Convert small to Big for simplicity
                let s_str = int_to_string(&Int::Small(s));
                match s_str.parse::<i128>() {
                    Ok(v) => Int::Big((-v).into()),
                    Err(_) => Int::new(),
                }
            }
        }
    }
}

impl AddAssign for Int {
    fn add_assign(&mut self, other: Self) {
    *self = self._add(&other).unwrap_or_else(|_| Int::new());
    }
}

impl SubAssign for Int {
    fn sub_assign(&mut self, other: Self) {
    *self = self._sub(&other).unwrap_or_else(|_| Int::new());
    }
}

impl MulAssign for Int {
    fn mul_assign(&mut self, other: Self) {
    *self = self._mul(&other).unwrap_or_else(|_| Int::new());
    }
}

impl DivAssign for Int {
    fn div_assign(&mut self, other: Self) {
    *self = self._div(&other).unwrap_or_else(|_| Int::new());
    }
}

impl RemAssign for Int {
    fn rem_assign(&mut self, other: Self) {
    *self = self._modulo(&other).unwrap_or_else(|_| Int::new());
    }
}

// PartialEq for Float is provided by the derived implementation in foundation::Float.
// The older string-based equality logic relied on normalizing mantissa and exponent;
// keep `normalize` helper available for any compat-based comparisons if needed.

fn normalize(mantissa: &str, exponent: i32) -> (String, i32) {
    let mut digits = mantissa.trim_start_matches('0').to_string();
    if digits.is_empty() {
        return ("0".to_string(), 0);
    }

    let mut exp = exponent;

    while digits.ends_with('0') {
        digits.pop();
        exp += 1;
    }

    if digits.is_empty() {
        return ("0".to_string(), 0);
    }

    (digits, exp)
}


impl PartialOrd for Int {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    let (self_digits, self_neg, _k) = int_to_parts(self);
    let (other_digits, other_neg, _k2) = int_to_parts(other);
    if self_neg && !other_neg { return Some(Ordering::Less); }
    if !self_neg && other_neg { return Some(Ordering::Greater); }
    let self_digits = self_digits.trim_start_matches('0');
    let other_digits = other_digits.trim_start_matches('0');

        let len_cmp = self_digits.len().cmp(&other_digits.len());

        if len_cmp != Ordering::Equal {
            return if self_neg {
                Some(len_cmp.reverse())
            } else {
                Some(len_cmp)
            };
        }

        for (a, b) in self_digits.chars().zip(other_digits.chars()) {
            if a != b {
                let cmp = a.cmp(&b);
                return if self_neg {
                    Some(cmp.reverse())
                } else {
                    Some(cmp)
                };
            }
        }

        Some(Ordering::Equal)
    }
}


impl Display for Int {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    if int_is_nan(self) { write!(f, "NaN")?; return Ok(()); }
    if int_is_infinite(self) { let (_d, neg, _k) = int_to_parts(self); if neg { write!(f, "-Infinity")?; } else { write!(f, "Infinity")?; } return Ok(()); }
    let (digits, neg, _k) = int_to_parts(self);
    if neg { write!(f, "-")?; }
    write!(f, "{}", digits)
    }
}

impl Add for Float {
    type Output = Result<Self, i16>;

    fn add(self, other: Self) -> Self::Output {
        self._add(&other.clone())
    }
}

impl Sub for Float {
    type Output = Result<Self, i16>;

    fn sub(self, other: Self) -> Self::Output {
        self._sub(&other.clone())
    }
}

impl Mul for Float {
    type Output = Result<Self, i16>;

    fn mul(self, other: Self) -> Self::Output {
        self._mul(&other.clone())
    }
}

impl Div for Float {
    type Output = Result<Self, i16>;

    fn div(self, other: Self) -> Self::Output {
        self._div(&other.clone())
    }
}

impl Rem for Float {
    type Output = Result<Self, i16>;

    fn rem(self, other: Self) -> Self::Output {
        self._modulo(&other.clone())
    }
}

impl Neg for Float {
    type Output = Self;

    fn neg(self) -> Self::Output {
    let (m,e,neg,k) = float_to_parts(&self);
    make_float_from_parts(m, e, !neg, k)
    }
}

impl AddAssign for Float {
    fn add_assign(&mut self, other: Self) {
    *self = self._add(&other).unwrap_or_else(|_| Float::NaN);
    }
}

impl SubAssign for Float {
    fn sub_assign(&mut self, other: Self) {
    *self = self._sub(&other).unwrap_or_else(|_| Float::NaN);
    }
}

impl MulAssign for Float {
    fn mul_assign(&mut self, other: Self) {
    *self = self._mul(&other).unwrap_or_else(|_| Float::NaN);
    }
}

impl DivAssign for Float {
    fn div_assign(&mut self, other: Self) {
    *self = self._div(&other).unwrap_or_else(|_| Float::NaN);
    }
}

impl RemAssign for Float {
    fn rem_assign(&mut self, other: Self) {
    *self = self._modulo(&other).unwrap_or_else(|_| Float::NaN);
    }
}

impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;
    let (self_man, self_exp, self_neg, _k1) = float_to_parts(self);
    let (other_man, other_exp, other_neg, _k2) = float_to_parts(other);
    if self_neg && !other_neg { return Some(Ordering::Less); }
    if !self_neg && other_neg { return Some(Ordering::Greater); }
    let sign = if self_neg { -1 } else { 1 };
    let (mut self_man, self_exp) = normalize(&self_man, self_exp);
    let (mut other_man, other_exp) = normalize(&other_man, other_exp);

        // Pad zeros on the LEFT to align exponents
        if self_exp > other_exp {
            let diff = (self_exp - other_exp) as usize;
            other_man = format!("{}{}", "0".repeat(diff), other_man);
        } else if other_exp > self_exp {
            let diff = (other_exp - self_exp) as usize;
            self_man = format!("{}{}", "0".repeat(diff), self_man);
        }

        // Now pad zeros on the RIGHT to equalize mantissa lengths
        let max_len = self_man.len().max(other_man.len());
        if self_man.len() < max_len {
            self_man = format!("{}{}", self_man, "0".repeat(max_len - self_man.len()));
        }
        if other_man.len() < max_len {
            other_man = format!("{}{}", other_man, "0".repeat(max_len - other_man.len()));
        }

        // Compare digit by digit
        for (a, b) in self_man.chars().zip(other_man.chars()) {
            if a != b {
                let cmp = a.cmp(&b);
                return Some(if sign == 1 { cmp } else { cmp.reverse() });
            }
        }
        Some(Ordering::Equal)
    }
}

impl Display for Float {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let k = float_kind(self);
        if k == FloatKind::NaN {
            write!(f, "NaN")?;
            return Ok(());
        } else if k == FloatKind::Infinity {
            write!(f, "Infinity")?;
            return Ok(());
        } else if k == FloatKind::NegInfinity {
            write!(f, "-Infinity")?;

            return Ok(());
        }

        let (mant, exp, neg, k) = float_to_parts(self);
        if neg { write!(f, "-")?; }
        if exp >= -50 && exp <= 50 {
            let mantissa = mant.trim_start_matches('0');
            let mantissa = if mantissa.is_empty() { "0" } else { mantissa };
            if exp == 0 {
                write!(f, "{}.0", mantissa)?;
            } else if exp > 0 {
                write!(f, "{}{}", mantissa, "0".repeat(exp as usize))?;
                write!(f, ".0")?;
            } else {
                let mantissa_len = mantissa.len() as i64;
                let point_pos = mantissa_len + (exp as i64);
                if point_pos > 0 {
                    let (int_part, frac_part) = mantissa.split_at(point_pos as usize);
                    if frac_part.is_empty() {
                        write!(f, "{}.0", int_part)?;
                    } else {
                        write!(f, "{}.{}", int_part, frac_part)?;
                    }
                } else {
                    write!(f, "0.{}{}", "0".repeat((-point_pos) as usize), mantissa)?;
                }
            }
        } else {
            write!(f, "{}e{}", mant, exp)?;
        }
        if k == FloatKind::Irrational { write!(f, "...")?; }
        Ok(())
    }
}
