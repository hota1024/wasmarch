use alloc::vec::Vec;
use types::global_type::GlobalType;

/// Global section.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/binary/modules.html#global-section
#[derive(Clone, Debug, PartialEq)]
pub struct Global {
    global_type: GlobalType,
    init_expr: GlobalInitExpr,
}

/// Global initialization expression.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/syntax/types.html#global-types
#[derive(Clone, Debug, PartialEq)]
pub enum GlobalInitExpr {
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),
}

/// Global section.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/binary/modules.html#global-section
pub type GlobalSection = Vec<Global>;
