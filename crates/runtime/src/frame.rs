use binary::Instruction;

use crate::value::Val;

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub pc: usize,
    pub sp: usize,
    pub instructions: Vec<Instruction>,
    pub locals: Vec<Val>,
}
