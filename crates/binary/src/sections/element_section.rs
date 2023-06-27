use alloc::boxed::Box;
use types::ref_type::RefType;

/// Element section.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/binary/modules.html#element-section
pub type ElementSection = Box<[Element]>;

#[derive(Clone, Debug, PartialEq)]
pub struct Element {
    pub kind: ElementKind,
    pub ref_type: RefType,
    pub init: Box<[u32]>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ElementKind {
    Passive,
    Active { table_index: u32, offset: Box<[u8]> },
    Declarative,
}
