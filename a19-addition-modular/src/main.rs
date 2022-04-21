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
    draw.background().color(BLACK);
    Model { draw }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    setup(&_model);
    _model.draw.to_frame(app, &frame).unwrap();
}

fn setup(model: &Model) {
    draw_dot(&model, 13);
}

fn draw_dot(model: &Model, modulo: u32) {
    for i in 0..modulo {
        for j in 0..modulo {
            let v = ((i + j) % modulo) as f32 * 1.0;
            let (x, y) = (i as f32 * 10.0, j as f32 * 10.0);
            model.draw.ellipse().w_h(v, v).x_y(x, y).color(WHITE);
        }
    }
}
