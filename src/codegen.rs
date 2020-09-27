use crate::{
    Program, OPERATIONS, REGISTER_NAMES, operations
};
use inkwell::OptimizationLevel;
use inkwell::IntPredicate;
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::{Linkage, Module};
use inkwell::passes::{PassManager, PassManagerBuilder};
use inkwell::types::{IntType};
use inkwell::values::{IntValue, PointerValue, FunctionValue};
use operations::{Operand, Transformation};
use std::collections::HashMap;
use std::error::Error;

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
    register_type: IntType<'ctx>,
    registers: HashMap<String, PointerValue<'ctx>>,
    labels: HashMap<String, BasicBlock<'ctx>>,
}

type EntryPoint = unsafe extern "C" fn();

pub fn run(program: &Program, print_ir: bool, optimization_level: OptimizationLevel) -> Result<(), Box<dyn Error>> {
    let context = Context::create();
    let module = context.create_module("business");
    let execution_engine = module.create_jit_execution_engine(optimization_level)?;    
    let builder = context.create_builder();
    let register_type = context.i32_type();

    // add builtins
    module.add_function("print_value", context.i32_type().fn_type(&[context.i32_type().into()], false), Some(Linkage::External));
    module.add_function("getchar", context.i32_type().fn_type(&[], false), Some(Linkage::External));
    module.add_function("randomize", context.i32_type().fn_type(&[], false), Some(Linkage::External));

    let mut codegen = CodeGen {
        context: &context,
        module: module,
        builder: builder,
        execution_engine: execution_engine,
        register_type: register_type,
        registers: HashMap::new(),
        labels: HashMap::new()
    };

    // optimize
    let pass_manager_builder = PassManagerBuilder::create();
    pass_manager_builder.set_optimization_level(optimization_level);
    let fpm = PassManager::create(&codegen.module);
    pass_manager_builder.populate_function_pass_manager(&fpm);

    // compile
    codegen.compile(&program)?;

    // optimize
    if let Some(function) = codegen.module.get_function("main") {
        println!("Running optimizer");
        fpm.run_on(&function);
    }
    
    

    // print module
    if print_ir {
        codegen.module.print_to_stderr();
    }

    // run program
    unsafe {
        let function: JitFunction<EntryPoint> = codegen.execution_engine.get_function("main")?;
        function.call();
    }

    Ok(())
}

impl<'ctx> CodeGen<'ctx> {
    fn create_basic_blocks(&mut self, function: FunctionValue<'ctx>, labels: &HashMap<String, usize>) {
        // basic block for entry point
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);

        // basic blocks for labels, in line order
        let labels_by_line: HashMap<usize, String> = labels.iter().map(|(label, line)| (*line, label.to_string())).collect();
        let mut lines_with_labels: Vec<_> = labels.values().map(|x|x).collect();
        lines_with_labels.sort();
        for line in lines_with_labels.iter() {
            let label = &labels_by_line[line];
            let block = self.context.append_basic_block(function, &label);
            self.labels.insert(label.to_string(), block);
        }
    }

    fn compile(&mut self, program: &Program) -> Result<(), Box<dyn Error>> {
        // create function
        let fn_type = self.context.void_type().fn_type(&[], false);
        let function = self.module.add_function("main", fn_type, None);

        // create basic blocks
        self.create_basic_blocks(function, &program.labels);

        // allocate registers
        self.registers = REGISTER_NAMES
            .iter()
            .map(|name| (name.to_string(), self.builder.build_alloca(self.register_type, name)))
            .collect();

        // compile code
        for i in 0..program.source.len() {        
            let line = &program.source[i];
            for op in OPERATIONS.iter() {
                if op.pattern.is_match(line) {
                    let operands = op.pattern.replace(&line, "").to_string();
                    (op.func)(&operands, i, &self)?;
                }
            }
        }
    
        // end function
        self.builder.build_return(None);

        Ok(())
    }

    pub fn has_register(&self, name: &str) -> bool {
        return self.registers.contains_key(name);
    }

    pub fn has_label(&self, label: &str) -> bool {
        return self.labels.contains_key(label);
    }

    fn const_int(&self, value: i32) -> IntValue<'ctx> {
        return self.register_type.const_int(value as u32 as u64, true);
    }

    fn op_reg_imm(&self, register: PointerValue<'ctx>, operand: i32, build_func: fn(&Builder<'ctx>, IntValue<'ctx>, IntValue<'ctx>, &str) -> IntValue<'ctx>) {
        let value = self.builder.build_load(register, "value").into_int_value();
        let value = (build_func)(&self.builder, value, self.const_int(operand), "value");
        self.builder.build_store(register, value);
    }

    fn op_reg_reg(&self, register: PointerValue<'ctx>, operand: &str, build_func: fn(&Builder<'ctx>, IntValue<'ctx>, IntValue<'ctx>, &str) -> IntValue<'ctx>) {
        let operand_reg = self.registers.get(&operand.to_string()).unwrap();
        let value = self.builder.build_load(register, "value").into_int_value();
        let operand_value = self.builder.build_load(*operand_reg, "operand").into_int_value();
        let value = (build_func)(&self.builder, value, operand_value, "value");
        self.builder.build_store(register, value);
    }
    
    pub fn gen_modify_register(&self, name: &str, transformation: operations::Transformation) {
        let register = self.registers.get(&name.to_string()).unwrap();

        match transformation {
            Transformation::Add(Operand::Literal(literal)) => {
                self.op_reg_imm(*register, *literal, Builder::build_int_add);
            }
            Transformation::Add(Operand::Register(operand_reg)) => {
                self.op_reg_reg(*register, operand_reg, Builder::build_int_add);
            }
            Transformation::Subtract(Operand::Literal(literal)) => {
                self.op_reg_imm(*register, *literal, Builder::build_int_sub);
            }
            Transformation::Subtract(Operand::Register(operand_reg)) => {
                self.op_reg_reg(*register, operand_reg, Builder::build_int_sub);
            }
            Transformation::Multiply(Operand::Literal(literal)) => {
                self.op_reg_imm(*register, *literal, Builder::build_int_mul);
            }
            Transformation::Multiply(Operand::Register(operand_reg)) => {
                self.op_reg_reg(*register, operand_reg, Builder::build_int_mul);
            }
            Transformation::Divide(Operand::Literal(literal)) => {
                self.op_reg_imm(*register, *literal, Builder::build_int_signed_div);
            }
            Transformation::Divide(Operand::Register(operand_reg)) => {
                self.op_reg_reg(*register, operand_reg, Builder::build_int_signed_div);
            }
            Transformation::Set(Operand::Literal(literal)) => {
                self.builder.build_store(*register, self.const_int(*literal));
            }
            Transformation::Set(Operand::Register(src)) => {
                let src_register = self.registers.get(&src.to_string()).unwrap();
                let value = self.builder.build_load(*src_register, "value");
                self.builder.build_store(*register, value);
            }
            _ => { panic!("Unhandled transformation!") }
        };
    }

    pub fn gen_print(&self, register: &str) {
        let register = self.registers.get(&register.to_string()).unwrap();
        let value = self.builder.build_load(*register, "value");
        self.builder.build_call(self.module.get_function("print_value").unwrap(), &[value], "print");
    }

    pub fn gen_read(&self, register: &str) {
        let register = self.registers.get(&register.to_string()).unwrap();
        let result = self.builder.build_call(self.module.get_function("getchar").unwrap(), &[], "read")
            .try_as_basic_value()
            .left()
            .unwrap();
        self.builder.build_store(*register, result);
    }

    pub fn gen_label(&self, name: &str) {
        let current_block = self.builder.get_insert_block().unwrap();
        let basic_block = self.labels[name];
        if current_block.get_terminator() == None {
            self.builder.build_unconditional_branch(basic_block);
        }
        self.builder.position_at_end(basic_block);
    }

    pub fn gen_jump(&self, label: &str) {
        let branch_block = self.labels[label];
        self.builder.build_unconditional_branch(branch_block);
    }

    fn gen_cond_zero_jump(&self, register: &str, cond: IntPredicate, label: &str) {
        // create a new basic block at the current insertion point
        // to be used as an "else block"
        let current_block = self.builder.get_insert_block().unwrap();
        let else_block_label = format!("{}'", current_block.get_name().to_str().unwrap());
        let else_block = self.context.insert_basic_block_after(current_block, &else_block_label);
        let then_block = self.labels[label];

        // comparison
        let register = self.registers.get(&register.to_string()).unwrap();
        let value = self.builder.build_load(*register, "value").into_int_value();
        let cond = self.builder.build_int_compare(cond, value, self.register_type.const_zero(), "cmp");
        
        self.builder.build_conditional_branch(cond, then_block, else_block);
        self.builder.position_at_end(else_block);
    }

    pub fn gen_jump_if_zero(&self, register: &str, label: &str) {
        self.gen_cond_zero_jump(register, IntPredicate::EQ, label);
    }

    pub fn gen_jump_if_neg(&self, register: &str, label: &str) {
        self.gen_cond_zero_jump(register, IntPredicate::SLT, label);
    }

    pub fn gen_randomize(&self, register: &str) {
        let register = self.registers.get(&register.to_string()).unwrap();
        let result = self.builder.build_call(self.module.get_function("randomize").unwrap(), &[], "randomize")
            .try_as_basic_value()
            .left()
            .unwrap();
        self.builder.build_store(*register, result);
    }
}
