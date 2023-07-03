use crate::{limits::Limits, ref_type::RefType};

/// Table type.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/syntax/types.html#table-types
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableType {
    pub element_type: RefType,
    pub limits: Limits,
}
