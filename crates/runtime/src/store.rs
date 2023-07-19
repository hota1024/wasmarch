use binary::{ExportDesc, GlobalInitExpr, Module, Type};

use crate::{
    instances::{
        DataInst, ElemInst, ExportInst, FuncInst, GlobalInst, InternalFuncInst, MemInst,
        ModuleInst, TableInst,
    },
    result::{Error, Result},
    value::{ExternalVal, Val},
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Store {
    pub funcs: Vec<FuncInst>,
    pub tables: Vec<TableInst>,
    pub mems: Vec<MemInst>,
    pub globals: Vec<GlobalInst>,
    pub elems: Vec<ElemInst>,
    pub datas: Vec<DataInst>,
    pub module: ModuleInst,
}

impl Store {
    pub fn from_module(module: Module) -> Result<Self> {
        let mut store = Store::default();

        for (code_index, type_index) in module.function_section.iter().enumerate() {
            let code = match module.code_section.get(code_index as usize) {
                Some(c) => c,
                None => return Err(Error::InvalidIndexForCode(code_index as usize)),
            };
            let ty = match module.type_section.get(*type_index as usize) {
                Some(t) => t,
                None => return Err(Error::InvalidIndexForType(*type_index as usize)),
            };

            match ty {
                Type::Func(func_type) => {
                    store.funcs.push(FuncInst::Internal(InternalFuncInst {
                        func_type: func_type.clone(),
                        body: code.clone(),
                    }));
                }
            }
        }

        for global in module.global_section {
            let value = match global.init_expr {
                GlobalInitExpr::I32Const { value } => Val::from(value),
                GlobalInitExpr::I64Const { value } => Val::from(value),
                GlobalInitExpr::F32Const { value } => Val::from(value),
                GlobalInitExpr::F64Const { value } => Val::from(value),
            };

            store.globals.push(GlobalInst {
                global_type: global.global_type,
                value,
            });
        }

        for export in module.export_section {
            match export.desc {
                ExportDesc::Func(func_index) => {
                    let _func = match store.funcs.get(func_index as usize) {
                        Some(f) => f,
                        None => return Err(Error::InvalidIndexForFunc(func_index as usize)),
                    };

                    // store.module.exports.push(ExportInst {
                    //     name: export.name,
                    //     value: ExternalVal::FuncAddr(func_index as usize),
                    // });
                    store.module.exports.insert(
                        export.name.clone(),
                        ExportInst {
                            name: export.name,
                            value: ExternalVal::FuncAddr(func_index as usize),
                        },
                    );
                }
                _ => unimplemented!("export {:?}", export.desc),
            }
        }
        Ok(store)
    }
}
