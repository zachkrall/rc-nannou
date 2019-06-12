use nannou::prelude::*;
use nannou::noise::*;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {

    let width  = 900 as u32;
    let height = 900 as u32;

    _app.new_window()
        .with_dimensions(width, height)
        .view(view)
        .build()
        .unwrap();
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(_app: &App, _model: &Model, frame: Frame) -> Frame {

    // let t    = _app.time;
    let t = _app.time;
    let win  = _app.window_rect();
    let draw = _app.draw();

    draw.background().color(BLACK);

    let circle_width = 10.0;

    for x in (0..(win.w() as i64)+1).step_by( (circle_width * 2.5) as usize){
        for y in (0..(win.h() as i64)+1).step_by( (circle_width * 2.5) as usize){

            let noise = Perlin::new();
            let size = noise.get([
                x as f64,
                y as f64,
                t as f64
            ]) as f32;
            //
            // if x == 0 && y == 0{
            //     println!("size value {}", size);
            // }

            draw.ellipse()
            .x_y( (x as f32) - win.w()*0.5,
                  (y as f32) - win.h()*0.5 )
            .radius( circle_width * size );
        }
    }

    draw.to_frame(_app, &frame).unwrap();
    frame
}
