use nannou::prelude::*;
use nannou::noise::*;

use palette::Rgb;
use palette::pixel::Srgb;
use palette::named;

use rand::Rng;

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

struct Model {
    background: Rgb,
    line: Rgb
}

fn model(_app: &App) -> Model {

    let width  = 900 as u32;
    let height = 900 as u32;

    _app.new_window()
        .with_dimensions(width, height)
        .build()
        .unwrap();

    // let background = Rgb::new(0.,0.,0.);
    let background = Srgb::from_pixel(&named::BLACK).into();
    let line = Rgb::new(1.,1.,0.);

    Model {
        background,
        line
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(_app: &App, _model: &Model, frame: Frame) -> Frame {

    let t = _app.time;
    let win  = _app.window_rect();
    let draw = _app.draw();

    draw.background().color(_model.background);

    let noise = Perlin::new();

    let n_points = 100;
    let vertices = (0..n_points)
        // A sine wave mapped to the range of the window.
        .map(|i| {
            // let amp = rand::thread_rng().gen_range(-1., 1.) as f32;
            let x = map_range(i, 0, n_points - 1, win.left(), win.right());
            let amp = noise.get([
                (t*0.1) as f64,
                x as f64
            ]) as f32 ;
            let y = map_range(amp, -1.0, 1.0, win.bottom() * 0.75, win.top() * 0.75);
            pt2(x, y)
        })
        .enumerate()
        // Colour each vertex uniquely based on its index.
        .map(|(i, p)| {
            let rgba = nannou::color::Rgba::new(0., 0., 1., 1.0);
            geom::vertex::Rgba(p, rgba)
        });

    draw.polyline().vertices(0.8, vertices);

    draw.to_frame(_app, &frame).unwrap();
    frame
}
