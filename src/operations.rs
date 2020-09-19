use crate::{Context, OpResult, RuntimeError};

pub fn no_op(operands: &str, context: &Context) -> OpResult {
    debug!("no op with operands: {}", operands);
    Ok(())
}

pub fn assign(operands: &str, context: &Context) -> OpResult {
    debug!("assignment with operands: {}", operands);

    //TODO
    Ok(())
}

pub fn negate(operands: &str, context: &Context) -> OpResult {
    debug!("negation with operands: {}", operands);

    //TODO
    Ok(())
}

pub fn print(operands: &str, context: &Context) -> OpResult {
    debug!("print with operands: {}", operands);

    //TODO
    Ok(())
}

pub fn increment(operands: &str, context: &Context) -> OpResult {
    debug!("increment with operands: {}", operands);

    //TODO
    Ok(())
}

pub fn decrement(operands: &str, context: &Context) -> OpResult {
    debug!("decrement with operands: {}", operands);

    //TODO
    Ok(())
}

pub fn jump_if_neg(operands: &str, context: &Context) -> OpResult {
    debug!("jump if negative with operands: {}", operands);

    //TODO
    Ok(())
}
