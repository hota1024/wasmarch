use types::Limits;

#[derive(Debug, Clone, PartialEq)]
pub struct MemInst {
    pub mem_type: Limits,
    pub data: Vec<u8>,
}
