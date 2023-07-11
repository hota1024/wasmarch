use binary::Instruction;

use crate::{
    frame::Frame,
    instances::{FuncInst, InternalFuncInst},
    label::Label,
    store::Store,
    value::{ExternalVal, Val},
    Error, Result,
};

#[derive(Debug)]
pub struct Runtime {
    store: Store,
    value_stack: Vec<Val>,
    label_stack: Vec<Label>,
    call_stack: Vec<Frame>,
}

impl Runtime {
    pub fn new(store: Store) -> Self {
        Self {
            store,
            value_stack: Vec::new(),
            label_stack: Vec::new(),
            call_stack: Vec::new(),
        }
    }

    pub fn invoke(&mut self, fn_name: &str, args: &[Val]) -> Result<Val> {
        let Some(export) = self.store.module.exports.get(fn_name) else {
            return Err(Error::ExportNotFound(fn_name.to_string()));
        };

        let func_addr = match export.value {
            ExternalVal::FuncAddr(addr) => addr,
            _ => return Err(Error::ExpectFuncAddr(export.clone())),
        };

        let Some(func) = self.store.funcs.get(func_addr) else {
            return Err(Error::InvalidIndexForFunc(func_addr));
        };

        match func {
            FuncInst::Internal(internal_func) => self.invoke_internal(internal_func.clone(), args),
            // FuncInst::External(external_func) => self.invoke_external(external_func, args),
            _ => unimplemented!(),
        }
    }

    fn invoke_internal(&mut self, func: InternalFuncInst, args: &[Val]) -> Result<Val> {
        let mut locals = Vec::new();

        for arg in args {
            locals.push(arg.clone());
        }

        for local in func.body.locals.iter() {
            locals.push(Val::default_of(&local));
        }

        let frame = Frame::new(locals, func.body.code);
        self.call_stack.push(frame);

        self.execute()?;

        if func.func_type.results.len() > 0 {
            return Ok(self.value_stack.pop().unwrap());
        }

        Ok(Val::None)
    }

    fn execute(&mut self) -> Result<()> {
        let value_stack = &mut self.value_stack;
        let call_stack = &mut self.call_stack;

        loop {
            let Some(frame) = call_stack.last_mut() else {
                break;
            };

            let Some(instruction) = frame.instructions.get(frame.pc) else {
                break;
            };

            match instruction {
                Instruction::I32Const { value } => value_stack.push(Val::I32(*value)),
                Instruction::End => {}
                _ => unimplemented!("unimplemented instruction: {:?}", instruction),
            }

            frame.pc += 1;
        }

        Ok(())
    }
}
