use nannou::glam::*;
use nannou::prelude::*;
use rand::Rng;
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
  let mut v: VecDeque<u64> = VecDeque::new();
  v.push_back(1);

  for i in 0..64 {
    draw(&model, &v, i as f32 * 8.);
    v = update_number(&v);
  }
}

fn transition(a: u64, b: u64, c: u64) -> u64 {
  let mut rng = rand::thread_rng();
  let n = rng.gen_range(0..1000);
  let d;
  if n < 997 {
    d = a + b + c;
  } else {
    d = b + c
  }
  d % 3
}

fn update_number(v: &VecDeque<u64>) -> VecDeque<u64> {
  let mut curr = v.clone();
  let mut next = VecDeque::new();

  curr.push_front(0);
  curr.push_front(0);
  curr.push_back(0);
  curr.push_back(0);

  for i in 1..curr.len() - 1 {
    next.push_back(transition(curr[i - 1], curr[i], curr[i + 1]))
  }
  next
}

fn draw(model: &Model, v: &VecDeque<u64>, y: f32) {
  let dx = 6.;
  let mut x = v.len() as f32 * dx * -0.5;
  for i in 0..v.len() {
    let j = v[i] as usize;
    let color = [SNOW, VIOLET, STEELBLUE][j];
    model
      .draw
      .text(&j.to_string())
      .font_size(8)
      .x_y(x, 200. - y)
      .color(color);
    x += dx;
  }
}
