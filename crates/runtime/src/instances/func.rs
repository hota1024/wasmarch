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
    pub module: String,
    pub field: String,
    pub func_type: FuncType,
}

impl ExternalFuncInst {
    pub fn match_module_field(&self, module: &str, field: &str) -> bool {
        &self.module == module && &self.field == field
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FuncInst {
    Internal(InternalFuncInst),
    External(ExternalFuncInst),
}
