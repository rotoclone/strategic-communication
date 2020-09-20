use crate::{Context, OpResult, RuntimeError, LITERALS, REGISTER_NAMES};
use regex::Regex;
use std::collections::hash_map::Entry::Occupied;

pub fn no_op(operands: &str, mut context: &mut Context) -> OpResult {
    debug!("no op with operands: {}", operands);
    Ok(())
}

pub fn increment(operands: &str, mut context: &mut Context) -> OpResult {
    debug!("increment with operands: {}", operands);

    Ok(modify_register(
        operands,
        Transformation::Add(1),
        &mut context,
    )?)
}

pub fn decrement(operands: &str, mut context: &mut Context) -> OpResult {
    debug!("decrement with operands: {}", operands);

    Ok(modify_register(
        operands,
        Transformation::Add(-1),
        &mut context,
    )?)
}

pub fn negate(operands: &str, mut context: &mut Context) -> OpResult {
    debug!("negate with operands: {}", operands);

    Ok(modify_register(
        operands,
        Transformation::Multiply(-1),
        &mut context,
    )?)
}

pub fn double(operands: &str, mut context: &mut Context) -> OpResult {
    debug!("double with operands: {}", operands);

    Ok(modify_register(
        operands,
        Transformation::Multiply(2),
        &mut context,
    )?)
}

pub fn halve(operands: &str, mut context: &mut Context) -> OpResult {
    debug!("halve with operands: {}", operands);

    Ok(modify_register(
        operands,
        Transformation::Divide(2),
        &mut context,
    )?)
}

pub fn zero(operands: &str, mut context: &mut Context) -> OpResult {
    debug!("zero with operands: {}", operands);

    Ok(modify_register(
        operands,
        Transformation::Set(0),
        &mut context,
    )?)
}

pub fn assign(operands: &str, mut context: &mut Context) -> OpResult {
    debug!("assignment with operands: {}", operands);

    let operands = parse_operands(operands)?;
    // should be a register followed by a register or literal
    if operands.len() != 2 {
        return Err(RuntimeError::new(
            "wrong number of operands for assignment",
            context,
        ));
    }

    let to_register = match &operands[0] {
        Operand::Register(name) => name,
        _ => {
            return Err(RuntimeError::new(
                "first operand for assignment must be a register",
                context,
            ))
        }
    };

    let new_value = match &operands[1] {
        Operand::Register(name) => get_register_value(name, context)?,
        Operand::Literal(val) => *val,
        _ => {
            return Err(RuntimeError::new(
                "second operand for assignment must be a register or literal",
                context,
            ))
        }
    };

    Ok(modify_register(
        to_register,
        Transformation::Set(new_value),
        &mut context,
    )?)
}

pub fn print(operands: &str, context: &mut Context) -> OpResult {
    debug!("print with operands: {}", operands);

    let operands = parse_operands(operands)?;
    // should be a register
    if operands.len() != 1 {
        return Err(RuntimeError::new(
            "wrong number of operands for print",
            context,
        ));
    }

    let register = match &operands[0] {
        Operand::Register(name) => name,
        _ => {
            return Err(RuntimeError::new(
                "operand for print must be a register",
                context,
            ))
        }
    };

    let to_print = get_register_value(register, context)?;
    if to_print < 0 {
        return Err(RuntimeError::new(
            &format!(
                "{} does not correspond to a valid UTF-8 character",
                to_print
            ),
            context,
        ));
    }

    match std::char::from_u32(to_print as u32) {
        Some(c) => print!("{}", c),
        _ => {
            return Err(RuntimeError::new(
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

pub fn jump_if_neg(operands: &str, context: &mut Context) -> OpResult {
    debug!("jump if negative with operands: {}", operands);

    let operands = parse_operands(operands)?;
    // should be a register and a label
    if operands.len() != 2 {
        return Err(RuntimeError::new(
            "wrong number of operands for jump if negative",
            context,
        ));
    }

    let register = match &operands[0] {
        Operand::Register(name) => name,
        _ => {
            return Err(RuntimeError::new(
                "first operand for jump if negative must be a register",
                context,
            ))
        }
    };

    let label = match &operands[1] {
        Operand::Label(name) => name,
        _ => {
            return Err(RuntimeError::new(
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

#[derive(Debug)]
enum Operand {
    Register(String),
    Literal(i32),
    Label(String),
}

fn parse_operands(operands: &str) -> Result<Vec<Operand>, RuntimeError> {
    let mut remaining_operands = operands.to_string();
    let mut parsed_operands = Vec::new();
    'outer: while !remaining_operands.is_empty() {
        trace!("remaining operands: {}", remaining_operands);
        for register_name in REGISTER_NAMES.iter() {
            if remaining_operands.starts_with(register_name) {
                parsed_operands.push(Operand::Register(register_name.to_string()));
                //TODO make this regex not be compiled a bunch of times
                let regex = Regex::new(&format!("^{}( and | with | to )?", register_name)).unwrap();
                remaining_operands = regex.replace(&remaining_operands, "").to_string();
                continue 'outer;
            }
        }

        for (literal_name, _) in LITERALS.iter() {
            if remaining_operands.starts_with(literal_name) {
                let parsed = parse_literal(&mut remaining_operands);
                parsed_operands.push(Operand::Literal(parsed));
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

fn parse_literal(operands: &mut String) -> i32 {
    let mut found_literals = Vec::new();
    'outer: while !operands.is_empty() {
        for (literal_name, literal_value) in LITERALS.iter() {
            if operands.starts_with(literal_name) {
                found_literals.push(*literal_value);
                //TODO make this regex not be compiled a bunch of times
                let regex = Regex::new(&format!("^{}(, and | and |, )?", literal_name)).unwrap();
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

fn get_register_value(name: &str, context: &Context) -> Result<i32, RuntimeError> {
    match context.registers.get(name) {
        Some(x) => Ok(*x),
        _ => Err(RuntimeError::new(
            &format!("invalid register name: {}", name),
            context,
        )),
    }
}

enum Transformation {
    Add(i32),
    Multiply(i32),
    Divide(i32),
    Set(i32),
}

fn modify_register(name: &str, transformation: Transformation, context: &mut Context) -> OpResult {
    let mut register = match context.registers.entry(name.to_string()) {
        Occupied(e) => e,
        _ => {
            return Err(RuntimeError::new(
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

fn jump_to_label(name: &str, mut context: &mut Context) -> OpResult {
    match context.labels.get(name) {
        Some(x) => context.current_line_number = *x,
        _ => {
            return Err(RuntimeError::new(
                &format!("unknown label: {}", name),
                context,
            ))
        }
    }

    Ok(())
}
