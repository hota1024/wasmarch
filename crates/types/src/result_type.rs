use alloc::vec::Vec;

use crate::value_type::ValueType;

/// Result type.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/syntax/types.html#result-types
pub type ResultType = Vec<ValueType>;
