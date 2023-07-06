use binary::{ExportDesc, Instruction, Module};

use crate::{
    frame::Frame,
    instances::FuncInst,
    result::{Error, Result},
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
                let has_results = wasm_func.func_type.results.len() > 0;
                let frame = Frame {
                    pc: 0,
                    sp: 0,
                    instructions: wasm_func.code.body.clone(),
                    locals,
                };

                self.call_frames.push(frame);

                self.execute()?;

                if has_results {
                    let result = self.stack.pop().unwrap();

                    return Ok(result);
                }
            }
            _ => unimplemented!("func: {:?}", func),
        }

        Ok(Val::I32(0))
    }

    fn execute(&mut self) -> Result<()> {
        loop {
            let frame = self.call_frames.last_mut().unwrap();
            let Some(inst) = frame.instructions.get(frame.pc) else {
                break;
            };
            frame.pc += 1;

            println!("inst: {:?}", inst);

            match inst {
                Instruction::If { block } => {}
                Instruction::Else => {}
                Instruction::Call { func_index } => {
                    let func = self.store.funcs.get(*func_index as usize).unwrap();
                    match func {
                        FuncInst::Host(host_func) => todo!(),
                        FuncInst::Wasm(wasm_func) => {
                            let locals = self
                                .stack
                                .split_off(self.stack.len() - wasm_func.func_type.params.len());
                            let has_results = wasm_func.func_type.results.len() > 0;
                            let frame = Frame {
                                pc: 0,
                                sp: 0,
                                instructions: wasm_func.code.body.clone(),
                                locals,
                            };

                            self.call_frames.push(frame);

                            self.execute()?;

                            println!("stack: {:?}", self.stack);

                            // if has_results {
                            //     let result = self.stack.pop().unwrap();

                            //     self.stack.push(result);
                            // }
                        }
                    }
                }
                Instruction::LocalGet { local_index } => {
                    let val = frame.locals[*local_index as usize].clone();
                    self.stack.push(val);
                }
                Instruction::I32Const { value } => {
                    self.stack.push(Val::I32(*value));
                }
                Instruction::I32LtS => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let result = match (val1, val2) {
                        (Val::I32(val1), Val::I32(val2)) => {
                            Val::I32(if val1 < val2 { 1 } else { 0 })
                        }
                        _ => todo!(),
                    };
                    self.stack.push(result);
                }
                Instruction::I32Add => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let result = match (val1, val2) {
                        (Val::I32(val1), Val::I32(val2)) => Val::I32(val1 + val2),
                        _ => todo!(),
                    };
                    self.stack.push(result);
                }
                Instruction::I32Sub => {
                    let val1 = self.stack.pop().unwrap();
                    let val2 = self.stack.pop().unwrap();
                    let result = match (val1, val2) {
                        (Val::I32(val1), Val::I32(val2)) => Val::I32(val1 - val2),
                        _ => todo!(),
                    };
                    self.stack.push(result);
                }
                _ => unimplemented!("inst: {:?}", inst),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use decoder::Decoder;
    use wabt::wat2wasm;

    #[test]
    fn add() {
        let source = "
        (module
            (func (export \"add\") (param i32 i32) (result i32) local.get 0 local.get 1 i32.add)
        )";
        let wasm = wat2wasm(source).unwrap();

        let mut decoder = Decoder::new(&wasm[..]);
        let module = decoder.decode().unwrap();

        let mut runtime = Runtime::from(module);
        let result = runtime
            .invoke("add".to_string(), vec![Val::I32(1), Val::I32(2)])
            .unwrap();

        println!("source: {}", source);
        println!("frame: {:?}", runtime.call_frames);
        println!("result: {:?}", result);
    }

    #[test]
    fn if_branch() {
        let source = "
        (module
            (func (export \"if_branch\") (result i32)
                i32.const 0

                if (result i32)
                    i32.const 2
                else
                    i32.const 3 
                end
            )
        )";
        let wasm = wat2wasm(source).unwrap();

        let mut decoder = Decoder::new(&wasm[..]);
        let module = decoder.decode().unwrap();

        let mut runtime = Runtime::from(module);
        let result = runtime.invoke("if_branch".to_string(), vec![]).unwrap();

        println!("source: {}", source);
        println!("frame: {:?}", runtime.call_frames);
        println!("result: {:?}", result);
    }

    #[test]
    fn fib() {
        let source = "
        (module
            (func $fib (export \"fib\") (param i32) (result i32)
                local.get 0
                i32.const 2
                i32.lt_s
                if (result i32)
                    local.get 0
                else
                    local.get 0
                    i32.const 1
                    i32.sub
                    call $fib
                    local.get 0
                    i32.const 2
                    i32.sub
                    call $fib
                    i32.add
                end
            )
        )";
        let wasm = wat2wasm(source).unwrap();

        let mut decoder = Decoder::new(&wasm[..]);
        let module = decoder.decode().unwrap();

        let mut runtime = Runtime::from(module);
        let result = runtime
            .invoke("fib".to_string(), vec![Val::I32(10)])
            .unwrap();

        println!("source: {}", source);
        println!("frame: {:?}", runtime.call_frames);
        println!("result: {:?}", result);
    }
}
