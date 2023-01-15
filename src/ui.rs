use gtk::{prelude::*, Label};
use gtk::{Application, ApplicationWindow, prelude::GLAreaExt};

const APP_ID: &str = "uglyluigi.Mango";
const DEFAULT_WIDTH: i32 = 1000;
const DEFAULT_HEIGHT: i32 = 500;

pub fn show() {
    let app = Application::builder()
    .application_id(APP_ID)
    .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let label: Label = Label::builder()
        .label("Mango v0.0.1")
        .margin_top(12)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Mango")
        .default_width(DEFAULT_WIDTH)
        .default_height(DEFAULT_HEIGHT)
        .child(&label)
        .build();

    window.present();
}
