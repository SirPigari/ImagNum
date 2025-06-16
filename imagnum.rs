/*
------- ImagNum Library -------
This library provides numeric types for Lucia programming language.

It's designed to handle any possible numeric value, including integers, floats, and complex numbers.

Refer to README.md

LICENSE: MIT License
2025 Lucia Programming Language
*/

#[path = "core/foundation.rs"]
pub mod foundation;

#[path = "core/impls.rs"]
pub mod impls;

#[path = "core/math.rs"]
pub mod math;

#[path = "core/ops.rs"]
pub mod ops;

#[path = "core/functions.rs"]
pub mod functions;

// #[path = "core/complex.rs"]
// pub mod complex;

pub use foundation::{Int, Float, NumberKind};
pub use functions::{
    create_int,
    create_float,
};
use math::{
    ERR_UNIMPLEMENTED,
    ERR_INVALID_FORMAT,
    ERR_DIV_BY_ZERO,
    ERR_NEGATIVE_RESULT,
    ERR_NEGATIVE_SQRT,
    ERR_NUMBER_TOO_LARGE,
    ERR_INFINITE_RESULT,
};
pub mod errors {
    use super::*;
    pub const UNIMPLEMENTED: i16 = ERR_UNIMPLEMENTED;
    pub const INVALID_FORMAT: i16 = ERR_INVALID_FORMAT;
    pub const DIV_BY_ZERO: i16 = ERR_DIV_BY_ZERO;
    pub const NEGATIVE_RESULT: i16 = ERR_NEGATIVE_RESULT;
    pub const NEGATIVE_SQRT: i16 = ERR_NEGATIVE_SQRT;
    pub const NUMBER_TOO_LARGE: i16 = ERR_NUMBER_TOO_LARGE;
    pub const INFINITE_RESULT: i16 = ERR_INFINITE_RESULT;

    pub use super::functions::get_error_message;
    pub use super::functions::get_error_code;
}
// pub use ops::{AddOps, SubOps, MulOps, DivOps, RemOps, NegOps};
// pub use complex::{Complex, ComplexFloat, ComplexInt};


// ------------------------ Tests ------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_int() {
        let int = create_int("123");
        assert_eq!(int.digits, "123");
        assert!(!int.negative);
        assert_eq!(int.kind, NumberKind::Finite);

        let neg_int = create_int("-456");
        assert_eq!(neg_int.digits, "456");
        assert!(neg_int.negative);
        assert_eq!(neg_int.kind, NumberKind::Finite);

        let nan_int = create_int("NaN");
        assert_eq!(nan_int.digits, "");
        assert!(!nan_int.negative);
        assert_eq!(nan_int.kind, NumberKind::NaN);

        let inf_int = create_int("Infinity");
        assert_eq!(inf_int.digits, "");
        assert!(!inf_int.negative);
        assert_eq!(inf_int.kind, NumberKind::Infinity);

        let neg_inf_int = create_int("-Infinity");
        assert_eq!(neg_inf_int.digits, "");
        assert!(neg_inf_int.negative);
        assert_eq!(neg_inf_int.kind, NumberKind::NegInfinity);
    }

    #[test]
    fn test_create_float() {
        let float = create_float("123.456");
        assert_eq!(float.mantissa, "123456");
        assert_eq!(float.exponent, -3);
        assert!(!float.negative);
        assert_eq!(float.kind, NumberKind::Finite);

        let neg_float = create_float("-789.012");
        assert_eq!(neg_float.mantissa, "789012");
        assert_eq!(neg_float.exponent, -3);
        assert!(neg_float.negative);
        assert_eq!(neg_float.kind, NumberKind::Finite);

        let nan_float = create_float("NaN");
        assert_eq!(nan_float.mantissa, "");
        assert_eq!(nan_float.exponent, 0);
        assert!(!nan_float.negative);
        assert_eq!(nan_float.kind, NumberKind::NaN);

        let inf_float = create_float("Infinity");
        assert_eq!(inf_float.mantissa, "");
        assert_eq!(inf_float.exponent, 0);
        assert!(!inf_float.negative);
        assert_eq!(inf_float.kind, NumberKind::Infinity);

        let neg_inf_float = create_float("-Infinity");
        assert_eq!(neg_inf_float.mantissa, "");
        assert_eq!(neg_inf_float.exponent, 0);
        assert!(neg_inf_float.negative);
        assert_eq!(neg_inf_float.kind, NumberKind::NegInfinity);
    }

    #[test]
    fn test_int_operations_extended() {
        let a = create_int("123");
        let b = create_int("456");
    
        let result_mul = a.mul(&b).unwrap();
        assert_eq!(result_mul.digits, "56088");
        assert!(!result_mul.negative);
        assert_eq!(result_mul.kind, NumberKind::Finite);
    
        let result_div = b.div(&a).unwrap();
        assert_eq!(result_div.digits, "3");
        assert!(!result_div.negative);
        assert_eq!(result_div.kind, NumberKind::Finite);
    
        let result_mod = b.modulo(&a).unwrap();
        assert_eq!(result_mod.digits, "87");
        assert!(!result_mod.negative);
        assert_eq!(result_mod.kind, NumberKind::Finite);
    
        let c = create_int("2");
        let result_pow = c.pow(&create_int("10")).unwrap();
        assert_eq!(result_pow.digits, "1024");
        assert!(!result_pow.negative);
        assert_eq!(result_pow.kind, NumberKind::Finite);
    }
    
    #[test]
    fn test_float_operations_extended() {
        let a = create_float("123.456");
        let b = create_float("78.9");
    
        let result_mul = a.mul(&b).unwrap();
        assert_eq!(result_mul.exponent, -6);
        assert!(!result_mul.negative);
        assert_eq!(result_mul.kind, NumberKind::Finite);
        assert!(result_mul.mantissa.starts_with("97406784"));
    
        let result_div = a.div(&b).unwrap();
        assert_eq!(result_div.exponent, 0);
        assert!(!result_div.negative);
        assert_eq!(result_div.kind, NumberKind::Finite);
    
        let exp = create_float("2");
        let result_pow = a.pow(&exp).unwrap();
        let approx = result_pow.to_f64().unwrap();
        assert!((approx - 123.456f64.powf(2.0)).abs() < 1e-6);
    
        let result_sqrt = a.sqrt().unwrap();
        let approx_sqrt = result_sqrt.to_f64().unwrap();
        assert!((approx_sqrt - 123.456f64.sqrt()).abs() < 1e-6);
    
        let neg_float = create_float("-1.23");
        assert!(neg_float.sqrt().is_err());
    }
}
