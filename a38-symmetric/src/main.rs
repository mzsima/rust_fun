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
  draw.background().color(WHITE);
  Model { draw }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
  setup(&_model);
  _model.draw.to_frame(app, &frame).unwrap();
}

fn setup(model: &Model) {
  for n in 0..6 {
    draw_rot(&model, (PI / 3.0) * n as f32);
  }
}

fn draw_rot(model: &Model, rot: f32) {
  let points_white = (0..=315).step_by(45).map(|i| {
    let radian = deg_to_rad(i as f32);
    let x = radian.sin() * 20.;
    let y = radian.cos() * 20.;
    (pt2(x, y), WHITE)
  });
  let points_blue = (0..=315).step_by(45).map(|i| {
    let radian = deg_to_rad(i as f32);
    let x = radian.sin() * 20.;
    let y = radian.cos() * 20.;
    (pt2(x, y), STEELBLUE)
  });

  let base = model.draw.rotate(rot).xy(pt2(24., 24.));
  base.polyline().weight(12.).points_colored(points_white);
  base.polyline().weight(8.).points_colored(points_blue);
}
