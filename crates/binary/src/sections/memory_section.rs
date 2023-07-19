use alloc::vec::Vec;
use types::MemoryType;

/// Memory section.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/binary/modules.html#memory-section
pub type MemorySection = Vec<MemoryType>;
