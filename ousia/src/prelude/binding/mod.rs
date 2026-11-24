use gtkrs::glib::Object;
use gtkrs::glib::object::BindingBuilder;
use gtkrs::prelude::{IsA, ObjectType, ToValue};

pub mod state;
pub mod bidirectional;

pub trait Cleanup: ObjectType {
    fn cleanup(&self, f: impl Fn() + 'static);
}

pub trait Handler: Clone {
    fn build_binding<'a>(&'a self, obj: &'a (impl IsA<Object> + Cleanup), prop: &'a str) -> BindingBuilder<'a, 'a, 'a>;

    fn handle(&self, obj: &(impl IsA<Object> + Cleanup), prop: &str) {
        self.build_binding(obj, prop)
            .build();
    }

    fn connect_notify(&self, f: impl (Fn()) + Send + Sync + 'static);
}

pub trait HandlerOf<T>: Handler where Self: 'static {
    fn get(&self) -> T;
}

pub trait HandlerWithType: HandlerOf<Self::Type> {
    type Type: ToValue + Send + Clone + 'static;
}