use binary::BlockType;

#[derive(Debug, Clone, PartialEq)]
pub enum LabelKind {
    If,
    Block,
    Loop { start_pc: usize },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    pub pc: usize,
    pub sp: usize,
    pub arity: usize,
    pub kind: LabelKind,
    pub result_type: BlockType,
}
