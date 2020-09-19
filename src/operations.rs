use crate::{Context, OpResult, RuntimeError};
use std::collections::hash_map::Entry::Occupied;

pub fn no_op(operands: &str, mut context: &mut Context) -> OpResult {
    debug!("no op with operands: {}", operands);
    trace!("context: {:?}", context);
    Ok(())
}

pub fn assign(operands: &str, mut context: &mut Context) -> OpResult {
    debug!("assignment with operands: {}", operands);
    trace!("context: {:?}", context);

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

pub fn negate(operands: &str, mut context: &mut Context) -> OpResult {
    debug!("negation with operands: {}", operands);
    trace!("context: {:?}", context);

    Ok(modify_register(
        operands,
        Transformation::Multiply(-1),
        &mut context,
    )?)
}

pub fn print(operands: &str, mut context: &mut Context) -> OpResult {
    debug!("print with operands: {}", operands);
    trace!("context: {:?}", context);

    //TODO
    Ok(())
}

pub fn increment(operands: &str, mut context: &mut Context) -> OpResult {
    debug!("increment with operands: {}", operands);
    trace!("context: {:?}", context);

    Ok(modify_register(
        operands,
        Transformation::Add(1),
        &mut context,
    )?)
}

pub fn decrement(operands: &str, mut context: &mut Context) -> OpResult {
    debug!("decrement with operands: {}", operands);
    trace!("context: {:?}", context);

    Ok(modify_register(
        operands,
        Transformation::Add(-1),
        &mut context,
    )?)
}

pub fn jump_if_neg(operands: &str, mut context: &mut Context) -> OpResult {
    debug!("jump if negative with operands: {}", operands);
    trace!("context: {:?}", context);

    //TODO
    Ok(())
}

enum Operand {
    Register(String),
    Literal(i32),
    Label(String),
}

fn parse_operands(operands: &str) -> Result<Vec<Operand>, RuntimeError> {
    Ok(vec![]) //TODO
}

fn get_register_value(name: &str, context: &mut Context) -> Result<i32, RuntimeError> {
    match context.registers.entry(name.to_string()) {
        Occupied(e) => Ok(*e.get()),
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
