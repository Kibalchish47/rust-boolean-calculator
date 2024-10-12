// Exercise for the reader: Implement boolean logic using only the type system.
// Use the notes about Values, Types as sets of Values, Traits as sets of Types,
// assoc. types as maps upon Types, etc. for your implmentation.
// ----
// Notes:
// Type: Sets of Values
// Functions: Maps of types: A -> B

// Traits: Sets of Types
// Traits (Associated Types):       Map: Type -> Type
// Traits (Associated Constants):   Map: Type -> Value

// Pointers: Map: value -> value

// FromStr: {u8, u16, f64, f32, ... }
// FromStr::Err {u8, u16, f64, f32 } -> { U8Error, U16Error, F64Error, }

// FnOnce (trait) - functions that you can only call once.
// FnMut (trait) - functions that
// Fn (trait) - functions that can inspect their environment.
// fn (trait, but also not) - pure functions.

use std::{
    io::Write,
    ops::{BitAnd, BitOr},
    str::FromStr,
};

pub type Boolean = bool;

pub enum Operation {
    // Replace all operations with logical operations
    And,
    Or,
    Nand,
    Nor,
    Xand,
    Xor,
    Xnand,
    Xnor,
}

impl Operation {
    fn op(&self) -> fn(Boolean, Boolean) -> Boolean {
        match self {
            // closures are not ideal here but they get the job done
            Operation::And => Boolean::bitand,
            Operation::Or => Boolean::bitor,
            Operation::Nand => |a, b| !(a & b),
            Operation::Nor => |a, b| !(a | b),
            Operation::Xand => |a, b| !(a && b),
            Operation::Xor => |a, b| (a ^ b),
            Operation::Xnand => |a, b| !(a ^ b),
            Operation::Xnor => |a, b| (a == b),
        }
    }
}

pub struct OperationParseError;

impl FromStr for Operation {
    type Err = OperationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "and" => Operation::And,
            "or" => Operation::Or,
            "nand" => Operation::Nand,
            "nor" => Operation::Nor,
            "xand" => Operation::Xand,
            "xor" => Operation::Xor,
            "xnand" => Operation::Xnand,
            "xnor" => Operation::Xnor,
            _ => return Err(OperationParseError),
        })
    }
}

// Values
// 'static -- maximal lifetime \exists 'static \in LTs (\forall 'lt \in LTs 'static >= 'lt)

fn parse<F, T>(msg: &str, validator: F) -> Result<T, Box<dyn std::error::Error>>
where
    T: FromStr,
    F: Fn(&str) -> Result<T, T::Err>,
{
    let mut input = String::new();

    loop {
        print!("{msg}");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut input)?;
        if let Ok(op) = validator(input.trim()) {
            return Ok(op);
        }

        input.clear();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut memory: bool = true;

    println!("BOOLEAN EVALUATOR!");
    println!("Special keywords (enter in the input sections): 'mem', 'not', and 'exit'.");
    println!("'MEM' -> use previous output in the input.");
    println!("'not' -> negates the input (i.e. 'not True' -> 'False').");
    println!("'exit' -> safely exits the program.");

    loop {
        let func = |s: &str| {
            if s.to_uppercase() == "MEM" {
                Ok(memory)
            } else if s.to_uppercase() == "NOT FALSE" {
                Ok(true)
            } else if s.to_uppercase() == "NOT TRUE" {
                Ok(false)
            } else if s.to_uppercase() == "EXIT" {
                std::process::exit(0);
            } else {
                Boolean::from_str(s)
            }
        };
        println!("-------------------------------------------------");
        let op: Operation = parse(
            "Enter the operation (and, or, nand, nor): ",
            Operation::from_str,
        )?;
        let a: Boolean = parse("Enter the first input: ", func)?;
        let b: Boolean = parse("Enter the second input: ", func)?;

        memory = op.op()(a, b);
        println!("Result: {}", memory);
    }
}
