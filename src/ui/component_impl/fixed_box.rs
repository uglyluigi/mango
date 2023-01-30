#[derive(Default)]
pub struct FixedDimBoxImpl {
    max_width: Option<i32>,
    max_height: Option<i32>,
}

use glib::Object;
use gtk::glib::object::ObjectBuilder;
use gtk::prelude::IsA;
use gtk::subclass::prelude::*;
use gtk::traits::BoxExt;
use gtk::{glib, Widget};

#[gtk::glib::object_subclass]
impl ObjectSubclass for FixedDimBoxImpl {
    const NAME: &'static str = "FixedDimBoxImpl";
    type Type = FixedDimBox;
    type ParentType = gtk::Box;
}

impl ObjectImpl for FixedDimBoxImpl {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for FixedDimBoxImpl {}

impl LayoutManagerImpl for FixedDimBoxImpl {
    fn allocate(&self, widget: &gtk::Widget, width: i32, height: i32, baseline: i32) {
        self.parent_allocate(
            widget,
            self.max_width.unwrap_or(width),
            self.max_height.unwrap_or(height),
            baseline,
        );
    }
}

impl BoxImpl for FixedDimBoxImpl {}

glib::wrapper! {
    pub struct FixedDimBox(ObjectSubclass<FixedDimBoxImpl>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

type Builder<'a> = ObjectBuilder<'a, FixedDimBox>;

impl FixedDimBox {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn builder<'a>() -> Builder<'a> {
        Object::builder()
    }
}
