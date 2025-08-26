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

#[path = "core/compat.rs"]
pub mod compat;

// #[path = "core/complex.rs"]
// pub mod complex;

pub use foundation::{Float, Int};
pub use functions::{create_float, create_int};
pub mod macros {
    pub use super::{float, int};
}
use math::{
    ERR_DIV_BY_ZERO, ERR_INFINITE_RESULT, ERR_INVALID_FORMAT, ERR_NEGATIVE_RESULT,
    ERR_NEGATIVE_SQRT, ERR_NUMBER_TOO_LARGE, ERR_UNIMPLEMENTED,
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

    pub use super::functions::get_error_code;
    pub use super::functions::get_error_message;
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const COPYRIGHT: &str = "2025 Lucia Programming Language";
pub const LICENSE: &str = "MIT License";
