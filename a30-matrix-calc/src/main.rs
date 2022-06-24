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
  let mtx_a: &[&[i32]] = &[&[2, 1], &[0, 1]];
  let mtx_b: &[&[i32]] = &[&[3], &[1]];
  let res = mult_mtx(&mtx_a, &mtx_b);
  println!("{:?}", res);
}

fn mult_mtx(mtx1: &[&[i32]], mtx2: &[&[i32]]) -> Vec<Vec<i32>> {
  let mut new_mtx = vec![vec![0; mtx2[0].len()]; mtx1.len()];
  for i in 0..mtx1.len() {
    for j in 0..mtx2[0].len() {
      let mut sum = 0;
      for k in 0..mtx2.len() {
        sum += mtx1[i][k] * mtx2[k][j];
      }
      new_mtx[i][j] = sum;
    }
  }
  new_mtx
}
