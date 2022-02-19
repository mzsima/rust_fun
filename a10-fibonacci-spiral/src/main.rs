use nannou::lyon;
use nannou::prelude::*;
use rand::Rng;

struct Model {
    draw: Draw,
    fibonacci: Vec<i32>,
    ratio: f32
}

fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::loop_once())
        .update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let draw = Draw::new();
    draw.background().color(SPRINGGREEN);
    Model { 
        draw,
        fibonacci: vec![1,1,2,3,5,8,13,21,34,55,89,144,233],
        ratio: 500.0 / 144.0
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame) {
    setup(&_model);
    _model.draw.to_frame(app, &frame).unwrap();
}

fn setup(model: &Model) {
    spiral(model);
}

fn arc(model: &Model, x: f32, y: f32, x0: f32, y0: f32, r0: f32, angle: f32) {
    use nannou::geom::path::Builder;
    use lyon::math::*;

    let mut builder = Builder::new().with_svg();
    builder.move_to(point(x0, y0));
    builder.arc(
        point(x, y),
        vector(r0, r0),
        -Angle::radians(angle),  // sweep angle
        Angle::radians(0.)  // x-rotation
    );
    let arc_path = builder.build();

    // draw arc
    model.draw.path()
        .stroke()
        .rgba(1.0, 0.5, 0.69, 1.0)
        .weight(8.0)
        .caps_round()
        .events(arc_path.iter());
}


fn spiral(model: &Model) {
    let mut x = 0.0;
    let mut y = 1.0;
    let fib = &model.fibonacci;
    let dir = vec![1.0, -1.0, -1.0, 1.0];


    for i in 0..fib.len()-1 {
        let wd = (fib[i] as f32) * model.ratio;
        let centerx = x * model.ratio + dir[i%4] as f32 * wd;
        let centery = y * model.ratio + dir[(i+1)%4] as f32 * wd;
        let startx = x * model.ratio - ((i%4) as f32 * PI/2.0).sin() * wd;
        let starty = y * model.ratio - ((i%4) as f32 * PI/2.0).cos() * wd;
        arc(&model,
            centerx, centery, 
            startx, starty, 
            wd, 
            PI / 2.);
        println!(" --- x: {}, y: {}, xc: {},  yc: {}, xs: {}, ys: {}", x, y, centerx, centery, startx, starty);
        if i % 2 == 0 {
            x += dir[i%4] * ((fib[i] + fib[i+1]) as f32);
        } else {
            y += dir[i%4] * ((fib[i] + fib[i+1]) as f32); 
        }
    }
}