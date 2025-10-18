use imagnum::{Float, Int, create_float, create_int, create_complex, create_imaginary, create_irrational, errors::get_error_message};
use std::io::{self, Write};
use std::collections::HashMap;

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

    fn sin(self) -> Result<Number, i16> {
        let f = self.promote()?;
        let res = f.sin()?;
        Ok(Number::Float(res))
    }

    fn cos(self) -> Result<Number, i16> {
        let f = self.promote()?;
        let res = f.cos()?;
        Ok(Number::Float(res))
    }

    fn tan(self) -> Result<Number, i16> {
        let f = self.promote()?;
        let res = f.tan()?;
        Ok(Number::Float(res))
    }

    fn ln(self) -> Result<Number, i16> {
        let f = self.promote()?;
        let res = f.ln()?;
        Ok(Number::Float(res))
    }

    fn exp(self) -> Result<Number, i16> {
        let f = self.promote()?;
        let res = f.exp()?;
        Ok(Number::Float(res))
    }

    fn log(self, base: Number) -> Result<Number, i16> {
        let f = self.promote()?;
        let base_f = base.promote()?;
        let res = f.log(&base_f)?;
        Ok(Number::Float(res))
    }

    fn abs(self) -> Number {
        match self {
            Number::Int(i) => Number::Int(i.abs()),
            Number::Float(f) => Number::Float(f.abs()),
        }
    }

    fn floor(self) -> Result<Number, i16> {
        let f = self.promote()?;
        let res = f.floor()?;
        Ok(Number::Float(res))
    }

    fn ceil(self) -> Result<Number, i16> {
        let f = self.promote()?;
        let res = f.ceil()?;
        Ok(Number::Float(res))
    }

    fn conj(self) -> Number {
        match self {
            Number::Int(i) => Number::Int(i),
            Number::Float(f) => Number::Float(f.conj()),
        }
    }

    #[allow(dead_code)]
    fn is_complex(&self) -> bool {
        match self {
            Number::Int(_) => false,
            Number::Float(f) => f.is_complex(),
        }
    }

    #[allow(dead_code)]
    fn is_nan(&self) -> bool {
        match self {
            Number::Int(_) => false,
            Number::Float(f) => f.is_nan(),
        }
    }

    #[allow(dead_code)]
    fn is_infinity(&self) -> bool {
        match self {
            Number::Int(_) => false,
            Number::Float(f) => f.is_infinity(),
        }
    }
}

fn parse_token(token: &str) -> Result<Number, i16> {
    // Handle complex numbers like "3+4i" or "2i"
    if token.ends_with('i') && token.len() > 1 {
        let without_i = &token[..token.len() - 1];
        let coeff = if without_i.is_empty() || without_i == "+" {
            "1"
        } else if without_i == "-" {
            "-1"
        } else {
            without_i
        };
        
        if coeff.contains('+') || coeff.contains('-') && coeff.len() > 1 {
            // Handle complex like "3+4i"
            return Ok(Number::Float(create_float(token)));
        } else {
            // Handle pure imaginary like "2i"
            return Ok(Number::Float(create_complex("0", coeff)));
        }
    }
    
    // Handle hexadecimal numbers
    if token.starts_with("0x") || token.starts_with("0X") {
        if token.contains('.') {
            return Ok(Number::Float(create_float(token)));
        } else {
            let result = Int::from_hex(&token[2..]);
            match result {
                Ok(i) => return Ok(Number::Int(i)),
                Err(_) => return Ok(Number::Int(create_int("0"))),
            }
        }
    }
    
    // Handle binary numbers
    if token.starts_with("0b") || token.starts_with("0B") {
        let result = Int::from_str_radix(&token[2..], 2);
        match result {
            Ok(i) => return Ok(Number::Int(i)),
            Err(_) => return Ok(Number::Int(create_int("0"))),
        }
    }
    
    // Handle octal numbers
    if token.starts_with("0o") || token.starts_with("0O") {
        let result = Int::from_str_radix(&token[2..], 8);
        match result {
            Ok(i) => return Ok(Number::Int(i)),
            Err(_) => return Ok(Number::Int(create_int("0"))),
        }
    }
    
    if token.contains('.') || token.contains('(') {
        Ok(Number::Float(create_float(token)))
    } else {
        Ok(Number::Int(create_int(token)))
    }
}



// Constants
fn get_constant(name: &str) -> Option<Number> {
    match name {
        "pi" | "PI" => Some(Number::Float(create_irrational("3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342117067"))),
        "e" | "E" => Some(Number::Float(create_irrational("2.718281828459045235360287471352662497757247093699959574966967627724076630353547594571382178525166427"))),
        "phi" | "PHI" => Some(Number::Float(create_irrational("1.618033988749894848204586834365638117720309179805762862135448622705260462818902449707207204189391137"))),
        "sqrt2" | "SQRT2" => Some(Number::Float(create_irrational("1.414213562373095048801688724209698078569671875376948073176679737990732478462107038850387534327641573"))),
        "inf" | "INF" | "infinity" | "INFINITY" => Some(Number::Float(create_float("inf"))),
        "nan" | "NaN" | "NAN" => Some(Number::Float(create_float("nan"))),
        "i" | "I" => Some(Number::Float(create_imaginary())),
        _ => None,
    }
}

// Enhanced command help
fn print_help() {
    println!("ImagNum Calculator REPL v{}", imagnum::VERSION);
    println!("=============================================");
    println!("Basic Operations:");
    println!("  +, -, *, /, %, ^        Arithmetic operators");
    println!("  ==, !=, <, >, <=, >=    Comparison operators");
    println!("  ( )                     Parentheses for grouping");
    println!();
    println!("Mathematical Functions:");
    println!("  sqrt(x)        Square root");
    println!("  abs(x)         Absolute value");
    println!("  sin(x)         Sine");
    println!("  cos(x)         Cosine");
    println!("  tan(x)         Tangent");
    println!("  ln(x)          Natural logarithm");
    println!("  exp(x)         e^x");
    println!("  log(x, base)   Logarithm with base");
    println!("  floor(x)       Floor function");
    println!("  ceil(x)        Ceiling function");
    println!("  round(x, n)    Round to n decimal places");
    println!("  trunc(x, n)    Truncate to n decimal places");
    println!("  conj(x)        Complex conjugate");
    println!();
    println!("Number Types:");
    println!("  123            Integer");
    println!("  123.45         Decimal number");
    println!("  123.45(67)     Recurring decimal");
    println!("  3+4i           Complex number");
    println!("  2i             Pure imaginary");
    println!("  0x1F           Hexadecimal");
    println!("  0b1010         Binary");
    println!("  0o17           Octal");
    println!();
    println!("Constants:");
    println!("  pi, e, phi, sqrt2, inf, nan, i");
    println!();
    println!("Variables:");
    println!("  x = 42         Assign value to variable");
    println!("  x              Use variable");
    println!();
    println!("Information:");
    println!("  info(x)        Show number type and properties");
    println!("  vars           List all variables");
    println!("  hex(x)         Show as hexadecimal");
    println!("  bin(x)         Show as binary");
    println!("  oct(x)         Show as octal");
    println!();
    println!("Commands:");
    println!("  help           Show this help");
    println!("  clear          Clear all variables");
    println!("  quit/exit      Exit calculator");
    println!();
}

fn main() {
    let mut variables: HashMap<String, Number> = HashMap::new();
    
    println!("ImagNum Calculator REPL v{}", imagnum::VERSION);
    println!("Type 'help' for assistance, 'quit' to exit");

    loop {
        print!("calc> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                
                // Handle special commands
                match line {
            "quit" | "exit" => {
                println!("Exiting!");
                std::process::exit(0);
            }
            "help" | "?" => {
                print_help();
                continue;
            }
            "clear" => {
                variables.clear();
                println!("All variables cleared.");
                continue;
            }
            "vars" => {
                if variables.is_empty() {
                    println!("No variables defined.");
                } else {
                    println!("Variables:");
                    for (name, value) in &variables {
                        println!("  {} = {}", name, value.display());
                    }
                }
                continue;
            }
            _ => {}
        }

                // Handle variable assignment
                if let Some(eq_pos) = line.find('=') {
                    if eq_pos > 0 {
                        let var_name = line[..eq_pos].trim();
                        let expr = line[eq_pos + 1..].trim();
                        
                        if var_name.chars().all(|c| c.is_alphanumeric() || c == '_') && var_name.chars().next().unwrap().is_alphabetic() {
                            match evaluate_expression(expr, &variables) {
                                Ok(result) => {
                                    println!("{} = {}", var_name, result.display());
                                    variables.insert(var_name.to_string(), result);
                                }
                                Err(code) => {
                                    println!("error [{}]: {}", code, get_error_message(code));
                                }
                            }
                            continue;
                        }
                    }
                }

                // Handle function calls like info(x), hex(x), etc.
                if let Some(result) = handle_special_functions(line, &variables) {
                    match result {
                        Ok(output) => println!("{}", output),
                        Err(code) => println!("error [{}]: {}", code, get_error_message(code)),
                    }
                    continue;
                }

                // Evaluate expression
                match evaluate_expression(line, &variables) {
                    Ok(result) => {
                        println!("    = {}", result.display());
                    }
                    Err(code) => {
                        println!("error [{}]: {}", code, get_error_message(code));
                    }
                }
            }
            Err(_) => {
                println!("Input error");
                continue;
            }
        }
    }
}

fn handle_special_functions(input: &str, variables: &HashMap<String, Number>) -> Option<Result<String, i16>> {
    let input = input.trim();
    
    // info(x) - show number information
    if input.starts_with("info(") && input.ends_with(')') {
        let expr = &input[5..input.len()-1];
        return Some(match evaluate_expression(expr, variables) {
            Ok(num) => {
                let mut info = vec![];
                match &num {
                    Number::Int(i) => {
                        info.push("Type: Integer".to_string());
                        info.push(format!("    Value: {}", i));
                        info.push(format!("    Negative: {}", i.is_negative()));
                        info.push(format!("    Zero: {}", i.is_zero()));
                    }
                    Number::Float(f) => {
                        info.push("Type: Float".to_string());
                        info.push(format!("    Value: {}", f));
                        
                        // Check special values first (NaN and Infinity take precedence)
                        if f.is_nan() {
                            info.push("    Special: NaN (Not a Number)".to_string());
                        } else if f.is_infinity() {
                            info.push("    Special: Infinity".to_string());
                            info.push(format!("    Negative: {}", f.is_negative()));
                        } else {
                            let mut type_str = "    Type: ".to_string();
                            type_str.push_str(match f {
                                Float::Big(_) | Float::Small(_) => "Real",
                                Float::Irrational(_) => "Irrational",
                                Float::Recurring(_) => "Recurring Decimal",
                                Float::Complex(_, _) => "Complex",
                                Float::NaN => "NaN",
                                Float::Infinity => "Infinity",
                                Float::NegInfinity => "Negative Infinity",
                            });
                            info.push(type_str);

                            if !f.is_zero() {
                                info.push(format!("    Negative: {}", f.is_negative()));
                            }
                        }
                    }
                }
                Ok(info.join("\n"))
            }
            Err(code) => Err(code),
        });
    }
    
    // hex(x) - show as hexadecimal
    if input.starts_with("hex(") && input.ends_with(')') {
        let expr = &input[4..input.len()-1];
        return Some(match evaluate_expression(expr, variables) {
            Ok(num) => {
                match num {
                    Number::Int(i) => Ok(format!("0x{}", i.to_str_radix(16).unwrap_or_else(|_| "error".to_string()))),
                    Number::Float(_) => Ok("Hexadecimal display only available for integers".to_string()),
                }
            }
            Err(code) => Err(code),
        });
    }
    
    // bin(x) - show as binary
    if input.starts_with("bin(") && input.ends_with(')') {
        let expr = &input[4..input.len()-1];
        return Some(match evaluate_expression(expr, variables) {
            Ok(num) => {
                match num {
                    Number::Int(i) => Ok(format!("0b{}", i.to_str_radix(2).unwrap_or_else(|_| "error".to_string()))),
                    Number::Float(_) => Ok("Binary display only available for integers".to_string()),
                }
            }
            Err(code) => Err(code),
        });
    }
    
    // oct(x) - show as octal
    if input.starts_with("oct(") && input.ends_with(')') {
        let expr = &input[4..input.len()-1];
        return Some(match evaluate_expression(expr, variables) {
            Ok(num) => {
                match num {
                    Number::Int(i) => Ok(format!("0o{}", i.to_str_radix(8).unwrap_or_else(|_| "error".to_string()))),
                    Number::Float(_) => Ok("Octal display only available for integers".to_string()),
                }
            }
            Err(code) => Err(code),
        });
    }
    
    None
}

fn evaluate_expression(expr: &str, variables: &HashMap<String, Number>) -> Result<Number, i16> {
    // Enhanced tokenizer
    fn tokenize(input: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;
        let n = chars.len();
        
        while i < n {
            let c = chars[i];
            if c.is_whitespace() {
                i += 1;
                continue;
            }
            
            // Handle two-character operators
            if i + 1 < n {
                let two = format!("{}{}", c, chars[i+1]);
                if ["==", "!=", ">=", "<="].contains(&two.as_str()) {
                    tokens.push(two);
                    i += 2;
                    continue;
                }
            }
            
            // Handle operators and parentheses
            if "+-*/%^()<>=!".contains(c) {
                tokens.push(c.to_string());
                i += 1;
                continue;
            }
            
            // Handle numbers (including complex, hex, binary, octal)
            if c.is_ascii_digit() || c == '.' || 
               (c == '0' && i + 1 < n && ['x', 'X', 'b', 'B', 'o', 'O'].contains(&chars[i+1])) {
                let start = i;
                
                // Handle hex/binary/octal prefixes
                if c == '0' && i + 1 < n && ['x', 'X', 'b', 'B', 'o', 'O'].contains(&chars[i+1]) {
                    i += 2; // Skip 0x/0b/0o
                    while i < n && chars[i].is_ascii_alphanumeric() {
                        i += 1;
                    }
                } else {
                    // Regular number
                    while i < n && (chars[i].is_ascii_digit() || chars[i] == '.') {
                        i += 1;
                    }
                    
                    // Handle recurring decimals like 0.3(3)
                    if i < n && chars[i] == '(' {
                        let mut j = i + 1;
                        while j < n && chars[j] != ')' {
                            j += 1;
                        }
                        if j < n && chars[j] == ')' {
                            i = j + 1;
                        }
                    }
                    
                    // Handle imaginary unit 'i'
                    if i < n && chars[i] == 'i' {
                        i += 1;
                    }
                }
                
                let token: String = chars[start..i].iter().collect();
                tokens.push(token);
                continue;
            }
            
            // Handle identifiers (variables, function names, constants)
            if c.is_alphabetic() || c == '_' {
                let start = i;
                while i < n && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let token: String = chars[start..i].iter().collect();
                tokens.push(token);
                continue;
            }
            
            // Handle commas for function arguments
            if c == ',' {
                tokens.push(c.to_string());
                i += 1;
                continue;
            }
            
            // Skip unknown characters
            i += 1;
        }
        
        tokens
    }

    let tokens = tokenize(expr);
    if tokens.is_empty() {
        return Err(-1);
    }

    // Handle single tokens
    if tokens.len() == 1 {
        let token = &tokens[0];
        
        // Check if it's a variable
        if let Some(value) = variables.get(token) {
            return Ok(value.clone());
        }
        
        // Check if it's a constant
        if let Some(value) = get_constant(token) {
            return Ok(value);
        }
        
        // Try to parse as number
        return parse_token(token);
    }

    // Handle unary minus (e.g., "-1" becomes ["-", "1"])
    if tokens.len() == 2 && tokens[0] == "-" {
        let operand = evaluate_expression(&tokens[1], variables)?;
        let zero = Number::Int(create_int("0"));
        return zero.sub(operand);
    }

    // Handle function calls
    if tokens.len() >= 4 && tokens[1] == "(" && tokens[tokens.len()-1] == ")" {
        let func_name = &tokens[0];
        let args_tokens = &tokens[2..tokens.len()-1];
        
        return handle_function_call(func_name, args_tokens, variables);
    }

    // Parse as mathematical expression using shunting yard algorithm
    parse_expression_shunting_yard(&tokens, variables)
}

fn handle_function_call(func_name: &str, args_tokens: &[String], variables: &HashMap<String, Number>) -> Result<Number, i16> {
    // Split arguments by commas
    let mut args = Vec::new();
    let mut current_arg = Vec::new();
    let mut paren_count = 0;
    
    for token in args_tokens {
        if token == "," && paren_count == 0 {
            if !current_arg.is_empty() {
                args.push(current_arg.clone());
                current_arg.clear();
            }
        } else {
            if token == "(" { paren_count += 1; }
            else if token == ")" { paren_count -= 1; }
            current_arg.push(token.clone());
        }
    }
    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    // Evaluate each argument
    let mut eval_args = Vec::new();
    for arg in args {
        let arg_expr = arg.join(" ");
        eval_args.push(evaluate_expression(&arg_expr, variables)?);
    }

    // Call the appropriate function
    match func_name {
        "sqrt" => {
            if eval_args.len() != 1 { return Err(-1); }
            eval_args[0].clone().sqrt()
        }
        "abs" => {
            if eval_args.len() != 1 { return Err(-1); }
            Ok(eval_args[0].clone().abs())
        }
        "sin" => {
            if eval_args.len() != 1 { return Err(-1); }
            eval_args[0].clone().sin()
        }
        "cos" => {
            if eval_args.len() != 1 { return Err(-1); }
            eval_args[0].clone().cos()
        }
        "tan" => {
            if eval_args.len() != 1 { return Err(-1); }
            eval_args[0].clone().tan()
        }
        "ln" => {
            if eval_args.len() != 1 { return Err(-1); }
            eval_args[0].clone().ln()
        }
        "exp" => {
            if eval_args.len() != 1 { return Err(-1); }
            eval_args[0].clone().exp()
        }
        "log" => {
            if eval_args.len() != 2 { return Err(-1); }
            eval_args[0].clone().log(eval_args[1].clone())
        }
        "floor" => {
            if eval_args.len() != 1 { return Err(-1); }
            eval_args[0].clone().floor()
        }
        "ceil" => {
            if eval_args.len() != 1 { return Err(-1); }
            eval_args[0].clone().ceil()
        }
        "round" => {
            if eval_args.len() != 2 { return Err(-1); }
            match &eval_args[1] {
                Number::Int(decimals) => {
                    if let Some(d) = decimals.to_string().parse::<usize>().ok() {
                        eval_args[0].clone().round(d)
                    } else {
                        Err(-1)
                    }
                }
                _ => Err(-1),
            }
        }
        "trunc" => {
            if eval_args.len() != 2 { return Err(-1); }
            match &eval_args[1] {
                Number::Int(decimals) => {
                    if let Some(d) = decimals.to_string().parse::<usize>().ok() {
                        eval_args[0].clone().truncate(d)
                    } else {
                        Err(-1)
                    }
                }
                _ => Err(-1),
            }
        }
        "conj" => {
            if eval_args.len() != 1 { return Err(-1); }
            Ok(eval_args[0].clone().conj())
        }
        _ => Err(-1),
    }
}

fn parse_expression_shunting_yard(tokens: &[String], variables: &HashMap<String, Number>) -> Result<Number, i16> {
    let mut output_queue: Vec<String> = Vec::new();
    let mut op_stack: Vec<String> = Vec::new();

    let precedence = |op: &str| match op {
        "==" | "!=" | ">" | "<" | ">=" | "<=" => 1,
        "+" | "-" => 2,
        "*" | "/" | "%" => 3,
        "^" => 4,
        _ => 0,
    };
    
    let is_right_assoc = |op: &str| op == "^";

    for token in tokens {
        if ["+", "-", "*", "/", "%", "^", "==", "!=", ">", "<", ">=", "<="].contains(&token.as_str()) {
            while let Some(top) = op_stack.last() {
                if top == "(" { break; }
                let p_top = precedence(top);
                let p_tok = precedence(token);
                if (is_right_assoc(token) && p_tok < p_top) || (!is_right_assoc(token) && p_tok <= p_top) {
                    output_queue.push(op_stack.pop().unwrap());
                } else {
                    break;
                }
            }
            op_stack.push(token.clone());
        } else if token == "(" {
            op_stack.push(token.clone());
        } else if token == ")" {
            while let Some(top) = op_stack.pop() {
                if top == "(" { break; }
                output_queue.push(top);
            }
        } else {
            output_queue.push(token.clone());
        }
    }

    while let Some(op) = op_stack.pop() {
        output_queue.push(op);
    }

    // Evaluate the postfix expression
    let mut eval_stack: Vec<Number> = Vec::new();
    
    for token in output_queue {
        if ["+", "-", "*", "/", "%", "^", "==", "!=", ">", "<", ">=", "<="].contains(&token.as_str()) {
            if eval_stack.len() < 2 {
                return Err(-1);
            }
            let rhs = eval_stack.pop().unwrap();
            let lhs = eval_stack.pop().unwrap();

            let result = match token.as_str() {
                "+" => lhs.add(rhs)?,
                "-" => lhs.sub(rhs)?,
                "*" => lhs.mul(rhs)?,
                "/" => lhs.div(rhs)?,
                "%" => lhs.rem(rhs)?,
                "^" => lhs.pow(rhs)?,
                _ => return Err(-1), // Comparison operators not yet implemented
            };
            eval_stack.push(result);
        } else {
            // It's a value (number, variable, or constant)
            let value = if let Some(var_value) = variables.get(&token) {
                var_value.clone()
            } else if let Some(const_value) = get_constant(&token) {
                const_value
            } else {
                parse_token(&token)?
            };
            eval_stack.push(value);
        }
    }

    if eval_stack.len() != 1 {
        return Err(-1);
    }

    Ok(eval_stack.pop().unwrap())
}
