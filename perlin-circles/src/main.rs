use nannou::prelude::*;
use nannou::noise::*;

use palette::Rgb;
use palette::pixel::Srgb;
use palette::named;

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

struct Model {
    background: Rgb,
    t: f64
}

fn model(_app: &App) -> Model {

    let width  = 900 as u32;
    let height = 900 as u32;

    _app.new_window()
        .with_dimensions(width, height)
        .key_released(key_released)
        .build()
        .unwrap();

    // let background = Rgb::new(0.,0.,0.);
    let background = Srgb::from_pixel(&named::BLACK).into();
    let t = 0.0;

    Model {
        background,
        t
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn key_released(_app: &App, _model: &mut Model, _key: Key) {

    let draw = _app.draw();

    // println!("key release: {:?}", _key);
    if _key == Key::Up{
        // _model.background = Rgb::new(1.,0.,0.);
        // _model.background = Srgb::from_pixel(&named::RED).into();
        _model.t = _model.t + 0.1;
    }
    if _key == Key::Down{
        _model.t = _model.t - 0.1;
    }

    println!("t equals: {}", _model.t);

}

fn view(_app: &App, _model: &Model, frame: Frame) -> Frame {

    // let t    = _app.time;
    let t = _app.time;
    // let t = _model.t;
    let win  = _app.window_rect();
    let draw = _app.draw();

    draw.background().color(_model.background);

    let circle_width = 10.0;

    for x in (0..(win.w() as i64)+1).step_by( (circle_width * 2.5) as usize){
        for y in (0..(win.h() as i64)+1).step_by( (circle_width * 2.5) as usize){

            let noise = Perlin::new();
            let size = noise.get([
                t as f64,
                x as f64,
                y as f64
            ]) as f32 ;
            //
            // if x == 0 && y == 0{
            //     println!("size value {}", size);
            // }

            draw.ellipse()
            .x_y( (x as f32) - win.w()*0.5,
                  (y as f32) - win.h()*0.5 )
            .radius( 5. + circle_width * size );
        }
    }

    draw.to_frame(_app, &frame).unwrap();
    frame
}
