use alloc::boxed::Box;
use types::table_type::TableType;

/// Table section.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/binary/modules.html#table-section
pub type TableSection = Box<[TableType]>;
