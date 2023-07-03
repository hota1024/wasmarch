use alloc::vec::Vec;

use crate::instruction::Instruction;

#[derive(Debug, PartialEq)]
pub struct FuncBody {
    pub locals: Vec<types::ValueType>,
    pub body: Vec<Instruction>,
}

pub type CodeSection = Vec<FuncBody>;
