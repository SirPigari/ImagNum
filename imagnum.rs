/*
------- ImagNum Library -------
This library provides numeric types for Lucia programming language.

It's designed to handle any possible numeric value, including integers, floats, and complex numbers.

Refer to README.md

LICENSE: MIT License
2025 Lucia Programming Language
*/

#![doc = include_str!("CRATE_DOCS.md")]

/// Core foundation types and traits
#[path = "core/foundation.rs"]
pub mod foundation;

/// Implementations of traits for numeric types
#[path = "core/impls.rs"]
pub mod impls;

/// Mathematical functions and constants
#[path = "core/math.rs"]
pub mod math;

/// Operations for numeric types
#[path = "core/ops.rs"]
pub mod ops;

/// Functions for creating numeric types
#[path = "core/functions.rs"]
pub mod functions;

/// Compatibility layer for older versions (will be removed in future)
#[path = "core/compat.rs"]
pub mod compat;

/// Features module containing optional features
#[path = "core/features.rs"]
pub mod features;

pub use foundation::{Float, Int};
pub use functions::{create_complex, create_float, create_imaginary, create_int, create_irrational};

/// Macros for creating numbers
pub mod macros {
    pub use super::{float, int};
}
use math::{
    ERR_DIV_BY_ZERO, ERR_INFINITE_RESULT, ERR_INVALID_FORMAT, ERR_NEGATIVE_RESULT,
    ERR_NEGATIVE_SQRT, ERR_NUMBER_TOO_LARGE, ERR_UNIMPLEMENTED, ERR_WRONG_SYNTAX,
};
pub use crate::impls::{ApproxEq, IntoSmallFloat, IntoSmallInt};

/// Error codes and error handling functions
pub mod errors {
    use super::*;
    pub const UNIMPLEMENTED: i8 = ERR_UNIMPLEMENTED;
    pub const INVALID_FORMAT: i8 = ERR_INVALID_FORMAT;
    pub const DIV_BY_ZERO: i8 = ERR_DIV_BY_ZERO;
    pub const NEGATIVE_RESULT: i8 = ERR_NEGATIVE_RESULT;
    pub const NEGATIVE_SQRT: i8 = ERR_NEGATIVE_SQRT;
    pub const NUMBER_TOO_LARGE: i8 = ERR_NUMBER_TOO_LARGE;
    pub const INFINITE_RESULT: i8 = ERR_INFINITE_RESULT;
    pub const WRONG_SYNTAX: i8 = ERR_WRONG_SYNTAX;

    pub use super::functions::get_error_code;
    pub use super::functions::get_error_message;
}

#[cfg(feature = "random")]
#[cfg(not(target_arch = "wasm32"))]
#[doc = "Random number generation features (enabled with `features = [\"random\"]`)"]
pub mod random {
    pub use super::features::feature_rand::*;
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const COPYRIGHT: &str = "2025 Lucia Programming Language";
pub const LICENSE: &str = "MIT License";
pub const LICENSE_FULL: &str = include_str!("LICENSE");
pub const LICENSE_URL: &str = "https://opensource.org/licenses/MIT";
pub const REPOSITORY: &str = "https://github.com/SirPigari/imagnum";
pub const DOCUMENTATION: &str = "https://docs.rs/imagnum";
pub const AUTHORS: &str = "SirPigari <leonardmarkovic015@gmail.com>";

/// List of all features available in the crate
pub const FEATURES: &[&str] = &["serde", "random", "cli"];

/// List of enabled features in the current build
pub const ENABLED_FEATURES: &[&str] = &[
    #[cfg(feature = "serde")]
    "serde",
    #[cfg(feature = "random")]
    "random",
    #[cfg(feature = "cli")]
    "cli",
];
