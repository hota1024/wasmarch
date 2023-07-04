use binary::{ExportDesc, Module};

use crate::{
    frame::Frame,
    instances::FuncInst,
    result::Result,
    store::Store,
    value::{ExternalVal, Val},
};

struct Runtime {
    store: Store,
    stack: Vec<Val>,
    call_frames: Vec<Frame>,
}

impl From<Module> for Runtime {
    fn from(module: Module) -> Self {
        Self {
            store: Store::from_module(module).unwrap(),
            stack: vec![],
            call_frames: vec![],
        }
    }
}

impl Runtime {
    pub fn invoke(&mut self, name: String, args: Vec<Val>) -> Result<Val> {
        // let func = self.store.funcs.get()
        let e = self.store.get_export_by_name(name).unwrap();
        println!("e: {:?}", e);

        let ExternalVal::FuncAddr(func_index) = e.value else {
            panic!("not a func");
        };

        let func = self.store.funcs.get(func_index).unwrap();

        for arg in args {
            self.stack.push(arg);
        }

        match func {
            FuncInst::Wasm(wasm_func) => {
                let locals = self
                    .stack
                    .split_off(self.stack.len() - wasm_func.func_type.params.len());
                let frame = Frame {
                    pc: 0,
                    sp: 0,
                    insts: wasm_func.code.body,
                    locals,
                };

                self.call_frames.push(frame);

                return self.execute();
            }
            _ => unimplemented!("func: {:?}", func),
        }

        Ok(Val::I32(0))
    }

    fn execute(&mut self) -> Result<Val> {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use decoder::Decoder;
    use wabt::wat2wasm;

    #[test]
    fn test() {
        let wasm = wat2wasm(
            "
        (module
            (func (export \"add\") (param i32 i32) (result i32) local.get 0 local.get 1 i32.add)
        )",
        )
        .unwrap();

        let mut decoder = Decoder::new(&wasm[..]);
        let module = decoder.decode().unwrap();

        let mut runtime = Runtime::from(module);
        let result = runtime
            .invoke("add".to_string(), vec![Val::I32(1), Val::I32(2)])
            .unwrap();

        println!("result: {:?}", result);
    }
}
