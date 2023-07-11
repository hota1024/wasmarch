use binary::BlockType;

#[derive(Debug, Clone, PartialEq)]
pub enum LabelKind {
    If,
    Block,
    Loop,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    pub kind: LabelKind,
    pub pc: usize,
    pub result_type: BlockType,
}
