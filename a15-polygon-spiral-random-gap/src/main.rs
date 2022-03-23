use nannou::glam::*;
use nannou::prelude::*;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

struct Model {
    draw: Draw,
    gon: usize,
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
    Model { draw, gon: 6 }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    setup(&_model);
    _model.draw.to_frame(app, &frame).unwrap();
}

fn setup(model: &Model) {
    let mut vectors = create_polygon_vector(&model);
    for _ in 0..200 {
        draw_polygon(&model, &vectors);
        vectors = get_next_vector(&model, vectors);
    }
}

fn create_polygon_vector(model: &Model) -> Vec<Vec2> {
    let r = 200.0;
    let mut _vectors = vec![];
    for i in 0..model.gon {
        let angle = (i as f32) * (2. * PI / (model.gon as f32));
        let (x, y) = (r * angle.sin(), r * angle.cos());
        _vectors.push(Vec2::new(x, y))
    }
    return _vectors;
}

fn draw_polygon(model: &Model, vectors: &Vec<Vec2>) {
    for i in 0..model.gon {
        let v = vectors[i];
        let v2 = vectors[(i + 1) % model.gon];
        model
            .draw
            .line()
            .start(pt2(v.x, v.y))
            .end(pt2(v2.x, v2.y))
            .weight(1.0)
            .color(SILVER);
    }
}

fn get_next_vector(model: &Model, vectors: Vec<Vec2>) -> Vec<Vec2> {
    let mut _vectors = vec![];

    let weights = [2, 1, 1, 1, 1];
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();

    for i in 0..model.gon {
        let gap = dist.sample(&mut rng) as f32 * 0.05; //rng.gen_range(0.01..0.07);
        let v = vectors[i];
        let v2 = vectors[(i + 1) % model.gon];
        let dir = v2 - v;
        _vectors.push(v + dir * gap);
    }
    return _vectors;
}
