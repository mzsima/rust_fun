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
    draw.background().color(SKYBLUE);
    Model { draw }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    setup(&_model);
    _model.draw.to_frame(app, &frame).unwrap();
}

fn setup(model: &Model) {
    draw_dot(&model, 0);
}

fn draw_dot(model: &Model, i: u32) {
    if i > 500 {
        return;
    }
    let rot = 17. / 55.;
    let r = 5. * (i as f32).sqrt();
    let angle = (i as f32) * rot * 2. * PI;
    let (x, y) = (r * angle.sin(), r * angle.cos());
    model.draw.ellipse().w_h(4., 4.).x_y(x, y).color(WHITE);
    draw_dot(&model, i + 1);
}
