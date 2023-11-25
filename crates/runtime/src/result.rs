use crate::instances::ExportInst;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    DecodeError(decoder::Error),

    InvalidIndexForType(usize),
    InvalidIndexForFunc(usize),
    InvalidIndexForFuncType(usize),
    InvalidIndexForCode(usize),
    InvalidIndexForMem(usize),
    InvalidIndexForGlobal(usize),

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
    ExpectMemAddr(ExportInst),
    ExpectGlobalAddr(ExportInst),

    // operations
    UndefinedBinaryOp,

    EmptyCallStack(String),

    UnexpectedEndOfInput,

    Custom(String),
}

pub type Result<T> = std::result::Result<T, Error>;
