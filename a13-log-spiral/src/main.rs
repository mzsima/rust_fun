use nannou::prelude::*;
use ndarray::{arr1};

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
    draw_spiral_square(model, 0.3);
    draw_log_spiral_line(model, 0.3)
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
        arr1(&[0., 0.]), 
        arr1(&[0., 0.]), 
        arr1(&[0., 0.]),
        ];

    for _ in 0..100 {
        for i in 0..4 {
            model.draw.line()
            .start(pt2(vectors[i][0], vectors[i][1]))
            .end(pt2(vectors[(i+1) % 4][0], vectors[(i+1) % 4][1]))
            .weight(1.0)
            .color(STEELBLUE);
            let mut dir = &vectors[(i+1) % 4] - &vectors[i];
            dir = dir * gap;
            let n = &vectors[i] + dir;
            next[i] = arr1(&[n[0], n[1]]);
        }
        vectors = next.to_owned();
    }
}

fn draw_log_spiral_line(model: &Model, gap: f32) {
    let step = 2. * PI * 0.001;
    let b = (2. * gap * gap - 2. * gap + 1.).sqrt();
    let c = (gap / (1. - gap)).atan();
    let vo = arr1(&[model.length / 2., model.length / 2.]);
    let mut v = arr1(&[0., 0.]);
    v = v - &vo;

    while ((v[0] * v[0]) + (v[1] * v[1])).sqrt() > 1. {
        let mut next = v.clone();
        (next[0], next[1]) = (
            step.cos() * v[0] - step.sin() * v[1], 
            step.sin() * v[0] + step.cos() * v[1]);
        next = next * (b.pow(step / c));
        model.draw.translate(vec3(vo[0], vo[1], 0.))
            .line()
            .start(pt2(v[0], v[1]))
            .end(pt2(next[0], next[1]))
            .weight(3.0)
            .color(TOMATO);
        v = next;
    }
}
