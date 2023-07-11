use binary::Instruction;

use crate::value::Val;

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub locals: Vec<Val>,
    pub pc: usize,
    pub instructions: Vec<Instruction>,
}

impl Frame {
    pub fn new(locals: Vec<Val>, instructions: Vec<Instruction>) -> Self {
        Self {
            locals,
            pc: 0,
            instructions,
        }
    }
}
