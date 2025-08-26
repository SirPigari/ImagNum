# Changelog

All notable changes to this project are documented in this file.

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

- Per project constraints, `core/foundation.rs` remained unchanged; the migration was implemented via compatibility helpers and refactors elsewhere.
- Floating-point operations that produce irrational results are truncated to exactly 137 decimal places and marked as `Float::Irrational`.

