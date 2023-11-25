use crate::{Error, Result, SectionId};
use binary::{
    instruction::Instruction, Block, BlockType, CodeSection, Element, ElementMode, ElementSection,
    Export, ExportDesc, ExportSection, FuncBody, FunctionSection, Global, GlobalInitExpr,
    GlobalSection, Import, ImportDesc, ImportSection, MemArg, MemorySection, Module, StartSection,
    TableSection, Type, TypeSection,
};
use core::panic;
use std::io::{BufReader, Read};
use types::{FuncType, GlobalType, Limits, MemoryType, RefType, TableType, ValueType};

pub struct Decoder<R> {
    reader: BufReader<R>,
}

impl<R: Read> Decoder<R> {
    pub fn new(reader: R) -> Self {
        Decoder {
            reader: BufReader::new(reader),
        }
    }

    pub fn decode(&mut self) -> Result<Module> {
        self.validate_magic_header()?;

        let version = self.get_version();
        if version != 1 {
            return Err(Error::InvalidVersion);
        }

        let mut module = Module::default();

        let result = loop {
            let result = self.decode_section();

            if result.is_err() {
                break result;
            }

            let (id, _) = result.unwrap();

            match id {
                // SectionId::Custom
                SectionId::Custom => {}
                SectionId::Type => {
                    module.type_section = self.decode_type_section()?;
                }
                SectionId::Import => {
                    module.import_section = self.decode_import_section()?;
                }
                SectionId::Function => {
                    module.function_section = self.decode_function_section()?;
                }
                SectionId::Table => {
                    module.table_section = self.decode_table_section()?;
                }
                SectionId::Memory => {
                    module.memory_section = self.decode_memory_section()?;
                }
                SectionId::Global => {
                    module.global_section = self.decode_global_section()?;
                }
                SectionId::Export => {
                    module.export_section = self.decode_export_section()?;
                }
                SectionId::Start => {
                    module.start_section = self.decode_start_section()?;
                }
                SectionId::Element => {
                    module.element_section = self.decode_element_section()?;
                }
                SectionId::Code => {
                    module.code_section = self.decode_code_section()?;
                }
                // SectionId::Data
                // SectionId::DataCount
                _ => unimplemented!("Section {:?} is not implemented", id),
            }
        };

        match result {
            Ok(_) => Ok(module),
            Err(Error::UnexpectedEof) => Ok(module),
            Err(err) => Err(err),
        }
    }

    fn validate_magic_header(&mut self) -> Result<()> {
        let mut magic = [0; 4];
        self.reader.read_exact(&mut magic).unwrap();

        match magic {
            [0x00, 0x61, 0x73, 0x6d] => Ok(()),
            _ => Err(Error::InvalidMagicHeader),
        }
    }

    fn get_version(&mut self) -> u32 {
        let mut version = [0; 4];
        self.reader.read_exact(&mut version).unwrap();

        u32::from_le_bytes(version)
    }

    fn decode_type_section(&mut self) -> Result<TypeSection> {
        let types = self.read_vec(|d| {
            let kind = d.read_u8()?;

            if kind != 0x60 {
                return Err(Error::InvalidTypeKind);
            }

            let param_types = d.read_vec(|d| {
                let result_type = ValueType::from(d.read_u8()?);
                Ok(result_type)
            })?;

            let result_types = d.read_vec(|d| {
                let result_type = ValueType::from(d.read_u8()?);
                Ok(result_type)
            })?;

            Ok(Type::Func(FuncType {
                params: param_types,
                results: result_types,
            }))
        })?;

        Ok(types)
    }

    fn decode_import_section(&mut self) -> Result<ImportSection> {
        let imports = self.read_vec(|d| {
            let module = d.read_name()?;
            let field = d.read_name()?;
            let desc_id = d.read_u8()?;

            let desc = match desc_id {
                0x00 => {
                    let type_index = d.read_size()?;
                    ImportDesc::Func(type_index)
                }
                _ => return Err(Error::InvalidImportDesc),
            };

            Ok(Import {
                module,
                field,
                desc,
            })
        })?;

        Ok(imports)
    }

    fn decode_function_section(&mut self) -> Result<FunctionSection> {
        let type_indexes = self.read_vec(|d| {
            let type_index = d.read_size()?;
            Ok(type_index)
        })?;

        Ok(type_indexes)
    }

    fn decode_table_section(&mut self) -> Result<TableSection> {
        let table = self.read_vec(|d| {
            Ok(TableType {
                element_type: d.read_reference_type()?,
                limits: d.read_limits()?,
            })
        })?;

        Ok(table)
    }

    fn decode_memory_section(&mut self) -> Result<MemorySection> {
        let memory = self.read_vec(|d| {
            Ok(MemoryType {
                limits: d.read_limits()?,
            })
        })?;

        Ok(memory)
    }

    fn decode_element_section(&mut self) -> Result<ElementSection> {
        let elements = self.read_vec(|d| {
            let prefix = d.read_size()?;

            let element = match prefix {
                0 => {
                    let Instruction::I32Const { value: offset } = d.read_const_expr()? else {
                        return Err(Error::Custom("element offset should be i32.const x".to_string()));
                    };
                    let init = d.read_vec(|d| {
                        d.read_size()
                    })?;

                    Element {
                        ref_type: RefType::FuncRef,
                        init,
                        mode: ElementMode::Active {
                            table_index: 0,
                            offset,
                        },
                    }
                }
                _ => return Err(Error::UnsupportedElementPrefix),
            };

            Ok(element)
        })?;

        Ok(elements)
    }

    fn decode_code_section(&mut self) -> Result<CodeSection> {
        let codes = self.read_vec(|d| {
            let func_size = d.read_size()?;
            let bytes = d.read_bytes(func_size as usize)?;
            let mut d = Decoder::new(bytes.as_slice());

            let local_decls = d.read_vec(|d| {
                let mut locals = vec![];
                let locals_count = d.read_size()?;
                let ty = ValueType::from(d.read_u8()?);

                for _ in 0..locals_count {
                    locals.push(ty.clone());
                }

                Ok(locals)
            })?;
            let locals = local_decls.into_iter().flatten().collect();

            let body = d.read_instructions()?;

            Ok(FuncBody { locals, code: body })
        })?;

        Ok(codes)
    }

    fn decode_global_section(&mut self) -> Result<GlobalSection> {
        let globals = self.read_vec(|d| {
            let value_type = ValueType::from(d.read_u8()?);
            let mutable = d.read_u8()? == 0x01;

            let init_expr = d.read_global_init_expr()?;

            Ok(Global {
                global_type: GlobalType {
                    value_type,
                    mutable,
                },
                init_expr,
            })
        })?;

        Ok(globals)
    }

    fn read_global_init_expr(&mut self) -> Result<GlobalInitExpr> {
        let opcode = self.read_u8()?;

        let expr = match opcode {
            0x41 => Ok(GlobalInitExpr::I32Const {
                value: self.read_i32()?,
            }),
            0x42 => Ok(GlobalInitExpr::I64Const {
                value: self.read_i64()?,
            }),
            0x43 => Ok(GlobalInitExpr::F32Const {
                value: self.read_f32()?,
            }),
            0x44 => Ok(GlobalInitExpr::F64Const {
                value: self.read_f64()?,
            }),
            _ => return Err(Error::InvalidGlobalInitExpr),
        };

        self.read_u8()?; // 0x0B

        expr
    }

    fn decode_export_section(&mut self) -> Result<ExportSection> {
        let exports = self.read_vec(|d| {
            let name = d.read_name()?;
            let desc_kind = d.read_u8()?;

            let desc = match desc_kind {
                0x00 => ExportDesc::Func(d.read_size()?),
                0x01 => ExportDesc::Table(d.read_size()?),
                0x02 => ExportDesc::Mem(d.read_size()?),
                0x03 => ExportDesc::Global(d.read_size()?),
                _ => return Err(Error::InvalidExportDesc),
            };

            Ok(Export { name, desc })
        })?;

        Ok(exports)
    }

    fn decode_start_section(&mut self) -> Result<StartSection> {
        Ok(StartSection {
            func_index: Some(self.read_size()?),
        })
    }

    fn read_const_expr(&mut self) -> Result<Instruction> {
        let opcode = self.read_u8()?;

        let result = match opcode {
            0x41 => Ok(Instruction::I32Const {
                value: self.read_i32()?,
            }),
            0x42 => Ok(Instruction::I64Const {
                value: self.read_i64()?,
            }),
            0x43 => Ok(Instruction::F32Const {
                value: self.read_f32()?,
            }),
            0x44 => Ok(Instruction::F64Const {
                value: self.read_f64()?,
            }),
            _ => return Err(Error::ExpectedConstExpression),
        };

        self.read_u8()?; // 0x0B

        result
    }

    fn read_limits(&mut self) -> Result<Limits> {
        let limits = match self.read_u8()? {
            0x00 => Limits {
                min: self.read_size()?,
                max: None,
            },
            0x01 => Limits {
                min: self.read_size()?,
                max: Some(self.read_size()?),
            },
            _ => return Err(Error::InvalidLimitsKind),
        };

        Ok(limits)
    }

    fn read_instructions(&mut self) -> Result<Vec<Instruction>> {
        let mut instructions = Vec::new();

        loop {
            let opcode = match self.read_u8() {
                Ok(opcode) => opcode,
                Err(err) => {
                    if err == Error::UnexpectedEof {
                        break;
                    }

                    return Err(err);
                }
            };

            let instr = match opcode {
                // control instructions
                0x00 => Instruction::Unreachable,
                0x01 => Instruction::Nop,
                0x02 => Instruction::Block {
                    block: Block {
                        block_type: self.read_block_type()?,
                    },
                },
                0x03 => Instruction::Loop {
                    block: Block {
                        block_type: self.read_block_type()?,
                    },
                },
                0x04 => Instruction::If {
                    block: Block {
                        block_type: self.read_block_type()?,
                    },
                },
                0x05 => Instruction::Else,
                0x0B => Instruction::End,
                0x0C => Instruction::Br {
                    level: self.read_size()?,
                },
                0x0D => Instruction::BrIf {
                    level: self.read_size()?,
                },
                0xE => Instruction::BrTable {
                    label_indexes: self.read_vec(|d| d.read_size())?,
                    default_index: self.read_size()?,
                },
                0x0F => Instruction::Return,
                0x10 => Instruction::Call {
                    func_index: self.read_size()?,
                },
                0x11 => Instruction::CallIndirect {
                    type_index: self.read_size()?,
                    table_index: self.read_size()?,
                },
                // reference instructions
                0xD0 => Instruction::RefNull {
                    ref_type: self.read_reference_type()?,
                },
                0xD1 => Instruction::RefIsNull,
                0xD2 => Instruction::RefFunc {
                    func_index: self.read_size()?,
                },
                // parametric instructions
                0x1A => Instruction::Drop,
                0x1B => Instruction::Select { result_types: None },
                0x1C => Instruction::Select {
                    result_types: Some(self.read_vec(|d| Ok(ValueType::from(d.read_u8()?)))?),
                },
                // variable instructions
                0x20 => Instruction::LocalGet {
                    local_index: self.read_size()?,
                },
                0x21 => Instruction::LocalSet {
                    local_index: self.read_size()?,
                },
                0x22 => Instruction::LocalTee {
                    local_index: self.read_size()?,
                },
                0x23 => Instruction::GlobalGet {
                    global_index: self.read_size()?,
                },
                0x24 => Instruction::GlobalSet {
                    global_index: self.read_size()?,
                },
                // table instructions
                0x25 => Instruction::TableGet {
                    table_index: self.read_size()?,
                },
                0x26 => Instruction::TableSet {
                    table_index: self.read_size()?,
                },
                0xFC => {
                    let id = self.read_size()?;

                    match id {
                        8 => {
                            let inst = Instruction::MemoryInit {
                                data_index: self.read_size()?,
                            };

                            self.read_u8()?; // 0x00

                            inst
                        }
                        9 => Instruction::DataDrop {
                            data_index: self.read_size()?,
                        },
                        10 => {
                            let inst = Instruction::MemoryCopy;

                            self.read_u8()?; // 0x00
                            self.read_u8()?; // 0x00

                            inst
                        }
                        11 => {
                            let inst = Instruction::MemoryFill;

                            self.read_u8()?; // 0x00

                            inst
                        }
                        12 => Instruction::TableInit {
                            element_index: self.read_size()?,
                            table_index: self.read_size()?,
                        },
                        13 => Instruction::ElemDrop {
                            element_index: self.read_size()?,
                        },
                        14 => Instruction::TableCopy {
                            dst_table_index: self.read_size()?,
                            src_table_index: self.read_size()?,
                        },
                        15 => Instruction::TableGrow {
                            table_index: self.read_size()?,
                        },
                        16 => Instruction::TableSize {
                            table_index: self.read_size()?,
                        },
                        17 => Instruction::TableFill {
                            table_index: self.read_size()?,
                        },
                        _ => return Err(Error::InvalidSubInstrId),
                    }
                }
                // memory instructions
                0x28 => Instruction::I32Load {
                    mem_arg: self.read_mem_arg()?,
                },
                0x29 => Instruction::I64Load {
                    mem_arg: self.read_mem_arg()?,
                },
                0x2A => Instruction::F32Load {
                    mem_arg: self.read_mem_arg()?,
                },
                0x2B => Instruction::F64Load {
                    mem_arg: self.read_mem_arg()?,
                },
                0x2C => Instruction::I32Load8S {
                    mem_arg: self.read_mem_arg()?,
                },
                0x2D => Instruction::I32Load8U {
                    mem_arg: self.read_mem_arg()?,
                },
                0x2E => Instruction::I32Load16S {
                    mem_arg: self.read_mem_arg()?,
                },
                0x2F => Instruction::I32Load16U {
                    mem_arg: self.read_mem_arg()?,
                },
                0x30 => Instruction::I64Load8S {
                    mem_arg: self.read_mem_arg()?,
                },
                0x31 => Instruction::I64Load8U {
                    mem_arg: self.read_mem_arg()?,
                },
                0x32 => Instruction::I64Load16S {
                    mem_arg: self.read_mem_arg()?,
                },
                0x33 => Instruction::I64Load16U {
                    mem_arg: self.read_mem_arg()?,
                },
                0x34 => Instruction::I64Load32S {
                    mem_arg: self.read_mem_arg()?,
                },
                0x35 => Instruction::I64Load32U {
                    mem_arg: self.read_mem_arg()?,
                },
                0x36 => Instruction::I32Store {
                    mem_arg: self.read_mem_arg()?,
                },
                0x37 => Instruction::I64Store {
                    mem_arg: self.read_mem_arg()?,
                },
                0x38 => Instruction::F32Store {
                    mem_arg: self.read_mem_arg()?,
                },
                0x39 => Instruction::F64Store {
                    mem_arg: self.read_mem_arg()?,
                },
                0x3A => Instruction::I32Store8 {
                    mem_arg: self.read_mem_arg()?,
                },
                0x3B => Instruction::I32Store16 {
                    mem_arg: self.read_mem_arg()?,
                },
                0x3C => Instruction::I64Store8 {
                    mem_arg: self.read_mem_arg()?,
                },
                0x3D => Instruction::I64Store16 {
                    mem_arg: self.read_mem_arg()?,
                },
                0x3E => Instruction::I64Store32 {
                    mem_arg: self.read_mem_arg()?,
                },
                0x3F => {
                    self.read_u8()?;
                    Instruction::MemorySize
                }
                0x40 => {
                    self.read_u8()?;
                    Instruction::MemoryGrow
                }
                /* numerics */
                0x41 => Instruction::I32Const {
                    value: self.read_i32()?,
                },
                0x42 => Instruction::I64Const {
                    value: self.read_i64()?,
                },
                0x43 => Instruction::F32Const {
                    value: self.read_f32()?,
                },
                0x44 => Instruction::F64Const {
                    value: self.read_f64()?,
                },
                0x45 => Instruction::I32Eqz,
                0x46 => Instruction::I32Eq,
                0x47 => Instruction::I32Ne,
                0x48 => Instruction::I32LtS,
                0x49 => Instruction::I32LtU,
                0x4A => Instruction::I32GtS,
                0x4B => Instruction::I32GtU,
                0x4C => Instruction::I32LeS,
                0x4D => Instruction::I32LeU,
                0x4E => Instruction::I32GeS,
                0x4F => Instruction::I32GeU,
                0x50 => Instruction::I64Eqz,
                0x51 => Instruction::I64Eq,
                0x52 => Instruction::I64Ne,
                0x53 => Instruction::I64LtS,
                0x54 => Instruction::I64LtU,
                0x55 => Instruction::I64GtS,
                0x56 => Instruction::I64GtU,
                0x57 => Instruction::I64LeS,
                0x58 => Instruction::I64LeU,
                0x59 => Instruction::I64GeS,
                0x5A => Instruction::I64GeU,
                0x5B => Instruction::F32Eq,
                0x5C => Instruction::F32Ne,
                0x5D => Instruction::F32Lt,
                0x5E => Instruction::F32Gt,
                0x5F => Instruction::F32Le,
                0x60 => Instruction::F32Ge,
                0x61 => Instruction::F64Eq,
                0x62 => Instruction::F64Ne,
                0x63 => Instruction::F64Lt,
                0x64 => Instruction::F64Gt,
                0x65 => Instruction::F64Le,
                0x66 => Instruction::F64Ge,
                0x67 => Instruction::I32Clz,
                0x68 => Instruction::I32Ctz,
                0x69 => Instruction::I32Popcnt,
                0x6A => Instruction::I32Add,
                0x6B => Instruction::I32Sub,
                0x6C => Instruction::I32Mul,
                0x6D => Instruction::I32DivS,
                0x6E => Instruction::I32DivU,
                0x6F => Instruction::I32RemS,
                0x70 => Instruction::I32RemU,
                0x71 => Instruction::I32And,
                0x72 => Instruction::I32Or,
                0x73 => Instruction::I32Xor,
                0x74 => Instruction::I32Shl,
                0x75 => Instruction::I32ShrS,
                0x76 => Instruction::I32ShrU,
                0x77 => Instruction::I32Rotl,
                0x78 => Instruction::I32Rotr,
                0x79 => Instruction::I64Clz,
                0x7A => Instruction::I64Ctz,
                0x7B => Instruction::I64Popcnt,
                0x7C => Instruction::I64Add,
                0x7D => Instruction::I64Sub,
                0x7E => Instruction::I64Mul,
                0x7F => Instruction::I64DivS,
                0x80 => Instruction::I64DivU,
                0x81 => Instruction::I64RemS,
                0x82 => Instruction::I64RemU,
                0x83 => Instruction::I64And,
                0x84 => Instruction::I64Or,
                0x85 => Instruction::I64Xor,
                0x86 => Instruction::I64Shl,
                0x87 => Instruction::I64ShrS,
                0x88 => Instruction::I64ShrU,
                0x89 => Instruction::I64Rotl,
                0x8A => Instruction::I64Rotr,
                0x8B => Instruction::F32Abs,
                0x8C => Instruction::F32Neg,
                0x8D => Instruction::F32Ceil,
                0x8E => Instruction::F32Floor,
                0x8F => Instruction::F32Trunc,
                0x90 => Instruction::F32Nearest,
                0x91 => Instruction::F32Sqrt,
                0x92 => Instruction::F32Add,
                0x93 => Instruction::F32Sub,
                0x94 => Instruction::F32Mul,
                0x95 => Instruction::F32Div,
                0x96 => Instruction::F32Min,
                0x97 => Instruction::F32Max,
                0x98 => Instruction::F32Copysign,
                0x99 => Instruction::F64Abs,
                0x9A => Instruction::F64Neg,
                0x9B => Instruction::F64Ceil,
                0x9C => Instruction::F64Floor,
                0x9D => Instruction::F64Trunc,
                0x9E => Instruction::F64Nearest,
                0x9F => Instruction::F64Sqrt,
                0xA0 => Instruction::F64Add,
                0xA1 => Instruction::F64Sub,
                0xA2 => Instruction::F64Mul,
                0xA3 => Instruction::F64Div,
                0xA4 => Instruction::F64Min,
                0xA5 => Instruction::F64Max,
                0xA6 => Instruction::F64Copysign,
                0xA7 => Instruction::I32WrapI64,
                0xA8 => Instruction::I32TruncF32S,
                0xA9 => Instruction::I32TruncF32U,
                0xAA => Instruction::I32TruncF64S,
                0xAB => Instruction::I32TruncF64U,
                0xAC => Instruction::I64ExtendI32S,
                0xAD => Instruction::I64ExtendI32U,
                0xAE => Instruction::I64TruncF32S,
                0xAF => Instruction::I64TruncF32U,
                0xB0 => Instruction::I64TruncF64S,
                0xB1 => Instruction::I64TruncF64U,
                0xB2 => Instruction::F32ConvertI32S,
                0xB3 => Instruction::F32ConvertI32U,
                0xB4 => Instruction::F32ConvertI64S,
                0xB5 => Instruction::F32ConvertI64U,
                0xB6 => Instruction::F32DemoteF64,
                0xB7 => Instruction::F64ConvertI32S,
                0xB8 => Instruction::F64ConvertI32U,
                0xB9 => Instruction::F64ConvertI64S,
                0xBA => Instruction::F64ConvertI64U,
                0xBB => Instruction::F64PromoteF32,
                0xBC => Instruction::I32ReinterpretF32,
                0xBD => Instruction::I64ReinterpretF64,
                0xBE => Instruction::F32ReinterpretI32,
                0xBF => Instruction::F64ReinterpretI64,
                0xC0 => Instruction::I32Extend8S,
                0xC1 => Instruction::I32Extend16S,
                0xC2 => Instruction::I64Extend8S,
                0xC3 => Instruction::I64Extend16S,
                0xC4 => Instruction::I64Extend32S,
                _ => unimplemented!("Opcode 0x{:02x} is not implemented", opcode),
            };

            instructions.push(instr);
        }

        Ok(instructions)
    }

    fn read_mem_arg(&mut self) -> Result<MemArg> {
        Ok(MemArg {
            align: self.read_size()?,
            offset: self.read_size()?,
        })
    }

    fn read_reference_type(&mut self) -> Result<RefType> {
        let type_id = self.read_u8()?;

        match type_id {
            0x70 => Ok(RefType::FuncRef),
            0x6F => Ok(RefType::ExternRef),
            _ => Err(Error::InvalidRefType),
        }
    }

    fn read_block_type(&mut self) -> Result<BlockType> {
        let type_id = self.read_u8()?;

        match type_id {
            0x40 => Ok(BlockType::Empty),
            0x7f => Ok(BlockType::Value(vec![ValueType::I32])),
            0x7e => Ok(BlockType::Value(vec![ValueType::I64])),
            0x7d => Ok(BlockType::Value(vec![ValueType::F32])),
            0x7c => Ok(BlockType::Value(vec![ValueType::F64])),
            _ => Err(Error::InvalidBlockType),
        }
    }

    fn decode_section(&mut self) -> Result<(SectionId, u32)> {
        let id = self.read_u8()?;
        let id = SectionId::from(id);
        if id.is_unknown() {
            return Err(Error::InvalidSectionId(id));
        }

        let size = self.read_size()?;

        Ok((id, size))
    }

    fn read_size(&mut self) -> Result<u32> {
        let size_result = leb128::read::unsigned(&mut self.reader);

        match size_result {
            Ok(size) => Ok(size as u32),
            Err(_) => Err(Error::UnexpectedEof),
        }
    }

    fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>> {
        let mut bytes = vec![0; size];
        let result = self.reader.read_exact(&mut bytes);

        match result {
            Ok(_) => Ok(bytes),
            Err(_) => Err(Error::UnexpectedEof),
        }
    }

    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0; 1];
        let result = self.reader.read_exact(&mut buf);

        match result {
            Ok(_) => Ok(buf[0]),
            Err(_) => Err(Error::UnexpectedEof),
        }
    }

    fn read_i32(&mut self) -> Result<i32> {
        let size_result = leb128::read::signed(&mut self.reader);

        match size_result {
            Ok(size) => Ok(size as i32),
            Err(_) => Err(Error::UnexpectedEof),
        }
    }

    fn read_i64(&mut self) -> Result<i64> {
        let size_result = leb128::read::signed(&mut self.reader);

        match size_result {
            Ok(size) => Ok(size),
            Err(_) => Err(Error::UnexpectedEof),
        }
    }

    fn read_f32(&mut self) -> Result<f32> {
        let mut buf = [0; 4];
        let result = self.reader.read_exact(&mut buf);

        match result {
            Ok(_) => Ok(f32::from_le_bytes(buf)),
            Err(_) => Err(Error::UnexpectedEof),
        }
    }

    fn read_f64(&mut self) -> Result<f64> {
        let mut buf = [0; 8];
        let result = self.reader.read_exact(&mut buf);

        match result {
            Ok(_) => Ok(f64::from_le_bytes(buf)),
            Err(_) => Err(Error::UnexpectedEof),
        }
    }

    fn read_vec<T>(&mut self, parser: impl Fn(&mut Self) -> Result<T>) -> Result<Vec<T>> {
        let size = self.read_size()?;
        let mut items = Vec::with_capacity(size as usize);

        for _ in 0..size {
            items.push(parser(self)?);
        }

        Ok(items)
    }

    fn read_name(&mut self) -> Result<String> {
        let size = self.read_size()?;
        let mut buf = vec![0; size as usize];
        let result = self.reader.read_exact(&mut buf);

        match result {
            Ok(_) => Ok(String::from_utf8(buf).unwrap()),
            Err(_) => Err(Error::UnexpectedEof),
        }
    }
}
