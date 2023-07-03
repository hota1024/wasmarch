use crate::value::ExternalVal;

#[derive(Debug, Clone, PartialEq)]
pub struct ExportInst {
    name: String,
    value: ExternalVal,
}
