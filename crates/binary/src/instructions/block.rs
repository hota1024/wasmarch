use types::value_type::ValueType;

#[derive(Debug, PartialEq)]
pub struct Block {
    pub block_type: BlockType,
}

#[derive(Debug, PartialEq)]
pub enum BlockType {
    Empty,
    Value(ValueType),
    TypeIndex(u32),
}
