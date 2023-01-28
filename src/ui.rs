use gtk::gdk::Display;
use gtk::{prelude::*, CssProvider, StyleContext};
use gtk::{Application, ApplicationWindow};

mod config_ui;
mod library_view;
mod chapter_view;

const APP_ID: &str = "uglyluigi.Mango";
const DEFAULT_WIDTH: i32 = 1000;
const DEFAULT_HEIGHT: i32 = 500;

pub fn show() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    app.connect_shutdown(quit);
    app.run();
}

fn quit(_app: &Application) {}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Mango")
        .default_width(DEFAULT_WIDTH)
        .default_height(DEFAULT_HEIGHT)
        .child(&library_view::get_library_view())
        .build();

    window.present();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("styles/root.css"));

    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
