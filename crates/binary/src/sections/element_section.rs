use alloc::vec::Vec;
use types::ref_type::RefType;

/// Element section.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/binary/modules.html#element-section
pub type ElementSection = Vec<Element>;

#[derive(Clone, Debug, PartialEq)]
pub struct Element {
    pub kind: ElementKind,
    pub ref_type: RefType,
    pub init: Vec<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ElementKind {
    Passive,
    Active { table_index: u32, offset: Vec<u8> },
    Declarative,
}
