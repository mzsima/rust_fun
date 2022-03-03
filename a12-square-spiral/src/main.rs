use nannou::prelude::*;
use ndarray::{arr1, Array1};

struct Model {
    draw: Draw,
    length: f32,
}

fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::loop_once())
        .update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let draw = Draw::new();
    draw.background().color(SNOW);
    Model { 
        draw,
        length: 200.
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame) {
    setup(&_model);
    _model.draw.to_frame(app, &frame).unwrap();
}

fn setup(model: &Model) {
    draw_spiral_square(model, 0.1);
}

fn draw_spiral_square(model: &Model, gap: f32) {
    let mut vectors = [
        arr1(&[0., 0.]), 
        arr1(&[model.length, 0.]), 
        arr1(&[model.length, model.length]),
        arr1(&[0., model.length])
        ];

    let mut next = [
        arr1(&[0., 0.]), 
        arr1(&[model.length, 0.]), 
        arr1(&[model.length, model.length]),
        arr1(&[0., model.length])
        ];

    for _ in 0..100 {
        for i in 0..4 {
            model.draw.line()
            .start(pt2(vectors[i][0], vectors[i][1]))
            .end(pt2(vectors[(i+1) % 4][0], vectors[(i+1) % 4][1]))
            .weight(1.0)
            .color(STEELBLUE);
        }
        for i in 0..4 {
            let mut dir = &vectors[(i+1) % 4] - &vectors[i];
            dir = dir * gap;
            let n = &vectors[i] + dir;
            next[i] = arr1(&[n[0], n[1]]);
        }
        for i in 0..4 {
            vectors[i] = arr1(&[next[i][0], next[i][1]]);
        }
    }
}
