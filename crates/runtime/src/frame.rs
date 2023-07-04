use binary::Instruction;

use crate::value::Val;

pub struct Frame {
    pub pc: usize,
    pub sp: usize,
    pub insts: Vec<Instruction>,
    pub locals: Vec<Val>,
}
