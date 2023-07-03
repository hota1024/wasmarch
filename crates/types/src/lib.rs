#![no_std]

extern crate alloc;

pub mod func_type;
pub mod global_type;
pub mod limits;
pub mod memory_type;
pub mod ref_type;
pub mod result_type;
pub mod table_type;
pub mod value_type;

pub use func_type::*;
pub use global_type::*;
pub use limits::*;
pub use memory_type::*;
pub use ref_type::*;
pub use result_type::*;
pub use table_type::*;
pub use value_type::*;
