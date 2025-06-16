use once_cell::sync::Lazy;

pub static NAN_INT: Lazy<Int> = Lazy::new(|| Int { digits: String::new(), negative: false, kind: NumberKind::NaN });
pub static NAN_FLOAT: Lazy<Float> = Lazy::new(|| Float { mantissa: String::new(), exponent: 0, negative: false, kind: NumberKind::NaN });
pub static INFINITY_FLOAT: Lazy<Float> = Lazy::new(|| Float { mantissa: String::new(), exponent: 0, negative: false, kind: NumberKind::Infinity });
pub static INFINITY_INT: Lazy<Int> = Lazy::new(|| Int { digits: String::new(), negative: false, kind: NumberKind::Infinity });
pub static NEG_INFINITY_FLOAT: Lazy<Float> = Lazy::new(|| Float { mantissa: String::new(), exponent: 0, negative: true, kind: NumberKind::NegInfinity });
pub static NEG_INFINITY_INT: Lazy<Int> = Lazy::new(|| Int { digits: String::new(), negative: true, kind: NumberKind::NegInfinity });

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Copy)]
pub enum NumberKind {
    NaN,
    Infinity,
    NegInfinity,
    Irrational,
    Finite,
    Imaginary,
    Complex,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Int {
    pub digits: String,
    pub negative: bool,
    pub kind: NumberKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Float {
    pub mantissa: String,
    pub exponent: i32,
    pub negative: bool,
    pub kind: NumberKind,
}

impl Int {
    pub fn new(digits: String, negative: bool, kind: NumberKind) -> Self {
        Self { digits, negative, kind }
    }
}

impl Float {
    pub fn new(mantissa: String, exponent: i32, negative: bool, kind: NumberKind) -> Self {
        Self { mantissa, exponent, negative, kind }
    }
}
