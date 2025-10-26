use crate::compat::{
    float_is_negative, float_is_neg_one, float_is_one, float_is_zero, float_kind,
    float_to_parts, int_is_infinite, int_is_nan, int_to_parts, int_to_string,
    make_float_from_parts,
};
use crate::foundation::{Float, FloatKind, Int};
use bigdecimal::BigDecimal;
use num_traits::ToPrimitive;
use std::cmp::{Ordering, PartialOrd};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{
    Add, AddAssign, BitAnd, BitOr, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, Shr, Sub, SubAssign,
};

impl Add for Int {
    type Output = Result<Self, i8>;

    fn add(self, other: Self) -> Self::Output {
        self._add(&other.clone())
    }
}

impl Sub for Int {
    type Output = Result<Self, i8>;

    fn sub(self, other: Self) -> Self::Output {
        self._sub(&other.clone())
    }
}

impl Mul for Int {
    type Output = Result<Self, i8>;

    fn mul(self, other: Self) -> Self::Output {
        self._mul(&other.clone())
    }
}

impl Div for Int {
    type Output = Result<Self, i8>;

    fn div(self, other: Self) -> Self::Output {
        self._div(&other.clone())
    }
}

impl Rem for Int {
    type Output = Result<Self, i8>;

    fn rem(self, other: Self) -> Self::Output {
        self._modulo(&other.clone())
    }
}

impl<'a> Add<&'a Int> for &'a Int {
    type Output = Result<Int, i8>;
    fn add(self, other: &'a Int) -> Self::Output {
        self._add(other)
    }
}

impl<'a> Add<&'a Int> for Int {
    type Output = Result<Int, i8>;
    fn add(self, other: &'a Int) -> Self::Output {
        self._add(other)
    }
}

impl<'a> Add<Int> for &'a Int {
    type Output = Result<Int, i8>;
    fn add(self, other: Int) -> Self::Output {
        self._add(&other)
    }
}

impl<'a> Sub<&'a Int> for Int {
    type Output = Result<Int, i8>;
    fn sub(self, other: &'a Int) -> Self::Output {
        self._sub(other)
    }
}

impl<'a> Sub<Int> for &'a Int {
    type Output = Result<Int, i8>;
    fn sub(self, other: Int) -> Self::Output {
        self._sub(&other)
    }
}

impl<'a> Mul<&'a Int> for Int {
    type Output = Result<Int, i8>;
    fn mul(self, other: &'a Int) -> Self::Output {
        self._mul(other)
    }
}

impl<'a> Mul<Int> for &'a Int {
    type Output = Result<Int, i8>;
    fn mul(self, other: Int) -> Self::Output {
        self._mul(&other)
    }
}

impl<'a> Div<&'a Int> for Int {
    type Output = Result<Int, i8>;
    fn div(self, other: &'a Int) -> Self::Output {
        self._div(other)
    }
}

impl<'a> Div<Int> for &'a Int {
    type Output = Result<Int, i8>;
    fn div(self, other: Int) -> Self::Output {
        self._div(&other)
    }
}

impl<'a> Rem<&'a Int> for Int {
    type Output = Result<Int, i8>;
    fn rem(self, other: &'a Int) -> Self::Output {
        self._modulo(other)
    }
}

impl<'a> Rem<Int> for &'a Int {
    type Output = Result<Int, i8>;
    fn rem(self, other: Int) -> Self::Output {
        self._modulo(&other)
    }
}

impl<'a> Sub<&'a Int> for &'a Int {
    type Output = Result<Int, i8>;
    fn sub(self, other: &'a Int) -> Self::Output {
        self._sub(other)
    }
}

impl<'a> Mul<&'a Int> for &'a Int {
    type Output = Result<Int, i8>;
    fn mul(self, other: &'a Int) -> Self::Output {
        self._mul(other)
    }
}

impl<'a> Div<&'a Int> for &'a Int {
    type Output = Result<Int, i8>;
    fn div(self, other: &'a Int) -> Self::Output {
        self._div(other)
    }
}

impl<'a> Rem<&'a Int> for &'a Int {
    type Output = Result<Int, i8>;
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
    type Output = Result<Self, i8>;

    fn add(self, other: Self) -> Self::Output {
        self._add(&other.clone())
    }
}

impl Sub for Float {
    type Output = Result<Self, i8>;

    fn sub(self, other: Self) -> Self::Output {
        self._sub(&other.clone())
    }
}

impl Mul for Float {
    type Output = Result<Self, i8>;

    fn mul(self, other: Self) -> Self::Output {
        self._mul(&other.clone())
    }
}

impl Div for Float {
    type Output = Result<Self, i8>;

    fn div(self, other: Self) -> Self::Output {
        self._div(&other.clone())
    }
}

impl Rem for Float {
    type Output = Result<Self, i8>;

    fn rem(self, other: Self) -> Self::Output {
        self._modulo(&other.clone())
    }
}

impl<'a> Add<&'a Float> for &'a Float {
    type Output = Result<Float, i8>;
    fn add(self, other: &'a Float) -> Self::Output {
        self._add(other)
    }
}

impl<'a> Sub<&'a Float> for &'a Float {
    type Output = Result<Float, i8>;
    fn sub(self, other: &'a Float) -> Self::Output {
        self._sub(other)
    }
}

impl<'a> Mul<&'a Float> for &'a Float {
    type Output = Result<Float, i8>;
    fn mul(self, other: &'a Float) -> Self::Output {
        self._mul(other)
    }
}

impl<'a> Div<&'a Float> for &'a Float {
    type Output = Result<Float, i8>;
    fn div(self, other: &'a Float) -> Self::Output {
        self._div(other)
    }
}

impl<'a> Rem<&'a Float> for &'a Float {
    type Output = Result<Float, i8>;
    fn rem(self, other: &'a Float) -> Self::Output {
        self._modulo(other)
    }
}

impl<'a> Add<&'a Float> for Float {
    type Output = Result<Float, i8>;
    fn add(self, other: &'a Float) -> Self::Output {
        self._add(other)
    }
}

impl<'a> Add<Float> for &'a Float {
    type Output = Result<Float, i8>;
    fn add(self, other: Float) -> Self::Output {
        self._add(&other)
    }
}

impl<'a> Sub<&'a Float> for Float {
    type Output = Result<Float, i8>;
    fn sub(self, other: &'a Float) -> Self::Output {
        self._sub(other)
    }
}

impl<'a> Sub<Float> for &'a Float {
    type Output = Result<Float, i8>;
    fn sub(self, other: Float) -> Self::Output {
        self._sub(&other)
    }
}

impl<'a> Mul<&'a Float> for Float {
    type Output = Result<Float, i8>;
    fn mul(self, other: &'a Float) -> Self::Output {
        self._mul(other)
    }
}

impl<'a> Mul<Float> for &'a Float {
    type Output = Result<Float, i8>;
    fn mul(self, other: Float) -> Self::Output {
        self._mul(&other)
    }
}

impl<'a> Div<&'a Float> for Float {
    type Output = Result<Float, i8>;
    fn div(self, other: &'a Float) -> Self::Output {
        self._div(other)
    }
}

impl<'a> Div<Float> for &'a Float {
    type Output = Result<Float, i8>;
    fn div(self, other: Float) -> Self::Output {
        self._div(&other)
    }
}

impl<'a> Rem<&'a Float> for Float {
    type Output = Result<Float, i8>;
    fn rem(self, other: &'a Float) -> Self::Output {
        self._modulo(other)
    }
}

impl<'a> Rem<Float> for &'a Float {
    type Output = Result<Float, i8>;
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

        if let Float::Complex(ref real, ref imag) = *self {
            if float_is_zero(imag) {
                return real.fmt(f);
            }
            
            if float_is_zero(real) {
                if float_is_one(imag) {
                    return write!(f, "i");
                } else if float_is_neg_one(imag) {
                    return write!(f, "-i");
                } else {
                    write!(f, "{}i", imag)?;
                    return Ok(());
                }
            }
            
            write!(f, "{}", real)?;
            
            let imag_neg = float_is_negative(imag);
            if imag_neg {
                write!(f, " - ")?;
                let abs_imag = Float::Big(BigDecimal::from(0))._sub(imag).unwrap_or_else(|_| *imag.clone());
                if float_is_one(&abs_imag) {
                    write!(f, "i")?;
                } else {
                    write!(f, "{}i", abs_imag)?;
                }
            } else {
                write!(f, " + ")?;
                if float_is_one(imag) {
                    write!(f, "i")?;
                } else {
                    write!(f, "{}i", imag)?;
                }
            }
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
        
        if k == FloatKind::Irrational || (exp >= -50 && exp <= 50) {
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

impl Int {
    pub fn _bitand(&self, other: &Int) -> Result<Int, i8> {
        use crate::compat::int_to_bigint;
        let a = int_to_bigint(self);
        let b = int_to_bigint(other);
        Ok(Int::Big(a & b))
    }

    pub fn _bitor(&self, other: &Int) -> Result<Int, i8> {
        use crate::compat::int_to_bigint;
        let a = int_to_bigint(self);
        let b = int_to_bigint(other);
        Ok(Int::Big(a | b))
    }

    pub fn _bitxor(&self, other: &Int) -> Result<Int, i8> {
        use crate::compat::int_to_bigint;
        let a = int_to_bigint(self);
        let b = int_to_bigint(other);
        Ok(Int::Big(a ^ b))
    }

    pub fn _xnor(&self, other: &Int) -> Result<Int, i8> {
        let xor = self._bitxor(other)?;
        Ok(xor._not())
    }

    pub fn _shl(&self, other: &Int) -> Result<Int, i8> {
        use crate::compat::int_to_bigint;
        use num_bigint::BigInt;
        use crate::math::ERR_NEGATIVE_RESULT;
        use crate::math::ERR_NUMBER_TOO_LARGE;
        let shift_big = int_to_bigint(other);
        if shift_big < BigInt::from(0) {
            return Err(ERR_NEGATIVE_RESULT);
        }
        if shift_big > BigInt::from(usize::MAX) {
            return Err(ERR_NUMBER_TOO_LARGE);
        }
        let shift = shift_big.to_usize().unwrap();
        let a = int_to_bigint(self);
        Ok(Int::Big(a << shift))
    }

    pub fn _shr(&self, other: &Int) -> Result<Int, i8> {
        use crate::compat::int_to_bigint;
        use num_bigint::BigInt;
        use crate::math::ERR_NEGATIVE_RESULT;
        use crate::math::ERR_NUMBER_TOO_LARGE;
        let shift_big = int_to_bigint(other);
        if shift_big < BigInt::from(0) {
            return Err(ERR_NEGATIVE_RESULT);
        }
        if shift_big > BigInt::from(usize::MAX) {
            return Err(ERR_NUMBER_TOO_LARGE);
        }
        let shift = shift_big.to_usize().unwrap();
        let a = int_to_bigint(self);
        Ok(Int::Big(a >> shift))
    }

    pub fn _not(&self) -> Int {
        use crate::compat::int_to_bigint;
        let a = int_to_bigint(self);
        Int::Big(!a)
    }

    pub fn xnor(&self, other: &Int) -> Result<Int, i8> {
        self._xnor(other)
    }
}

impl BitAnd for Int {
    type Output = Result<Self, i8>;
    fn bitand(self, other: Self) -> Self::Output {
        self._bitand(&other)
    }
}

impl<'a> BitAnd<&'a Int> for &'a Int {
    type Output = Result<Int, i8>;
    fn bitand(self, other: &'a Int) -> Self::Output {
        self._bitand(other)
    }
}

impl<'a> BitAnd<Int> for &'a Int {
    type Output = Result<Int, i8>;
    fn bitand(self, other: Int) -> Self::Output {
        self._bitand(&other)
    }
}

impl<'a> BitAnd<&'a Int> for Int {
    type Output = Result<Int, i8>;
    fn bitand(self, other: &'a Int) -> Self::Output {
        self._bitand(other)
    }
}

impl BitOr for Int {
    type Output = Result<Self, i8>;
    fn bitor(self, other: Self) -> Self::Output {
        self._bitor(&other)
    }
}

impl<'a> BitOr<&'a Int> for &'a Int {
    type Output = Result<Int, i8>;
    fn bitor(self, other: &'a Int) -> Self::Output {
        self._bitor(other)
    }
}

impl<'a> BitOr<Int> for &'a Int {
    type Output = Result<Int, i8>;
    fn bitor(self, other: Int) -> Self::Output {
        self._bitor(&other)
    }
}

impl<'a> BitOr<&'a Int> for Int {
    type Output = Result<Int, i8>;
    fn bitor(self, other: &'a Int) -> Self::Output {
        self._bitor(other)
    }
}

impl BitXor for Int {
    type Output = Result<Self, i8>;
    fn bitxor(self, other: Self) -> Self::Output {
        self._bitxor(&other)
    }
}

impl<'a> BitXor<&'a Int> for &'a Int {
    type Output = Result<Int, i8>;
    fn bitxor(self, other: &'a Int) -> Self::Output {
        self._bitxor(other)
    }
}

impl<'a> BitXor<Int> for &'a Int {
    type Output = Result<Int, i8>;
    fn bitxor(self, other: Int) -> Self::Output {
        self._bitxor(&other)
    }
}

impl<'a> BitXor<&'a Int> for Int {
    type Output = Result<Int, i8>;
    fn bitxor(self, other: &'a Int) -> Self::Output {
        self._bitxor(other)
    }
}

impl Not for Int {
    type Output = Self;
    fn not(self) -> Self::Output {
        self._not()
    }
}

impl<'a> Not for &'a Int {
    type Output = Int;
    fn not(self) -> Self::Output {
        self._not()
    }
}

impl Shl for Int {
    type Output = Result<Self, i8>;
    fn shl(self, other: Self) -> Self::Output {
        self._shl(&other)
    }
}

impl<'a> Shl<&'a Int> for &'a Int {
    type Output = Result<Int, i8>;
    fn shl(self, other: &'a Int) -> Self::Output {
        self._shl(other)
    }
}

impl<'a> Shl<Int> for &'a Int {
    type Output = Result<Int, i8>;
    fn shl(self, other: Int) -> Self::Output {
        self._shl(&other)
    }
}

impl<'a> Shl<&'a Int> for Int {
    type Output = Result<Int, i8>;
    fn shl(self, other: &'a Int) -> Self::Output {
        self._shl(other)
    }
}

impl Shr for Int {
    type Output = Result<Self, i8>;
    fn shr(self, other: Self) -> Self::Output {
        self._shr(&other)
    }
}

impl<'a> Shr<&'a Int> for &'a Int {
    type Output = Result<Int, i8>;
    fn shr(self, other: &'a Int) -> Self::Output {
        self._shr(other)
    }
}

impl<'a> Shr<Int> for &'a Int {
    type Output = Result<Int, i8>;
    fn shr(self, other: Int) -> Self::Output {
        self._shr(&other)
    }
}

impl<'a> Shr<&'a Int> for Int {
    type Output = Result<Int, i8>;
    fn shr(self, other: &'a Int) -> Self::Output {
        self._shr(other)
    }
}

impl Float {
    pub fn _bitand(&self, other: &Float) -> Result<Float, i8> {
        use crate::math::ERR_UNIMPLEMENTED;
        match (self, other) {
            (Float::Small(crate::foundation::SmallFloat::F32(a)), Float::Small(crate::foundation::SmallFloat::F32(b))) => {
                let a_bits = a.to_bits();
                let b_bits = b.to_bits();
                let res_bits = a_bits & b_bits;
                Ok(Float::Small(crate::foundation::SmallFloat::F32(f32::from_bits(res_bits))))
            }
            (Float::Small(crate::foundation::SmallFloat::F64(a)), Float::Small(crate::foundation::SmallFloat::F64(b))) => {
                let a_bits = a.to_bits();
                let b_bits = b.to_bits();
                let res_bits = a_bits & b_bits;
                Ok(Float::Small(crate::foundation::SmallFloat::F64(f64::from_bits(res_bits))))
            }
            (Float::Big(a), Float::Big(b)) => {
                let (a_mant, _) = a.as_bigint_and_exponent();
                let (b_mant, _) = b.as_bigint_and_exponent();
                let res_int = a_mant & b_mant;
                Ok(Float::Big(BigDecimal::from_bigint(res_int, 0)))
            }
            _ => Err(ERR_UNIMPLEMENTED),
        }
    }

    pub fn _bitor(&self, other: &Float) -> Result<Float, i8> {
        use crate::math::ERR_UNIMPLEMENTED;
        match (self, other) {
            (Float::Small(crate::foundation::SmallFloat::F32(a)), Float::Small(crate::foundation::SmallFloat::F32(b))) => {
                let a_bits = a.to_bits();
                let b_bits = b.to_bits();
                let res_bits = a_bits | b_bits;
                let res_f32 = f32::from_bits(res_bits);
                if res_f32.is_infinite() {
                    if res_f32.is_sign_positive() {
                        Ok(Float::Infinity)
                    } else {
                        Ok(Float::NegInfinity)
                    }
                } else if res_f32.is_nan() {
                    Ok(Float::NaN)
                } else {
                    Ok(Float::Small(crate::foundation::SmallFloat::F32(res_f32)))
                }
            }
            (Float::Small(crate::foundation::SmallFloat::F64(a)), Float::Small(crate::foundation::SmallFloat::F64(b))) => {
                let a_bits = a.to_bits();
                let b_bits = b.to_bits();
                let res_bits = a_bits | b_bits;
                let res_f64 = f64::from_bits(res_bits);
                if res_f64.is_infinite() {
                    if res_f64.is_sign_positive() {
                        Ok(Float::Infinity)
                    } else {
                        Ok(Float::NegInfinity)
                    }
                } else if res_f64.is_nan() {
                    Ok(Float::NaN)
                } else {
                    Ok(Float::Small(crate::foundation::SmallFloat::F64(res_f64)))
                }
            }
            (Float::Big(a), Float::Big(b)) => {
                let (a_mant, _) = a.as_bigint_and_exponent();
                let (b_mant, _) = b.as_bigint_and_exponent();
                let res_int = a_mant | b_mant;
                Ok(Float::Big(BigDecimal::from_bigint(res_int, 0)))
            }
            _ => Err(ERR_UNIMPLEMENTED),
        }
    }

    pub fn _bitxor(&self, other: &Float) -> Result<Float, i8> {
        use crate::math::ERR_UNIMPLEMENTED;
        match (self, other) {
            (Float::Small(crate::foundation::SmallFloat::F32(a)), Float::Small(crate::foundation::SmallFloat::F32(b))) => {
                let a_bits = a.to_bits();
                let b_bits = b.to_bits();
                let res_bits = a_bits ^ b_bits;
                let res_f32 = f32::from_bits(res_bits);
                if res_f32.is_infinite() {
                    if res_f32.is_sign_positive() {
                        Ok(Float::Infinity)
                    } else {
                        Ok(Float::NegInfinity)
                    }
                } else if res_f32.is_nan() {
                    Ok(Float::NaN)
                } else {
                    Ok(Float::Small(crate::foundation::SmallFloat::F32(res_f32)))
                }
            }
            (Float::Small(crate::foundation::SmallFloat::F64(a)), Float::Small(crate::foundation::SmallFloat::F64(b))) => {
                let a_bits = a.to_bits();
                let b_bits = b.to_bits();
                let res_bits = a_bits ^ b_bits;
                let res_f64 = f64::from_bits(res_bits);
                if res_f64.is_infinite() {
                    if res_f64.is_sign_positive() {
                        Ok(Float::Infinity)
                    } else {
                        Ok(Float::NegInfinity)
                    }
                } else if res_f64.is_nan() {
                    Ok(Float::NaN)
                } else {
                    Ok(Float::Small(crate::foundation::SmallFloat::F64(res_f64)))
                }
            }
            (Float::Big(a), Float::Big(b)) => {
                let (a_mant, _) = a.as_bigint_and_exponent();
                let (b_mant, _) = b.as_bigint_and_exponent();
                let res_int = a_mant ^ b_mant;
                Ok(Float::Big(BigDecimal::from_bigint(res_int, 0)))
            }
            _ => Err(ERR_UNIMPLEMENTED),
        }
    }

    pub fn _xnor(&self, other: &Float) -> Result<Float, i8> {
        let xor = self._bitxor(other)?;
        Ok(xor._not())
    }

    pub fn _shl(&self, shift: &Int) -> Result<Float, i8> {
        use crate::compat::int_to_bigint;
        use num_bigint::BigInt;
        use crate::math::ERR_NEGATIVE_RESULT;
        use crate::math::ERR_NUMBER_TOO_LARGE;
        let shift_big = int_to_bigint(shift);
        if shift_big < BigInt::from(0) {
            return Err(ERR_NEGATIVE_RESULT);
        }
        if shift_big > BigInt::from(usize::MAX) {
            return Err(ERR_NUMBER_TOO_LARGE);
        }
        let shift_usize = shift_big.to_usize().unwrap();
        match self {
            Float::Small(crate::foundation::SmallFloat::F32(a)) => {
                let bits = a.to_bits();
                let res_bits = bits << shift_usize;
                Ok(Float::Small(crate::foundation::SmallFloat::F32(f32::from_bits(res_bits))))
            }
            Float::Small(crate::foundation::SmallFloat::F64(a)) => {
                let bits = a.to_bits();
                let res_bits = bits << shift_usize;
                Ok(Float::Small(crate::foundation::SmallFloat::F64(f64::from_bits(res_bits))))
            }
            Float::Big(a) => {
                let (a_mant, _) = a.as_bigint_and_exponent();
                let res_int = a_mant << shift_usize;
                Ok(Float::Big(BigDecimal::from_bigint(res_int, 0)))
            }
            _ => Err(crate::math::ERR_UNIMPLEMENTED),
        }
    }

    pub fn _shr(&self, shift: &Int) -> Result<Float, i8> {
        use crate::compat::int_to_bigint;
        use num_bigint::BigInt;
        use crate::math::ERR_NEGATIVE_RESULT;
        use crate::math::ERR_NUMBER_TOO_LARGE;
        let shift_big = int_to_bigint(shift);
        if shift_big < BigInt::from(0) {
            return Err(ERR_NEGATIVE_RESULT);
        }
        if shift_big > BigInt::from(usize::MAX) {
            return Err(ERR_NUMBER_TOO_LARGE);
        }
        let shift_usize = shift_big.to_usize().unwrap();
        match self {
            Float::Small(crate::foundation::SmallFloat::F32(a)) => {
                let bits = a.to_bits();
                let res_bits = bits >> shift_usize;
                Ok(Float::Small(crate::foundation::SmallFloat::F32(f32::from_bits(res_bits))))
            }
            Float::Small(crate::foundation::SmallFloat::F64(a)) => {
                let bits = a.to_bits();
                let res_bits = bits >> shift_usize;
                Ok(Float::Small(crate::foundation::SmallFloat::F64(f64::from_bits(res_bits))))
            }
            Float::Big(a) => {
                let (a_mant, _) = a.as_bigint_and_exponent();
                let res_int = a_mant >> shift_usize;
                Ok(Float::Big(BigDecimal::from_bigint(res_int, 0)))
            }
            _ => Err(crate::math::ERR_UNIMPLEMENTED),
        }
    }

    pub fn _not(&self) -> Float {
        match self {
            Float::Small(crate::foundation::SmallFloat::F32(a)) => {
                let bits = a.to_bits();
                let res_f32 = f32::from_bits(!bits);
                if res_f32.is_infinite() {
                    if res_f32.is_sign_positive() {
                        Float::Infinity
                    } else {
                        Float::NegInfinity
                    }
                } else if res_f32.is_nan() {
                    Float::NaN
                } else {
                    Float::Small(crate::foundation::SmallFloat::F32(res_f32))
                }
            }
            Float::Small(crate::foundation::SmallFloat::F64(a)) => {
                let bits = a.to_bits();
                let res_f64 = f64::from_bits(!bits);
                if res_f64.is_infinite() {
                    if res_f64.is_sign_positive() {
                        Float::Infinity
                    } else {
                        Float::NegInfinity
                    }
                } else if res_f64.is_nan() {
                    Float::NaN
                } else {
                    Float::Small(crate::foundation::SmallFloat::F64(res_f64))
                }
            }
            Float::Big(a) => {
                let (a_mant, _) = a.as_bigint_and_exponent();
                let res_int = !a_mant;
                Float::Big(BigDecimal::from_bigint(res_int, 0))
            }
            Float::Infinity => Float::NegInfinity,
            Float::NegInfinity => Float::Infinity,
            Float::NaN => Float::NaN,
            _ => Float::NaN,
        }
    }

    pub fn xnor(&self, other: &Float) -> Result<Float, i8> {
        self._xnor(other)
    }
}

impl BitAnd for Float {
    type Output = Result<Self, i8>;
    fn bitand(self, other: Self) -> Self::Output {
        self._bitand(&other)
    }
}

impl<'a> BitAnd<&'a Float> for &'a Float {
    type Output = Result<Float, i8>;
    fn bitand(self, other: &'a Float) -> Self::Output {
        self._bitand(other)
    }
}

impl<'a> BitAnd<Float> for &'a Float {
    type Output = Result<Float, i8>;
    fn bitand(self, other: Float) -> Self::Output {
        self._bitand(&other)
    }
}

impl<'a> BitAnd<&'a Float> for Float {
    type Output = Result<Float, i8>;
    fn bitand(self, other: &'a Float) -> Self::Output {
        self._bitand(other)
    }
}

impl BitOr for Float {
    type Output = Result<Self, i8>;
    fn bitor(self, other: Self) -> Self::Output {
        self._bitor(&other)
    }
}

impl<'a> BitOr<&'a Float> for &'a Float {
    type Output = Result<Float, i8>;
    fn bitor(self, other: &'a Float) -> Self::Output {
        self._bitor(other)
    }
}

impl<'a> BitOr<Float> for &'a Float {
    type Output = Result<Float, i8>;
    fn bitor(self, other: Float) -> Self::Output {
        self._bitor(&other)
    }
}

impl<'a> BitOr<&'a Float> for Float {
    type Output = Result<Float, i8>;
    fn bitor(self, other: &'a Float) -> Self::Output {
        self._bitor(other)
    }
}

impl BitXor for Float {
    type Output = Result<Self, i8>;
    fn bitxor(self, other: Self) -> Self::Output {
        self._bitxor(&other)
    }
}

impl<'a> BitXor<&'a Float> for &'a Float {
    type Output = Result<Float, i8>;
    fn bitxor(self, other: &'a Float) -> Self::Output {
        self._bitxor(other)
    }
}

impl<'a> BitXor<Float> for &'a Float {
    type Output = Result<Float, i8>;
    fn bitxor(self, other: Float) -> Self::Output {
        self._bitxor(&other)
    }
}

impl<'a> BitXor<&'a Float> for Float {
    type Output = Result<Float, i8>;
    fn bitxor(self, other: &'a Float) -> Self::Output {
        self._bitxor(other)
    }
}

impl Not for Float {
    type Output = Self;
    fn not(self) -> Self::Output {
        self._not()
    }
}

impl<'a> Not for &'a Float {
    type Output = Float;
    fn not(self) -> Self::Output {
        self._not()
    }
}

impl Shl<Int> for Float {
    type Output = Result<Self, i8>;
    fn shl(self, other: Int) -> Self::Output {
        self._shl(&other)
    }
}

impl<'a> Shl<&'a Int> for &'a Float {
    type Output = Result<Float, i8>;
    fn shl(self, other: &'a Int) -> Self::Output {
        self._shl(other)
    }
}

impl<'a> Shl<Int> for &'a Float {
    type Output = Result<Float, i8>;
    fn shl(self, other: Int) -> Self::Output {
        self._shl(&other)
    }
}

impl<'a> Shl<&'a Int> for Float {
    type Output = Result<Float, i8>;
    fn shl(self, other: &'a Int) -> Self::Output {
        self._shl(other)
    }
}

impl Shr<Int> for Float {
    type Output = Result<Self, i8>;
    fn shr(self, other: Int) -> Self::Output {
        self._shr(&other)
    }
}

impl<'a> Shr<&'a Int> for &'a Float {
    type Output = Result<Float, i8>;
    fn shr(self, other: &'a Int) -> Self::Output {
        self._shr(other)
    }
}

impl<'a> Shr<Int> for &'a Float {
    type Output = Result<Float, i8>;
    fn shr(self, other: Int) -> Self::Output {
        self._shr(&other)
    }
}

impl<'a> Shr<&'a Int> for Float {
    type Output = Result<Float, i8>;
    fn shr(self, other: &'a Int) -> Self::Output {
        self._shr(other)
    }
}
