use runtime::{runtime::Runtime, value::Val};
use types::ValueType;

use crate::std_lib::StdLib;

pub fn start_std(instance: Runtime, invoke_args: Vec<String>) {
    let mut instance = instance.clone();

    instance.set_call_external_hook(|inst, args| {
        let inst = inst.clone();
        let args = args.clone();

        let result = StdLib::hook(&inst, &args);
        result.unwrap()
    });

    instance.start().unwrap();

    if invoke_args.len() > 0 {
        let fn_name = &invoke_args[0];

        let func = instance.get_func(&fn_name).unwrap();

        match func {
            runtime::instances::FuncInst::Internal(inst) => {
                let mut args: Vec<Val> = vec![];
                let mut i = 1;

                for param in inst.func_type.params {
                    let arg_str = &invoke_args[i];

                    let arg = match param {
                        ValueType::I32 => Val::I32(arg_str.parse::<i32>().unwrap()),
                        ValueType::I64 => Val::I64(arg_str.parse::<i64>().unwrap()),
                        ValueType::F32 => Val::F32(arg_str.parse::<f32>().unwrap()),
                        ValueType::F64 => Val::F64(arg_str.parse::<f64>().unwrap()),
                        // _ => panic!("unsupported arg type {}: {:?}", arg_str, param),
                    };
                    args.push(arg);

                    i += 1;
                }

                match instance.invoke(fn_name, &args) {
                    Ok(result) => println!("{}", result),
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
