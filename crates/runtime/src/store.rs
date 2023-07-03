use binary::{ImportDesc, Module, Type};
use types::FuncType;

use crate::{
    instances::{
        DataInst, ElemInst, FuncInst, GlobalInst, HostFuncInst, MemInst, TableInst, WasmFuncInst,
    },
    result::{Error, Result},
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Store {
    pub funcs: Vec<FuncInst>,
    pub tables: Vec<TableInst>,
    pub mems: Vec<MemInst>,
    pub globals: Vec<GlobalInst>,
    pub elems: Vec<ElemInst>,
    pub datas: Vec<DataInst>,
}

impl Store {
    pub fn from_module(module: Module) -> Result<Self> {
        let mut store = Store::default();

        for import in module.import_section {
            match import.desc {
                ImportDesc::Func(type_idx) => {
                    let ty = match module.type_section.get(type_idx as usize) {
                        Some(t) => t,
                        None => return Err(Error::InvalidIndexForType(type_idx as usize)),
                    };

                    match ty {
                        Type::Func(func_type) => {
                            store.funcs.push(FuncInst::Host(HostFuncInst {
                                func_type: func_type.clone(),
                            }));
                        }
                        _ => {
                            return Err(Error::InvalidIndexForFuncType(type_idx as usize));
                        }
                    }
                }
                _ => unimplemented!("import {:?}", import.desc),
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
                    store.funcs.push(FuncInst::Wasm(WasmFuncInst {
                        func_type: func_type.clone(),
                        code: code.clone(),
                    }));
                }
            }
        }

        println!("store: {:?}", store);
        Ok(store)
    }
}
