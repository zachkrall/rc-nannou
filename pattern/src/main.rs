use nannou::prelude::*;
use nannou::rand;

fn main() {
  nannou::app(model)
    .update(update)
    .run();
}

struct Model {

}

fn model(_app: &App) -> Model {
  _app.new_window()
    .with_dimensions(500 as u32, 500 as u32)
    .view(view)
    .build()
    .unwrap();
  Model {

  }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

// fn draw_line_box(i, j, _app: &App){
//   let x = i as f32;
//   let y = j as f32;

//   _app.draw().line()
//   .color(WHITE)
//   .start(Point2::new(x,y))
//   .end(Point2::new(10.+x, 10.+y));
// }

fn view(_app: &App, _model: &Model, frame: Frame) -> Frame {

  let t = _app.time;
  let w = _app.window_rect();
  let draw = _app.draw();

  let randpoints: Vec<(i32,i32)>;

  for num in 1..10{
    randpoints.push( (rand::random_range::<i32>( -250, 250 ),rand::random_range::<i32>( -250, 250 )) );
  }

  for (i,j) in randpoints{

    dbg!(i);
    // draw_line_box(i,j,_app);
  }

  frame.clear(BLACK);
  draw.to_frame(_app, &frame).unwrap();
  frame
}
