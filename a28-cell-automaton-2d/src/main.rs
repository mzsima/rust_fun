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
  let mut v: Vec<Vec<u8>> = vec![vec![0; 250]; 250];

  // init
  for x in 0..250 {
    for y in 0..250 {
      if x == 125 && y == 125 {
        v[x][y] = 1;
      }
    }
  }

  // update
  // tooooo slow!
  for _ in 0..60000 {
    let mut next: Vec<Vec<u8>> = vec![vec![0; 250]; 250];
    for x in 0..250 {
      for y in 0..250 {
        next[x][y] = transition(&v, x, y);
      }
    }
    v = next
  }

  draw(&model, &v);
}

fn transition(v: &Vec<Vec<u8>>, x: usize, y: usize) -> u8 {
  let num = 250;
  let modulo = 6;
  let next = v[(x + num - 1) % num][y]
    + v[x][(y + num - 1) % num]
    + v[x][y]
    + v[(x + 1 + num) % num][y]
    + v[x][(y + 1 + num) % num];
  return next % modulo;
}

fn draw(model: &Model, v: &Vec<Vec<u8>>) {
  for x in 0..250 {
    for y in 0..250 {
      let j = v[x][y] as usize;
      let color = [
        SNOW,
        LIGHTBLUE,
        BLUEVIOLET,
        STEELBLUE,
        DARKSLATEBLUE,
        DODGERBLUE,
      ][j];
      model
        .draw
        .text("■")
        .font_size(6)
        .x_y((x as f32) * 4. - 500., (y as f32) * 4. - 500.)
        .color(color);
    }
  }
}
