use alloc::vec::Vec;
use types::limits::Limits;

/// Memory section.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/binary/modules.html#memory-section
pub type MemorySection = Vec<Limits>;
