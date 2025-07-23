use std::io::{self, stdin};

enum OperationType {
    Add,
    Subtract,
    Multiply,
    Divide,
}

struct Stacks {
    operations: Vec<Segment>,
    numbers: Vec<Segment>,
}

#[derive(PartialEq, Debug)]
enum Segment {
    Add,
    Subtract,
    Multiply,
    Divide,
    Number(i32),
    None,
}

struct Operation {
    method: OperationType,
    op1: i32,
    op2: i32
}

trait ExecuteOp {
    fn execute(&self) -> i32;
}

impl ExecuteOp for Operation {
    fn execute(&self) -> i32 {
        let result: i32;
        match &self.method {
            OperationType::Add => result = &self.op1 + &self.op2,
            OperationType::Subtract => result = &self.op1 - &self.op2,
            OperationType::Multiply => result = &self.op1 * &self.op2,
            OperationType::Divide => result = &self.op1 / &self.op2,
        }
        return result;
    }
}

fn enumerate_string(string: String) -> Stacks {
    let string_vec: Vec<char> = string.chars().collect();
    let mut operations_vec: Vec<Segment> = Vec::new();
    let mut numbers_vec: Vec<Segment> = Vec::new();
    let mut skip_iterations: u16 = 0;

    for (i, c) in string_vec.iter().enumerate() {
        if skip_iterations != 0 {
            skip_iterations = skip_iterations - 1;
            continue;
        } else if c.is_ascii_digit() {
            let mut num_string: String = String::new();
            for (i2, c2) in string_vec[i..].iter().enumerate() {
                if c2.is_ascii_digit() {
                    num_string.push(*c2)
                } else {
                    skip_iterations = skip_iterations + i2 as u16;
                    numbers_vec.push(Segment::Number(num_string.parse::<i32>().unwrap()));
                    break;
                }
            }
        } else if *c == '+' {
            operations_vec.push(Segment::Add);
            continue;
        } else if *c == '-' {
            operations_vec.push(Segment::Subtract);
            continue;
        } else if *c == '*' {
            operations_vec.push(Segment::Multiply);
            continue;
        } else if *c == '/' {
            operations_vec.push(Segment::Divide);
            continue;
        }
    }

    if operations_vec.is_empty() {
        operations_vec.push(Segment::None);
    }

    let output = Stacks { operations: operations_vec, numbers: numbers_vec };
    return output
}

fn 

fn main() {
    let mut input: String = String::new();
    println!("Enter your expression:");
    stdin().read_line(&mut input).expect("Input could not be read.");
    let mut enum_vec: Stacks = enumerate_string(input);
    

}