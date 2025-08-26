use crate::compat::{
    float_kind, float_to_parts, int_is_infinite, int_is_nan, int_to_parts, int_to_string,
    make_float_from_parts,
};
use crate::foundation::{Float, FloatKind, Int};
// (no unused bigdecimal helpers/imports here)
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
        if self_neg && !other_neg {
            return Some(Ordering::Less);
        }
        if !self_neg && other_neg {
            return Some(Ordering::Greater);
        }
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
        let (self_man, self_exp, self_neg, _k1) = float_to_parts(self);
        let (other_man, other_exp, other_neg, _k2) = float_to_parts(other);
        if self_neg && !other_neg {
            return Some(Ordering::Less);
        }
        if !self_neg && other_neg {
            return Some(Ordering::Greater);
        }
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

        // Special formatting for recurring rationals: show smallest repeating cycle like 0.(3) or 0.1(6)
        if k == FloatKind::Recurring {
            // `self` is a &Float; destructure without moving.
            if let Float::Recurring(ref bd) = *self {
                // Convert BigDecimal to a normalized decimal string and reconstruct digits
                let s = bd.normalized().to_string();
                let parts: Vec<&str> = s.split('E').collect();
                let base = parts[0];
                let exp_from_e: i32 = if parts.len() == 2 {
                    parts[1].parse().unwrap_or(0)
                } else {
                    0
                };

                // Build a digits string (mantissa without dot) and compute final exponent
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

                // Determine integer and fractional parts using final_exp
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

                // track negativity
                let neg = base.starts_with('-');

                // Limit analysis length to avoid pathological long scans
                let max_check = frac_part.len().min(500);
                let frac = &frac_part[..max_check];

                // Find smallest repeating cycle: try every possible nonrepeating prefix length (p)
                // and repeating cycle length (c). Prefer smallest c, then smallest p.
                let mut found: Option<(usize, usize)> = None;
                'outer: for rep_len in 1..=frac.len() {
                    for nonrep_len in 0..=frac.len().saturating_sub(rep_len) {
                        // candidate repeating cycle
                        let start = nonrep_len;
                        if start + rep_len > frac.len() {
                            continue;
                        }
                        let rep = &frac[start..start + rep_len];
                        // verify that the remainder of `frac` starting at `start` is consistent
                        // with rep repeated (last repetition may be truncated)
                        let mut ok = true;
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
                // fallback: let the generic formatter below render it
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
