use alloc::boxed::Box;

use crate::instruction::Instruction;

#[derive(Debug, PartialEq)]
pub struct FuncBody {
    pub locals: Box<[types::ValueType]>,
    pub body: Box<[Instruction]>,
}

pub type CodeSection = Box<[FuncBody]>;
