use app::model::Model;
use nannou::{prelude::{Rgb, Update}, winit::event::WindowEvent, App, Frame};

mod app;
mod utils;

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

fn model(app: &App) -> Model {
    let id = app.new_window()
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(id).unwrap();
    Model::new(window)
}

fn raw_window_event(_: &App, model: &mut Model, event: &WindowEvent) {
    model.handle_raw_event(event);
}

fn update(_: &App, model: &mut Model, update: Update) {
    model.update(update.since_start, update.since_last);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(Rgb::new(25u8, 25, 25));
    model.view(app, &draw, frame);
}
