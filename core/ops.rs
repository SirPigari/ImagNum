use crate::compat::{
    float_kind, float_to_parts, int_is_infinite, int_is_nan, int_to_parts, int_to_string,
    make_float_from_parts,
};
use crate::foundation::{Float, FloatKind, Int};
use std::cmp::{Ordering, PartialOrd};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

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

impl<'a> Add<&'a Int> for &'a Int {
    type Output = Result<Int, i16>;
    fn add(self, other: &'a Int) -> Self::Output {
        self._add(other)
    }
}

impl<'a> Add<&'a Int> for Int {
    type Output = Result<Int, i16>;
    fn add(self, other: &'a Int) -> Self::Output {
        self._add(other)
    }
}

impl<'a> Add<Int> for &'a Int {
    type Output = Result<Int, i16>;
    fn add(self, other: Int) -> Self::Output {
        self._add(&other)
    }
}

impl<'a> Sub<&'a Int> for Int {
    type Output = Result<Int, i16>;
    fn sub(self, other: &'a Int) -> Self::Output {
        self._sub(other)
    }
}

impl<'a> Sub<Int> for &'a Int {
    type Output = Result<Int, i16>;
    fn sub(self, other: Int) -> Self::Output {
        self._sub(&other)
    }
}

impl<'a> Mul<&'a Int> for Int {
    type Output = Result<Int, i16>;
    fn mul(self, other: &'a Int) -> Self::Output {
        self._mul(other)
    }
}

impl<'a> Mul<Int> for &'a Int {
    type Output = Result<Int, i16>;
    fn mul(self, other: Int) -> Self::Output {
        self._mul(&other)
    }
}

impl<'a> Div<&'a Int> for Int {
    type Output = Result<Int, i16>;
    fn div(self, other: &'a Int) -> Self::Output {
        self._div(other)
    }
}

impl<'a> Div<Int> for &'a Int {
    type Output = Result<Int, i16>;
    fn div(self, other: Int) -> Self::Output {
        self._div(&other)
    }
}

impl<'a> Rem<&'a Int> for Int {
    type Output = Result<Int, i16>;
    fn rem(self, other: &'a Int) -> Self::Output {
        self._modulo(other)
    }
}

impl<'a> Rem<Int> for &'a Int {
    type Output = Result<Int, i16>;
    fn rem(self, other: Int) -> Self::Output {
        self._modulo(&other)
    }
}

impl<'a> Sub<&'a Int> for &'a Int {
    type Output = Result<Int, i16>;
    fn sub(self, other: &'a Int) -> Self::Output {
        self._sub(other)
    }
}

impl<'a> Mul<&'a Int> for &'a Int {
    type Output = Result<Int, i16>;
    fn mul(self, other: &'a Int) -> Self::Output {
        self._mul(other)
    }
}

impl<'a> Div<&'a Int> for &'a Int {
    type Output = Result<Int, i16>;
    fn div(self, other: &'a Int) -> Self::Output {
        self._div(other)
    }
}

impl<'a> Rem<&'a Int> for &'a Int {
    type Output = Result<Int, i16>;
    fn rem(self, other: &'a Int) -> Self::Output {
        self._modulo(other)
    }
}

impl Neg for Int {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Int::Big(b) => {
                Int::Big(-b)
            }
            Int::Small(s) => {
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

impl PartialOrd for Int {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use crate::compat::int_to_bigint;

        match (self, other) {
            (crate::foundation::Int::Big(a), crate::foundation::Int::Big(b)) => {
                return Some(a.cmp(b));
            }

            (crate::foundation::Int::Small(a), crate::foundation::Int::Small(b)) => {
                return Some(a.cmp(b));
            }

            (crate::foundation::Int::Small(_), crate::foundation::Int::Big(_))
            | (crate::foundation::Int::Big(_), crate::foundation::Int::Small(_)) => {
                let a_big = int_to_bigint(self);
                let b_big = int_to_bigint(other);
                return Some(a_big.cmp(&b_big));
            }
        }
    }
}

impl Display for Int {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if int_is_nan(self) {
            write!(f, "NaN")?;
            return Ok(());
        }
        if int_is_infinite(self) {
            let (_d, neg, _k) = int_to_parts(self);
            if neg {
                write!(f, "-Infinity")?;
            } else {
                write!(f, "Infinity")?;
            }
            return Ok(());
        }
        let (digits, neg, _k) = int_to_parts(self);
        if neg {
            write!(f, "-")?;
        }
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

impl<'a> Add<&'a Float> for &'a Float {
    type Output = Result<Float, i16>;
    fn add(self, other: &'a Float) -> Self::Output {
        self._add(other)
    }
}

impl<'a> Sub<&'a Float> for &'a Float {
    type Output = Result<Float, i16>;
    fn sub(self, other: &'a Float) -> Self::Output {
        self._sub(other)
    }
}

impl<'a> Mul<&'a Float> for &'a Float {
    type Output = Result<Float, i16>;
    fn mul(self, other: &'a Float) -> Self::Output {
        self._mul(other)
    }
}

impl<'a> Div<&'a Float> for &'a Float {
    type Output = Result<Float, i16>;
    fn div(self, other: &'a Float) -> Self::Output {
        self._div(other)
    }
}

impl<'a> Rem<&'a Float> for &'a Float {
    type Output = Result<Float, i16>;
    fn rem(self, other: &'a Float) -> Self::Output {
        self._modulo(other)
    }
}

impl<'a> Add<&'a Float> for Float {
    type Output = Result<Float, i16>;
    fn add(self, other: &'a Float) -> Self::Output {
        self._add(other)
    }
}

impl<'a> Add<Float> for &'a Float {
    type Output = Result<Float, i16>;
    fn add(self, other: Float) -> Self::Output {
        self._add(&other)
    }
}

impl<'a> Sub<&'a Float> for Float {
    type Output = Result<Float, i16>;
    fn sub(self, other: &'a Float) -> Self::Output {
        self._sub(other)
    }
}

impl<'a> Sub<Float> for &'a Float {
    type Output = Result<Float, i16>;
    fn sub(self, other: Float) -> Self::Output {
        self._sub(&other)
    }
}

impl<'a> Mul<&'a Float> for Float {
    type Output = Result<Float, i16>;
    fn mul(self, other: &'a Float) -> Self::Output {
        self._mul(other)
    }
}

impl<'a> Mul<Float> for &'a Float {
    type Output = Result<Float, i16>;
    fn mul(self, other: Float) -> Self::Output {
        self._mul(&other)
    }
}

impl<'a> Div<&'a Float> for Float {
    type Output = Result<Float, i16>;
    fn div(self, other: &'a Float) -> Self::Output {
        self._div(other)
    }
}

impl<'a> Div<Float> for &'a Float {
    type Output = Result<Float, i16>;
    fn div(self, other: Float) -> Self::Output {
        self._div(&other)
    }
}

impl<'a> Rem<&'a Float> for Float {
    type Output = Result<Float, i16>;
    fn rem(self, other: &'a Float) -> Self::Output {
        self._modulo(other)
    }
}

impl<'a> Rem<Float> for &'a Float {
    type Output = Result<Float, i16>;
    fn rem(self, other: Float) -> Self::Output {
        self._modulo(&other)
    }
}

impl Neg for Float {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let (m, e, neg, k) = float_to_parts(&self);
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
        use crate::compat::float_to_bigdecimal;

        match (self, other) {
            (Float::Big(a_bd), Float::Big(b_bd)) => {
                return Some(a_bd.normalized().cmp(&b_bd.normalized()));
            }

            (Float::Small(a_sf), Float::Small(b_sf)) => {
                let a_v = match a_sf {
                    crate::foundation::SmallFloat::F32(v) => *v as f64,
                    crate::foundation::SmallFloat::F64(v) => *v,
                };
                let b_v = match b_sf {
                    crate::foundation::SmallFloat::F32(v) => *v as f64,
                    crate::foundation::SmallFloat::F64(v) => *v,
                };
                return a_v.partial_cmp(&b_v);
            }

            (Float::Small(_), Float::Big(_)) | (Float::Big(_), Float::Small(_)) => {
                if let (Some(a_bd), Some(b_bd)) = (float_to_bigdecimal(self), float_to_bigdecimal(other)) {
                    return Some(a_bd.normalized().cmp(&b_bd.normalized()));
                }
                return None;
            }

            (Float::NaN, Float::NaN) => return Some(Ordering::Equal),
            (Float::NaN, _) | (_, Float::NaN) => return None,
            (Float::Infinity, Float::Infinity) => return Some(Ordering::Equal),
            (Float::NegInfinity, Float::NegInfinity) => return Some(Ordering::Equal),
            (Float::Infinity, _) => return Some(Ordering::Greater),
            (_, Float::Infinity) => return Some(Ordering::Less),
            (Float::NegInfinity, _) => return Some(Ordering::Less),
            (_, Float::NegInfinity) => return Some(Ordering::Greater),

            _ => {
                if let (Some(a_bd), Some(b_bd)) = (float_to_bigdecimal(self), float_to_bigdecimal(other)) {
                    return Some(a_bd.normalized().cmp(&b_bd.normalized()));
                }
                return None;
            }
        }
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

        if k == FloatKind::Recurring {
            if let Float::Recurring(ref bd) = *self {
                let s = bd.normalized().to_string();
                let parts: Vec<&str> = s.split('E').collect();
                let base = parts[0];
                let exp_from_e: i32 = if parts.len() == 2 {
                    parts[1].parse().unwrap_or(0)
                } else {
                    0
                };

                let (digits, exp_decimal) = if let Some(dot) = base.find('.') {
                    let mantissa = base[..dot].to_string() + &base[dot + 1..];
                    (
                        mantissa.trim_start_matches('0').to_string(),
                        -((base.len() - dot - 1) as i32),
                    )
                } else {
                    (base.trim_start_matches('0').to_string(), 0)
                };
                let final_exp = exp_decimal + exp_from_e;

                let (int_part, frac_part) = if digits.is_empty() {
                    ("0".to_string(), String::new())
                } else {
                    let point_pos = (digits.len() as i32) + final_exp;
                    if point_pos > 0 {
                        let pp = point_pos as usize;
                        if pp >= digits.len() {
                            (digits.clone(), String::new())
                        } else {
                            (digits[..pp].to_string(), digits[pp..].to_string())
                        }
                    } else {
                        (
                            "0".to_string(),
                            format!("{}{}", "0".repeat((-point_pos) as usize), digits),
                        )
                    }
                };

                let neg = base.starts_with('-');

                let max_check = frac_part.len().min(500);
                let frac = &frac_part[..max_check];

                let mut found: Option<(usize, usize)> = None;
                'outer: for rep_len in 1..=frac.len() {
                    for nonrep_len in 0..=frac.len().saturating_sub(rep_len) {
                        let start = nonrep_len;
                        if start + rep_len > frac.len() {
                            continue;
                        }
                        let rep = &frac[start..start + rep_len];
                        let mut ok = true;
                        let remaining = frac.len() - start;
                        if remaining < rep_len * 2 {
                            ok = false;
                        }
                        let mut i = start;
                        while i < frac.len() {
                            let take = (rep_len).min(frac.len() - i);
                            if &frac[i..i + take] != &rep[..take] {
                                ok = false;
                                break;
                            }
                            i += take;
                        }
                        if ok {
                            found = Some((nonrep_len, rep_len));
                            break 'outer;
                        }
                    }
                }

                if let Some((nonrep_len, rep_len)) = found {
                    let nonrep = &frac[..nonrep_len];
                    let rep = &frac[nonrep_len..nonrep_len + rep_len];
                    if neg {
                        write!(f, "-{}.", int_part)?;
                    } else {
                        write!(f, "{}.", int_part)?;
                    }
                    write!(f, "{}({})", nonrep, rep)?;
                    return Ok(());
                }
            }
        }

        let (mant, exp, neg, k) = float_to_parts(self);
        if neg {
            write!(f, "-")?;
        }
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
        if k == FloatKind::Irrational {
            write!(f, "...")?;
        }
        Ok(())
    }
}
