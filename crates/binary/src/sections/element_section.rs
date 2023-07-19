use alloc::vec::Vec;
use types::ref_type::RefType;

use crate::Instruction;

/// Element section.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/binary/modules.html#element-section
pub type ElementSection = Vec<Element>;

#[derive(Clone, Debug, PartialEq)]
pub struct Element {
    pub mode: ElementMode,
    pub ref_type: RefType,
    // pub init: Vec<Instruction>,
    pub init: Vec<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ElementMode {
    Passive,
    Active {
        table_index: u32,
        // offset: Instruction,
        offset: i32,
    },
    Declarative,
}
