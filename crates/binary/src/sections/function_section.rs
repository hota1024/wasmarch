use alloc::boxed::Box;

/// Function section.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/binary/modules.html#binary-funcsec
pub type FunctionSection = Box<[u32]>;
