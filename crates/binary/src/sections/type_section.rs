use alloc::boxed::Box;
use types::func_type::FuncType;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Func(FuncType),
}

pub type TypeSection = Box<[Type]>;
