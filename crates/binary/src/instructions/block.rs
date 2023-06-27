use alloc::boxed::Box;
use types::value_type::ValueType;

pub struct Block {
    pub block_type: BlockType,
}

pub enum BlockType {
    Empty,
    Value(ValueType),
    TypeIndex(u32),
}
