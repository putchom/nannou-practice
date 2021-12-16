use nannou::prelude::*;
use nannou_osc as osc;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    texture: wgpu::Texture,
    sender: osc::Sender<osc::Connected>,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();

    let assets = app.assets_path().unwrap();
    let img_path = assets.join("images").join("dvd.png");
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();

    let port = 1234;
    let target_addr = format!("{}:{}", "127.0.0.1", port);
    let sender = osc::sender()
        .expect("Could not bind to default socket")
        .connect(target_addr)
        .expect("Could not connect to socket at address");

    Model { _window, texture, sender }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    // Use app time to progress through a sine wave
    let sine = app.time.sin();
    let slowersine = (app.time / 2.0).sin();

    // Get boundary of the window (to constrain the movements of our circle)
    let boundary = app.window_rect();

    // Map the sine wave functions to ranges between the boundaries of the window
    let x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
    let y = map_range(slowersine, -1.0, 1.0, boundary.bottom(), boundary.top());

    // Send x-y coordinates as OSC
    let osc_addr = "/circle/position".to_string();
    let args = vec![osc::Type::Float(x), osc::Type::Float(y)];
    let packet = (osc_addr, args);

    model.sender.send(packet).ok();

    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to black.
    draw.background().color(BLACK);

    // Draw a texture at the x/y coordinates 0.0, 0.0
    draw.texture(&model.texture).x_y(x, y);

    draw.to_frame(app, &frame).unwrap();
}
