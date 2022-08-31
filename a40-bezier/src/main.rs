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
  for i in 0..10 {
    draw_line(&model, i as f32, 10. - i as f32);
  }
  model
    .draw
    .line()
    .start(pt2(0., 0.))
    .end(pt2(200., 0.))
    .weight(2.0)
    .color(STEELBLUE);
}

fn draw_line(model: &Model, x: f32, y: f32) {
  model
    .draw
    .line()
    .start(pt2(20. * x, 0.))
    .end(pt2(0., 20. * y))
    .weight(2.0)
    .color(STEELBLUE);
}
