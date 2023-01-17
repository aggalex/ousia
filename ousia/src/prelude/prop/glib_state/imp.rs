use std::cell::{RefCell};
use gtkrs::glib;
use gtkrs::glib::{BoxedAnyObject, ParamSpec, ParamSpecObject, ToValue, Value};
use gtkrs::glib::once_cell::sync::Lazy;
use gtkrs::subclass::prelude::{ObjectImpl, ObjectImplExt, ObjectSubclass};

pub struct State {
    data: RefCell<BoxedAnyObject>
}

impl Default for State {
    fn default() -> Self {
        State {
            data: RefCell::new(BoxedAnyObject::new(0))
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for State {
    const NAME: &'static str = "State";
    type Type = super::State;
    type ParentType = glib::Object;
}

impl ObjectImpl for State {
    fn constructed(&self) {
        self.parent_constructed();
    }

    fn properties() -> &'static [ParamSpec] {
        static PROPS: Lazy<Vec<ParamSpec>> = Lazy::new(|| vec![
            ParamSpecObject::builder::<BoxedAnyObject>("value")
                .build()
        ]);

        PROPS.as_ref()
    }

    fn property(&self, _: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "value" => self.data.borrow().to_value(),
            _ => unimplemented!()
        }
    }

    fn set_property(&self, _: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "value" => {
                self.data.replace(value.get().unwrap());
            },
            _ => unimplemented!()
        }
    }
}