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
    draw.background().color(SNOW);
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
    spiral_archimedes(model);
    spiral_fermat(model);
    spiral_log(model);
}

fn spiral_archimedes(model: &Model) {
    let mut t = 0.0;
    let dt = 2. * PI * 0.01;
    let rad  = |t: f32| -> f32 {1.5 * t};
    for _ in 0..1000 {
        let center = pt2(-240., 0.);
        model.draw.line()
            .start(pt2(rad(t) * t.cos() + center.x, rad(t) * t.sin() + center.y))
            .end(pt2(rad(t + dt) * (t + dt).cos() + center.x, rad(t + dt) * (t + dt).sin() + center.y))
            .weight(2.0)
            .color(STEELBLUE);
        t += dt;
    }
}

fn spiral_fermat(model: &Model) {
    let mut t = 0.0;
    let dt = 2. * PI * 0.01;
    let rad  = |t: f32| -> f32 {12. * t.sqrt()};
    for _ in 0..1000 {
        let center = pt2(0., 0.);
        model.draw.line()
            .start(pt2(rad(t) * t.cos() + center.x, rad(t) * t.sin() + center.y))
            .end(pt2(rad(t + dt) * (t + dt).cos() + center.x, rad(t + dt) * (t + dt).sin() + center.y))
            .weight(2.0)
            .color(STEELBLUE);
        t += dt;
    }
}

fn spiral_log(model: &Model) {
    let mut t = 0.0;
    let dt = 2. * PI * 0.01;
    let rad  = |t: f32| -> f32 {0.3 * (1.1).powf(t)};
    for _ in 0..1000 {
        let center = pt2(240., 0.);
        model.draw.line()
            .start(pt2(rad(t) * t.cos() + center.x, rad(t) * t.sin() + center.y))
            .end(pt2(rad(t + dt) * (t + dt).cos() + center.x, rad(t + dt) * (t + dt).sin() + center.y))
            .weight(2.0)
            .color(STEELBLUE);
        t += dt;
    }
}