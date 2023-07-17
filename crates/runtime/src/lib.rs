use decoder::Decoder;
use runtime::Runtime;

mod frame;
mod instances;
mod label;
mod result;
mod runtime;
mod rust_value;
mod store;
pub mod value;

pub use crate::result::*;

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
