use crate::value_type::ValueType;

/// Global type.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/syntax/types.html#global-types
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GlobalType {
    value_type: ValueType,
    mutable: bool,
}
