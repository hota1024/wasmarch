use alloc::vec::Vec;
use types::table_type::TableType;

/// Table section.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/binary/modules.html#table-section
pub type TableSection = Vec<TableType>;
