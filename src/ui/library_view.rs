use gtk::{
    builders::{BoxBuilder, GridBuilder, LabelBuilder, PictureBuilder},
    ffi::GtkBox,
    gdk::Display,
    prelude::{GestureExt, IsA},
    traits::{BoxExt, GestureSingleExt, GridExt, OrientableExt, StyleContextExt, WidgetExt},
    CssProvider, Grid, Orientation, Picture, StyleContext, Widget,
};

use crate::{
    categorizer_service::library::{Library, LIBRARY},
    config::MANGO_CONFIG,
};

use super::chapter_view::open_chapter_view;

pub fn get_library_view() -> Grid {
    let library_grid = GridBuilder::new().column_homogeneous(true).build();
    attach_covers(make_covers_with_boxes(&LIBRARY), &library_grid);
    library_grid
}

fn make_covers_with_boxes(library: &Library) -> Vec<gtk::Box> {
    let mut ret = Vec::new();

    for series in library.series() {
        // todo multiple cover support
        let first_cover = &series.covers()[0];
        let img = PictureBuilder::new()
            .css_classes(vec![String::from("library_view")])
            .file(&gtk::gio::File::for_path(&first_cover.path))
            .build();

        let series_name = series.title.clone();
        let label = LabelBuilder::new()
            .css_classes(vec![String::from("library_view")])
            .label(&series_name.as_str())
            .build();

        let gesture = gtk::GestureClick::new();
        gesture.set_button(gtk::gdk::ffi::GDK_BUTTON_PRIMARY as u32);

        gesture.connect_pressed(move |gesture, _, _, _| {
            let series_name_ = &series_name;
            gesture.set_state(gtk::EventSequenceState::Claimed);
            open_chapter_view(&series_name_);
        });

        img.add_controller(&gesture);

        let img_box = BoxBuilder::new()
            .css_classes(vec![String::from("library_view")])
            .orientation(Orientation::Vertical)
            .build();
        img_box.append(&img);
        img_box.append(&label);

        ret.push(img_box);
    }

    ret
}

fn attach_covers<T>(covers: Vec<T>, grid: &Grid)
where
    T: IsA<Widget>,
{
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
