# ImagNum

A Rust library for arbitrary-precision numbers. Built for the Lucia programming language, it handles integers, floats, complex numbers, and more. Think of it as a math backend that other tools can build on top of.

Includes a basic REPL (`imagnum-cli`) that you can disable with feature flags.

## Quick Start

```rust
use imagnum::{create_int, create_float};

fn main() -> Result<(), i8> {
    let big_num = create_int("12345678901234567890");
    let pi_approx = create_float("3.14159");

    let result = (big_num + create_int("1"))?;
    println!("{}", result); // 12345678901234567891
    Ok(())
}
```

## Number Types

### Int
Arbitrary-precision integers. Uses small optimized types when possible, falls back to BigInt for huge numbers.

```rust
use imagnum::{create_int, Int};

// Small numbers are optimized
let small = Int::new_small(42i32);  // Uses SmallInt::I32

// Big numbers use BigInt
let big = create_int("999999999999999999999");  // Uses BigInt
```

### Float
Handles everything from simple decimals to complex numbers and irrationals.

```rust
use imagnum::{create_float, create_irrational, create_complex, create_imaginary};

// Regular decimals
let decimal = create_float("3.14159");

// Irrational numbers (keeps exact representation)
let pi = create_irrational("3.141592653589793238462643383279502884197");

// Complex numbers
let complex = create_complex("3", "4");  // 3 + 4i
let imaginary = create_imaginary();       // i

// Special values
let inf = create_float("inf");
let nan = create_float("nan");

// Recurring decimals
let third = create_float("0.3(3)");  // 0.333...
```

## Math Operations

All operations return `Result<T, i8>` where the i8 is an error code.

### Basic Arithmetic

```rust
use imagnum::{create_int, create_float};

let a = create_int("10");
let b = create_int("3");

let sum = a.clone() + b.clone();     // 13
let diff = a.clone() - b.clone();    // 7
let product = a.clone() * b.clone(); // 30
let quotient = a.clone() / b.clone(); // 3
let remainder = a.clone() % b.clone(); // 1
let power = a.pow(&b);               // 10^3 = 1000
```

Same works for floats, plus you get more functions:

```rust
use imagnum::create_float;

fn main() -> Result<(), i8> {
    let x = create_float("1.5");

    let sqrt_x = x.sqrt()?;     // √1.5
    let sin_x = x.sin()?;       // sin(1.5)
    let exp_x = x.exp()?;       // e^1.5
    let ln_x = x.ln()?;         // ln(1.5)
    let floor_x = x.floor()?;   // floor(1.5) = 1
    let round_x = x.round(2);   // Round to 2 decimal places
    Ok(())
}
```

## Creating Numbers

### From Strings
```rust
use imagnum::{create_int, create_float, create_irrational, create_complex};

// Integers
let int1 = create_int("123");
let int2 = create_int("-456");

// Floats
let float1 = create_float("3.14159");
let float2 = create_float("2.718e5");  // Scientific notation

// Special syntax
let complex1 = create_float("3+4i");   // Alternative to create_complex
let hex = create_int("0xFF");          // Hex
let binary = create_int("0b1010");     // Binary
let octal = create_int("0o777");       // Octal
```

### From Rust Types
```rust
use imagnum::{Int, Float};

let from_i32 = Int::new_small(42i32);
let from_f64 = Float::new_small(3.14f64);
```

### Macros
```rust
use imagnum::{int, float, create_int, create_float};

let num1 = int!("123456789");
let num2 = float!("3.141592653589793");
```

## Error Handling

Operations can fail. Error codes are `i8` values:

```rust
use imagnum::errors::*;
use imagnum::{create_int, functions::get_error_message};

fn example() -> Result<(), i8> {
    let result = create_int("10") / create_int("0"); // This will fail
    match result {
        Ok(val) => println!("Got: {}", val),
        Err(DIV_BY_ZERO) => println!("Can't divide by zero"),
        Err(INVALID_FORMAT) => println!("Invalid number format!"),
        Err(code) => println!("Error {}: {}", code, get_error_message(code)),
    }
    Ok(())
}
```

Common errors:
- `-1`: UNIMPLEMENTED - Feature not implemented
- `1`: INVALID_FORMAT - Bad number format
- `2`: DIV_BY_ZERO - Division by zero
- `3`: NEGATIVE_RESULT - Unexpected negative result
- `4`: NEGATIVE_SQRT - Square root of negative number (deprecated)
- `5`: NUMBER_TOO_LARGE - Number too big to handle
- `6`: INFINITE_RESULT - Result is infinite
- `7`: WRONG_SYNTAX - Syntax error

## Optional Features

### Random Numbers
Add `features = ["random"]` to get random number functions.

```rust
// With features = ["random"] enabled:
use imagnum::random::*;
use imagnum::{create_int, create_float};

// Random float [0, 1)
let r = rand();

// Random in range
let min = create_int("1");
let max = create_int("100");
let rand_int = randint(&min, &max);

// Random float in range
let rand_float = randfloat(&create_float("0"), &create_float("10"));
```

### Serialization
Add `features = ["serde"]` for JSON support.

```rust
// With features = ["serde"] enabled:
use imagnum::{Int, Float, create_int, create_float};

#[derive(serde::Serialize, serde::Deserialize)]
struct Data {
    num: Int,
    val: Float,
}

fn example() -> Result<(), Box<dyn std::error::Error>> {
    let data = Data {
        num: create_int("42"),
        val: create_float("3.14"),
    };

    let json = serde_json::to_string(&data)?;
    println!("JSON: {}", json);

    let deserialized: Data = serde_json::from_str(&json)?;
    println!("Deserialized: {:?}", deserialized.num);
    Ok(())
}
```

## The REPL

ImagNum includes a simple calculator REPL. It's enabled by default but you can disable it:

```toml
[dependencies]
imagnum = { version = "0.2", default-features = false }  # Disables CLI
```

Run it with:
```bash
cargo run
```

Features:
- Basic math: `2 + 2`, `3 * 4`, `10 / 3`
- Functions: `sqrt(16)`, `sin(pi)`, `ln(e)`
- Variables: `x = 42; x * 2`
- Constants: `pi`, `e`, `phi`, `i`
- Random (with feature): `rand()`, `randint(1, 100)`

## Performance Notes

- Small numbers (i32, f64, etc.) use optimized representations
- Big numbers use arbitrary precision but cost more memory/CPU
- Complex numbers box their real/imaginary parts
- Irrational numbers preserve exact representations when possible

## For Lucia Language

This library is primarily built as the numeric backend for the [Lucia](https://github.com/SirPigari/lucia-rust) programming language. If you're building tools for [Lucia](https://github.com/SirPigari/lucia-rust) or need a robust number system, this is what you'll use under the hood.

*Note* that `imagnum` is still in development and not everything has to work. We have tests but there still might be edge cases we forgot about.
