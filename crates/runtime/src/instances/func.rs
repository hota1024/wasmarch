use binary::FuncBody;
use types::FuncType;

#[derive(Debug, Clone, PartialEq)]
pub struct InternalFuncInst {
    pub func_type: FuncType,
    // pub module: ModuleInst,
    pub body: FuncBody,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExternalFuncInst {
    pub func_type: FuncType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FuncInst {
    Internal(InternalFuncInst),
    External(ExternalFuncInst),
}
