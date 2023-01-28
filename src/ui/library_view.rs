use gtk::{
    builders::{GridBuilder, PictureBuilder},
    prelude::GestureExt,
    traits::{GestureSingleExt, GridExt, WidgetExt}, Grid, Picture,
};

use crate::{
    categorizer_service::library::{Library, LIBRARY},
    config::MANGO_CONFIG,
};

pub fn get_library_view() -> Grid {
    let library_grid = GridBuilder::new().build();
    attach_covers(make_covers(&LIBRARY), &library_grid);
	library_grid
}

fn make_covers(library: &Library) -> Vec<Picture> {
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

        if col == *MANGO_CONFIG.max_columns() as i32 {
            row = row + 1;
            col = 0;
        }
    }
}
