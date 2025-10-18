#![allow(unused)]
use crate::foundation::{Float, Int};
use num_bigint::BigInt;
use bigdecimal::FromPrimitive;

#[cfg(feature = "serde")]
pub mod feature_serde {
    use serde::{Serialize, Deserialize};
    use serde::ser::{Serializer};
    use serde::de::{self, Deserializer, Visitor};
    use super::*;

    impl Serialize for Int {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let s = self.to_str();
            serializer.serialize_str(&s)
        }
    }

    impl<'de> Deserialize<'de> for Int {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct IntVisitor;

            impl<'de> Visitor<'de> for IntVisitor {
                type Value = Int;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("a string representing an integer")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    Int::from_str(v).map_err(de::Error::custom)
                }
            }

            deserializer.deserialize_str(IntVisitor)
        }
    }

    impl Serialize for Float {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let s = self.to_str();
            serializer.serialize_str(&s)
        }
    }

    impl<'de> Deserialize<'de> for Float {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct FloatVisitor;

            impl<'de> Visitor<'de> for FloatVisitor {
                type Value = Float;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("a string representing a floating-point number")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    Float::from_str(v).map_err(de::Error::custom)
                }
            }

            deserializer.deserialize_str(FloatVisitor)
        }
    }
}

#[cfg(feature = "random")]
pub mod feature_rand {
    use rand::{Rng, RngCore};
    use super::*;
    use bigdecimal::BigDecimal;
    use num_bigint::{BigInt, RandBigInt};
    use std::f64::consts::PI;

    // -----------------------
    // Random Float in [0, 1)
    // -----------------------
    /// Generates a random Float in the range [0, 1) with arbitrary precision.
    /// The precision is determined using a normal distribution centered around 12 with a standard deviation of 6.
    pub fn rand() -> Float {
        let mut rng = rand::rng();

        let min_bd = BigDecimal::from(0);
        let max_bd = BigDecimal::from(1);

        let frac_f64: f64 = rng.random_range(0.0..1.0) as f64;

        // --- Box-Muller normal distribution for precision ---
        let mean: f64 = 12.0;
        let std_dev: f64 = 6.0;
        let mut prec: f64;

        loop {
            let mut u1: f64 = rng.random_range(0.0..1.0) as f64;
            if u1 < 1e-10 { u1 = 1e-10; }
            let u2: f64 = rng.random_range(0.0..1.0) as f64;

            let z0: f64 = (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos();
            prec = mean + z0 * std_dev;

            if prec >= 0.0 && prec <= 42.0 {
                break;
            }
        }

        let prec: i64 = prec.round() as i64;

        let mut frac = BigDecimal::from_f64(frac_f64).unwrap_or(BigDecimal::from(0));
        frac = frac.with_scale(prec);

        let value = (max_bd - min_bd.clone()) * frac + min_bd;

        Float::Big(value)
    }

    // -----------------------
    // Random Int
    // -----------------------
    /// Generates a random Int in the inclusive range [min, max].
    pub fn randint(min: &Int, max: &Int) -> Int {
        let mut rng = rand::rng();

        let min_big = min.to_bigint().unwrap();
        let max_big = max.to_bigint().unwrap();
        let range = &max_big - &min_big + 1u32;
        let bits = range.bits();
        let mut r: BigInt;

        loop {
            let mut bytes = vec![0u8; (bits as usize + 7) / 8];
            rng.fill_bytes(&mut bytes);
            r = BigInt::from_bytes_le(num_bigint::Sign::Plus, &bytes);
            if &r < &range {
                break;
            }
        }
        r += min_big;
        Int::Big(r)
    }

    // -----------------------
    // Random Float (limited precision: word * 2)
    // -----------------------
    /// Generates a random Float in the range [min, max] with limited precision based on system word size.
    pub fn randfloat(min: &Float, max: &Float) -> Float {
        let word = std::mem::size_of::<usize>() as u64;
        let precision = word * 2;

        let mut rng = rand::rng();
        let (min_bd_opt, _) = min.to_bigdecimal();
        let (max_bd_opt, _) = max.to_bigdecimal();

        let min_bd = min_bd_opt.unwrap_or(BigDecimal::from(0));
        let max_bd = max_bd_opt.unwrap_or(BigDecimal::from(0));

        let frac = BigDecimal::from_f64(rng.random_range(0.0..1.0)).unwrap_or(BigDecimal::from(0));
        let result = min_bd.clone() + (max_bd - min_bd) * frac;

        Float::Big(result.with_scale(precision as i64))
    }

    // -----------------------
    // Random Decimal (arbitrary precision)
    // -----------------------
    /// Generates a random Float in the range [min, max] with specified precision.
    pub fn randdecimal(min: &Float, max: &Float, precision: u64) -> Float {
        let mut rng = rand::rng();
        let (min_bd_opt, _) = min.to_bigdecimal();
        let (max_bd_opt, _) = max.to_bigdecimal();

        let min_bd = min_bd_opt.unwrap_or(BigDecimal::from(0));
        let max_bd = max_bd_opt.unwrap_or(BigDecimal::from(0));

        let frac_val: f64 = rng.random_range(0.0..1.0);
        let frac = BigDecimal::from_f64(frac_val).unwrap_or(BigDecimal::from(0));
        let result = min_bd.clone() + (max_bd - min_bd) * frac;

        Float::Big(result.with_scale(precision as i64))
    }

    // -----------------------
    // Random Complex Float
    // -----------------------
    /// Generates a random Complex Float where both real and imaginary parts are in the range [min, max].
    pub fn randcomplex(min: &Float, max: &Float) -> Float {
        let real = randfloat(min, max);
        let imag = randfloat(min, max);
        Float::complex(real, imag)
    }

    // -----------------------
    // Random Real (mostly Big, sometimes Recurring/Irrational)
    // -----------------------
    /// Generates a random Float in the range [min, max].
    pub fn randreal(min: &Float, max: &Float) -> Float {
        let mut rng = rand::rng();
        let (min_bd_opt, _) = min.to_bigdecimal();
        let (max_bd_opt, _) = max.to_bigdecimal();
        let min_bd = min_bd_opt.unwrap_or(BigDecimal::from(0));
        let max_bd = max_bd_opt.unwrap_or(BigDecimal::from(0));

        let frac = BigDecimal::from_f64(rng.random_range(0.0..1.0)).unwrap_or(BigDecimal::from(0));
        let value = (max_bd - min_bd.clone()) * frac + min_bd;

        // 80% Big, 10% Recurring, 10% Irrational
        let choice = rng.random_range(0..100);
        match choice {
            0..=79 => Float::Big(value),
            80..=89 => Float::Recurring(value),
            _ => Float::Irrational(value),
        }
    }
}

