use crate::{Error, Result, SectionId};
use binary::{
    instruction::Instruction, CodeSection, FuncBody, FunctionSection, Import, ImportDesc,
    ImportSection, Module, Type, TypeSection,
};
use std::io::{BufReader, Read};
use types::{FuncType, ValueType};

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
                SectionId::Type => {
                    module.type_section = Some(self.decode_type_section()?);
                }
                SectionId::Import => {
                    module.import_section = Some(self.decode_import_section()?);
                }
                SectionId::Function => {
                    module.function_section = Some(self.decode_function_section()?);
                }
                SectionId::Code => {
                    module.code_section = Some(self.decode_code_section()?);
                }
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

    fn decode_code_section(&mut self) -> Result<CodeSection> {
        let codes = self.read_vec(|d| {
            let size = d.read_size()?;
            println!("size: {}", size);

            let locals = d.read_vec(|d| {
                d.read_size()?;

                Ok(ValueType::from(d.read_u8()?))
            })?;

            let body = d.decode_expr()?;
            println!("{:?}", body);

            Ok(FuncBody { locals, body })
        })?;

        // let codes = vec![];
        Ok(codes)
    }

    fn decode_expr(&mut self) -> Result<Vec<Instruction>> {
        let mut instructions = Vec::new();

        loop {
            let opcode = self.read_u8()?;

            let instr = match opcode {
                /* variables */
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
                /* numerics */
                0x41 => Instruction::I32Const {
                    value: self.read_i32()?,
                },
                0x42 => Instruction::I64Const {
                    value: self.read_i64()?,
                },
                0x43 => Instruction::F32Const { value: todo!() },
                0x44 => Instruction::F64Const { value: todo!() },
                0x6a => Instruction::I32Add,
                0x0b => break,
                _ => unimplemented!("Opcode {:02x} is not implemented", opcode),
            };

            instructions.push(instr);
        }

        Ok(instructions)
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
