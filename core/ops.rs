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

impl PartialOrd for Int {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.negative && !other.negative {
            return Some(Ordering::Less);
        }
        if !self.negative && other.negative {
            return Some(Ordering::Greater);
        }
        if self.digits == other.digits {
            return Some(Ordering::Equal);
        }
        if self.negative {
            Some(other.digits.cmp(&self.digits).reverse())
        } else {
            Some(self.digits.cmp(&other.digits))
        }
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
        if self.mantissa == other.mantissa && self.exponent == other.exponent {
            return Some(Ordering::Equal);
        }
        if self.negative {
            Some(other.to_f64().unwrap().partial_cmp(&self.to_f64().unwrap()).unwrap().reverse())
        } else {
            Some(self.to_f64().unwrap().partial_cmp(&other.to_f64().unwrap()).unwrap())
        }
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
                write!(f, "{}", mantissa)?;
            } else if exp > 0 {
                write!(f, "{}{}", mantissa, "0".repeat(exp as usize))?;
            } else {
                let mantissa_len = mantissa.len() as i64;
                let point_pos = mantissa_len + (exp as i64); // cast to i64

                if point_pos > 0 {
                    let (int_part, frac_part) = mantissa.split_at(point_pos as usize);
                    write!(f, "{}.{}", int_part, frac_part)?;
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

