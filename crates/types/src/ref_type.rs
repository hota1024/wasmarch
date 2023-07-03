/// Ref type.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/syntax/types.html#syntax-reftype
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RefType {
    FuncRef,
    ExternRef,
}
