use alloc::vec::Vec;
use types::value_type::ValueType;

use crate::Instruction;

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub block_type: BlockType,
    pub body: Vec<Instruction>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockType {
    Empty,
    Value(ValueType),
    TypeIndex(u32),
}
