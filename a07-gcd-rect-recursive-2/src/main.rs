use nannou::prelude::*;
use rand::Rng;


static NUM_A: f32 = 7.0;
static mut RATIO: f32 = 8.0 / NUM_A;
static WIDTH: f32 = 500.0;
static mut THRESHOULD: f32 = 20.0;

fn main() {
    nannou::app(model)
        // .loop_mode(LoopMode::loop_once())
        .loop_mode(LoopMode::Wait)
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
        TouchPressure(_touch) => { 
            let mut rng = rand::thread_rng();
            unsafe { 
                RATIO = rng.gen_range(8.0..30.0) / NUM_A;
                THRESHOULD = rng.gen_range(10.0..100.0);
            }
        }
        _ => {}
    }
}

fn setup(draw: &Draw) {
    let wd = WIDTH;
    div_square(&draw, -wd * 0.5, -wd * 0.5, wd);
}

fn div_square(draw: &Draw, mut x_pos: f32, mut y_pos: f32, mut wd: f32) {
    let mut itr = 0;
    let x_end = wd + x_pos;
    let y_end = wd + y_pos;
    let mut rng = rand::thread_rng();

    draw.rect().x_y(x_pos + wd * 0.5, y_pos + wd * 0.5).w_h(wd, wd)
        .color(hsl(rng.gen_range(0.0..1.0), 0.6, 0.6))
        .stroke_weight(1.0)
        .stroke(WHITE);
    while unsafe { wd > THRESHOULD } {
        itr += 1;
        if itr % 2 == 1 {
            while x_pos + wd * unsafe { RATIO } < x_end + 0.1  {
                div_rect(&draw, x_pos, y_pos, wd * unsafe { RATIO });
                x_pos += wd * unsafe { RATIO }
            }
            wd = x_end - x_pos;
        } else {
            while y_pos + wd / unsafe { RATIO } < y_end + 0.1 {
                div_rect(&draw, x_pos, y_pos, wd);
                y_pos += wd / unsafe { RATIO };
            }
            wd = y_end - y_pos;
        }
    }
}

fn div_rect(draw: &Draw, mut x_pos: f32, mut y_pos: f32, mut wd: f32) {
    let mut itr = 0;
    let x_end = wd + x_pos;
    let y_end = wd / unsafe { RATIO }  + y_pos;
    let mut rng = rand::thread_rng();
    
    draw.rect().x_y(x_pos + wd * 0.5, y_pos + wd / unsafe { RATIO } * 0.5).w_h(wd, wd / unsafe { RATIO })
        .color(hsl(rng.gen_range(0.0..1.0), 0.6, 0.6))
        .stroke_weight(1.0)
        .stroke(WHITE);

    while unsafe { wd > THRESHOULD } {
        itr += 1;
        if itr % 2 == 0 {
            while x_pos + wd < x_end + 0.1  {
                div_square(&draw, x_pos, y_pos, wd);
                x_pos += wd;
            }
            wd = x_end - x_pos;
        } else {
            while y_pos + wd < y_end + 0.1 {
                div_square(&draw, x_pos, y_pos, wd);
                y_pos += wd;
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