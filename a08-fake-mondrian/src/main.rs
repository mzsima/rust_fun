use nannou::prelude::*;
use rand::Rng;

struct Model {
    draw: Draw,
    ratio: f32,
    width: f32,
    threshould: f32
}

fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::loop_once())
        .update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let draw = Draw::new();
    Model { 
        draw,
        ratio: (((5.0).sqrt() + 1.0) / 2.0),
        width: 500.0,
        threshould: 40.0
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {

}

fn view(app: &App, _model: &Model, frame: Frame) {
    setup(&_model);
    _model.draw.to_frame(app, &frame).unwrap();
}

fn setup(model: &Model) {
    let wd = model.width;
    div_square(model, -0.5 * wd, -0.5 * wd, wd);
}

fn randColor(model: &Model) -> nannou::color::rgb::Srgb {
    let red = rgb(125.0/255.0, 0.0/255.0, 2.0/255.0);
    let blue = rgb(0.0/255.0, 90.0/255.0, 153.0/255.0);
    let yellow = rgb(254.0/255.0, 202.0/255.0, 0.0/255.0);
    let white = rgb(255.0/255.0, 255.0/255.0, 255.0/255.0);
    let mut rng = rand::thread_rng();

    let seed = rng.gen_range(0..80);
    match seed {
        0..=9 => red,
        10..=39 => blue,
        40..=45 => yellow,
        _ => white
    }
}

fn div_square(model: &Model, mut x_pos: f32, mut y_pos: f32, mut wd: f32) {
    let mut itr = 0;
    let x_end = wd + x_pos;
    let y_end = wd + y_pos;
    
    let xy = pt2(x_pos, y_pos) + vec2(wd * 0.5, wd * 0.5);
    
    model.draw.rect()
        .xy(xy)
        .w_h(wd, wd)
        .color(randColor(&model))
        .stroke_weight(2.0)
        .stroke(BLACK);

    while wd > model.threshould {
        itr += 1;
        if itr % 2 == 1 {
            while x_pos + wd * model.ratio < x_end + 0.1 {
                div_rect(&model, x_pos, y_pos, wd * model.ratio);
                x_pos += wd * model.ratio;
            }
            wd = x_end - x_pos;
        } else {
            while y_pos + wd / model.ratio < y_end + 0.1 {
                div_rect(&model, x_pos, y_pos, wd);
                y_pos += wd / model.ratio;
            }
            wd = y_end - y_pos;
        }
    }
}

fn div_rect(model: &Model, mut x_pos: f32, mut y_pos: f32, mut wd: f32) {
    let mut itr = 0;
    let x_end = wd + x_pos;
    let y_end = wd / model.ratio + y_pos;

    let xy = pt2(x_pos, y_pos) + vec2(wd * 0.5, wd / model.ratio * 0.5);
    model.draw.rect()
        .xy(xy)
        .w_h(wd, wd / model.ratio)
        .color(randColor(&model))
        .stroke_weight(2.0)
        .stroke(BLACK);

    while wd > model.threshould {
        itr += 1;
        if itr % 2 == 0 {
            while x_pos + wd < x_end + 0.1  {
                div_square(&model, x_pos, y_pos, wd);
                x_pos += wd;
            }
            wd = x_end - x_pos;
        } else {
            while y_pos + wd < y_end + 0.1 {
                div_square(&model, x_pos, y_pos, wd);
                y_pos += wd;
            }
            wd = y_end - y_pos;
        }
    }
}