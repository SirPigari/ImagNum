# Changelog

All notable changes to this project are documented in this file.

## [0.2.11] - 2025-8-29

- Bugfix: `to_usize` on `Int` now allows zero 

## [0.2.10] - 2025-8-29

- Fixed recurring parsing

## [0.2.9] - 2025-8-29

- Parsing: support recurring-decimal notation in `Float` constructors (e.g. "0.(9)", "-1.2(34)").
- Equality: numeric equality now treats recurring decimals by value (e.g. `0.(9) == 1`, `0.4(9) == 0.5`).
- Display: `Float::to_str` normalizes recurring values that equal integers or terminating decimals (prints "1" for `0.(9)` and "0.5" for `0.4(9)`).
- Tests: added `tests/recurring_equality.rs` to validate recurring parsing, equality, and formatting.
- Internals: implemented numeric `PartialEq` for `Float` and cross-type equality with `Int`.

## [0.2.8] - 2025-8-29

- Added `is_recurring` and `is_irrational` to `Float` types.

## [0.2.7] - 2025-08-27

- Small optimizations

## [0.2.6] - 2025-08-27

- Mixed reference/value operator implementations for `Int` and `Float` (`a + &b`, `&a + b`, and the same for `-`, `*`, `/`, `%`).
- Unit tests: `tests/mixed_ref_value_ops.rs` to exercise mixed `&`/value operator forms.

## [0.2.5] - 2025-08-27

- API: added reference-based operator implementations for `Int` and `Float` so expressions like `&a + &b` call arithmetic without cloning (Add/Sub/Mul/Div/Rem for `&Int` and `&Float`).
- Tests: added `tests/ref_ops.rs` verifying `&Int` and `&Float` ops (add/sub/mul/div/rem) behave as expected.

## [0.2.4] - 2025-08-26

- Display: recurring decimal results now render using the shortest repeating-cycle notation (example: `1/3` prints as `0.(3)`). The display logic reconstructs digits from the stored `BigDecimal` and finds the minimal repeating cycle at format-time.
- Cleanup: fixed compiler warnings in `core/ops.rs` (removed unused imports and eliminated unused/mutable variable warnings) to keep builds clean.
 - Tests: added `tests/display_format.rs` covering recurring and terminating display cases (e.g. `1/3 -> 0.(3)`, `1/8 -> 0.125`) and preservation after arithmetic.
 - CI: added GitHub Actions workflow `.github/workflows/imagnum.yml` to run `cargo build` and `cargo test` on push and pull requests to `main`.

## [0.2.3] - 2025-08-26

- Math: added BigDecimal rational-exponent support in `core/math.rs` via `pow_bigdecimal_rational` (with helpers `bigdecimal_pow_integer` and `bigdecimal_nth_root`) to compute base^(num/den) without converting to f64 when possible.
- Tests: added `tests/pow_bigdecimal.rs` covering rational-power cases (e.g. cube roots) to verify correctness of the new path.
- Dependencies: added `num-integer` to provide integer utilities used by the rational conversion and GCD reduction.

## [0.2.2] - 2025-08-26

- Compatibility: `core/compat.rs::int_to_string` now returns digits only (leading `-` trimmed) so internal math helpers receive unsigned digit strings consistently.
- Bugfix: corrected integer addition edge-case so `(-A) + B` computes correctly.
- Tests: added `tests/negative_numbers.rs` covering negative integer/float arithmetic and parsing; full test-suite verified green after changes.

## [0.2.1] - 2025-08-26

- Added `.is_negative()` methods on `Int` and `Float` to expose negative-sign checks directly on the enum types.
- Replaced internal `float_is_negative` compatibility helper with the new methods across the codebase.
- Documentation: updated `README.md` to document the `.is_negative()` API and minor usage examples.
- Tests: updated/added tests to cover the new API surface; full test-suite verified green after the change.

## [0.2.0] - 2025-08-25

- Internal migration to an enum-based numeric foundation (`Int`, `Float`, `FloatKind`) while preserving the public API and REPL behavior.
- Introduced a compatibility layer (`core/compat.rs`) so callers and the public API remain unchanged while internals were refactored.
- Rewrote numeric core to use `num-bigint` and `bigdecimal` for arbitrary-size integer and decimal arithmetic (`core/math.rs`).
- Added transcendental helpers (sin, cos, tan, ln, exp, log10) implemented via a controlled approximation flow (f64 -> BigDecimal) and truncated to 137 decimal places when results are irrational.
- Implemented square-root helpers for `Int` and `Float` that return finite or `Float::Irrational` results truncated to 137 decimals when needed.
- Preserved small-number variants (`SmallInt` / `SmallFloat`) and only promote to big representations on overflow.
- Added comprehensive tests under `tests/` (transcendental, small types, overflow, and broad feature coverage).
- Updated `README.md` with examples and migration notes.

### Notes

- Floating-point operations that produce irrational results are truncated to exactly 137 decimal places and marked as `Float::Irrational`.