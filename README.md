# ImagNum

**ImagNum** is a numeric types library for the **Lucia programming language**.

It provides a unified and extensible framework for representing and manipulating all types of numeric values - from simple integers to complex and imaginary numbers.

ImagNum supports **arbitrary-sized numbers** - integers and floats are not limited by built-in width types like `u64` or `f32`. Instead, both `Int` and `Float` are backed by string representations, allowing seamless handling of extremely large or precise values.

---

## Features

- Custom numeric types: `Int`, `Float`, `Complex`, etc.
- Universal number classification with `NumberKind`
- Operator overloading via `impl` blocks
- Supports symbolic values: `NaN`, `Infinity`, `NegInfinity`
- Designed for symbolic, algebraic, and numeric computation

---

## Number Classification

Every numeric value is categorized using the `NumberKind` enum.  
It includes classifications like `Rational`, `Irrational`, `Imaginary`, and `Complex`, alongside special values like `NaN` and infinities.

This structure enables precise behavior during evaluation and transformation.

---

## ImagNum

ImagNum is a numeric-types library used by the Lucia language. It now uses an enum-based, ergonomic API for integers and floats while preserving high precision and very large numbers.

Key points
- Primary types: `Int` and `Float` (defined in `core/foundation.rs`).
- Small and big variants: `Int::Small` / `Int::Big`, `Float::Small` / `Float::Big` plus `Float::Irrational`, `Float::NaN`, `Float::Infinity`, `Float::NegInfinity`, and `Float::Complex`.
- `FloatKind` classifies float variants (Finite, Irrational, NaN, etc.).
- Transcendental functions and sqrt produce `Float::Irrational` when results are irrational; irrational results are truncated to 137 decimal places.

Goals of this README
- Quick reference for the new public API.
- Short usage examples.

## Public constructors
- `create_int(&str) -> Int` - parse integer-like strings (rejects floats/NaN/Infinity for Int).
- `create_float(&str) -> Float` - parse floats, `NaN`, `Infinity`, `-Infinity`, and imaginary forms like `3i`.

These constructors are exposed at the crate root. Example:

```rust
use imagnum::{create_int, create_float, Int, Float};

let i: Int = create_int("123");
let f: Float = create_float("3.14");
```

## Useful methods (on `Int`)
- `is_negative(&self) -> bool` - true when negative.
- `_add`, `_sub`, `_mul`, `_div`, `_modulo`, `pow`, `sqrt`, `abs` - arithmetic operations (return `Result`).
- `to_f64`, `to_i64`, `to_i128`, `to_usize` - conversion helpers.
- `is_zero`, `is_nan`, `is_infinity` - predicates.

Example

```rust
let a = create_int("2");
let b = create_int("3");
let sum = a._add(&b).unwrap();
assert!(!sum.is_negative());
```

## Useful methods (on `Float`)
- `is_negative(&self) -> bool` - negative sign.
- `is_zero`, `is_nan`, `is_infinity` - predicates.
- `to_f64`, `to_int` - conversions (with error handling for NaN/Infinity/etc.).
- `_add`, `_sub`, `_mul`, `_div`, `_modulo`, `_pow`, `sqrt`, `abs`, `round`, `truncate`, `normalize` - math operations.
- Transcendental wrappers: `sin`, `cos`, `tan`, `ln`, `exp`, `log`, `floor`, `ceil`.

Important: when a float-producing operation yields an irrational result (transcendental or non-terminating roots), the value is truncated to 137 decimal places and the `Float` variant is set to `Float::Irrational`. Use `make_irrational()` to mark an existing `Float` as irrational explicitly.

Example

```rust
let x = create_float("2");
let s = x.sqrt().unwrap();
match s {
    Float::Irrational(bd) => println!("sqrt(2) (truncated): {}", bd),
    Float::Big(bd) => printnl!("exact: {}", bd),
    _ => println!("other"),
}
```

## Small vs Big behavior
- Small variants exist for performance (e.g., `SmallInt::I32`, `SmallFloat::F64`). The library preserves small variants when results fit; on overflow or when higher precision is needed it promotes to `Big` variants.

## Compatibility helpers
- A `core/compat.rs` compatibility layer provides helpers such as `int_to_parts`, `float_to_parts`, `make_int_from_parts`, and `make_float_from_parts` used internally during the migration. You generally should use the public constructors and methods above.

## Tests
- The repository includes integration tests in `tests/` covering arithmetic, transcendental functions, truncation to 137 decimals, and overflow/promotion behavior.

## Contributing
- Use `cargo test` to run the test-suite after making changes.

## License
MIT License - © 2025 Lucia Programming Language Project
