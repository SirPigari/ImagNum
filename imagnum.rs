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

pub use foundation::{Int, Float};
pub use functions::{
    create_int,
    create_float,
};
pub mod macros {
    pub use super::{int, float};
}
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
