use alloc::{boxed::Box, string::String};

/// Export section.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/binary/modules.html#export-section
pub type ExportSection = Box<[Export]>;

#[derive(Clone, Debug, PartialEq)]
pub struct Export {
    pub name: String,
    pub desc: ExportDesc,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExportDesc {
    Func(u32),
    Table(u32),
    Mem(u32),
    Global(u32),
}
