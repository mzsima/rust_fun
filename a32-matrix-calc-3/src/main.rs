use nannou::prelude::*;

struct Model {
  draw: Draw,
}

type Mat2d = Vec<Vec<i32>>;

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
  let mtx_a: Mat2d = vec![vec![0, 0, 1], vec![0, 1, 0], vec![1, 0, 0]];
  let mtx_b: Mat2d = vec![vec![1, 1, 0], vec![0, 1, 0], vec![1, 1, 1]];
  let mtx_c: Mat2d = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
  let res = mult_mtx(&mult_mtx(&mtx_a, &tr_mtx(&mtx_b)), &mtx_c);
  println!("{:?}", res);

  draw_matrix(&model, &mtx_a, 0., 0., GRAY);
  draw_matrix(&model, &mtx_b, 0., 62., GRAY);
  draw_matrix(&model, &mtx_c, 62., 62., GRAY);
  draw_matrix(&model, &res, 62., 0., ORANGE);
}

fn draw_matrix(model: &Model, mtx: &Mat2d, x: f32, y: f32, c: Srgb<u8>) {
  for i in 0..mtx.len() {
    for j in 0..mtx[0].len() {
      let color = if mtx[i][j] == 1 { BLACK } else { WHITE };
      model
        .draw
        .rect()
        .xy(pt2(x + 20. * j as f32, y + 20. * i as f32))
        .w_h(20., 20.)
        .color(color)
        .stroke_weight(4.0)
        .stroke(c);
    }
  }
}

fn mult_mtx(mtx1: &Mat2d, mtx2: &Mat2d) -> Mat2d {
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

fn tr_mtx(mtx: &Mat2d) -> Mat2d {
  let mut new_mtx = vec![vec![0; mtx.len()]; mtx[0].len()];
  for i in 0..mtx.len() {
    for j in 0..mtx[0].len() {
      new_mtx[j][i] = mtx[i][j];
    }
  }
  new_mtx
}
