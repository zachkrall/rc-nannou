use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    _app.new_window()
        .with_dimensions(300 as u32, 300 as u32)
        .view(view)
        .build()
        .unwrap();
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(_app: &App, _model: &Model, frame: Frame) -> Frame {
    let app = _app;
    let draw = app.draw();

    draw.background().color(WHITE);

    let _win = app.window_rect();

    let (bx, by) = (app.mouse.x , app.mouse.y);
    // let (bx, by) = (0.,0.);

    // Black Outer Border
    draw.rect()
        .x_y(bx+0., by+41.)
        .w(180.).h(149.)
        .color(BLACK);

    // White Border
    draw.rect()
        .x_y(bx+0., by+41.)
        .w(151.).h(121.)
        .color(WHITE);

    // Inner Black Screen
    draw.rect()
        .x_y(bx+0., by+41.)
        .w(118.).h(89.)
        .color(BLACK);

    // Monitor stand
    draw.rect()
        .x_y(bx+0., by - 40.5 )
        .w(60.).h(15.)
        .color(BLACK);

    draw.rect()
        .x_y(bx+0., by - 55.5 )
        .w(149.).h(15.)
        .color(BLACK);

    draw.rect()
        .x_y(bx+0., by - 85.5 )
        .w(180.).h(45.)
        .color(BLACK);

    // green code squares
    draw.rect()
        .x_y(bx-(118.*0.5)+7.5, by+53.+7.5)
        .w(15.).h(15.)
        .color(GREEN);

    draw.rect()
        .x_y(bx-22., by+53.+7.5)
        .w(15.).h(15.)
        .color(GREEN);

    draw.rect()
        .x_y(bx+7.5, by+53.+7.5)
        .w(15.).h(15.)
        .color(GREEN);

    draw.rect()
        .x_y(bx-29., by+31.)
        .w(30.).h(15.)
        .color(GREEN);

    draw.rect()
        .x_y(bx+15., by+31.)
        .w(30.).h(15.)
        .color(GREEN);


    // keys
    let kb = [
        (-53.,-89.),
        (-38.,-74.),
        (-23.,-89.),
        (-8.,-74.),
        (7.,-89.),
        (22.,-74.),
        (37.,-89.),
        (52.,-74.)
    ];

    for (x,y) in &kb{

        draw.rect()
            .x_y(bx+x, by+y)
            .w(15.).h(15.)
            .color(WHITE);

    }



    // frame.clear(PURPLE);
    draw.to_frame(app, &frame).unwrap();
    frame
}
