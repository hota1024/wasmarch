use binary::{ExportDesc, GlobalInitExpr, ImportDesc, Module, Type};

use crate::{
    instances::{
        DataInst, ElemInst, ExportInst, ExternalFuncInst, FuncInst, GlobalInst, InternalFuncInst,
        MemInst, ModuleInst, TableInst,
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
    pub start_func_index: Option<u32>,
}

impl Store {
    pub fn from_module(module: Module) -> Result<Self> {
        let mut store = Store::default();

        for import in module.import_section {
            match import.desc {
                ImportDesc::Func(type_idx) => {
                    let Some(ty) = module.type_section.get(type_idx as usize) else {
                        return Err(Error::Custom("invalid type index".to_string()))
                    };
                    let Type::Func(func_type) = ty;
                    store.funcs.push(FuncInst::External(ExternalFuncInst {
                        module: import.module.clone(),
                        field: import.field.clone(),
                        func_type: func_type.clone(),
                    }));
                }
                _ => return Err(Error::Custom("".to_string())),
            }
        }

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
                ExportDesc::Mem(mem_index) => {
                    store.module.exports.insert(
                        export.name.clone(),
                        ExportInst {
                            name: export.name,
                            value: ExternalVal::MemAddr(mem_index as usize),
                        },
                    );
                }
                ExportDesc::Global(global_index) => {
                    store.module.exports.insert(
                        export.name.clone(),
                        ExportInst {
                            name: export.name,
                            value: ExternalVal::GlobalAddr(global_index as usize),
                        },
                    );
                }
                _ => unimplemented!("export {:?}", export.desc),
            }
        }

        // TODO: table
        // for table in module.table_section {
        //     store.tables.push(TableInst {
        //         table_type: table,
        //         elems: vec![],
        //     });
        // }

        // for elem in module.element_section {
        //     match elem.mode {
        //         ElementMode::Active {
        //             table_index,
        //             offset,
        //         } => {
        //             store.elems.push(ElemInst {
        //                 ref_type: elem.ref_type,
        //                 elems: vec![],
        //             });
        //         }
        //         _ => {}
        //     }
        // }

        for memory in module.memory_section {
            store.mems.push(MemInst {
                mem_type: memory.limits,
                data: vec![0u8; 65_536],
            });
        }

        store.start_func_index = module.start_section.func_index;

        Ok(store)
    }
}
