use decoder::decoder::{Decoder, Error};
use wabt::wat2wasm;

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
    let wasm =
        wat2wasm("(module (type $returns_i32 (func (result i32))))").expect("Failed to parse wat");
    println!("{:?}", wasm);
}
