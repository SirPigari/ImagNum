# ImagNum

**ImagNum** is a numeric types library for the **Lucia programming language**.

It provides a unified and extensible framework for representing and manipulating all types of numeric values — from simple integers to complex and imaginary numbers.

ImagNum supports **arbitrary-sized numbers** — integers and floats are not limited by built-in width types like `u64` or `f32`. Instead, both `Int` and `Float` are backed by string representations, allowing seamless handling of extremely large or precise values.

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

## Numeric Representations

The core number types are `Int` and `Float`.  
Both types use string-based storage for digits and mantissas to allow unbounded size and high precision.

`Irrational` is a classification used for floats with exponents beyond `i64` bounds, useful for symbolic math and infinite-scale values.

---

## Version

```txt
0.1.0
```

---

## Architecture Overview

```txt
imagnum/
├── imagnum.rs       # Public API surface
└── core/
    ├── foundation.rs  # Basic numeric types
    ├── impls.rs       # Internal utility implementations
    ├── ops.rs         # Operator implementations (via `impl`)
    └── complex.rs     # Complex and imaginary number types
```

---

## License

MIT License  
© 2025 Lucia Programming Language Project
