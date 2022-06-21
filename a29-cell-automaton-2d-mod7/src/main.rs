use nannou::glam::*;
use nannou::prelude::*;
use rayon::prelude::*;

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
  let mut v: Vec<Vec<u8>> = vec![vec![0; 100]; 100];

  // init
  for x in 0..100 {
    for y in 0..100 {
      if x == 50 && y == 50 {
        v[x][y] = 1;
      }
    }
  }

  // update
  // tooooo slow!
  for _ in 0..40_000 {
    let mut next: Vec<Vec<u8>> = vec![(0..100).collect::<Vec<u8>>(); 100];
    for x in 0..100 {
      next[x]
        .par_iter_mut()
        .for_each(|y| *y = transition(&v, x, *y as usize));
    }
    v = next
  }

  draw(&model, &v);
}

fn transition(v: &Vec<Vec<u8>>, x: usize, y: usize) -> u8 {
  let num = 100;
  let modulo = 7;
  let next = v[(x + num - 1) % num][y]
    + v[x][(y + num - 1) % num]
    + v[x][y]
    + v[(x + 1 + num) % num][y]
    + v[x][(y + 1 + num) % num];
  return next % modulo;
}

fn draw(model: &Model, v: &Vec<Vec<u8>>) {
  for x in 0..100 {
    for y in 0..100 {
      let j = v[x][y] as usize;
      let color = [
        SNOW, LIGHTBLUE, OLIVE, BLUEVIOLET, STEELBLUE, ORANGE, DODGERBLUE,
      ][j];
      model
        .draw
        .text("■")
        .font_size(6)
        .x_y((x as f32) * 4. - 200., (y as f32) * 4. - 200.)
        .color(color);
    }
  }
}
