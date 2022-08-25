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
  for n in 0..16 {
    draw_rot(&model, n);
  }
  for n in 0..24 {
    draw_rot2(&model, n);
  }
}

fn draw_rot(model: &Model, n: i32) {
  let rot = (PI / 8.0) * n as f32;
  let scale = if n % 2 == 0 { 1. } else { -1. };
  let base = model.draw.rotate(rot).scale_y(scale).xy(pt2(64., 24.));
  base.text("hello\nworld").font_size(28);
}

fn draw_rot2(model: &Model, n: i32) {
  let rot = (PI / 12.0) * n as f32;
  let scale = if n % 2 == 0 { 1. } else { -1. };
  let base = model.draw.rotate(rot).scale_y(scale).xy(pt2(200., 4.));
  base.text("hello\nworld").font_size(28);
}
