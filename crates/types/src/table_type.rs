use crate::{limits::Limits, ref_type::RefType};

/// Table type.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/syntax/types.html#table-types
pub struct TableType {
    element_type: RefType,
    limits: Limits,
}
