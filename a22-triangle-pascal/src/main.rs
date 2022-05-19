use nannou::glam::*;
use nannou::prelude::*;
use std::collections::VecDeque;

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
  let mut v: VecDeque<u32> = VecDeque::new();
  v.push_back(1);

  for i in 0..32 {
    draw(&model, &v, i as f32 * 8.);
    v = update_number(&v);
  }
}

fn update_number(v: &VecDeque<u32>) -> VecDeque<u32> {
  let mut curr = v.clone();
  let mut next = VecDeque::new();

  curr.push_back(0);
  curr.push_front(0);

  for i in 0..curr.len() - 1 {
    next.push_back(curr[i] + curr[i + 1])
  }
  next
}

fn draw(model: &Model, v: &VecDeque<u32>, y: f32) {
  let dx = 8.;
  let mut x = v.len() as f32 * dx * -0.5;

  for i in 0..v.len() {
    model
      .draw
      .text(&(v[i] % 2).to_string())
      .font_size(8)
      .x_y(x, 100. - y)
      .color(WHITE);
    x += dx;
  }
}
