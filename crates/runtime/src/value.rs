use types::ValueType;

#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    None,
}

impl Val {
    pub fn is_true(&self) -> bool {
        match self {
            Val::I32(val) => *val != 0,
            _ => false,
        }
    }

    pub fn default_of(ty: &ValueType) -> Self {
        match ty {
            ValueType::I32 => Val::I32(0),
            ValueType::I64 => Val::I64(0),
            ValueType::F32 => Val::F32(0.0),
            ValueType::F64 => Val::F64(0.0),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExternalVal {
    FuncAddr(usize),
    TableAddr(usize),
    MemAddr(usize),
    GlobalAddr(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ref {
    Null,
    Func(usize),
    Extern(usize),
}
