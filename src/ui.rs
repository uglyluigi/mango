use std::path::Path;

use gtk::builders::{GridBuilder, ImageBuilder, PictureBuilder};
use gtk::ffi::{GtkImage, GtkPicture};
use gtk::gdk::Texture;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::{prelude::GLAreaExt, Application, ApplicationWindow};
use gtk::{prelude::*, Image, Label};

use crate::categorizer_service::library::Library;

const APP_ID: &str = "uglyluigi.Mango";
const DEFAULT_WIDTH: i32 = 1000;
const DEFAULT_HEIGHT: i32 = 500;

pub fn show() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    app.run();
}

const COVER_PATH: &'static str = "./MangaLibrary/Mieruko-Chan/cover.jpg";

fn build_ui(app: &Application) {
    let label: Label = Label::builder().label("Mango v0.0.1").build();

    let img = ImageBuilder::new()
        .file("cover.jpg")
        .width_request(100)
        .height_request(300)
        .build();
    let library_grid = GridBuilder::new().build();
    library_grid.attach(&img, 0, 0, 1, 1);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Mango")
        .default_width(DEFAULT_WIDTH)
        .default_height(DEFAULT_HEIGHT)
        .child(&label)
        .child(&library_grid)
        .build();

    window.present();
}

pub fn make_covers(library: &Library) -> Vec<Image> {
    let mut ret = Vec::new();

    for series in library.series() {
        let first_cover = &series.covers()[0];
        ret.push(
            ImageBuilder::new()
                .file(first_cover.path.to_str().unwrap())
                .build(),
        );
    }

    ret
}
