use types::RefType;

use crate::value::Ref;

#[derive(Debug, Clone, PartialEq)]
pub struct ElemInst {
    pub ref_type: RefType,
    pub elems: Vec<Ref>,
}
