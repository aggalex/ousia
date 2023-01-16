use gtkrs::glib;
use gtkrs::glib::{BoxedAnyObject, Object, ObjectExt, SignalHandlerId};

mod imp;

glib::wrapper! {
    pub struct State(ObjectSubclass<imp::State>);
}

impl State {
    pub fn new() -> State {
        Object::builder().build()
    }

    pub fn connect_updated(&self, f: impl Fn(BoxedAnyObject) + 'static) -> SignalHandlerId {
        self.connect_notify_local(
            Some("value"),
            move |this, _| {
                f(this.property("value"))
            }
        )
    }
}