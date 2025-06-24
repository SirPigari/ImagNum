use imagnum::{
    create_float, create_int, Float, Int,
    errors::get_error_message,
};
use std::io::{self, Write};

#[derive(Debug, Clone)]
enum Number {
    Int(Int),
    Float(Float),
}

impl Number {
    fn promote(&self) -> Result<Float, i16> {
        match self {
            Number::Int(i) => Ok(create_float(&i.to_string())),
            Number::Float(f) => Ok(f.clone()),
        }
    }

    fn display(&self) -> String {
        match self {
            Number::Int(i) => i.to_string(),
            Number::Float(f) => f.to_string(),
        }
    }

    fn add(self, other: Number) -> Result<Number, i16> {
        match (self, other) {
            (Number::Int(a), Number::Int(b)) => Ok(Number::Int((a + b)?)),
            (a, b) => Ok(Number::Float((a.promote()? + b.promote()?)?)),
        }
    }

    fn sub(self, other: Number) -> Result<Number, i16> {
        match (self, other) {
            (Number::Int(a), Number::Int(b)) => Ok(Number::Int((a - b)?)),
            (a, b) => Ok(Number::Float((a.promote()? - b.promote()?)?)),
        }
    }

    fn mul(self, other: Number) -> Result<Number, i16> {
        match (self, other) {
            (Number::Int(a), Number::Int(b)) => Ok(Number::Int((a * b)?)),
            (a, b) => Ok(Number::Float((a.promote()? * b.promote()?)?)),
        }
    }

    fn div(self, other: Number) -> Result<Number, i16> {
        Ok(Number::Float((self.promote()? / other.promote()?)?))
    }

    fn sqrt(self) -> Result<Number, i16> {
        let f = self.promote()?;
        let res = f.sqrt()?;
        Ok(Number::Float(res))
    }

    fn pow(self, other: Number) -> Result<Number, i16> {
        match (self, other) {
            (Number::Int(a), Number::Int(b)) => Ok(Number::Int(a.pow(&b)?)),
            (a, b) => Ok(Number::Float(a.promote()?.pow(&b.promote()?)?)),
        }
    }

    fn rem(self, other: Number) -> Result<Number, i16> {
        let f_self = self.promote()?;
        let f_other = other.promote()?;
        Ok(Number::Float((f_self % f_other)?))
    }

    fn round(self, decimals: usize) -> Result<Number, i16> {
        let f = self.promote()?;
        let rounded = f.round(decimals);
        Ok(Number::Float(rounded))
    }
    fn truncate(self, decimals: usize) -> Result<Number, i16> {
        let f = self.promote()?;
        let truncated = f.truncate(decimals);
        Ok(Number::Float(truncated))
    }
}

fn parse_token(token: &str) -> Result<Number, i16> {
    if token.contains('.') {
        Ok(Number::Float(create_float(token)))
    } else {
        Ok(Number::Int(create_int(token)))
    }
}

fn main() {
    loop {
        print!("calc> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            println!("input error");
            continue;
        }

        let tokens: Vec<&str> = line.trim().split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        if tokens.len() == 2 && tokens[1] == "sqrt" {
            match parse_token(tokens[0]) {
                Ok(num) => match num.sqrt() {
                    Ok(res) => println!("= {}", res.display()),
                    Err(code) => println!("error [{}]: {}", code, get_error_message(code)),
                },
                Err(code) => println!("error [{}]: {}", code, get_error_message(code)),
            }
            continue;
        }

        if tokens.len() < 3 {
            println!("format: num op num [op num ...]");
            continue;
        }

        let mut iter = tokens.into_iter();

        let mut acc = match parse_token(iter.next().unwrap()) {
            Ok(n) => n,
            Err(code) => {
                println!("error [{}]: {}", code, get_error_message(code));
                continue;
            }
        };

        let mut error_occurred = false;

        while let Some(op) = iter.next() {
            if op == "sqrt" {
                match acc.clone().sqrt() {
                    Ok(res) => acc = res,
                    Err(code) => {
                        println!("error [{}]: {}", code, get_error_message(code));
                        error_occurred = true;
                        break;
                    }
                }
                continue;
            }

            if op == "round" {
                let rhs_opt = iter.next();
                let decimals = match rhs_opt {
                    Some(d_str) => match d_str.parse::<u32>() {
                        Ok(val) => val,
                        Err(_) => {
                            println!("invalid decimals argument for round: {}", d_str);
                            error_occurred = true;
                            break;
                        }
                    },
                    None => {
                        println!("missing decimals argument for round");
                        error_occurred = true;
                        break;
                    }
                };

                match acc.clone().round(decimals as usize) {
                    Ok(res) => acc = res,
                    Err(code) => {
                        println!("error [{}]: {}", code, get_error_message(code));
                        error_occurred = true;
                        break;
                    }
                }
                continue;
            }

            if op == "trunc" {
                let rhs_opt = iter.next();
                let decimals = match rhs_opt {
                    Some(d_str) => match d_str.parse::<u32>() {
                        Ok(val) => val,
                        Err(_) => {
                            println!("invalid decimals argument for round: {}", d_str);
                            error_occurred = true;
                            break;
                        }
                    },
                    None => {
                        println!("missing decimals argument for round");
                        error_occurred = true;
                        break;
                    }
                };

                match acc.clone().truncate(decimals as usize) {
                    Ok(res) => acc = res,
                    Err(code) => {
                        println!("error [{}]: {}", code, get_error_message(code));
                        error_occurred = true;
                        break;
                    }
                }
                continue;
            }

            if op == "int-like" {
                match acc.clone() {
                    Number::Int(i) => println!("{} is already an int-like number", i),
                    Number::Float(f) => {
                        let int_value = f.is_integer_like();
                        if int_value {
                            println!("{} is an int-like number", f);
                        } else {
                            println!("{} is not an int-like number", f);
                        }
                    }
                }
                continue;
            }

            let rhs = match iter.next() {
                Some(t) => match parse_token(t) {
                    Ok(n) => n,
                    Err(code) => {
                        println!("error [{}]: {}", code, get_error_message(code));
                        error_occurred = true;
                        break;
                    }
                },
                None => {
                    println!("missing operand after operator '{}'", op);
                    error_occurred = true;
                    break;
                }
            };

            let result = match op {
                "+" => acc.clone().add(rhs),
                "-" => acc.clone().sub(rhs),
                "*" => acc.clone().mul(rhs),
                "/" => acc.clone().div(rhs),
                "^" => acc.clone().pow(rhs),
                "%" => acc.clone().rem(rhs),
                _ => {
                    println!("unknown operator: {}", op);
                    error_occurred = true;
                    break;
                }
            };

            acc = match result {
                Ok(n) => n,
                Err(code) => {
                    println!("error [{}]: {}", code, get_error_message(code));
                    error_occurred = true;
                    break;
                }
            };
        }

        if !error_occurred {
            println!("{}= {}", " ".repeat(4), acc.display());
        }
    }
}
