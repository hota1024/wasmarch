use runtime::value::Val;
use wabt::wat2wasm;

macro_rules! instantiate_test_wat {
    ($wat_path: tt) => {{
        let wat = include_bytes!(concat!("wasm/", $wat_path, ".wat"));
        let wasm = wat2wasm(wat).unwrap();
        runtime::instantiate(&wasm).unwrap()
    }};
}

#[test]
fn test_return_const() {
    let mut instance = instantiate_test_wat!("return_const");
    let result = instance.invoke("returns_i32", &[]).unwrap();

    println!("result: {:?}", result);
}
