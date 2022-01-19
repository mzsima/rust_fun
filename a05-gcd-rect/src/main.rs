use nannou::prelude::*;
use rand::Rng;

fn main() {
    let n = gcd(64, 12);
    println!("{}", n);
    nannou::app(model)
        .loop_mode(LoopMode::loop_once())
        .update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn gcd(m: i64, n: i64) -> i64 {
    match m {
        0 => return n,
        _ => return gcd(n % m, m)
    }
}

fn display(draw: &Draw) {
    let scaler = 20.0;
    let num_a = 10.0;
    let num_b = 6.0;
    let mut x_pos = 0.0;
    let mut y_pos = 0.0;
    let mut itr = 0;
    let mut wd = num_b * scaler;
    let mut rng = rand::thread_rng();

    while wd > 0.0 {
        itr += 1;
        if itr % 2 == 1 {
            while x_pos + wd <= num_a * scaler  {
                draw.rect().x_y(x_pos + wd * 0.5 , y_pos + wd * 0.5).w_h(wd, wd)
                    .color(hsl(rng.gen_range(0.0..1.0), 0.5, 0.5))
                    .stroke_weight(1.0)
                    .stroke(WHITE);
                x_pos += wd 
            }
            wd = num_a * scaler - x_pos;
        } else {
            while y_pos + wd <= num_b * scaler {
                draw.rect().x_y(x_pos + wd * 0.5, y_pos + wd * 0.5).w_h(wd, wd)
                    .color(hsl(rng.gen_range(0.0..1.0), 0.6, 0.6))
                    .stroke_weight(1.0)
                    .stroke(WHITE);
                y_pos += wd;
            }
            wd = num_b * scaler - y_pos;
        }
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    display(&draw);
    draw.to_frame(app, &frame).unwrap();
}