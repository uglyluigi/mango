use std::path::Path;

use gtk::builders::{BoxBuilder, GridBuilder, ImageBuilder, PictureBuilder};
use gtk::ffi::{GtkImage, GtkPicture, GtkWidget};
use gtk::gdk::Texture;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::{prelude::GLAreaExt, Application, ApplicationWindow};
use gtk::{prelude::*, EventController, GestureClick, Grid, Image};

use crate::categorizer_service;
use crate::categorizer_service::library::Library;

const APP_ID: &str = "uglyluigi.Mango";
const DEFAULT_WIDTH: i32 = 1000;
const DEFAULT_HEIGHT: i32 = 500;

pub fn show() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    app.connect_shutdown(quit);
    app.run();
}

fn quit(app: &Application) {}

fn build_ui(app: &Application) {
    let library_grid = GridBuilder::new().build();

    if let Ok(library) = categorizer_service::library::deserialize_from_disk() {
        attach_covers(make_covers(&library), &library_grid);
    }

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Mango")
        .default_width(DEFAULT_WIDTH)
        .default_height(DEFAULT_HEIGHT)
        .child(&library_grid)
        .build();

    window.present();
}

fn make_covers(library: &Library) -> Vec<Image> {
    let mut ret = Vec::new();

    const HEIGHT: i32 = 200;
    const WIDTH: i32 = ((HEIGHT as f32) * 0.703) as i32;

    for series in library.series() {
        // todo multiple cover support
        let first_cover = &series.covers()[0];
        let img = ImageBuilder::new()
            .file(first_cover.path.to_str().unwrap())
            .width_request(WIDTH)
            .height_request(HEIGHT)
            .build();

        let gtk_box = BoxBuilder::new().build();
        gtk_box.append(&img);

        let gesture = gtk::GestureClick::new();
        gesture.set_button(gtk::gdk::ffi::GDK_BUTTON_PRIMARY as u32);

        gesture.connect_pressed(|gesture, num_consecutive_clicks, x, y| {
            gesture.set_state(gtk::EventSequenceState::Claimed);
            println!("{} {} {}", num_consecutive_clicks, x, y);
        });

        img.add_controller(&gesture);
        ret.push(img);
    }

    ret
}

fn make_event_controller() {}

fn attach_covers(covers: Vec<Image>, grid: &Grid) {
    let (mut row, mut col) = (0, 0);

    const WIDTH: i32 = 1;
    const HEIGHT: i32 = 1;

    for cover in covers {
        grid.attach(&cover, col, row, WIDTH, HEIGHT);
        col = col + 1;
    }
}
