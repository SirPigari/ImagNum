use imagnum::{Float, Int, create_float, create_int, errors::get_error_message};
use std::io::{self, Write};
// std::cmp::Ordering no longer needed

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

#[derive(Debug, Clone)]
enum Value {
    Num(Number),
    Bool(bool),
}

fn parse_value(token: &str) -> Result<Value, i16> {
    match token {
        "true" => Ok(Value::Bool(true)),
        "false" => Ok(Value::Bool(false)),
        _ => match parse_token(token) {
            Ok(n) => Ok(Value::Num(n)),
            Err(code) => Err(code),
        },
    }
}

impl Value {
    fn display(&self) -> String {
        match self {
            Value::Bool(b) => b.to_string(),
            Value::Num(n) => n.display(),
        }
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
        if line.trim().is_empty() {
            continue;
        }
        if line.starts_with("quit") {
            std::process::exit(0);
        }

        // Tokenize the input so "1/3" -> ["1","/","3"] and "0.4(9)" stays a single token.
        fn tokenize(input: &str) -> Vec<String> {
            let mut toks = Vec::new();
            let chars: Vec<char> = input.chars().collect();
            let mut i = 0usize;
            let n = chars.len();
            let mut last_was_value = false;
            while i < n {
                let c = chars[i];
                if c.is_whitespace() { i += 1; continue; }
                // two-char operators
                if i + 1 < n {
                    let two = format!("{}{}", c, chars[i+1]);
                    if two == "==" || two == "!=" || two == ">=" || two == "<=" {
                        toks.push(two);
                        i += 2;
                        last_was_value = false;
                        continue;
                    }
                }
                // single-char operators/parentheses
                if "+-*/%^()<>".contains(c) {
                    // treat + or - as part of number if it's a sign at a value position
                    if (c == '+' || c == '-') && !last_was_value && i + 1 < n && (chars[i+1].is_ascii_digit() || chars[i+1] == '.') {
                        // parse signed number below
                    } else {
                        toks.push(c.to_string());
                        i += 1;
                        last_was_value = false;
                        continue;
                    }
                }
                // number token (digits, optional dot, optional recurring parentheses)
                if chars[i].is_ascii_digit() || chars[i] == '.' || ((chars[i] == '+' || chars[i] == '-') && i + 1 < n && (chars[i+1].is_ascii_digit() || chars[i+1] == '.')) {
                    let start = i;
                    if chars[i] == '+' || chars[i] == '-' { i += 1; }
                    while i < n && (chars[i].is_ascii_digit() || chars[i] == '.') { i += 1; }
                    // if next is '(' collect recurring group
                    if i < n && chars[i] == '(' {
                        let mut j = i + 1;
                        while j < n && chars[j] != ')' { j += 1; }
                        if j < n && chars[j] == ')' {
                            i = j + 1;
                        }
                    }
                    let tok: String = chars[start..i].iter().collect();
                    toks.push(tok);
                    last_was_value = true;
                    continue;
                }
                // identifier token (letters)
                if chars[i].is_alphabetic() {
                    let start = i;
                    while i < n && (chars[i].is_alphanumeric() || chars[i] == '-') { i += 1; }
                    let tok: String = chars[start..i].iter().collect();
                    toks.push(tok);
                    last_was_value = true;
                    continue;
                }
                // fallback single char
                toks.push(c.to_string());
                i += 1;
                last_was_value = false;
            }
            toks
        }

        let mut tokens_owned = tokenize(line.trim());
        // fallback: if tokenizer returned a single token that still contains operator chars,
        // attempt a conservative split so expressions like "1/3" become ["1","/","3"].
        if tokens_owned.len() == 1 {
            let s = &tokens_owned[0];
            if s.contains('/') || s.contains('*') || s.contains('+') || s.contains('-') || s.contains('%') || s.contains('^') {
                let mut parts: Vec<String> = Vec::new();
                let mut cur = String::new();
                for ch in s.chars() {
                    if "+-*/%^()<>".contains(ch) {
                        if !cur.is_empty() { parts.push(cur.clone()); cur.clear(); }
                        parts.push(ch.to_string());
                    } else {
                        cur.push(ch);
                    }
                }
                if !cur.is_empty() { parts.push(cur); }
                if parts.len() > 1 {
                    tokens_owned = parts;
                }
            }
        }
        let mut tokens: Vec<&str> = tokens_owned.iter().map(|s| s.as_str()).collect();
        if tokens.is_empty() { continue; }

        // Allow a trailing '=' token to request result printing (e.g. "3 + 4 =").
        if tokens.len() > 1 && tokens.last() == Some(&"=") {
            tokens.pop();
        };

        // If input is a single number (with optional trailing '='), just print it.
        if tokens.len() == 1 {
            match parse_token(tokens[0]) {
                Ok(n) => println!("{}= {}", " ".repeat(4), n.display()),
                Err(code) => println!("error [{}]: {}", code, get_error_message(code)),
            }
            continue;
        }

        let special_ops = ["sqrt", "round", "trunc", "int-like", "exit"];
        if tokens.iter().any(|t| special_ops.contains(t)) {
            let mut iter = tokens.into_iter();

            let first = iter.next().unwrap();
            let mut acc = if first == "-" || first == "+" {
                match iter.next() {
                    Some(next_tok) => {
                        let signed = if first == "-" { format!("-{}", next_tok) } else { format!("{}", next_tok) };
                        match parse_token(signed.as_str()) {
                            Ok(n) => n,
                            Err(code) => { println!("error [{}]: {}", code, get_error_message(code)); continue; }
                        }
                    }
                    None => { println!("missing operand after unary '{}'", first); continue; }
                }
            } else {
                match parse_token(first) {
                    Ok(n) => n,
                    Err(code) => { println!("error [{}]: {}", code, get_error_message(code)); continue; }
                }
            };

            let mut error_occurred = false;

            while let Some(op) = iter.next() {
                if op == "sqrt" {
                    match acc.clone().sqrt() {
                        Ok(res) => acc = res,
                        Err(code) => { println!("error [{}]: {}", code, get_error_message(code)); error_occurred = true; break; }
                    }
                    continue;
                }

                if op == "round" {
                    let rhs_opt = iter.next();
                    let decimals = match rhs_opt {
                        Some(d_str) => match d_str.parse::<u32>() { Ok(val) => val, Err(_) => { println!("invalid decimals argument for round: {}", d_str); error_occurred = true; break; } },
                        None => { println!("missing decimals argument for round"); error_occurred = true; break; }
                    };

                    match acc.clone().round(decimals as usize) {
                        Ok(res) => acc = res,
                        Err(code) => { println!("error [{}]: {}", code, get_error_message(code)); error_occurred = true; break; }
                    }
                    continue;
                }

                if op == "trunc" {
                    let rhs_opt = iter.next();
                    let decimals = match rhs_opt {
                        Some(d_str) => match d_str.parse::<u32>() { Ok(val) => val, Err(_) => { println!("invalid decimals argument for trunc: {}", d_str); error_occurred = true; break; } },
                        None => { println!("missing decimals argument for trunc"); error_occurred = true; break; }
                    };

                    match acc.clone().truncate(decimals as usize) {
                        Ok(res) => acc = res,
                        Err(code) => { println!("error [{}]: {}", code, get_error_message(code)); error_occurred = true; break; }
                    }
                    continue;
                }

                if op == "int-like" {
                    match acc.clone() {
                        Number::Int(i) => println!("{} is already an int-like number", i),
                        Number::Float(f) => {
                            let int_value = f.is_integer_like();
                            if int_value { println!("{} is an int-like number", f); } else { println!("{} is not an int-like number", f); }
                        }
                    }
                    continue;
                }

                if op == "exit" {
                    std::process::exit(0);
                }

                // Fetch the right-hand side operand; allow unary +/- before the number.
                let rhs = match iter.next() {
                    Some(t) => {
                        if t == "-" || t == "+" {
                            match iter.next() {
                                Some(next_tok) => {
                                    let signed = if t == "-" { format!("-{}", next_tok) } else { format!("{}", next_tok) };
                                    match parse_token(signed.as_str()) { Ok(n) => n, Err(code) => { println!("error [{}]: {}", code, get_error_message(code)); error_occurred = true; break; } }
                                }
                                None => { println!("missing operand after unary '{}'", t); error_occurred = true; break; }
                            }
                        } else {
                            match parse_token(t) { Ok(n) => n, Err(code) => { println!("error [{}]: {}", code, get_error_message(code)); error_occurred = true; break; } }
                        }
                    }
                    None => { println!("missing operand after operator '{}'", op); error_occurred = true; break; }
                };

                let result = match op {
                    "+" => acc.clone().add(rhs),
                    "-" => acc.clone().sub(rhs),
                    "*" => acc.clone().mul(rhs),
                    "/" => acc.clone().div(rhs),
                    "^" => acc.clone().pow(rhs),
                    "%" => acc.clone().rem(rhs),
                    _ => { println!("unknown operator: {}", op); error_occurred = true; break; }
                };

                acc = match result { Ok(n) => n, Err(code) => { println!("error [{}]: {}", code, get_error_message(code)); error_occurred = true; break; } };
            }

            if !error_occurred {
                println!("{}= {}", " ".repeat(4), acc.display());
            }

            continue;
        }

        // Use shunting-yard to handle operator precedence for arithmetic and comparison expressions.
        // Supported operators: + - * / % ^  == != > < >= <=
        let mut output_queue: Vec<&str> = Vec::new();
        let mut op_stack: Vec<&str> = Vec::new();

        // Helper for operator precedence and associativity
        let precedence = |op: &str| match op {
            "==" | "!=" | ">" | "<" | ">=" | "<=" => 1,
            "+" | "-" => 2,
            "*" | "/" | "%" => 3,
            "^" => 4,
            _ => 0,
        };
        let is_right_assoc = |op: &str| op == "^";

        // Preprocess to combine unary +/- (when sign appears where an operand is expected)
        let mut preprocessed: Vec<String> = Vec::new();
        let mut i = 0usize;
        while i < tokens.len() {
            let t = tokens[i];
            if (t == "+" || t == "-") && (i == 0 || ["+","-","*","/","%","^"] .contains(&tokens[i-1])) {
                // unary sign, combine with next token
                if i + 1 < tokens.len() {
                    preprocessed.push(format!("{}{}", t, tokens[i+1]));
                    i += 2;
                    continue;
                } else {
                    println!("missing operand after unary '{}'", t);
                    break;
                }
            } else {
                preprocessed.push(t.to_string());
                i += 1;
            }
        }

        // Shunting-yard using preprocessed tokens
        for tok in preprocessed.iter().map(|s| s.as_str()) {
            if ["+","-","*","/","%","^","==","!=",">","<",">=","<="].contains(&tok) {
                while let Some(&top) = op_stack.last() {
                    if top == "(" { break; }
                    let p_top = precedence(top);
                    let p_tok = precedence(tok);
                    if (is_right_assoc(tok) && p_tok < p_top) || (!is_right_assoc(tok) && p_tok <= p_top) {
                        output_queue.push(op_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                op_stack.push(tok);
            } else if tok == "(" {
                op_stack.push(tok);
            } else if tok == ")" {
                while let Some(top) = op_stack.pop() {
                    if top == "(" { break; }
                    output_queue.push(top);
                }
            } else {
                // assume number
                output_queue.push(tok);
            }
        }

        while let Some(op) = op_stack.pop() {
            output_queue.push(op);
        }

        // Evaluate RPN
        let mut eval_stack: Vec<Value> = Vec::new();
        let mut error_occurred = false;
        for tok in output_queue {
            if ["+","-","*","/","%","^","==","!=",">","<",">=","<="].contains(&tok) {
                let rhs = eval_stack.pop();
                let lhs = eval_stack.pop();
                if rhs.is_none() || lhs.is_none() {
                    println!("malformed expression");
                    error_occurred = true;
                    break;
                }
                let lhs = lhs.unwrap();
                let rhs = rhs.unwrap();

                // Comparisons require numeric operands.
                if ["==","!=",">","<",">=","<="].contains(&tok) {
                    let lnum = match lhs {
                        Value::Num(n) => n,
                        Value::Bool(_) => { println!("left operand to comparison is boolean"); error_occurred = true; break; }
                    };
                    let rnum = match rhs {
                        Value::Num(n) => n,
                        Value::Bool(_) => { println!("right operand to comparison is boolean"); error_occurred = true; break; }
                    };

                    // Promote both to Float for comparison
                    let lfp = lnum.promote();
                    let rfp = rnum.promote();
                    let bool_res = match (lfp, rfp) {
                        (Ok(la), Ok(rb)) => match tok {
                            "==" => la == rb,
                            "!=" => la != rb,
                            ">" => la > rb,
                            "<" => la < rb,
                            ">=" => la >= rb,
                            "<=" => la <= rb,
                            _ => unreachable!(),
                        },
                        _ => { println!("comparison failed"); error_occurred = true; break; }
                    };
                    eval_stack.push(Value::Bool(bool_res));
                    continue;
                }

                // Arithmetic ops
                let lnum = match lhs { Value::Num(n) => n, Value::Bool(_) => { println!("left operand to arithmetic is boolean"); error_occurred = true; break; } };
                let rnum = match rhs { Value::Num(n) => n, Value::Bool(_) => { println!("right operand to arithmetic is boolean"); error_occurred = true; break; } };

                let res = match tok {
                    "+" => lnum.add(rnum),
                    "-" => lnum.sub(rnum),
                    "*" => lnum.mul(rnum),
                    "/" => lnum.div(rnum),
                    "%" => lnum.rem(rnum),
                    "^" => lnum.pow(rnum),
                    _ => unreachable!(),
                };
                match res {
                    Ok(n) => eval_stack.push(Value::Num(n)),
                    Err(code) => { println!("error [{}]: {}", code, get_error_message(code)); error_occurred = true; break; }
                }
            } else {
                match parse_value(tok) {
                    Ok(v) => eval_stack.push(v),
                    Err(code) => { println!("error [{}]: {}", code, get_error_message(code)); error_occurred = true; break; }
                }
            }
        }

        if error_occurred { continue; }
        if eval_stack.len() != 1 { println!("malformed expression"); continue; }
        let result = eval_stack.pop().unwrap();
        println!("{}= {}", " ".repeat(4), result.display());
    }
}
