use gtk::{
    builders::{
        BoxBuilder, LabelBuilder, NotebookBuilder, PictureBuilder, StackBuilder,
        StackSidebarBuilder, StackSwitcherBuilder,
    },
    traits::{BoxExt, GtkWindowExt},
    ApplicationWindow, Notebook, Orientation, PositionType, StackSidebar, StackSwitcher,
};

use crate::categorizer_service::library::Series;

pub fn open_chapter_view(series: Series) {
    let window = ApplicationWindow::builder()
        .title(series.title.as_str())
        .default_width(1000)
        .default_height(700)
        .child(&get_panes(&series))
        .build();

    window.present();
}

fn chapter_selector(series: &Series) -> StackSwitcher {
    let stack = StackBuilder::new().build();

    for chapter in &series.chapters {
        let chapter_num = chapter.chapter_number as usize;
        stack.add_titled(
            &generate_notebook(series, chapter_num - 1),
            Option::<&str>::None,
            &format!("Chapter {}", chapter_num),
        );
    }

    StackSwitcherBuilder::new().stack(&stack).build()
}

fn get_panes(series: &Series) -> gtk::Paned {
    gtk::Paned::builder()
        .start_child(&chapter_selector(series))
        .end_child(&generate_notebook(series, 0))
        .build()
}

fn generate_notebook(series: &Series, chapter: usize) -> Notebook {
    let notebook = NotebookBuilder::new()
        .tab_pos(PositionType::Bottom)
        .scrollable(true)
        .build();

    let mut page_no = 1;
    let chapter = &series.chapters[chapter];

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
