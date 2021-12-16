use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    texture: wgpu::Texture,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();

    let assets = app.assets_path().unwrap();
    let img_path = assets.join("images").join("dvd.png");
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();
    Model { _window, texture }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let sine = app.time.sin();
    let slowersine = (app.time / 2.0).sin();
    let boundary = app.window_rect();
    let x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
    let y = map_range(slowersine, -1.0, 1.0, boundary.bottom(), boundary.top());

    draw.background().color(BLACK);
    draw.texture(&model.texture).x_y(x, y);
    draw.to_frame(app, &frame).unwrap();
}
