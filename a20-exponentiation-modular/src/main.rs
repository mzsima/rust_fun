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
  draw_dot(&model, 23);
}

fn modpow(mut base: u32, mut exp: u32, modulo: u32) -> u32 {
  let mut res = 1;
  while exp > 0 {
    if exp & 1 == 1 {
      res = (res * base) % modulo;
    }
    base = (base * base) % modulo;
    exp >>= 1;
  }
  return res;
}

fn draw_dot(model: &Model, modulo: u32) {
  for i in 1..modulo {
    for j in 1..modulo {
      let v = modpow(j, i, modulo) as f32;
      let offset = vec2((modulo as f32) * 12.0, (modulo as f32) * 12.0);
      let (x, y) = (
        i as f32 * 24.0 - offset.x,
        (modulo - j) as f32 * 24.0 - offset.y,
      );

      model
        .draw
        // .text(&v.to_string())
        .ellipse()
        .w_h(v, v)
        .x_y(x, y)
        .color(WHITE);
    }
  }
}
