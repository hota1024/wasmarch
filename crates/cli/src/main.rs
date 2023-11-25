mod modes;
pub mod std_lib;

use core::panic;
use std::fs;

use clap::{Parser, ValueEnum};
use runtime::instantiate;
use wabt::wat2wasm;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    wasm: String,

    #[arg(short, long)]
    #[clap(value_enum, default_value_t = Mode::Std)]
    mode: Mode,

    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    invoke: Vec<String>,
}

#[derive(ValueEnum, Clone, Debug)]
enum Mode {
    Std,
    Grid,
    Sketch,
}

fn main() {
    let args = Args::parse();
    let wasm = if args.wasm.ends_with(".wat") {
        let wat_text = fs::read_to_string(args.wasm).unwrap();
        wat2wasm(wat_text).unwrap()
    } else if args.wasm.ends_with(".wasm") {
        fs::read(args.wasm).unwrap()
    } else if args.wasm.ends_with(".was") || args.wasm.ends_with(".wasc") {
        let was = fs::read_to_string(args.wasm).unwrap();
        let wat = wasc::compile(&was);
        wat2wasm(wat).unwrap()
    } else {
        panic!("wasm path should end with \".wasm\" or \".wat\" or \".was\" or \".wasc\"");
    };
    let instance = instantiate(&wasm).unwrap();

    match args.mode {
        Mode::Std => {
            modes::std::start_std(instance, args.invoke);
        }
        Mode::Grid => {
            modes::grid::grid_main(instance);
        }
        Mode::Sketch => {
            modes::sketch::sketch_main(instance);
        } // _ => unimplemented!("unimplemented mode {:?}", args.mode),
    }
}
