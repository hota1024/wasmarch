/// Start section.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/binary/modules.html#start-section
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StartSection {
    pub func_index: Option<u32>,
}
