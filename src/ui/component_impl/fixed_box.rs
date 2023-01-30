use std::cell::Cell;

use glib::Object;
use gtk::glib::object::ObjectBuilder;
use gtk::glib::{once_cell, ParamSpec, Value};
use gtk::subclass::prelude::*;
use gtk::{glib};

#[derive(Default)]
pub struct FixedDimBoxImpl {
    max_width: Cell<Option<i32>>,
    max_height: Cell<Option<i32>>,
}

#[gtk::glib::object_subclass]
impl ObjectSubclass for FixedDimBoxImpl {
    const NAME: &'static str = "FixedDimBoxImpl";
    type Type = FixedDimBox;
    type ParentType = gtk::Box;

    fn new() -> Self {
        Self {
            max_width: None.into(),
            max_height: None.into(),
        }
    }
}

impl ObjectImpl for FixedDimBoxImpl {
    fn constructed(&self) {
        self.parent_constructed();
    }

    fn set_property(&self, _id: usize, _value: &Value, _pspec: &ParamSpec) {
        match _pspec.name() {
            "max-width" => {
                let input_number = _value.get().expect("Property max-width must be type i32.");
                self.max_width.replace(if input_number > 0 {
                    Some(input_number)
                } else {
                    None
                });
            }
            "max-height" => {
                let input_number = _value.get().expect("Property max-height must be type i32.");
                self.max_height.replace(if input_number > 0 {
                    Some(input_number)
                } else {
                    None
                });
            }
            _ => unimplemented!(),
        }
    }

    fn properties() -> &'static [glib::ParamSpec] {
        use once_cell::sync::Lazy;

        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
            vec![
                glib::ParamSpecInt::new(
                    "max-width",
                    "Max-width",
                    "The max width the FixedDimBox should request",
                    0,
                    i32::MAX,
                    0,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecInt::new(
                    "max-height",
                    "Max-height",
                    "The max height the FixedDimBox should request",
                    0,
                    i32::MAX,
                    0,
                    glib::ParamFlags::READWRITE,
                ),
            ]
        });

        PROPERTIES.as_ref()
    }
}

impl WidgetImpl for FixedDimBoxImpl {}

impl LayoutManagerImpl for FixedDimBoxImpl {
    fn allocate(&self, widget: &gtk::Widget, width: i32, height: i32, baseline: i32) {
        self.parent_allocate(
            widget,
            self.max_width.get().unwrap_or(width),
            self.max_height.get().unwrap_or(height),
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
    pub fn new(max_width: i32, max_height: i32) -> Self {
        glib::Object::new(&[("max-width", &max_width), ("max-height", &max_height)])
    }

    pub fn fixed_width(width: i32) -> Self {
        Self::new(width, 0)
    }

    pub fn builder<'a>() -> Builder<'a> {
        Object::builder()
    }
}
