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
