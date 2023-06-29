use alloc::boxed::Box;

pub struct Code {
    pub locals: Box<[types::ValueType]>,
}

pub type CodeSection = Box<[Code]>;
