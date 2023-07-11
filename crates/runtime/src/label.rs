use binary::BlockType;

#[derive(Debug, Clone, PartialEq)]
pub enum LabelKind {
    If,
    Block,
    Loop,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    pub pc: usize,
    pub kind: LabelKind,
    pub result_type: BlockType,
}
