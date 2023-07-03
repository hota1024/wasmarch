use types::TableType;

use crate::value::Ref;

#[derive(Debug, Clone, PartialEq)]
pub struct TableInst {
    pub table_type: TableType,
    pub elems: Vec<Ref>,
}
