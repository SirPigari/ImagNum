#[allow(unused_imports)]
use crate::foundation::{Int, Float, NumberKind, NAN_FLOAT, NAN_INT};
#[allow(unused_imports)]
use crate::math::{
    ERR_UNIMPLEMENTED,
    ERR_INVALID_FORMAT,
    ERR_DIV_BY_ZERO,
    ERR_NEGATIVE_RESULT,
    ERR_NEGATIVE_SQRT,
    ERR_NUMBER_TOO_LARGE
};
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
        Int::new(self.digits, !self.negative, self.kind)
    }
}

impl AddAssign for Int {
    fn add_assign(&mut self, other: Self) {
        *self = self._add(&other).unwrap_or_else(|_| NAN_INT.clone());
    }
}

impl SubAssign for Int {
    fn sub_assign(&mut self, other: Self) {
        *self = self._sub(&other).unwrap_or_else(|_| NAN_INT.clone());
    }
}

impl MulAssign for Int {
    fn mul_assign(&mut self, other: Self) {
        *self = self._mul(&other).unwrap_or_else(|_| NAN_INT.clone());
    }
}

impl DivAssign for Int {
    fn div_assign(&mut self, other: Self) {
        *self = self._div(&other).unwrap_or_else(|_| NAN_INT.clone());
    }
}

impl RemAssign for Int {
    fn rem_assign(&mut self, other: Self) {
        *self = self._modulo(&other).unwrap_or_else(|_| NAN_INT.clone());
    }
}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        if self.kind != other.kind || self.negative != other.negative {
            return false;
        }

        let (nm1, exp1) = normalize(&self.mantissa, self.exponent);
        let (nm2, exp2) = normalize(&other.mantissa, other.exponent);

        nm1 == nm2 && exp1 == exp2
    }
}

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

    (digits, exp)
}

impl PartialOrd for Int {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.negative && !other.negative {
            return Some(Ordering::Less);
        }
        if !self.negative && other.negative {
            return Some(Ordering::Greater);
        }

        let self_digits = self.digits.trim_start_matches('0');
        let other_digits = other.digits.trim_start_matches('0');

        let len_cmp = self_digits.len().cmp(&other_digits.len());

        if len_cmp != Ordering::Equal {
            return if self.negative {
                Some(len_cmp.reverse())
            } else {
                Some(len_cmp)
            };
        }

        for (a, b) in self_digits.chars().zip(other_digits.chars()) {
            if a != b {
                let cmp = a.cmp(&b);
                return if self.negative {
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
        if self.kind == NumberKind::NaN {
            write!(f, "NaN")?;
            return Ok(());
        } else if self.kind == NumberKind::Infinity {
            write!(f, "Infinity")?;
            return Ok(());
        } else if self.kind == NumberKind::NegInfinity {
            write!(f, "-Infinity")?;
            return Ok(());
        }
        if self.negative {
            write!(f, "-")?;
        }
        write!(f, "{}", self.digits)
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
        Float::new(self.mantissa, self.exponent, !self.negative, self.kind)
    }
}

impl AddAssign for Float {
    fn add_assign(&mut self, other: Self) {
        *self = self._add(&other).unwrap_or_else(|_| NAN_FLOAT.clone());
    }
}

impl SubAssign for Float {
    fn sub_assign(&mut self, other: Self) {
        *self = self._sub(&other).unwrap_or_else(|_| NAN_FLOAT.clone());
    }
}

impl MulAssign for Float {
    fn mul_assign(&mut self, other: Self) {
        *self = self._mul(&other).unwrap_or_else(|_| NAN_FLOAT.clone());
    }
}

impl DivAssign for Float {
    fn div_assign(&mut self, other: Self) {
        *self = self._div(&other).unwrap_or_else(|_| NAN_FLOAT.clone());
    }
}

impl RemAssign for Float {
    fn rem_assign(&mut self, other: Self) {
        *self = self._modulo(&other).unwrap_or_else(|_| NAN_FLOAT.clone());
    }
}

impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.negative && !other.negative {
            return Some(Ordering::Less);
        }
        if !self.negative && other.negative {
            return Some(Ordering::Greater);
        }
        let sign = if self.negative { -1 } else { 1 };

        fn normalize(mantissa: &str, exponent: i32) -> (String, i32) {
            let trimmed = mantissa.trim_start_matches('0');
            if trimmed.is_empty() {
                ("0".to_string(), 0)
            } else {
                let zeros_trimmed = mantissa.len() - trimmed.len();
                (trimmed.to_string(), exponent - zeros_trimmed as i32)
            }
        }

        let (mut self_man, self_exp) = normalize(&self.mantissa, self.exponent);
        let (mut other_man, other_exp) = normalize(&other.mantissa, other.exponent);

        if self_exp > other_exp {
            let diff = (self_exp - other_exp) as usize;
            other_man.push_str(&"0".repeat(diff));
        } else if other_exp > self_exp {
            let diff = (other_exp - self_exp) as usize;
            self_man.push_str(&"0".repeat(diff));
        }

        let max_len = self_man.len().max(other_man.len());
        if self_man.len() < max_len {
            self_man = format!("{}{}", "0".repeat(max_len - self_man.len()), self_man);
        }
        if other_man.len() < max_len {
            other_man = format!("{}{}", "0".repeat(max_len - other_man.len()), other_man);
        }

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
        if self.kind == NumberKind::NaN {
            write!(f, "NaN")?;
            return Ok(());
        } else if self.kind == NumberKind::Infinity {
            write!(f, "Infinity")?;
            return Ok(());
        } else if self.kind == NumberKind::NegInfinity {
            write!(f, "-Infinity")?;
            return Ok(());
        }

        if self.negative {
            write!(f, "-")?;
        }

        if self.exponent >= -50 && self.exponent <= 50 {
            let mantissa = self.mantissa.trim_start_matches('0');
            let mantissa = if mantissa.is_empty() { "0" } else { mantissa };

            let exp = self.exponent;
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
            write!(f, "{}e{}", self.mantissa, self.exponent)?;
        }

        if self.kind == NumberKind::Irrational {
            write!(f, "...")?;
        }

        Ok(())
    }
}
