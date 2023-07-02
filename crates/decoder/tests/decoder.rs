use binary::Type;
use decoder::{Decoder, Error};
use types::FuncType;
use wabt::wat2wasm;

macro_rules! print_wasm {
    ($wasm:expr) => {
        println!(
            "wasm: {:?}",
            $wasm
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<_>>()
        );
    };
}

#[test]
fn test_should_returns_error_when_magic_header_is_invalid() {
    let mut wasm = wat2wasm("(module)").expect("Failed to parse wat");
    wasm[0] = 0x01;

    let mut decoder = Decoder::new(&wasm[..]);
    let result = decoder.decode();

    assert_eq!(result.err(), Some(Error::InvalidMagicHeader));
}

#[test]
fn test_should_returns_error_when_version_is_not_supported() {
    let mut wasm = wat2wasm("(module)").expect("Failed to parse wat");
    wasm[4] = 0x02;

    let mut decoder = Decoder::new(&wasm[..]);
    let result = decoder.decode();

    assert_eq!(result.err(), Some(Error::InvalidVersion));
}

#[test]
fn test_should_decode_type_section() {
    let wasm = wat2wasm("(module (type $returns_i32 (func (param i64) (result i32))))")
        .expect("Failed to parse wat");

    let mut decoder = Decoder::new(&wasm[..]);
    let result = decoder.decode().unwrap();

    assert_eq!(
        result.type_section,
        Some(
            vec![Type::Func(FuncType {
                params: Box::from([types::ValueType::I64]),
                results: Box::from([types::ValueType::I32]),
            })]
            .into_boxed_slice()
        )
    );
}

#[test]
fn test_should_decode_import_section() {
    let wasm = wat2wasm(
        "
        (module
            (import \"env\" \"add\" (func $add (param i32 i32) (result i32)))
            (import \"env\" \"addf\" (func $sub (param f32 f32) (result f32)))
        )",
    )
    .expect("Failed to parse wat");

    let mut decoder = Decoder::new(&wasm[..]);
    let result = decoder.decode().unwrap();

    assert_eq!(
        result.import_section,
        Some(
            vec![
                binary::Import {
                    module: "env".to_string(),
                    field: "add".to_string(),
                    desc: binary::ImportDesc::Func(0),
                },
                binary::Import {
                    module: "env".to_string(),
                    field: "addf".to_string(),
                    desc: binary::ImportDesc::Func(1),
                },
            ]
            .into_boxed_slice()
        )
    );
}

#[test]
fn test_should_decode_function_section() {
    let wasm = wat2wasm(
        "
        (module
            (func $add (param i32 i32) (result i32) local.get 0 local.get 1 i32.add)
        )",
    )
    .expect("Failed to parse wat");

    let mut decoder = Decoder::new(&wasm[..]);
    let result = decoder.decode().unwrap();

    assert_eq!(result.function_section, Some(Box::from([0])));
    assert_eq!(
        result.code_section,
        Some(Box::from([binary::FuncBody {
            locals: Box::from([]),
            body: Box::from([
                binary::Instruction::LocalGet { local_index: 0 },
                binary::Instruction::LocalGet { local_index: 1 },
                binary::Instruction::I32Add,
            ])
        }]))
    )
}
