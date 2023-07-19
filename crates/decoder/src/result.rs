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
}

pub type Result<T> = std::result::Result<T, Error>;
