use alloc::boxed::Box;
use types::limits::Limits;

/// Memory section.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/binary/modules.html#memory-section
pub type MemorySection = Box<[Limits]>;
