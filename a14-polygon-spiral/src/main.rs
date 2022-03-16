use nannou::prelude::*;
use nannou::glam::*;

struct Model {
    draw: Draw,
    gon: usize,
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
        gon: 8,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame) {
    setup(&_model);
    _model.draw.to_frame(app, &frame).unwrap();
}

fn setup(model: &Model) {
    let mut vectors = create_polygon_vector(&model);
    for _ in 0..20 {
        draw_polygon(&model, &vectors);
        vectors = getNextVector(&model, vectors);
    }
}

fn create_polygon_vector(model: &Model) -> Vec<Vec2> {
    let r = 100.0;
    let mut _vectors = vec![];
    for i in 0..model.gon {
        let angle = (i as f32) * (2. * PI / (model.gon as f32));
        let x = r * angle.sin();
        let y = r * angle.cos();
        _vectors.push(Vec2::new(x, y))
    }
    return _vectors;
}

fn draw_polygon(model: &Model, vectors: &Vec<Vec2>) {
    for i in 0..model.gon {
        let v = vectors[i];
        let v2 = vectors[(i+1) % model.gon];
        model.draw
            .line()
            .start(pt2(v.x, v.y))
            .end(pt2(v2.x, v2.y))
            .weight(4.0)
            .color(STEELBLUE);
    }
}

fn getNextVector(model: &Model, vectors: Vec<Vec2>) -> Vec<Vec2> {
    let r = 100.0;
    let mut _vectors = vec![];
    for i in 0..model.gon {
        let v = vectors[i];
        let v2 = vectors[(i+1) % model.gon];
        let dir = v2 - v;
        _vectors.push(v + dir * 0.3);
    }
    return _vectors;
}