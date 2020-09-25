use crate::{
    Context, OpResult, Error, LITERALS, LITERAL_CONNECTORS, OPERAND_CONNECTORS,
    REGISTER_NAMES,
};
use rand::Rng;
use regex::Regex;
use std::collections::hash_map::Entry::Occupied;
use std::io::{Read, Write};

/// Does nothing.
pub fn no_op(operands: &str, _context: &mut Context) -> OpResult {
    debug!("no op with operands: {}", operands);
    Ok(())
}

/// Increments a register's value by 1.
pub fn increment(operands: &str, mut context: &mut Context) -> OpResult {
    debug!("increment with operands: {}", operands);

    Ok(modify_register(
        operands,
        Transformation::Add(1),
        &mut context,
    )?)
}

/// Decrements a register's value by 1.
pub fn decrement(operands: &str, mut context: &mut Context) -> OpResult {
    debug!("decrement with operands: {}", operands);

    Ok(modify_register(
        operands,
        Transformation::Add(-1),
        &mut context,
    )?)
}

/// Multiplies a register's value by -1.
pub fn negate(operands: &str, mut context: &mut Context) -> OpResult {
    debug!("negate with operands: {}", operands);

    Ok(modify_register(
        operands,
        Transformation::Multiply(-1),
        &mut context,
    )?)
}

/// Multiplies a register's value by 2.
pub fn double(operands: &str, mut context: &mut Context) -> OpResult {
    debug!("double with operands: {}", operands);

    Ok(modify_register(
        operands,
        Transformation::Multiply(2),
        &mut context,
    )?)
}

/// Divides a register's value by 2.
pub fn halve(operands: &str, mut context: &mut Context) -> OpResult {
    debug!("halve with operands: {}", operands);

    Ok(modify_register(
        operands,
        Transformation::Divide(2),
        &mut context,
    )?)
}

/// Sets a register's value to a random number between 0 and 9 (inclusive).
pub fn randomize(operands: &str, mut context: &mut Context) -> OpResult {
    debug!("randomize with operands: {}", operands);

    let random_number = rand::thread_rng().gen_range(0, 10);

    Ok(modify_register(
        operands,
        Transformation::Set(random_number),
        &mut context,
    )?)
}

/// Sets a register's value to the value in another register or a literal value.
pub fn assign(operands: &str, mut context: &mut Context) -> OpResult {
    debug!("assignment with operands: {}", operands);

    let operands = parse_operands(operands)?;
    // should be either a register followed by a register or literal, or a literal followed by a register
    if operands.len() != 2 {
        return Err(Error::new(
            "wrong number of operands for assignment",
            context,
        ));
    }

    match &operands[0] {
        Operand::Register(to_register) => {
            let new_value = match &operands[1] {
                Operand::Register(name) => get_register_value(name, context)?,
                Operand::Literal(val) => *val,
                _ => return Err(Error::new(
                    "second operand for assignment must be a register or literal",
                    context,
                ))
            };

            Ok(modify_register(
                to_register,
                Transformation::Set(new_value),
                &mut context,
            )?)
        },
        Operand::Literal(new_value) => {
            match &operands[1] {
                Operand::Register(to_register) => {
                    Ok(modify_register(
                        to_register,
                        Transformation::Set(*new_value),
                        &mut context,
                    )?)
                },
                _ => Err(Error::new(
                    "second operand for assignment must be a register if the first operand is a literal",
                    context,
                ))
            }
        },
        _ => Err(Error::new(
            "first operand for assignment must be a register or literal",
            context,
        ))
    }
}

/// Adds a register's value to another register's value.
pub fn add(operands: &str, mut context: &mut Context) -> OpResult {
    debug!("add with operands: {}", operands);

    let operands = parse_operands(operands)?;
    // should be a register followed by a register
    if operands.len() != 2 {
        return Err(Error::new(
            "wrong number of operands for add",
            context,
        ));
    }

    let register = match &operands[0] {
        Operand::Register(name) => name,
        _ => {
            return Err(Error::new(
                "first operand for add must be a register",
                context,
            ))
        }
    };

    let to_add = match &operands[1] {
        Operand::Register(name) => get_register_value(name, context)?,
        _ => {
            return Err(Error::new(
                "second operand for add must be a register",
                context,
            ))
        }
    };

    Ok(modify_register(
        register,
        Transformation::Add(to_add),
        &mut context,
    )?)
}

/// Subtracts a register's value from another register's value.
pub fn subtract(operands: &str, mut context: &mut Context) -> OpResult {
    debug!("subtract with operands: {}", operands);

    let operands = parse_operands(operands)?;
    // should be a register followed by a register
    if operands.len() != 2 {
        return Err(Error::new(
            "wrong number of operands for subtract",
            context,
        ));
    }

    let register = match &operands[0] {
        Operand::Register(name) => name,
        _ => {
            return Err(Error::new(
                "first operand for subtract must be a register",
                context,
            ))
        }
    };

    let to_sub = match &operands[1] {
        Operand::Register(name) => get_register_value(name, context)?,
        _ => {
            return Err(Error::new(
                "second operand for subtract must be a register",
                context,
            ))
        }
    };

    Ok(modify_register(
        register,
        Transformation::Add(-to_sub),
        &mut context,
    )?)
}

/// Reads a byte from stdin.
pub fn read(operands: &str, mut context: &mut Context) -> OpResult {
    debug!("read with operands: {}", operands);

    let new_value = match std::io::stdin().bytes().next() {
        Some(b) => match b {
            Ok(b) => b as i32,
            Err(e) => {
                return Err(Error::new(
                    &format!("error reading from stdin: {}", e),
                    context,
                ))
            }
        },
        None => -1,
    };

    Ok(modify_register(
        operands,
        Transformation::Set(new_value),
        &mut context,
    )?)
}

/// Prints a register's value.
pub fn print(operands: &str, context: &mut Context) -> OpResult {
    debug!("print with operands: {}", operands);

    let operands = parse_operands(operands)?;
    // should be a register
    if operands.len() != 1 {
        return Err(Error::new(
            "wrong number of operands for print",
            context,
        ));
    }

    let register = match &operands[0] {
        Operand::Register(name) => name,
        _ => {
            return Err(Error::new(
                "operand for print must be a register",
                context,
            ))
        }
    };

    let to_print = get_register_value(register, context)?;
    if to_print < 0 {
        return Err(Error::new(
            &format!(
                "{} does not correspond to a valid UTF-8 character",
                to_print
            ),
            context,
        ));
    }

    match std::char::from_u32(to_print as u32) {
        Some(c) => {
            print!("{}", c);
            std::io::stdout().flush().unwrap();
        }
        _ => {
            return Err(Error::new(
                &format!(
                    "{} does not correspond to a valid UTF-8 character",
                    to_print
                ),
                context,
            ))
        }
    }

    Ok(())
}

/// Jumps to a label.
pub fn jump(operands: &str, context: &mut Context) -> OpResult {
    debug!("jump with operands: {}", operands);

    jump_to_label(operands, context)
}

/// Jumps to a label if a register's value is 0.
pub fn jump_if_zero(operands: &str, context: &mut Context) -> OpResult {
    debug!("jump if zero with operands: {}", operands);

    let operands = parse_operands(operands)?;
    // should be a register and a label
    if operands.len() != 2 {
        return Err(Error::new(
            "wrong number of operands for jump if zero",
            context,
        ));
    }

    let register = match &operands[0] {
        Operand::Register(name) => name,
        _ => {
            return Err(Error::new(
                "first operand for jump if zero must be a register",
                context,
            ))
        }
    };

    let label = match &operands[1] {
        Operand::Label(name) => name,
        _ => {
            return Err(Error::new(
                "second operand for jump if zero must be a label",
                context,
            ))
        }
    };

    if get_register_value(register, context)? == 0 {
        jump_to_label(label, context)?;
    }

    Ok(())
}

/// Jumps to a label if a register's value is negative.
pub fn jump_if_neg(operands: &str, context: &mut Context) -> OpResult {
    debug!("jump if negative with operands: {}", operands);

    let operands = parse_operands(operands)?;
    // should be a register and a label
    if operands.len() != 2 {
        return Err(Error::new(
            "wrong number of operands for jump if negative",
            context,
        ));
    }

    let register = match &operands[0] {
        Operand::Register(name) => name,
        _ => {
            return Err(Error::new(
                "first operand for jump if negative must be a register",
                context,
            ))
        }
    };

    let label = match &operands[1] {
        Operand::Label(name) => name,
        _ => {
            return Err(Error::new(
                "second operand for jump if negative must be a label",
                context,
            ))
        }
    };

    if get_register_value(register, context)? < 0 {
        jump_to_label(label, context)?;
    }

    Ok(())
}

/// An operand for an operation.
#[derive(Debug)]
enum Operand {
    /// The name of a register.
    Register(String),
    /// A literal value.
    Literal(i32),
    /// The name of a label.
    Label(String),
}

/// Parses a string of operands to a list of `Operand`s.
fn parse_operands(operands: &str) -> Result<Vec<Operand>, Error> {
    let mut remaining_operands = operands.to_string();
    let mut parsed_operands = Vec::new();
    'outer: while !remaining_operands.is_empty() {
        trace!("remaining operands: {}", remaining_operands);
        for register_name in REGISTER_NAMES.iter() {
            if remaining_operands.starts_with(register_name) {
                parsed_operands.push(Operand::Register(register_name.to_string()));
                let regex = Regex::new(&format!(
                    "^{}({})?",
                    register_name,
                    OPERAND_CONNECTORS.join("|")
                ))
                .unwrap();
                remaining_operands = regex.replace(&remaining_operands, "").to_string();
                continue 'outer;
            }
        }

        for (literal_name, _) in LITERALS.iter() {
            if remaining_operands.starts_with(literal_name) {
                let parsed = parse_literal(&mut remaining_operands);
                parsed_operands.push(Operand::Literal(parsed));
                let regex = Regex::new(&format!("^({})?", OPERAND_CONNECTORS.join("|"))).unwrap();
                remaining_operands = regex.replace(&remaining_operands, "").to_string();
                continue 'outer;
            }
        }

        // didn't find any registers or literals, so it must be a label
        parsed_operands.push(Operand::Label(remaining_operands));
        remaining_operands = "".to_string();
    }
    debug!("parsed operands: {:?}", parsed_operands);
    Ok(parsed_operands)
}

/// Parses a literal value from a string of operands.
/// The string representation of the literal will be removed from the provided string.
fn parse_literal(operands: &mut String) -> i32 {
    let mut found_literals = Vec::new();
    'outer: while !operands.is_empty() {
        for (literal_name, literal_value) in LITERALS.iter() {
            if operands.starts_with(literal_name) {
                found_literals.push(*literal_value);
                let regex = Regex::new(&format!(
                    "^{}({})?",
                    literal_name,
                    LITERAL_CONNECTORS.join("|")
                ))
                .unwrap();
                *operands = regex.replace(operands, "").to_string();
                continue 'outer;
            }
        }

        // got to a non-literal, so stop parsing here
        break;
    }

    let mut combined: i32 = 0;
    let mut current_place = 1;
    for x in found_literals.iter().rev() {
        combined += (*x as i32) * current_place;
        current_place *= 10;
    }

    combined
}

/// Gets the value stored in the register with the provided name.
fn get_register_value(name: &str, context: &Context) -> Result<i32, Error> {
    match context.registers.get(name) {
        Some(x) => Ok(*x),
        _ => Err(Error::new(
            &format!("invalid register name: {}", name),
            context,
        )),
    }
}

/// A transformation to apply to a register's value.
enum Transformation {
    Add(i32),
    Multiply(i32),
    Divide(i32),
    Set(i32),
}

/// Modifies the register with the provided name using the provided `Transformation`.
fn modify_register(name: &str, transformation: Transformation, context: &mut Context) -> OpResult {
    let mut register = match context.registers.entry(name.to_string()) {
        Occupied(e) => e,
        _ => {
            return Err(Error::new(
                &format!("invalid register name: {}", name),
                context,
            ))
        }
    };

    match transformation {
        Transformation::Add(x) => register.insert(register.get() + x),
        Transformation::Multiply(x) => register.insert(register.get() * x),
        Transformation::Divide(x) => register.insert(register.get() / x),
        Transformation::Set(x) => register.insert(x),
    };

    Ok(())
}

/// Sets the provided context's `current_line_number` to the line the provided label is defined on.
fn jump_to_label(name: &str, mut context: &mut Context) -> OpResult {
    match context.labels.get(name) {
        Some(x) => context.current_line_number = *x,
        _ => {
            return Err(Error::new(
                &format!("unknown label: {}", name),
                context,
            ))
        }
    }

    Ok(())
}
