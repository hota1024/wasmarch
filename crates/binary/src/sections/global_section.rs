use alloc::vec::Vec;
use types::global_type::GlobalType;

/// Global section.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/binary/modules.html#global-section
#[derive(Clone, Debug, PartialEq)]
pub struct Global {
    pub global_type: GlobalType,
    pub init_expr: GlobalInitExpr,
}

/// Global initialization expression.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/syntax/types.html#global-types
#[derive(Clone, Debug, PartialEq)]
pub enum GlobalInitExpr {
    I32Const { value: i32 },
    I64Const { value: i64 },
    F32Const { value: f32 },
    F64Const { value: f64 },
}

/// Global section.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/binary/modules.html#global-section
pub type GlobalSection = Vec<Global>;
