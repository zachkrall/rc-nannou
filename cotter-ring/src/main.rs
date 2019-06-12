use nannou::prelude::*;
use nannou::noise::*;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    _app.new_window()
        .with_dimensions(500 as u32, 500 as u32)
        .view(view)
        .build()
        .unwrap();
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(_app: &App, _model: &Model, frame: Frame) -> Frame {

    let t = _app.time;
    let draw = _app.draw();

    for i in 0..720{
        let rad = i as f32 * PI / 180.0;
        let x = rad.cos();
        let y = rad.sin();
        let ring_size = 180.0;

        let noise = Perlin::new();
        let size = noise.get([
                x as f64,
                y as f64,
                (t*0.8) as f64
            ]) as f32;

        draw.rect()
        .x_y(x * ring_size, y * ring_size)
        .w( size * 45.0 + 24.0)
        .h( size * 45.0 + 24.0)
        .rgb(((rad+t).sin() + 1.0) / 2.0,
             ((rad+t).cos() + 1.0) / 2.0,
             ((rad-t).sin() + 1.0) / 2.0);
    }

    frame.clear(GRAY);
    draw.to_frame(_app, &frame).unwrap();
    frame
}
