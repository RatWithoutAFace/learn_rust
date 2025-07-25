use std::{io::stdin};

#[derive(PartialEq, Debug, Clone, Copy)]
enum Segment {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exp,
    Number(f64),
    LParen,
    RParen,
    None,
}

impl Segment {
    fn op_precedence(&self) -> u8 {
        match *self {
            Segment::Add => 1,
            Segment::Subtract => 1,
            Segment::Multiply => 2,
            Segment::Divide => 2,
            Segment::Exp => 3,
            Segment::LParen => 0,
            Segment::RParen => 0,
            Segment::Number(_) => 0,
            Segment::None => 0,
        }
    }
}

fn tokenize_string(string: String) -> Vec<Segment> {
    let mut string_vec: Vec<char> = string.chars().filter(|c| c.is_ascii_digit() || "+-*/^()".contains(*c)).collect();
    string_vec.push(' ');
    let mut output_vec: Vec<Segment> = Vec::new();
    let mut skip_iterations: u16 = 0;

    for (i, c) in string_vec.iter().enumerate() {
        if skip_iterations != 0 {
            skip_iterations = skip_iterations - 1;
            continue;
        } else if c.is_ascii_digit() {
            
            let mut num_string: String = String::new();
            if i > 0 {
                if i == 1 && string_vec[0] == '-' {
                    num_string.push('-');
                    output_vec.pop();
                } else if string_vec[i-1] == '-' && !string_vec[i-2].is_ascii_digit() {
                    num_string.push('-');
                    output_vec.pop();
                }
            }
            for (i2, c2) in string_vec[i..].iter().enumerate() {
                if c2.is_ascii_digit() || *c2 == '.' {
                    num_string.push(*c2)
                } else {
                    skip_iterations = skip_iterations + (i2 as u16 - 1);
                    output_vec.push(Segment::Number(num_string.parse::<f64>().unwrap()));
                    break;
                }
            }
        } else if *c == '+' {
            output_vec.push(Segment::Add);
            continue;
        } else if *c == '-' {
            output_vec.push(Segment::Subtract);
            continue;
        } else if *c == '*' {
            output_vec.push(Segment::Multiply);
            continue;
        } else if *c == '/' {
            output_vec.push(Segment::Divide);
            continue;
        } else if *c == '(' {
            output_vec.push(Segment::LParen);
            continue;
        } else if *c == ')' {
            output_vec.push(Segment::RParen);
            continue;
        } else if *c == '^' {
            output_vec.push(Segment::Exp);
            continue;
        } else {
            continue;
        }
    }

    if output_vec.is_empty() {
        output_vec.push(Segment::None);
    }

    return output_vec
}

fn prec_of_last(op_stack: &Vec<Segment>) -> u8 {
    op_stack.last().unwrap_or(&Segment::None).op_precedence()
}

fn exec_from_stacks(num_stack: &mut Vec<f64>, op_stack: &mut Vec<Segment>) -> f64 {
    let num2: f64 = num_stack.pop().unwrap_or(0.0);
    let num1: f64 = num_stack.pop().unwrap_or(0.0);
    let op: Segment = op_stack.pop().unwrap_or(Segment::None);

    if op == Segment::None {
        return 0.0
    } else if op == Segment::Add {
        return num1 + num2
    } else if op == Segment::Subtract {
        return num1 - num2
    } else if op == Segment::Multiply {
        return num1 * num2
    } else if op == Segment::Divide {
        return num1 / num2
    } else if op == Segment::Exp {
        return num1.powf(num2)
    } else {
        return 0.0
    }
}

fn calc_from_tokens(tokens: Vec<Segment>) -> Result<f64, String> {
    let mut num_stack: Vec<f64> = Vec::new();
    let mut op_stack: Vec<Segment> = Vec::new();
    for (_, t) in tokens.iter().enumerate() {
        // Push value to stack
        if let Segment::Number(n) = t {
            num_stack.push(*n);
        } else if t == &Segment::None {
            return Err("No tokens were given to calculate from!".to_string());
        } else if t == &Segment::RParen {
            while prec_of_last(&op_stack) != 0 {
                let result: f64 = exec_from_stacks(&mut num_stack, &mut op_stack);
                num_stack.push(result);
            }
            op_stack.pop();
        } else if t.op_precedence() >= prec_of_last(&op_stack) || t == &Segment::LParen {
            op_stack.push(*t);
        } else {
            while !(t.op_precedence() >= prec_of_last(&op_stack)) {
                let result: f64 = exec_from_stacks(&mut num_stack, &mut op_stack);
                num_stack.push(result);
            }
            op_stack.push(*t);
        }
    }
    
    if num_stack.len() == 1 {
        return Ok(num_stack[0]);
    } else if num_stack.len() != op_stack.len() + 1 {
        return Err("Stack size mismatch".to_string());
    } else {
        while num_stack.len() != 1 {
            let result: f64 = exec_from_stacks(&mut num_stack, &mut op_stack);
            num_stack.push(result);
        }
        return Ok(num_stack[0]);
    }
}

fn main() {
    let mut input: String = String::new();
    println!("Enter your expression:");
    stdin().read_line(&mut input).expect("Input could not be read.");
    let tokens: Vec<Segment> = tokenize_string(input.trim().to_string());
    let result = calc_from_tokens(tokens);
    match result {
        Ok(n) => println!("Result: {:?}", n),
        Err(msg) => println!("{}", msg),
    }
}