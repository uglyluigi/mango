use std::path::Path;

use gtk::builders::{BoxBuilder, GridBuilder, ImageBuilder, PictureBuilder};
use gtk::ffi::{GtkGrid, GtkImage, GtkPicture, GtkWidget};
use gtk::gdk::{Display, Texture};
use gtk::gdk_pixbuf::Pixbuf;
use gtk::{prelude::GLAreaExt, Application, ApplicationWindow};
use gtk::{
    prelude::*, CssProvider, EventController, GestureClick, Grid, Image, Picture, StyleContext,
};

use crate::categorizer_service;
use crate::categorizer_service::library::Library;

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

fn quit(app: &Application) {}

fn build_ui(app: &Application) {
    let library_grid = GridBuilder::new().build();
    
    attach_covers(
        make_covers(&categorizer_service::library::LIBRARY, &library_grid),
        &library_grid,
    );

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Mango")
        .default_width(DEFAULT_WIDTH)
        .default_height(DEFAULT_HEIGHT)
        .child(&library_grid)
        .build();

    window.present();
}

fn make_covers(library: &Library, grid: &Grid) -> Vec<Picture> {
    let mut ret = Vec::new();

    for series in library.series() {
        // todo multiple cover support
        let first_cover = &series.covers()[0];
        let img = PictureBuilder::new()
            .css_classes(vec![String::from("cover")])
            .file(&gtk::gio::File::for_path(&first_cover.path))
            .build();

        let series_name = series.title.clone();

        let gesture = gtk::GestureClick::new();
        gesture.set_button(gtk::gdk::ffi::GDK_BUTTON_PRIMARY as u32);

        gesture.connect_pressed(move |gesture, _, _, _| {
            let series_name_ = &series_name;
            gesture.set_state(gtk::EventSequenceState::Claimed);
            println!("button series = {}", series_name_);
        });

        img.add_controller(&gesture);
        ret.push(img);
    }

    ret
}

fn attach_covers(covers: Vec<Picture>, grid: &Grid) {
    let (mut row, mut col) = (0, 0);

    const WIDTH: i32 = 1;
    const HEIGHT: i32 = 1;

    for cover in covers {
        grid.attach(&cover, col, row, WIDTH, HEIGHT);
        col = col + 1;

        if col == 4 {
            row = row + 1;
            col = 0;
        }
    }
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("styles/library_view.css"));

    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
