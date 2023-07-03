use binary::FuncBody;
use types::FuncType;

#[derive(Debug, Clone, PartialEq)]
pub struct WasmFuncInst {
    pub func_type: FuncType,
    // pub module: ModuleInst,
    pub code: FuncBody,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HostFuncInst {
    pub func_type: FuncType,
    // TODO: hostcode
}

#[derive(Debug, Clone, PartialEq)]
pub enum FuncInst {
    Wasm(WasmFuncInst),
    Host(HostFuncInst),
}
