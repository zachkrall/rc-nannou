use nannou::prelude::*;
// use nannou::noise::*;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    _app.new_window()
        .with_dimensions(900 as u32, 900 as u32)
        .view(view)
        .build()
        .unwrap();
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(_app: &App, _model: &Model, frame: Frame) -> Frame {

    let t = _app.time;
    let win = _app.window_rect();
    let draw = _app.draw();

    draw.background().color(BLACK);

    let step  = 10;
    let loopx = (win.w()*0.50) as i64;
    let loopy = (win.h()*0.50) as i64;

    for x in (-loopx-step..loopx+step).step_by(step as usize){
        for y in (-loopy-step..loopy+step).step_by(step as usize){

            draw.ellipse()
                .x_y(x as f32, y as f32)
                .w(5.)
                .h(5.)
                .rgb(
                    0.0,
                    ((x as f32).abs() / (win.w().abs() * 0.5)) - t.sin(),
                    ((y as f32).abs() / (win.h().abs() * 0.5)) - t.cos()
                );

        }
    }

    draw.to_frame(_app, &frame).unwrap();
    frame
}
