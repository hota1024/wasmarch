use alloc::vec::Vec;

use crate::instruction::Instruction;

#[derive(Debug, Clone, PartialEq)]
pub struct FuncBody {
    pub locals: Vec<types::ValueType>,
    pub code: Vec<Instruction>,
}

pub type CodeSection = Vec<FuncBody>;
