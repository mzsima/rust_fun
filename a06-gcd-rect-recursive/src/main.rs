use nannou::prelude::*;
use rand::Rng;


static NUM_A: f32 = 14.0;
static NUM_B: f32 = 9.0;
static RATIO: f32 = NUM_B / NUM_A;
static WIDTH: f32 = 500.0;

fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::loop_once())
        // .loop_mode(LoopMode::rate_fps(1.0))
        .update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().event(event).view(view).build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn event(_app: &App, _model: &mut Model, event: WindowEvent) {
    match event {
        // Touch events
        TouchPressure(_touch) => { println!("{:?}", event); }
        _ => {}
    }
}

fn setup(draw: &Draw) {
    let mut x_pos = 0.0;
    let mut y_pos = 0.0;
    let mut itr = 0;
    let mut wd = WIDTH * RATIO;

    while wd > 0.1 {
        itr += 1;
        if itr % 2 == 1 {
            while x_pos + wd < WIDTH + 0.1  {
                div_square(draw, x_pos, y_pos, wd);
                x_pos += wd;
            }
            wd = WIDTH - x_pos;
        } else {
            while y_pos + wd < WIDTH * RATIO + 0.1 {
                div_square(draw, x_pos, y_pos, wd);
                y_pos += wd;
            }
            wd = WIDTH * RATIO - y_pos;
        }
    }
}

fn div_square(draw: &Draw, mut x_pos: f32, mut y_pos: f32, mut wd: f32) {
    let mut itr = 0;
    let x_end = wd + x_pos;
    let y_end = wd + y_pos;
    let mut rng = rand::thread_rng();

    while wd > 0.1 {
        itr += 1;
        if itr % 2 == 1 {
            while x_pos + wd * RATIO < x_end + 0.1  {
                draw.rect().x_y(x_pos + wd * RATIO * 0.5 , y_pos + wd * 0.5).w_h(wd * RATIO, wd)
                    .color(hsl(rng.gen_range(0.0..1.0), 0.5, 0.5))
                    .stroke_weight(1.0)
                    .stroke(WHITE);
                x_pos += wd * RATIO
            }
            wd = x_end - x_pos;
        } else {
            while y_pos + wd / RATIO < y_end + 0.1 {
                draw.rect().x_y(x_pos + wd * 0.5, y_pos + wd / RATIO * 0.5).w_h(wd, wd / RATIO)
                    .color(hsl(rng.gen_range(0.0..1.0), 0.6, 0.6))
                    .stroke_weight(1.0)
                    .stroke(WHITE);
                y_pos += wd / RATIO;
            }
            wd = y_end - y_pos;
        }
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    setup(&draw);
    draw.to_frame(app, &frame).unwrap();
}