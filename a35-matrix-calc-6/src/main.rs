use nannou::prelude::*;
use rand::Rng;

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
  let mtx_b = create_random_matrix(8);
  let mtx_a = create_matrix_a(90, 8);
  let mtx_c = tr_mtx(&mtx_a);
  let res = mult_mtx(&mult_mtx(&mtx_a, &tr_mtx(&mtx_b)), &mtx_c);

  draw_matrix(&model, &mtx_b, 0., 0., BLACK, WHITE);
  draw_matrix(&model, &mtx_a, 0., 32., BLACK, WHITE);
  draw_matrix(&model, &mtx_c, 32., 0., BLACK, WHITE);
  draw_matrix(&model, &res, 32., 32., SKYBLUE, SLATEBLUE);
}

fn create_random_matrix(size: i32) -> Mat2d {
  let mut rng = rand::thread_rng();
  let mut res: Mat2d = Vec::new();
  for _i in 0..size {
    let mut nrow = Vec::new();
    for _j in 0..size {
      nrow.push(rng.gen_range(0..2) as i32)
    }
    res.push(nrow)
  }
  res
}

fn create_matrix_a(row: i32, col: i32) -> Mat2d {
  let mut res: Mat2d = Vec::new();
  for i in 0..row {
    let mut nrow = vec![0; col as usize];
    if (i / col) % 2 == 0 {
      nrow[(i % col) as usize] = 1;
    } else {
      nrow[(col - (i % col) - 1) as usize] = 1;
    }
    res.push(nrow);
  }
  res
}

fn draw_matrix(model: &Model, mtx: &Mat2d, x: f32, y: f32, c1: Srgb<u8>, c0: Srgb<u8>) {
  for i in 0..mtx.len() {
    for j in 0..mtx[0].len() {
      let color = if mtx[i][j] == 1 { c1 } else { c0 };
      model
        .draw
        .rect()
        .xy(pt2(x + 4. * j as f32, y + 4. * i as f32))
        .w_h(4., 4.)
        .color(color)
        .stroke_weight(1.0)
        .stroke(GRAY);
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
