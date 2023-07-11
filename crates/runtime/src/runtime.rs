use binary::{ExportDesc, Instruction, Module};

use crate::{
    frame::Frame,
    instances::FuncInst,
    label::{Label, LabelKind},
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
                    label_stack: vec![],
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
            let Some(frame) = self.call_frames.last_mut() else {
                break;
            };
            let instructions = &frame.instructions;
            let Some(inst) = instructions.get(frame.pc) else {
                break;
            };
            let mut next_pc = frame.pc + 1;

            println!("inst[{}]: {:?}", frame.pc, inst);

            match inst {
                Instruction::If { block } => {
                    let val = self.stack.pop().unwrap();

                    let end_pc = get_corresponding_end(&frame.instructions, frame.pc)?;

                    if !val.is_true() {
                        next_pc = get_corresponding_end_or_else(&frame.instructions, frame.pc)?;
                        println!("next_pc: {:?}", next_pc);
                    }

                    let label = Label {
                        kind: LabelKind::If,
                        pc: end_pc,
                        result_type: block.block_type.clone(),
                    };
                    frame.label_stack.push(label);
                }
                Instruction::Else => {
                    let label = frame.label_stack.pop().unwrap();

                    next_pc = label.pc;
                }
                Instruction::End => match frame.label_stack.pop() {
                    Some(label) => {
                        next_pc = label.pc;
                    }
                    None => {
                        // let frame = self.call_frames.pop().unwrap();
                    }
                },
                Instruction::Call { func_index } => {}
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

            frame.pc = next_pc;
        }

        Ok(())
    }
}

fn get_corresponding_end_or_else(instructions: &[Instruction], pc: usize) -> Result<usize> {
    let mut pc = pc;
    let mut depth = 0;

    loop {
        pc += 1;

        if let Some(inst) = instructions.get(pc) {
            match inst {
                Instruction::If { .. } => {
                    depth += 1;
                }
                Instruction::Else => {
                    if depth == 0 {
                        return Ok(pc);
                    }
                }
                Instruction::End => {
                    if depth == 0 {
                        return Ok(pc);
                    } else {
                        depth -= 1;
                    }
                }
                _ => {}
            }
        } else {
            return Err(Error::UnexpectedEndOfInput);
        }
    }
}

fn get_corresponding_end(instructions: &[Instruction], pc: usize) -> Result<usize> {
    let mut pc = pc;
    let mut depth = 0;

    loop {
        pc += 1;

        if let Some(inst) = instructions.get(pc) {
            match inst {
                Instruction::If { .. } | Instruction::Block { .. } | Instruction::Loop { .. } => {
                    depth += 1;
                }
                Instruction::End => {
                    if depth == 0 {
                        return Ok(pc);
                    } else {
                        depth -= 1;
                    }
                }
                _ => {}
            }
        } else {
            return Err(Error::UnexpectedEndOfInput);
        }
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
