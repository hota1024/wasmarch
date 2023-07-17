/// Value type.
///
/// WebAssembly specification: https://webassembly.github.io/spec/core/syntax/types.html#value-types
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValueType {
    // Number types.
    I32, // 0x7F
    I64, // 0x7E
    F32, // 0x7D
    F64, // 0x7C
         // TODO: add vectype
         // TODO: add reftype
}

impl From<u8> for ValueType {
    fn from(value: u8) -> Self {
        match value {
            0x7F => ValueType::I32,
            0x7E => ValueType::I64,
            0x7D => ValueType::F32,
            0x7C => ValueType::F64,
            _ => panic!("Invalid value type: 0x{:02x}", value),
        }
    }
}
