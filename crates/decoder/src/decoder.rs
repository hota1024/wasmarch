use std::io::{BufReader, Read};

use binary::{Module, TypeSection};

pub struct Decoder<R> {
    reader: BufReader<R>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidMagicHeader,
    InvalidVersion,
    InvalidSectionId,
    InvalidSectionSize,
}

impl<R: Read> Decoder<R> {
    pub fn new(reader: R) -> Self {
        Decoder {
            reader: BufReader::new(reader),
        }
    }

    pub fn decode(&mut self) -> Result<Module, Error> {
        self.validate_magic_header()?;

        let version = self.get_version();
        if version != 1 {
            return Err(Error::InvalidVersion);
        }

        Ok(Module::default())
    }

    fn validate_magic_header(&mut self) -> Result<(), Error> {
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

    fn decode_type_section(&mut self) -> Option<TypeSection> {
        None
    }

    fn decode_section(&mut self) -> Result<(u8, u32), Error> {
        let mut id = [0; 1];
        self.reader.read_exact(&mut id).unwrap();
        let size_result = leb128::read::unsigned(&mut self.reader);

        let size = match size_result {
            Ok(size) => size as u32,
            Err(_) => return Err(Error::InvalidSectionSize),
        };

        Ok((id[0], size))
    }
}
