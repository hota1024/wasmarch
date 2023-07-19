use core::panic;

use crate::rust_value::*;
use types::ValueType;

use crate::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    None,
}

macro_rules! uniop {
    (@all, $($op: ident), *) => {
        $(
            pub fn $op(&self) -> Result<Self> {
                match self {
                    Val::I32(v) => Ok(Val::from(<i32 as IntOp>::$op(v)?)),
                    Val::I64(v) => Ok(Val::from(<i64 as IntOp>::$op(v)?)),
                    Val::F32(v) => Ok(Val::from(<f32 as FloatOp>::$op(v)?)),
                    Val::F64(v) => Ok(Val::from(<f64 as FloatOp>::$op(v)?)),
                    _ => panic!("cannot `{}` with {:?}", stringify!($op), self)
                }
            }
        )*
    };
    (@int, $($op: ident), *) => {
        $(
            pub fn $op(&self) -> Result<Self> {
                match self {
                    Val::I32(v) => Ok(Val::from(<i32 as IntOp>::$op(v)?)),
                    Val::I64(v) => Ok(Val::from(<i64 as IntOp>::$op(v)?)),
                    _ => panic!("cannot `{}` with {:?}", stringify!($op), self)
                }
            }
        )*
    };
    (@float, $($op: ident), *) => {
        $(
            pub fn $op(&self) -> Result<Self> {
                match self {
                    Val::F32(v) => Ok(Val::from(<f32 as FloatOp>::$op(v)?)),
                    Val::F64(v) => Ok(Val::from(<f64 as FloatOp>::$op(v)?)),
                    _ => panic!("cannot `{}` with {:?}", stringify!($op), self)
                }
            }
        )*
    };
}

macro_rules! binop {
    (@all, $($op: ident), *) => {
        $(
            pub fn $op(&self, rhs: &Self) -> Result<Self> {
                match (self, rhs) {
                    (Val::I32(l), Val::I32(r)) => Ok(Val::from(<i32 as IntOp>::$op(l, r)?)),
                    (Val::I64(l), Val::I64(r)) => Ok(Val::from(<i64 as IntOp>::$op(l, r)?)),
                    (Val::F32(l), Val::F32(r)) => Ok(Val::from(<f32 as FloatOp>::$op(l, r)?)),
                    (Val::F64(l), Val::F64(r)) => Ok(Val::from(<f64 as FloatOp>::$op(l, r)?)),
                    _ => panic!("cannot `{}` with {:?} and {:?}", stringify!($op), self, rhs)
                }
            }
        )*
    };
    (@int, $($op: ident), *) => {
        $(
            pub fn $op(&self, rhs: &Self) -> Result<Self> {
                match (self, rhs) {
                    (Val::I32(l), Val::I32(r)) => Ok(Val::from(<i32 as IntOp>::$op(l, r)?)),
                    (Val::I64(l), Val::I64(r)) => Ok(Val::from(<i64 as IntOp>::$op(l, r)?)),
                    _ => panic!("cannot `{}` with {:?} and {:?}", stringify!($op), self, rhs)
                }
            }
        )*
    };
    (@float, $($op: ident), *) => {
        $(
            pub fn $op(&self, rhs: &Self) -> Result<Self> {
                match (self, rhs) {
                    (Val::F32(l), Val::F32(r)) => Ok(Val::from(<f32 as FloatOp>::$op(l, r)?)),
                    (Val::F64(l), Val::F64(r)) => Ok(Val::from(<f64 as FloatOp>::$op(l, r)?)),
                    _ => panic!("cannot `{}` with {:?} and {:?}", stringify!($op), self, rhs)
                }
            }
        )*
    };
}

macro_rules! into {
    ($ident: ident, $ty: ty) => {
        pub fn $ident(&self) -> $ty {
            match self {
                Self::I32(v) => *v as $ty,
                Self::I64(v) => *v as $ty,
                Self::F32(v) => *v as $ty,
                Self::F64(v) => *v as $ty,
                _ => panic!("cannot convert None to {:?}", stringify!($ty)),
            }
        }
    };
}

impl Val {
    uniop![@int, eqz, clz, ctz, popcnt];
    uniop![@float, abs, neg, ceil, floor, trunc, nearest, sqrt];

    binop![@all, add, sub, mul, eq, ne];
    binop![@int, div_s, div_u, lt_s, lt_u, gt_s, gt_u, le_s, le_u, ge_s, ge_u, rem_s, rem_u, and, or, xor, shl, shr_s, shr_u, rotl, rotr];
    binop![@float, lt, gt, le, ge, div, min, max, copysign];

    into!(into_i32, i32);
    into!(into_i64, i64);
    into!(into_f32, f32);
    into!(into_f64, f64);

    pub fn is_true(&self) -> bool {
        match self {
            Val::I32(val) => *val != 0,
            _ => false,
        }
    }

    pub fn is_false(&self) -> bool {
        match self {
            Val::I32(val) => *val == 0,
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

    pub fn i32_wrap_i64(&self) -> Result<Self> {
        match self {
            Self::I64(v) => Ok(Self::I32(*v as i32)),
            _ => panic!("cannot wrap"),
        }
    }

    pub fn i32_trunc_s(&self) -> Result<Self> {
        match self {
            Self::F32(v) => Ok(Self::I32(*v as i32)),
            Self::F64(v) => Ok(Self::I32(*v as i32)),
            _ => panic!("cannot trunc"),
        }
    }

    pub fn i32_trunc_u(&self) -> Result<Self> {
        match self {
            Self::F32(v) => Ok(Self::I32((*v as u32) as i32)),
            Self::F64(v) => Ok(Self::I32((*v as u64) as i32)),
            _ => panic!("cannot trunc"),
        }
    }

    pub fn i64_extend_i32_s(&self) -> Result<Self> {
        match self {
            Self::I32(v) => Ok(Self::I64(*v as i64)),
            _ => panic!("cannot extend"),
        }
    }

    pub fn i64_extend_i32_u(&self) -> Result<Self> {
        match self {
            Self::I32(v) => Ok(Self::I64((*v as u64) as i64)),
            _ => panic!("cannot extend"),
        }
    }

    pub fn f32_convert_s(&self) -> Result<Self> {
        match self {
            Self::I32(v) => Ok(Self::F32(*v as f32)),
            Self::I64(v) => Ok(Self::F32(*v as f32)),
            _ => panic!("cannot convert"),
        }
    }

    pub fn f32_convert_u(&self) -> Result<Self> {
        match self {
            Self::I32(v) => Ok(Self::F32((*v as u32) as f32)),
            Self::I64(v) => Ok(Self::F32((*v as u64) as f32)),
            _ => panic!("cannot convert"),
        }
    }

    pub fn f32_demote_f64(&self) -> Result<Self> {
        match self {
            Self::F64(v) => Ok(Self::F32(*v as f32)),
            _ => panic!("cannot demote"),
        }
    }

    pub fn f64_convert_s(&self) -> Result<Self> {
        match self {
            Self::I32(v) => Ok(Self::F64(*v as f64)),
            Self::I64(v) => Ok(Self::F64(*v as f64)),
            _ => panic!("cannot convert"),
        }
    }

    pub fn f64_convert_u(&self) -> Result<Self> {
        match self {
            Self::I32(v) => Ok(Self::F64((*v as u32) as f64)),
            Self::I64(v) => Ok(Self::F64((*v as u64) as f64)),
            _ => panic!("cannot convert"),
        }
    }

    pub fn f64_promote_f32(&self) -> Result<Self> {
        match self {
            Self::F32(v) => Ok(Self::F64(*v as f64)),
            _ => panic!("cannot promote"),
        }
    }

    pub fn i32_reinterpret_f32(&self) -> Result<Self> {
        match self {
            Self::F32(v) => Ok(Self::I32(*v as i32)),
            _ => panic!("cannot reinterpret"),
        }
    }

    pub fn i64_reinterpret_f64(&self) -> Result<Self> {
        match self {
            Self::F64(v) => Ok(Self::I64(*v as i64)),
            _ => panic!("cannot reinterpret"),
        }
    }

    pub fn f32_reinterpret_i32(&self) -> Result<Self> {
        match self {
            Self::I32(v) => Ok(Self::F32(*v as f32)),
            _ => panic!("cannot reinterpret"),
        }
    }

    pub fn f64_reinterpret_i64(&self) -> Result<Self> {
        match self {
            Self::I64(v) => Ok(Self::F64(*v as f64)),
            _ => panic!("cannot reinterpret"),
        }
    }

    pub fn i32_extend8_s(&self) -> Result<Self> {
        match self {
            Self::I32(v) => Ok(Self::I32((*v as i8) as i32)),
            _ => panic!("cannot extend"),
        }
    }

    pub fn i32_extend16_s(&self) -> Result<Self> {
        match self {
            Self::I32(v) => Ok(Self::I32((*v as i16) as i32)),
            _ => panic!("cannot extend"),
        }
    }

    pub fn i64_extend8_s(&self) -> Result<Self> {
        match self {
            Self::I64(v) => Ok(Self::I64((*v as i8) as i64)),
            _ => panic!("cannot extend"),
        }
    }

    pub fn i64_extend16_s(&self) -> Result<Self> {
        match self {
            Self::I64(v) => Ok(Self::I64((*v as i16) as i64)),
            _ => panic!("cannot extend"),
        }
    }

    pub fn i64_extend32_s(&self) -> Result<Self> {
        match self {
            Self::I64(v) => Ok(Self::I64((*v as i32) as i64)),
            _ => panic!("cannot extend"),
        }
    }
}

impl From<i32> for Val {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<i64> for Val {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl From<f32> for Val {
    fn from(value: f32) -> Self {
        Self::F32(value)
    }
}

impl From<f64> for Val {
    fn from(value: f64) -> Self {
        Self::F64(value)
    }
}

impl From<bool> for Val {
    fn from(value: bool) -> Self {
        if value {
            Self::I32(1)
        } else {
            Self::I32(0)
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
