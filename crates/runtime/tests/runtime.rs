use runtime::value::Val;
use wabt::wat2wasm;

macro_rules! instantiate_test_wat {
    ($wat_path: tt) => {{
        let wat = include_bytes!(concat!("wasm/", $wat_path, ".wat"));
        let wasm = wat2wasm(wat).unwrap();
        runtime::instantiate(&wasm).unwrap()
    }};
}

macro_rules! values {
    ($($v: expr), *) => {
        &[$(Val::from($v)), *]
    };
}

#[test]
fn test_return_const() {
    let mut instance = instantiate_test_wat!("return_const");

    let val_i32 = instance.invoke("returns_i32", &[]).unwrap();
    let val_i64 = instance.invoke("returns_i64", &[]).unwrap();
    let val_f32 = instance.invoke("returns_f32", &[]).unwrap();
    let val_f64 = instance.invoke("returns_f64", &[]).unwrap();

    assert_eq!(val_i32, Val::I32(1234));
    assert_eq!(val_i64, Val::I64(9_223_372_036_854_775_807));
    assert_eq!(val_f32, Val::F32(3.14));
    assert_eq!(val_f64, Val::F64(-3.14));
}

#[test]
fn test_return() {
    let mut instance = instantiate_test_wat!("return");

    let return_1 = instance.invoke("return_1", &[]).unwrap();

    assert_eq!(return_1, Val::I32(1));
}

#[test]
fn test_if_else() {
    let mut instance = instantiate_test_wat!("if_else");

    let if_1 = instance.invoke("if_1", &[]).unwrap();
    assert_eq!(if_1, Val::I32(1));

    let if_0 = instance.invoke("if_0", &[]).unwrap();
    assert_eq!(if_0, Val::I32(0));

    let if_if_1 = instance.invoke("if_if_1", &[]).unwrap();
    assert_eq!(if_if_1, Val::I32(1));

    let if_if_0 = instance.invoke("if_if_0", &[]).unwrap();
    assert_eq!(if_if_0, Val::I32(0));
}

#[test]
fn test_local() {
    let mut instance = instantiate_test_wat!("local");

    let local_get = instance.invoke("local_get", &[]).unwrap();

    assert_eq!(local_get, Val::I32(1234));
}

#[test]
fn test_param() {
    let mut instance = instantiate_test_wat!("param");

    let add_5 = instance.invoke("add", &[Val::I32(2), Val::I32(3)]).unwrap();

    assert_eq!(add_5, Val::I32(5));
}

#[test]
fn test_call_internal() {
    let mut instance = instantiate_test_wat!("call_internal");

    let result = instance.invoke("call_internal", values![]).unwrap();

    assert_eq!(result, Val::from(7));
}

#[test]
fn test_fib() {
    let mut instance = instantiate_test_wat!("fib");

    let result = instance.invoke("fib_rec", values![10]).unwrap();

    assert_eq!(result, Val::from(55));
}
