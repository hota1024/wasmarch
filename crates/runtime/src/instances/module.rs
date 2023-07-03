use types::FuncType;

use super::export::ExportInst;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ModuleInst {
    pub types: Vec<FuncType>,
    pub func_addrs: Vec<usize>,
    pub table_addrs: Vec<usize>,
    pub mem_addrs: Vec<usize>,
    pub global_addrs: Vec<usize>,
    pub elem_addrs: Vec<usize>,
    pub data_addrs: Vec<usize>,
    pub exports: Vec<ExportInst>,
}
