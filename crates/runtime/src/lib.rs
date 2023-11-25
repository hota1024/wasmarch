use decoder::Decoder;
use instances::ExternalFuncInst;
use runtime::Runtime;
use value::Val;

pub mod frame;
pub mod instances;
pub mod label;
pub mod result;
pub mod runtime;
pub mod rust_value;
pub mod store;
pub mod value;

pub use crate::result::*;
pub use crate::runtime::*;

pub fn instantiate(reader: &[u8]) -> Result<Runtime> {
    let mut decoder = Decoder::new(reader);
    let decode_result = decoder.decode();

    match decode_result {
        Ok(module) => {
            let store = store::Store::from_module(module)?;
            Ok(Runtime::new(store))
        }
        Err(err) => Err(Error::DecodeError(err)),
    }
}
