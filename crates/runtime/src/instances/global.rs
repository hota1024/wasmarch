use types::GlobalType;

use crate::value::Val;

#[derive(Debug, Clone, PartialEq)]
pub struct GlobalInst {
    pub global_type: GlobalType,
    pub value: Val,
}
