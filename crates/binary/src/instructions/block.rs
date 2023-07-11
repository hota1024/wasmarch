use types::value_type::ValueType;

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub block_type: BlockType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockType {
    Empty,
    Value(ValueType),
    TypeIndex(u32),
}
