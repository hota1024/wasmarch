use crate::result_type::ResultType;

/// Function type.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/syntax/types.html#syntax-functype
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FuncType {
    pub params: ResultType,
    pub results: ResultType,
}
