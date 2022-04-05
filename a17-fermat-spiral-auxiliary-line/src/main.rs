use nannou::glam::*;
use nannou::prelude::*;

struct Model {
    draw: Draw,
}

fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::loop_once())
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let draw = Draw::new();
    draw.background().color(SNOW);
    Model { draw }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    setup(&_model);
    _model.draw.to_frame(app, &frame).unwrap();
}

fn setup(model: &Model) {
    let n = 12.;
    draw_line(&model, 0, n);
    draw_curve(&model, n);
    draw_dot(&model, 0, n);
}

fn draw_dot(model: &Model, i: u32, n: f32) {
    if i > 50 {
        return;
    }
    let rot = 1. / n;
    let r = 20. * (i as f32).sqrt();
    let angle = (i as f32) * rot * 2. * PI;
    let (x, y) = (r * angle.sin(), r * angle.cos());
    model.draw.ellipse().w_h(8., 8.).x_y(x, y).color(BLACK);
    draw_dot(&model, i + 1, n);
}

fn draw_line(model: &Model, i: u32, n: f32) {
    if i > (n as u32) {
        return;
    }
    let rot = 1. / n;
    let r = 20. * (60.0).sqrt();
    let angle = (i as f32) * rot * 2. * PI;
    let (x, y) = (r * angle.sin(), r * angle.cos());
    model
        .draw
        .line()
        .start(pt2(x, y))
        .end(pt2(-x, -y))
        .weight(2.0)
        .color(SKYBLUE);
    draw_line(&model, i + 1, n)
}

fn draw_curve(model: &Model, n: f32) {
    let step = 2. * PI * 0.01;
    let mut theta = 0.;
    let rot = 1. / n;
    let (mut x0, mut y0) = (0., 0.);
    for _ in 0..420 {
        let r = 20. * (theta / (2. * PI * rot)).sqrt();
        let (x, y) = (r * theta.sin(), r * theta.cos());
        model
            .draw
            .line()
            .start(pt2(x0, y0))
            .end(pt2(x, y))
            .weight(2.0)
            .color(HOTPINK);
        (x0, y0) = (x, y);
        theta += step;
    }
}
