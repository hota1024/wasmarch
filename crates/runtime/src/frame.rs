use binary::Instruction;

use crate::{label::Label, value::Val};

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub locals: Vec<Val>,
    pub arity: usize,
    pub sp: usize,
    pub pc: usize,
    pub instructions: Vec<Instruction>,
    pub label_stack: Vec<Label>,
}

impl Frame {
    pub fn new(locals: Vec<Val>, arity: usize, sp: usize, instructions: Vec<Instruction>) -> Self {
        Self {
            locals,
            arity,
            sp,
            pc: 0,
            instructions,
            label_stack: vec![],
        }
    }
}
