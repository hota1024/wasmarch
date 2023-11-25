use rand::prelude::*;

use runtime::{instances::ExternalFuncInst, value::Val};

pub struct StdLib {}

impl StdLib {
    pub fn hook(inst: &ExternalFuncInst, args: &Vec<Val>) -> Option<Val> {
        let module = inst.module.clone();
        let field = inst.field.clone();

        match (&module[..], &field[..]) {
            ("std", "newline") => {
                println!("");
                Some(Val::None)
            }
            ("std", "log_i32") => {
                println!("{}", args[0].into_i32());
                Some(Val::None)
            }
            ("std", "log_i32x2") => {
                println!("{}, {}", args[0].into_i32(), args[1].into_i32());
                Some(Val::None)
            }
            ("std", "log_i32x3") => {
                println!(
                    "{}, {}, {}",
                    args[0].into_i32(),
                    args[1].into_i32(),
                    args[2].into_i32()
                );
                Some(Val::None)
            }

            ("std", "log_i64") => {
                println!("{}", args[0].into_i64());
                Some(Val::None)
            }
            ("std", "log_i64x2") => {
                println!("{}, {}", args[0].into_i64(), args[1].into_i64());
                Some(Val::None)
            }
            ("std", "log_i64x3") => {
                println!(
                    "{}, {}, {}",
                    args[0].into_i64(),
                    args[1].into_i64(),
                    args[2].into_i64()
                );
                Some(Val::None)
            }

            ("std", "log_f32") => {
                println!("{}", args[0].into_f32());
                Some(Val::None)
            }
            ("std", "log_f32x2") => {
                println!("{}, {}", args[0].into_f32(), args[1].into_f32());
                Some(Val::None)
            }
            ("std", "log_f32x3") => {
                println!(
                    "{}, {}, {}",
                    args[0].into_f32(),
                    args[1].into_f32(),
                    args[2].into_f32()
                );
                Some(Val::None)
            }

            ("std", "log_f64") => {
                println!("{}", args[0].into_f64());
                Some(Val::None)
            }
            ("std", "log_f64x2") => {
                println!("{}, {}", args[0].into_f64(), args[1].into_i64());
                Some(Val::None)
            }
            ("std", "log_f64x3") => {
                println!(
                    "{}, {}, {}",
                    args[0].into_f64(),
                    args[1].into_f64(),
                    args[2].into_f64()
                );
                Some(Val::None)
            }

            ("std", "random_bool") => {
                let v = random::<bool>();
                Some(Val::from(v))
            }
            _ => None,
        }
    }
}
