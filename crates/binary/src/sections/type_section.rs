use alloc::vec::Vec;
use types::func_type::FuncType;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Func(FuncType),
}

pub type TypeSection = Vec<Type>;
