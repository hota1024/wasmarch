use crate::instances::ExportInst;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    DecodeError(decoder::Error),

    InvalidIndexForType(usize),
    InvalidIndexForFunc(usize),
    InvalidIndexForFuncType(usize),
    InvalidIndexForCode(usize),

    Trapped,

    ExpectedInstruction,
    ExpectedValue,
    ExpectedLabel,
    IfConditionShouldBeI32,

    LocalNotFound,
    GlobalNotFound,

    // invoke
    ExportNotFound(String),
    ExpectFuncAddr(ExportInst),

    // operations
    UndefinedBinaryOp,

    EmptyCallStack(String),

    UnexpectedEndOfInput,
}

pub type Result<T> = std::result::Result<T, Error>;
