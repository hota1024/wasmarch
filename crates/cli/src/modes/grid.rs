use nannou::{color::rgb_u32, prelude::*};
use runtime::runtime::Runtime;

use crate::std_lib::StdLib;

static mut MODEL: Option<Model> = None;

pub fn grid_main(instance: Runtime) {
    let mut instance = instance.clone();

    let grid_width = instance.get_global("grid_width").unwrap().into_i32();
    let grid_height = instance.get_global("grid_height").unwrap().into_i32();
    let grid_pixel_size = instance.get_global("grid_pixel_size").unwrap().into_f32();

    instance.set_call_external_hook(|inst, args| {
        let inst = inst.clone();
        let args = args.clone();

        let result = StdLib::hook(&inst, &args);
        result.unwrap()
    });

    instance.start().unwrap();

    unsafe {
        MODEL = Some(Model { instance });
    }

    nannou::app(model)
        .update(update)
        .size(
            grid_width as u32 * grid_pixel_size as u32,
            grid_height as u32 * grid_pixel_size as u32,
        )
        .simple_window(view)
        .run();
}

#[derive(Clone)]
struct Model {
    instance: Runtime,
}

fn model(_app: &App) -> Model {
    unsafe { MODEL.clone().unwrap() }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.instance.invoke("grid_frame", &[]).unwrap();
    let grid_width = model.instance.get_global("grid_width").unwrap().into_i32();
    let grid_height = model.instance.get_global("grid_height").unwrap().into_i32();
    app.main_window()
        .set_title(format!("[wasmarch] grid {}x{}", grid_width, grid_height).as_str());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let grid_width = model.instance.get_global("grid_width").unwrap().into_i32();
    let grid_height = model.instance.get_global("grid_height").unwrap().into_i32();
    let grid_pixel_size = model
        .instance
        .get_global("grid_pixel_size")
        .unwrap()
        .into_f32();
    app.main_window().set_inner_size_pixels(
        grid_width as u32 * grid_pixel_size as u32,
        grid_height as u32 * grid_pixel_size as u32,
    );
    app.main_window().set_inner_size_points(
        grid_width as f32 * grid_pixel_size,
        grid_height as f32 * grid_pixel_size,
    );

    let draw = app.draw();

    let memory = model.instance.get_memory("grid_memory").unwrap();
    frame.clear(rgba(0u8, 0u8, 0u8, 64u8));

    let bx = (grid_width as f32 * grid_pixel_size) / -2.0 + grid_pixel_size / 2.0;
    let by = (grid_height as f32 * grid_pixel_size) / 2.0 - grid_pixel_size / 2.0;

    for y in 0..grid_height {
        for x in 0..grid_width {
            let i = (y * grid_width + x) as usize * 4;
            let bytes = &memory[i..(i + 4)];
            let color = u32::from_le_bytes(bytes.try_into().unwrap());

            draw.rect()
                .x(bx + x as f32 * grid_pixel_size)
                .y(by - y as f32 * grid_pixel_size)
                .w(grid_pixel_size)
                .h(grid_pixel_size)
                .color(rgb_u32(color));
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
