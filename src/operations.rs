use crate::{
    OpResult, Error, LITERALS, LITERAL_CONNECTORS, OPERAND_CONNECTORS,
    REGISTER_NAMES, Context
};
use regex::Regex;

/// Adds a label.
pub fn label(operands: &str, _line_number: usize, context: &mut Context) -> OpResult {
    debug!("label with operands: {}", operands);
    context.gen_label(operands);
    Ok(())
}

/// Increments a register's value by 1.
pub fn increment(operands: &str, line_number: usize, context: &mut Context) -> OpResult {
    debug!("increment with operands: {}", operands);

    Ok(modify_register(
        operands,
        Transformation::Add(&Operand::Literal(1)),
        line_number,
        context,
    )?)
}

/// Decrements a register's value by 1.
pub fn decrement(operands: &str, line_number: usize, context: &mut Context) -> OpResult {
    debug!("decrement with operands: {}", operands);

    Ok(modify_register(
        operands,
        Transformation::Add(&Operand::Literal(-1)),
        line_number,
        context,
    )?)
}

/// Multiplies a register's value by -1.
pub fn negate(operands: &str, line_number: usize, context: &mut Context) -> OpResult {
    debug!("negate with operands: {}", operands);

    Ok(modify_register(
        operands,
        Transformation::Multiply(&Operand::Literal(-1)),
        line_number,
        context,
    )?)
}

/// Multiplies a register's value by 2.
pub fn double(operands: &str, line_number: usize, context: &mut Context) -> OpResult {
    debug!("double with operands: {}", operands);

    Ok(modify_register(
        operands,
        Transformation::Multiply(&Operand::Literal(2)),
        line_number,
        context,
    )?)
}

/// Divides a register's value by 2.
pub fn halve(operands: &str, line_number: usize, context: &mut Context) -> OpResult {
    debug!("halve with operands: {}", operands);

    Ok(modify_register(
        operands,
        Transformation::Divide(&Operand::Literal(2)),
        line_number,
        context,
    )?)
}

/// Sets a register's value to a random number between 0 and 9 (inclusive).
pub fn randomize(operands: &str, line_number: usize, context: &mut Context) -> OpResult {
    debug!("randomize with operands: {}", operands);

    let operands = parse_operands(operands)?;
    // should be a register
    if operands.len() != 1 {
        return Err(Error::new(
            "wrong number of operands for randomize",
            line_number,
        ));
    }

    let register = match &operands[0] {
        Operand::Register(name) => name,
        _ => {
            return Err(Error::new(
                "operand for randomize must be a register",
                line_number,
            ))
        }
    };

    context.gen_randomize(register);
    Ok(())
}

/// Sets a register's value to the value in another register or a literal value.
pub fn assign(operands: &str, line_number: usize, context: &mut Context) -> OpResult {
    debug!("assignment with operands: {}", operands);

    let operands = parse_operands(operands)?;
    // should be either a register followed by a register or literal, or a literal followed by a register
    if operands.len() != 2 {
        return Err(Error::new(
            "wrong number of operands for assignment",
            line_number,
        ));
    }

    match &operands[0] {
        Operand::Register(to_register) => {
            let new_value = match &operands[1] {
                Operand::Register(_) => &operands[1],
                Operand::Literal(_) => &operands[1],
                _ => return Err(Error::new(
                    "second operand for assignment must be a register or literal",
                    line_number,
                ))
            };

            Ok(modify_register(
                to_register,
                Transformation::Set(new_value),
                line_number,
                context,
            )?)
        },
        Operand::Literal(_) => {
            match &operands[1] {
                Operand::Register(to_register) => {
                    Ok(modify_register(
                        to_register,
                        Transformation::Set(&operands[0]),
                        line_number,
                        context,
                    )?)
                },
                _ => Err(Error::new(
                    "second operand for assignment must be a register if the first operand is a literal",
                    line_number,
                ))
            }
        },
        _ => Err(Error::new(
            "first operand for assignment must be a register or literal",
            line_number,
        ))
    }
}

/// Adds a register's value to another register's value.
pub fn add(operands: &str, line_number: usize, context: &mut Context) -> OpResult {
    debug!("add with operands: {}", operands);

    let operands = parse_operands(operands)?;
    // should be a register followed by a register
    if operands.len() != 2 {
        return Err(Error::new(
            "wrong number of operands for add",
            line_number,
        ));
    }

    let register = match &operands[0] {
        Operand::Register(name) => name,
        _ => {
            return Err(Error::new(
                "first operand for add must be a register",
                line_number,
            ))
        }
    };

    let to_add = match &operands[1] {
        Operand::Register(_) => &operands[1],
        _ => {
            return Err(Error::new(
                "second operand for add must be a register",
                line_number,
            ))
        }
    };

    Ok(modify_register(
        register,
        Transformation::Add(to_add),
        line_number,
        context,
    )?)
}

/// Subtracts a register's value from another register's value.
pub fn subtract(operands: &str, line_number: usize, context: &mut Context) -> OpResult {
    debug!("subtract with operands: {}", operands);

    let operands = parse_operands(operands)?;
    // should be a register followed by a register
    if operands.len() != 2 {
        return Err(Error::new(
            "wrong number of operands for subtract",
            line_number,
        ));
    }

    let register = match &operands[0] {
        Operand::Register(name) => name,
        _ => {
            return Err(Error::new(
                "first operand for subtract must be a register",
                line_number,
            ))
        }
    };

    let to_sub = match &operands[1] {
        Operand::Register(_) => &operands[1],
        _ => {
            return Err(Error::new(
                "second operand for subtract must be a register",
                line_number,
            ))
        }
    };

    Ok(modify_register(
        register,
        Transformation::Subtract(to_sub),
        line_number,
        context,
    )?)
}

/// Reads a byte from stdin.
pub fn read(operands: &str, line_number: usize, context: &mut Context) -> OpResult {
    debug!("read with operands: {}", operands);

    let operands = parse_operands(operands)?;
    // should be a register
    if operands.len() != 1 {
        return Err(Error::new(
            "wrong number of operands for read",
            line_number,
        ));
    };

    let register = match &operands[0] {
        Operand::Register(name) => name,
        _ => {
            return Err(Error::new(
                "operand for read must be a register",
                line_number,
            ))
        }
    };

    context.gen_read(register);

    Ok(())
}

/// Prints a register's value.
pub fn print(operands: &str, line_number: usize, context: &mut Context) -> OpResult {
    debug!("print with operands: {}", operands);

    let operands = parse_operands(operands)?;
    // should be a register
    if operands.len() != 1 {
        return Err(Error::new(
            "wrong number of operands for print",
            line_number,
        ));
    }

    let register = match &operands[0] {
        Operand::Register(name) => name,
        _ => {
            return Err(Error::new(
                "operand for print must be a register",
                line_number,
            ))
        }
    };

    context.gen_print(register);
    Ok(())
}

/// Jumps to a label.
pub fn jump(operands: &str, line_number: usize, context: &mut Context) -> OpResult {
    debug!("jump with operands: {}", operands);

    let operands = parse_operands(operands)?;
    // should be a label
    if operands.len() != 1 {
        return Err(Error::new(
            "wrong number of operands for jump",
            line_number,
        ));
    }

    let label = match &operands[0] {
        Operand::Label(name) => name,
        _ => {
            return Err(Error::new(
                "operand for jump must be a label",
                line_number,
            ))
        }
    };

    if !context.has_label(label) {
        return Err(Error::new(
            &format!("jump to undefined label “{}”", label),
            line_number,
        ))
    }

    context.gen_jump(label);
    Ok(())
}

/// Jumps to a label if a register's value is 0.
pub fn jump_if_zero(operands: &str, line_number: usize, context: &mut Context) -> OpResult {
    debug!("jump if zero with operands: {}", operands);

    let operands = parse_operands(operands)?;
    // should be a register and a label
    if operands.len() != 2 {
        return Err(Error::new(
            "wrong number of operands for jump if zero",
            line_number,
        ));
    }

    let register = match &operands[0] {
        Operand::Register(name) => name,
        _ => {
            return Err(Error::new(
                "first operand for jump if zero must be a register",
                line_number,
            ))
        }
    };

    let label = match &operands[1] {
        Operand::Label(name) => name,
        _ => {
            return Err(Error::new(
                "second operand for jump if zero must be a label",
                line_number,
            ))
        }
    };

    if !context.has_label(label) {
        return Err(Error::new(
            &format!("jump to undefined label “{}”", label),
            line_number,
        ))
    }

    context.gen_jump_if_zero(register, label);

    Ok(())
}

/// Jumps to a label if a register's value is negative.
pub fn jump_if_neg(operands: &str, line_number: usize, context: &mut Context) -> OpResult {
    debug!("jump if negative with operands: {}", operands);

    let operands = parse_operands(operands)?;
    // should be a register and a label
    if operands.len() != 2 {
        return Err(Error::new(
            "wrong number of operands for jump if negative",
            line_number,
        ));
    }

    let register = match &operands[0] {
        Operand::Register(name) => name,
        _ => {
            return Err(Error::new(
                "first operand for jump if negative must be a register",
                line_number,
            ))
        }
    };

    let label = match &operands[1] {
        Operand::Label(name) => name,
        _ => {
            return Err(Error::new(
                "second operand for jump if negative must be a label",
                line_number,
            ))
        }
    };

    if !context.has_label(label) {
        return Err(Error::new(
            &format!("jump to undefined label “{}”", label),
            line_number,
        ))
    }

    context.gen_jump_if_neg(register, label);

    Ok(())
}

/// An operand for an operation.
#[derive(Debug)]
pub enum Operand {
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

/// A transformation to apply to a register's value.
pub enum Transformation<'op> {
    Add(&'op Operand),
    Subtract(&'op Operand),
    Multiply(&'op Operand),
    Divide(&'op Operand),
    Set(&'op Operand),
}

/// Modifies the register with the provided name using the provided `Transformation`.
fn modify_register(name: &str, transformation: Transformation, line_number: usize, context: &mut Context) -> OpResult {
    if context.has_register(name) {
        context.gen_modify_register(name, transformation);
        Ok(())
    } else {
        Err(Error::new(
            &format!("invalid register name: {}", name),
            line_number,
        ))
    }
}
