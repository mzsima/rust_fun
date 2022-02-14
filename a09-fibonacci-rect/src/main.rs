use nannou::prelude::*;
use rand::Rng;

struct Model {
    draw: Draw,
    fibonacci: Vec<i32>,
    ratio: f32,
    width: f32,
    threshould: f32
}

fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::loop_once())
        .update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let draw = Draw::new();
    Model { 
        draw,
        fibonacci: vec![1,1,2,3,5,8,13,21,34,55,89,144],
        ratio: 500.0 / 144.0,
        width: 500.0,
        threshould: 40.0
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

fn randColor() -> nannou::color::Hsl {
    let mut rng = rand::thread_rng();
    let seed = rng.gen_range(0..80);
    hsl((seed as f32)/100.0, 0.8, 0.8)
}


fn spiral(model: &Model) {
    let mut x = 0.0;
    let mut y = 0.0;
    let fib = &model.fibonacci;
    let dir = vec![-1.0, 1.0, 1.0, -1.0];

    for i in 0..fib.len()-1 {
        let wd = (fib[i] as f32) * model.ratio;
        let xy = pt2(x * model.ratio, y * model.ratio) + vec2(dir[(i+1)%4] * wd * 0.5, dir[i%4] * wd * 0.5);
        model.draw.rect()
            .xy(xy)
            .w_h(dir[(i+1)%4] * wd, dir[i%4] * wd)
            .color(randColor())
            .stroke_weight(2.0)
            .stroke(BLACK);
        if i % 2 == 1 {
            x += dir[i%4] * ((fib[i] + fib[i+1]) as f32);
        } else {
            y += dir[i%4] * ((fib[i] + fib[i+1]) as f32); 
        }
    }
}