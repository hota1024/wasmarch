use binary::{Block, Instruction};

use crate::{
    frame::Frame,
    instances::{FuncInst, InternalFuncInst},
    label::{Label, LabelKind},
    store::Store,
    value::{ExternalVal, Val},
    Error, Result,
};

#[derive(Debug)]
pub struct Runtime {
    store: Store,
    value_stack: Vec<Val>,
    call_stack: Vec<Frame>,
}

macro_rules! pop_value {
    ($this: ident) => {{
        let Some(value) = $this.value_stack.pop() else {
                                                                return Err(Error::ExpectedValue);
                                                            };
        value
    }};
}

macro_rules! uniop {
    ($this: ident, $op: ident) => {{
        let lhs = pop_value!($this);

        $this.value_stack.push(lhs.$op()?);
    }};
}

macro_rules! binop {
    ($this: ident, $op: ident) => {{
        let rhs = pop_value!($this);
        let lhs = pop_value!($this);

        $this.value_stack.push(lhs.$op(&rhs)?);
    }};
}

impl Runtime {
    pub fn new(store: Store) -> Self {
        Self {
            store,
            value_stack: Vec::new(),
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

        let arity = func.func_type.results.len();
        let sp = self.value_stack.len();

        let frame = Frame::new(locals, arity, sp, func.body.code);

        self.call_stack = vec![frame];
        self.value_stack = vec![];

        self.execute()?;

        println!("value stack");
        for value in self.value_stack.clone() {
            println!("{:?}", value);
        }

        if func.func_type.results.len() > 0 {
            return Ok(self.value_stack.pop().unwrap());
        }

        Ok(Val::None)
    }

    fn push_call(&mut self, func: &InternalFuncInst) {
        let mut locals = self
            .value_stack
            .split_off(self.value_stack.len() - func.func_type.params.len());

        for local in func.body.locals.iter() {
            locals.push(Val::default_of(&local));
        }

        let sp = self.value_stack.len();
        let frame = Frame::new(
            locals,
            func.func_type.results.len(),
            sp,
            func.body.code.clone(),
        );

        self.call_stack.push(frame);
    }

    fn execute(&mut self) -> Result<()> {
        while let Some(instruction) = self.get_current_instruction()? {
            // println!("{}: {:?}", self.pc()?, instruction);
            let mut do_increment = true;

            match instruction {
                Instruction::Unreachable => return Err(Error::Trapped),
                Instruction::Nop => {}
                Instruction::Block { block } => {
                    self.r_block(&block)?;
                }
                Instruction::Loop { block } => {
                    self.r_loop(&block)?;
                }
                Instruction::If { block } => {
                    self.r_if(&block)?;
                }
                Instruction::Else => {
                    self.r_else()?;
                }
                Instruction::End => {
                    self.r_end()?;
                }
                Instruction::Br { level } => {
                    self.r_br(level)?;
                }
                Instruction::BrIf { level } => self.r_br_if(level)?,
                // br_table
                Instruction::Return => {
                    self.r_return()?;
                }
                Instruction::Call { func_index } => {
                    self.r_call(func_index)?;
                    do_increment = false;
                }
                // call_indirect
                // ref_null
                // ref_is_null
                // ref_func
                // drop
                // select
                // select_result
                Instruction::LocalGet { local_index } => self.r_local_get(local_index)?,
                Instruction::LocalSet { local_index } => self.r_local_set(local_index)?,
                Instruction::LocalTee { local_index } => self.r_local_tee(local_index)?,
                Instruction::I32Const { value } => self.value_stack.push(Val::I32(value)),
                Instruction::I64Const { value } => self.value_stack.push(Val::I64(value)),
                Instruction::F32Const { value } => self.value_stack.push(Val::F32(value)),
                Instruction::F64Const { value } => self.value_stack.push(Val::F64(value)),
                Instruction::I32Eqz | Instruction::I64Eqz => uniop!(self, eqz),
                Instruction::I32Eq
                | Instruction::I64Eq
                | Instruction::F32Eq
                | Instruction::F64Eq => binop!(self, eq),
                Instruction::I32Ne
                | Instruction::I64Ne
                | Instruction::F32Ne
                | Instruction::F64Ne => binop!(self, ne),
                Instruction::I32LtS | Instruction::I64LtS => binop!(self, lt_s),
                Instruction::I32LtU | Instruction::I64LtU => binop!(self, lt_u),
                Instruction::I32GtS | Instruction::I64GtS => binop!(self, gt_s),
                Instruction::I32GtU | Instruction::I64GtU => binop!(self, gt_u),
                Instruction::I32LeS | Instruction::I64LeS => binop!(self, le_s),
                Instruction::I32GeS | Instruction::I64GeS => binop!(self, ge_s),
                Instruction::I32GeU | Instruction::I64GeU => binop!(self, ge_u),
                Instruction::I32LeU | Instruction::I64LeU => binop!(self, le_u),
                Instruction::F32Lt | Instruction::F64Lt => binop!(self, lt),
                Instruction::F32Gt | Instruction::F64Gt => binop!(self, gt),
                Instruction::F32Le | Instruction::F64Le => binop!(self, le),
                Instruction::F32Ge | Instruction::F64Ge => binop!(self, ge),
                Instruction::I32Clz | Instruction::I64Clz => uniop!(self, clz),
                Instruction::I32Ctz | Instruction::I64Ctz => uniop!(self, ctz),
                Instruction::I32Popcnt | Instruction::I64Popcnt => uniop!(self, popcnt),
                Instruction::I32Add
                | Instruction::I64Add
                | Instruction::F32Add
                | Instruction::F64Add => binop!(self, add),
                Instruction::I32Sub
                | Instruction::I64Sub
                | Instruction::F32Sub
                | Instruction::F64Sub => binop!(self, sub),
                Instruction::I32Mul
                | Instruction::I64Mul
                | Instruction::F32Mul
                | Instruction::F64Mul => binop!(self, mul),
                Instruction::I32DivS | Instruction::I64DivS => binop!(self, div_s),
                Instruction::I32DivU | Instruction::I64DivU => binop!(self, div_u),
                Instruction::F32Div | Instruction::F64Div => binop!(self, div),
                Instruction::I32RemS | Instruction::I64RemS => binop!(self, rem_s),
                Instruction::I32RemU | Instruction::I64RemU => binop!(self, rem_u),
                Instruction::I32And | Instruction::I64And => binop!(self, and),
                Instruction::I32Or | Instruction::I64Or => binop!(self, or),
                Instruction::I32Xor | Instruction::I64Xor => binop!(self, xor),
                Instruction::I32Shl | Instruction::I64Shl => binop!(self, shl),
                Instruction::I32ShrS | Instruction::I64ShrS => binop!(self, shr_s),
                Instruction::I32ShrU | Instruction::I64ShrU => binop!(self, shr_u),
                Instruction::I32Rotl | Instruction::I64Rotl => binop!(self, rotl),
                Instruction::I32Rotr | Instruction::I64Rotr => binop!(self, rotr),
                Instruction::F32Abs | Instruction::F64Abs => uniop!(self, abs),
                Instruction::F32Neg | Instruction::F64Neg => uniop!(self, neg),
                Instruction::F32Ceil | Instruction::F64Ceil => uniop!(self, ceil),
                Instruction::F32Floor | Instruction::F64Floor => uniop!(self, floor),
                Instruction::F32Trunc | Instruction::F64Trunc => uniop!(self, trunc),
                Instruction::F32Nearest | Instruction::F64Nearest => uniop!(self, nearest),
                Instruction::F32Sqrt | Instruction::F64Sqrt => uniop!(self, sqrt),
                Instruction::F32Min | Instruction::F64Min => binop!(self, min),
                Instruction::F32Max | Instruction::F64Max => binop!(self, max),
                Instruction::F32Copysign | Instruction::F64Copysign => binop!(self, copysign),
                Instruction::I32WrapI64 => uniop!(self, i32_wrap_i64),
                Instruction::I32TruncF32S | Instruction::I32TruncF64S => uniop!(self, i32_trunc_s),
                Instruction::I32TruncF32U | Instruction::I32TruncF64U => uniop!(self, i32_trunc_u),
                Instruction::I64ExtendI32S => uniop!(self, i64_extend_i32_s),
                Instruction::I64ExtendI32U => uniop!(self, i64_extend_i32_u),
                Instruction::F32ConvertI32S | Instruction::F32ConvertI64S => {
                    uniop!(self, f32_convert_s)
                }
                Instruction::F32ConvertI32U | Instruction::F32ConvertI64U => {
                    uniop!(self, f32_convert_u)
                }
                Instruction::F32DemoteF64 => uniop!(self, f32_demote_f64),
                Instruction::F64ConvertI32S | Instruction::F64ConvertI64S => {
                    uniop!(self, f64_convert_s)
                }
                Instruction::F64ConvertI32U | Instruction::F64ConvertI64U => {
                    uniop!(self, f64_convert_u)
                }
                Instruction::F64PromoteF32 => uniop!(self, f64_promote_f32),
                Instruction::I32ReinterpretF32 => uniop!(self, i32_reinterpret_f32),
                Instruction::I64ReinterpretF64 => uniop!(self, i64_reinterpret_f64),
                Instruction::F32ReinterpretI32 => uniop!(self, f32_reinterpret_i32),
                Instruction::F64ReinterpretI64 => uniop!(self, f64_reinterpret_i64),
                Instruction::I32Extend8S => uniop!(self, i32_extend8_s),
                Instruction::I32Extend16S => uniop!(self, i32_extend16_s),
                Instruction::I64Extend8S => uniop!(self, i64_extend8_s),
                Instruction::I64Extend16S => uniop!(self, i64_extend16_s),
                Instruction::I64Extend32S => uniop!(self, i64_extend32_s),
                _ => unimplemented!("unimplemented instruction: {:?}", instruction),
            }

            if do_increment {
                self.increment_pc()?;
            }
        }

        Ok(())
    }

    fn r_if(&mut self, block: &Block) -> Result<()> {
        let Some(condition) = self.value_stack.pop() else {
            return Err(Error::ExpectedValue)
        };

        let end_pc = self.get_end_pc()?;

        if condition.is_false() {
            let next_pc = self.get_else_or_end_pc()?;

            // if end_pc == next_pc {
            self.set_pc(next_pc)?;
            // } else {
            // self.set_pc(next_pc + 1)?;
            // }
        }

        let label = Label {
            kind: LabelKind::If,
            pc: end_pc,
            sp: self.value_stack.len(),
            arity: block.block_type.result_count(),
            result_type: block.block_type.clone(),
        };

        self.last_mut_frame()?.label_stack.push(label);

        Ok(())
    }

    fn r_else(&mut self) -> Result<()> {
        let Some(label) = self.last_mut_frame()?.label_stack.pop() else {
            return Err(Error::ExpectedLabel);
        };

        self.set_pc(label.pc)?;

        Ok(())
    }

    fn r_end(&mut self) -> Result<()> {
        let label = self.last_mut_frame()?.label_stack.pop();

        match label {
            Some(label) => {
                self.clean_value_stack(label.arity, label.sp)?;
                self.set_pc(label.pc)?;
            }
            None => {
                let Some(frame) = self.call_stack.pop() else {
                    return Err(Error::EmptyCallStack("end".to_string()));
                };

                self.clean_value_stack(frame.arity, frame.sp)?;
            }
        }

        Ok(())
    }

    fn r_return(&mut self) -> Result<()> {
        let Some(frame) = self.call_stack.pop() else {
            return Err(Error::EmptyCallStack("return".to_string()));
        };

        self.clean_value_stack(frame.arity, frame.sp)?;

        Ok(())
    }

    fn r_call(&mut self, func_index: u32) -> Result<()> {
        let Some(func) = self.store.funcs.get(func_index as usize).cloned() else {
            return Err(Error::InvalidIndexForFunc(func_index as usize));
        };

        match func {
            FuncInst::Internal(func) => {
                self.push_call(&func);
            }
            _ => unimplemented!(),
        }

        Ok(())
    }

    fn r_block(&mut self, block: &Block) -> Result<()> {
        let end_pc = self.get_end_pc()?;
        let label = Label {
            kind: LabelKind::Block,
            pc: end_pc,
            sp: self.value_stack.len(),
            arity: block.block_type.result_count(),
            result_type: block.block_type.clone(),
        };

        self.last_mut_frame()?.label_stack.push(label);

        Ok(())
    }

    fn r_loop(&mut self, block: &Block) -> Result<()> {
        let start_pc = self.pc()?;
        let end_pc = self.get_end_pc()?;

        let label = Label {
            kind: LabelKind::Loop { start_pc },
            pc: end_pc,
            sp: self.value_stack.len(),
            arity: block.block_type.result_count(),
            result_type: block.block_type.clone(),
        };

        self.last_mut_frame()?.label_stack.push(label);

        Ok(())
    }

    fn r_br(&mut self, level: u32) -> Result<()> {
        let label_stack = &mut self.last_mut_frame()?.label_stack;
        let label_index = label_stack.len() - 1 - level as usize;

        let Some(label) = label_stack.get(label_index).cloned() else {
            return Err(Error::ExpectedLabel);
        };

        let pc = match label.kind {
            LabelKind::Loop { start_pc } => {
                self.clean_value_stack(label.arity, label.sp)?;
                start_pc
            }
            _ => {
                label_stack.drain(label_index..);
                self.clean_value_stack(label.arity, label.sp)?;
                label.pc
            }
        };

        self.set_pc(pc)?;

        Ok(())
    }

    fn r_br_if(&mut self, level: u32) -> Result<()> {
        let Some( value )= self.value_stack.pop() else {
            return Err(Error::ExpectedValue)
        };

        if value.is_true() {
            self.r_br(level)?;
        }

        Ok(())
    }

    fn r_local_get(&mut self, local_index: u32) -> Result<()> {
        let Some(frame) = self.call_stack.last() else {
            return Err(Error::EmptyCallStack("local get".to_string()));
        };

        let Some(local) = frame.locals.get(local_index as usize) else {
            return Err(Error::LocalNotFound);
        };

        self.value_stack.push(local.clone());

        Ok(())
    }

    fn r_local_set(&mut self, local_index: u32) -> Result<()> {
        let Some(value) = self.value_stack.pop() else {
            return Err(Error::ExpectedValue);
        };
        let frame = self.last_mut_frame()?;

        frame.locals[local_index as usize] = value;

        Ok(())
    }

    fn r_local_tee(&mut self, local_index: u32) -> Result<()> {
        let Some(value) = self.value_stack.last().cloned() else {
            return Err(Error::ExpectedValue);
        };
        let frame = self.last_mut_frame()?;

        frame.locals[local_index as usize] = value;

        Ok(())
    }

    fn last_mut_frame(&mut self) -> Result<&mut Frame> {
        let Some(frame) = self.call_stack.last_mut() else {
            return Err(Error::EmptyCallStack("last mut frame".to_string()));
        };

        Ok(frame)
    }

    fn increment_pc(&mut self) -> Result<()> {
        let frame_result = self.last_mut_frame();

        match frame_result {
            Ok(frame) => {
                frame.pc += 1;
            }
            Err(err) => {
                if !matches!(err, Error::EmptyCallStack(..)) {
                    return Err(err);
                }
            }
        }

        Ok(())
    }

    fn get_else_or_end_pc(&self) -> Result<usize> {
        let mut depth = 0;
        let mut pc = self.pc()?;

        loop {
            pc += 1;

            let Some(instruction)  = self.get_instruction(pc)? else {
                return Err(Error::ExpectedInstruction);
            };

            match instruction {
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
                    }

                    depth -= 1;
                }
                _ => {}
            }
        }
    }

    fn get_end_pc(&mut self) -> Result<usize> {
        let mut depth = 0;
        let mut pc = self.pc()?;

        loop {
            pc += 1;

            let Some(instruction)  = self.get_instruction(pc)? else {
                return Err(Error::ExpectedInstruction);
            };

            match instruction {
                Instruction::If { .. } | Instruction::Block { .. } | Instruction::Loop { .. } => {
                    depth += 1;
                }
                Instruction::End => {
                    if depth == 0 {
                        return Ok(pc);
                    }

                    depth -= 1;
                }
                _ => {}
            }
        }
    }

    fn pc(&self) -> Result<usize> {
        let Some(frame) = self.call_stack.last() else {
            return Err(Error::EmptyCallStack("get pc".to_string()));
        };

        Ok(frame.pc)
    }

    fn set_pc(&mut self, pc: usize) -> Result<()> {
        let frame = self.last_mut_frame()?;
        frame.pc = pc;

        Ok(())
    }

    fn get_current_instruction(&self) -> Result<Option<Instruction>> {
        let Some(frame) = self.call_stack.last() else {
            // return Err(Error::EmptyCallStack);
            return Ok(None)
        };
        let pc = frame.pc;

        self.get_instruction(pc)
    }

    fn get_instruction(&self, index: usize) -> Result<Option<Instruction>> {
        let Some(frame) = self.call_stack.last() else {
            return Err(Error::EmptyCallStack("get instruction".to_string()));
            // return Ok(None)
        };

        Ok(frame.instructions.get(index).cloned())
    }

    fn clean_value_stack(&mut self, arity: usize, sp: usize) -> Result<()> {
        let value_stack = &mut self.value_stack;

        if arity > 0 {
            let Some(value) = value_stack.pop() else {
                return Err(Error::ExpectedValue);
            };
            value_stack.drain(sp..);
            value_stack.push(value);
            // let len = value_stack.len();
            // let mut return_values = value_stack.drain(len - arity..len).collect::<Vec<Val>>();

            // value_stack.drain(sp..);
            // value_stack.append(&mut return_values);
        } else {
            value_stack.drain(sp..);
        }

        Ok(())
    }
}
