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
    let p1 = pt2(20. * (i as f32), 0.);
    let p2 = pt2(0., 20. * (10. - (i as f32)));
    let p3 = pt2(20. * (10. - (i as f32)), 200.);
    draw_line(&model, p1, p2);
    draw_line(&model, p2, p3);
  }
  model
    .draw
    .line()
    .start(pt2(0., 0.))
    .end(pt2(200., 0.))
    .weight(2.0)
    .color(STEELBLUE);
}

fn draw_line(model: &Model, p1: Point2, p2: Point2) {
  model
    .draw
    .line()
    .start(p1)
    .end(p2)
    .weight(2.0)
    .color(STEELBLUE);
}
