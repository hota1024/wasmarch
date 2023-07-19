use crate::SectionId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidMagicHeader,
    InvalidVersion,
    InvalidSectionId(SectionId),
    InvalidRefType,
    InvalidTableInstructionId,
    InvalidSubInstrId, // 0xFC xx
    UnexpectedEof,

    InvalidBlockType,

    InvalidTypeKind,

    InvalidImportDesc,
    InvalidExportDesc,

    InvalidGlobalInitExpr,
    InvalidLimitsKind,
    InvalidElementKind,

    ExpectedConstExpression,
    UnsupportedElementPrefix,

    Custom(String),
}

pub type Result<T> = std::result::Result<T, Error>;
