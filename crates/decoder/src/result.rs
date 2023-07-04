use crate::SectionId;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidMagicHeader,
    InvalidVersion,
    InvalidSectionId(SectionId),
    UnexpectedEof,

    InvalidTypeKind,

    InvalidImportDesc,
    InvalidExportDesc,
}

pub type Result<T> = std::result::Result<T, Error>;
