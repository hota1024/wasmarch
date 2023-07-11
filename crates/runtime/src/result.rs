use crate::instances::ExportInst;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    DecodeError(decoder::Error),

    InvalidIndexForType(usize),
    InvalidIndexForFunc(usize),
    InvalidIndexForFuncType(usize),
    InvalidIndexForCode(usize),
    IfConditionShouldBeI32,

    // invoke
    ExportNotFound(String),
    ExpectFuncAddr(ExportInst),

    UnexpectedEndOfInput,
}

pub type Result<T> = std::result::Result<T, Error>;
