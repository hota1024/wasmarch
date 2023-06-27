use alloc::boxed::Box;

use crate::value_type::ValueType;

/// Result type.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/syntax/types.html#result-types
pub type ResultType = Box<[ValueType]>;
