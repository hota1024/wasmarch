use std::{cell::RefCell, rc::Rc};

use nannou::prelude::*;
use runtime::{instances::InternalFuncInst, runtime::Runtime, value::Val};

use crate::std_lib::StdLib;

static mut MODEL: Option<Model> = None;

pub fn sketch_main(instance: Runtime) {
    let mut instance = instance.clone();

    let canvas_width = instance.get_global("canvas_width").unwrap().into_i32();
    let canvas_height = instance.get_global("canvas_height").unwrap().into_i32();

    instance.start().unwrap();

    unsafe {
        MODEL = Some(Model { instance });
    }

    nannou::sketch(view)
        .size(canvas_width as u32, canvas_height as u32)
        .run();
}

#[derive(Clone)]
struct Model {
    instance: Runtime,
}

fn view(app: &App, frame: Frame) {
    let mut instance = unsafe { MODEL.clone().unwrap().instance };

    let draw = app.draw();
    draw.background().color(BLACK);

    // instance.set_call_external_hook(|inst: &InternalFuncInst, args: &Vec<Val>| {
    //     let inst = inst.clone();
    //     let args = args.clone();

    //     let result = StdLib::hook(&inst, &args);

    //     draw.background().color(WHITE);
    //     result.unwrap()
    // });

    // let mut a = Test { f: None };
    // println!("OK");
    // a.set(|c| {
    //     println!("call");
    //     draw.background().color(RED);
    // });
    // a.call();

    draw.to_frame(app, &frame).unwrap();
}

struct Test<F>
where
    F: FnMut(i32) -> (),
{
    pub f: Option<F>,
}

impl<F> Test<F>
where
    F: FnMut(i32) -> (),
{
    pub fn set(&mut self, mut f: F)
    where
        F: FnMut(i32),
    {
        self.f = Some(f)
    }

    pub fn call(&mut self) {
        (self.f.as_mut().unwrap())(10);
    }
}
