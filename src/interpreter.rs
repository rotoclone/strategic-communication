use crate::{
    Error, OPERATIONS, operations, Opts, Program, REGISTER_NAMES, lib
};

use operations::{Operand, Transformation};
use std::collections::HashMap;
use std::io::Read;

/// A representation of the state of "memory" during the execution of a program.
#[derive(Debug)]
pub struct Context<'ctx> {
    /// Program being executed.
    program: &'ctx Program,
    /// Map of register names to their current values.
    registers: HashMap<String, i32>,
    /// The 0-indexed line number currently being executed.
    current_line_number: usize,
}

impl Context<'_> {
    pub fn run(program: &Program, _opts: &Opts) -> Result<(), Box<dyn std::error::Error>> {
        let mut context = Context {
            program: &program,
            registers: REGISTER_NAMES
                .iter()
                .map(|name| (name.to_string(), 0))
                .collect(),
            current_line_number: 0,
        };

        debug!("created context: {:?}", context);
        while context.current_line_number < context.program.source.len() {
            context.execute_current_line()?;
        }

        Ok(())
    }

    /// Executes the line at `source[current_line_number]` and sets `current_line_number` to the index of the next line to execute.
    fn execute_current_line(&mut self) -> Result<(), Error> {
        if self.current_line_number >= self.program.source.len() {
            return Err(Error::new("invalid line number", self.current_line_number));
        }

        let line = &self.program.source[self.current_line_number];
        debug!("executing line {}: {}", self.current_line_number, line);

        for op in OPERATIONS.iter() {
            if op.pattern.is_match(line) {
                let operands = op.pattern.replace(&line, "").to_string();
                trace!("registers before: {:?}", self.registers);
                (op.func)(&operands, self.current_line_number, self)?;
                trace!("registers after: {:?}", self.registers);
                self.current_line_number += 1;
                return Ok(());
            }
        }

        Err(Error::new("unexpected expression", self.current_line_number))
    }
    
    pub fn has_register(&self, name: &str) -> bool {
        return self.registers.contains_key(name);
    }

    pub fn has_label(&self, label: &str) -> bool {
        return self.program.labels.contains_key(label);
    }

    pub fn get_register_value(&self, name: &str) -> i32 {
        self.registers[name]
    }

    pub fn set_register_value(&mut self, name: &str, value: i32) {
        self.registers.insert(name.to_string(), value);
    }

    pub fn gen_modify_register(&mut self, name: &str, transformation: operations::Transformation) {
        let value = self.get_register_value(name);

        match transformation {
            Transformation::Add(Operand::Literal(literal)) => {
                self.set_register_value(name, value + (*literal));
            }
            Transformation::Add(Operand::Register(operand_reg)) => {
                let value2 = self.get_register_value(operand_reg);
                self.set_register_value(name, value + value2);
            }
            Transformation::Subtract(Operand::Literal(literal)) => {
                self.set_register_value(name, value - (*literal));
            }
            Transformation::Subtract(Operand::Register(operand_reg)) => {
                let value2 = self.get_register_value(operand_reg);
                self.set_register_value(name, value - value2);
            }
            Transformation::Multiply(Operand::Literal(literal)) => {
                self.set_register_value(name, value * (*literal));
            }
            Transformation::Multiply(Operand::Register(operand_reg)) => {
                let value2 = self.get_register_value(operand_reg);
                self.set_register_value(name, value * value2);
            }
            Transformation::Divide(Operand::Literal(literal)) => {
                self.set_register_value(name, value / (*literal));
            }
            Transformation::Divide(Operand::Register(operand_reg)) => {
                let value2 = self.get_register_value(operand_reg);
                self.set_register_value(name, value / value2);
            }
            Transformation::Set(Operand::Literal(literal)) => {
                self.set_register_value(name, *literal);
            }
            Transformation::Set(Operand::Register(src)) => {
                self.set_register_value(name, self.get_register_value(src));
            }
            _ => { panic!("Unhandled transformation!") }
        };
    }

    pub fn gen_print(&mut self, register: &str) {
        let value = self.get_register_value(register);
        lib::print_value(value);
    }

    pub fn gen_read(&mut self, register: &str) {
        let new_value = match std::io::stdin().bytes().next() {
            Some(b) => match b {
                Ok(b) => b as i32,
                Err(_) => -1,
            },
            None => -1,
        };
        self.set_register_value(register, new_value);
    }

    pub fn gen_label(&mut self, _name: &str) {
        // nothing to do here
    }

    pub fn gen_jump(&mut self, label: &str) {
        let line_number = self.program.labels[label];
        self.current_line_number = line_number;
    }

    pub fn gen_jump_if_zero(&mut self, register: &str, label: &str) {
        let value = self.get_register_value(register);
        if value == 0 {
            self.gen_jump(label);
        }
    }

    pub fn gen_jump_if_neg(&mut self, register: &str, label: &str) {
        let value = self.get_register_value(register);
        if value < 0 {
            self.gen_jump(label);
        }
    }

    pub fn gen_randomize(&mut self, register: &str) {
        self.set_register_value(register, lib::randomize());
    }
}