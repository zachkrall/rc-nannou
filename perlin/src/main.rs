use nannou::prelude::*;

use nannou::noise::{Perlin,Worley,NoiseFn};

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

            let noise = Worley::new();
            let size = (noise.get([
                t as f64,
                map_range(x, -hw, hw, 0, 10) as f64,
                map_range(y, -hh, hh, 0, 10) as f64
            ])*20.0).abs() as f32 ;

            // let size = (noise.get([
            //     t as f64,
            //     x as f64,
            //     y as f64
            // ])*20.0).abs() as f32 ;

            draw.line()
                .start( Point2::new(x, y) )
                .end( Point2::new(x+size, y-size))
                .thickness(1.0)
                .color(WHITE);

        }
    }

    draw.to_frame(_app, &frame).unwrap();
    frame
}
