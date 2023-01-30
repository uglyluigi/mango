use gtk::{
    builders::{
        LabelBuilder, NotebookBuilder, PictureBuilder, StackBuilder,
        StackSidebarBuilder,
    },
    subclass::{
        prelude::{ObjectSubclass},
    },
    traits::{BoxExt, GtkWindowExt},
    ApplicationWindow, Notebook, PositionType, StackSidebar,
};

use crate::categorizer_service::library::Series;

use super::component_impl::{
    fixed_box::{FixedDimBox},
};

pub fn open_chapter_view(series: Series) {
    let window = ApplicationWindow::builder()
        .title(series.title.as_str())
        .default_width(1000)
        .default_height(700)
        .child(&get_panes(&series))
        .build();

    window.present();
}

fn chapter_selector(series: &Series) -> StackSidebar {
    let stack = StackBuilder::new().build();

    for chapter in series.chapters.iter().rev() {
        let chapter_num = chapter.chapter_number;

        stack.add_titled(
            &generate_notebook(series, chapter_num),
            Option::<&str>::None,
            &format!("Chapter {}", chapter_num),
        );
    }

    StackSidebarBuilder::new()
        .css_classes(vec![String::from("chapter_view")])
        .stack(&stack)
        .build()
}

fn get_panes(series: &Series) -> gtk::Paned {
    let switcher = chapter_selector(series);
    let max_width_box = FixedDimBox::fixed_width(30);
    max_width_box.append(&switcher);

    gtk::Paned::builder()
        .start_child(&max_width_box)

        .end_child(&switcher.stack().unwrap())
        .build()
}

fn generate_notebook(series: &Series, chapter: i32) -> Notebook {
    let notebook = NotebookBuilder::new()
        .tab_pos(PositionType::Bottom)
        .scrollable(true)
        .build();

    let mut page_no = 1;

    let chapter = series.chapter(chapter).expect("Couldn\'t find chapter!!!");

    for image_path in &chapter.image_paths {
        notebook.append_page(
            &PictureBuilder::new()
                .file(&gtk::gio::File::for_path(image_path))
                .build(),
            Some(
                &LabelBuilder::new()
                    .label(&format!("Page {}", page_no))
                    .build(),
            ),
        );
        page_no += 1;
    }

    notebook
}
