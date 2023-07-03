use binary::Module;

use crate::{result::Result, store::Store, value::Val};

struct Runtime {
    store: Store,
}

impl From<Module> for Runtime {
    fn from(module: Module) -> Self {
        Self {
            store: Store::from_module(module).unwrap(),
        }
    }
}

impl Runtime {
    pub fn invoke(&mut self, name: String, args: Vec<Val>) -> Result<Val> {
        Ok(Val::I32(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use decoder::Decoder;
    use wabt::wat2wasm;

    #[test]
    fn test() {
        let wasm = wat2wasm(
            "
        (module
            (func $add (param i32 i32) (result i32) local.get 0 local.get 1 i32.add)
        )",
        )
        .unwrap();

        let mut decoder = Decoder::new(&wasm[..]);
        let module = decoder.decode().unwrap();

        let mut runtime = Runtime::from(module);
        let result = runtime
            .invoke("add".to_string(), vec![Val::I32(1), Val::I32(2)])
            .unwrap();

        println!("result: {:?}", result);
    }
}
