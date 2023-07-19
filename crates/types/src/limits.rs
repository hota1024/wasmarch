/// Limits.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/syntax/types.html#limits
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Limits {
    pub min: u32,
    pub max: Option<u32>,
}
