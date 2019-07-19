use nannou::prelude::*;

use nannou::noise::{Worley,NoiseFn};

use palette::Rgb;

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

struct Model {
}

fn model(_app: &App) -> Model {
    let width  = 900 as u32;
    let height = 900 as u32;

    _app.new_window()
        .with_dimensions(width, height)
        .build()
        .unwrap();

    Model {
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(_app: &App, _model: &Model, frame: Frame) -> Frame {
    let t = _app.time;
    let win  = _app.window_rect();
    let draw = _app.draw();

    let bg = Rgb::new(0.012, 0.012, 0.012);

    draw.background().color(bg);

    let w = win.w() as i32;
    let h = win.h() as i32;

    let hw = win.w() / 2.;
    let hh = win.h() / 2.;

    let step = 15;
    let m = 15;

    for i in (-w/2-m..w/2+m).step_by(step) {
        for j in (-h/2-m..h/2+m).step_by(step) {

            let x = i as f32;
            let y = j as f32;

            let size = x.sin() * y.sin() * 5.0;

            draw.ellipse()
                .x_y(x, y)
                .radius(size)
                .color(WHITE);

        }
    }

    draw.to_frame(_app, &frame).unwrap();
    frame
}
