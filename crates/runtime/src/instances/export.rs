use crate::value::ExternalVal;

#[derive(Debug, Clone, PartialEq)]
pub struct ExportInst {
    pub name: String,
    pub value: ExternalVal,
}
