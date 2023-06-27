use alloc::{boxed::Box, string::String};

/// Import section.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/binary/modules.html#binary-importsec
pub type ImportSection = Box<[Import]>;

/// Import description.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/binary/modules.html#binary-importdesc
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImportDesc {
    Func(u32),
}

/// Import.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/binary/modules.html#binary-import
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Import {
    pub module: String,
    pub field: String,
    pub desc: ImportDesc,
}
