use alloc::vec::Vec;

/// Function section.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/binary/modules.html#binary-funcsec
pub type FunctionSection = Vec<u32>;
