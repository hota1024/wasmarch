/// Limits.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/syntax/types.html#limits
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Limits {
    min: u32,
    max: Option<u32>,
}
