use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use once_cell::sync::Lazy;
use std::str::FromStr;

pub static NAN: Lazy<Float> = Lazy::new(|| Float::NaN);
pub static INFINITY: Lazy<Float> = Lazy::new(|| Float::Infinity);
pub static NEG_INFINITY: Lazy<Float> = Lazy::new(|| Float::NegInfinity);
pub static FLOAT_ZERO: Lazy<Float> = Lazy::new(|| Float::new());
pub static INT_ZERO: Lazy<Int> = Lazy::new(|| Int::new());
pub static FLOAT_ONE: Lazy<Float> = Lazy::new(|| Float::from(1.0));
pub static INT_ONE: Lazy<Int> = Lazy::new(|| Int::from(1));

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Copy)]
pub enum FloatKind {
    NaN,
    Infinity,
    NegInfinity,
    Irrational,
    Recurring,
    Finite,
    Imaginary,
    Complex,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Copy)]
pub enum SmallInt {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    I128(i128),
    U128(u128),
    USize(usize),
    ISize(isize),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
pub enum SmallFloat {
    F32(f32),
    F64(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Int {
    Big(BigInt),
    Small(SmallInt),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Float {
    Small(SmallFloat),
    Big(BigDecimal),
    Irrational(BigDecimal),
    Recurring(BigDecimal),
    Complex(Box<Float>, Box<Float>),
    NaN,
    Infinity,
    NegInfinity,
}

impl Int {
    pub fn new() -> Self {
        Self::Big(BigInt::from(0))
    }
}

impl Float {
    pub fn new() -> Self {
        Self::Big(BigDecimal::from_str("0").unwrap())
    }
}
