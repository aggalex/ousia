use std::cell::RefCell;
use gtkrs::glib::{IsA, ObjectExt, Value};
use gtkrs::glib;
use rxrust::observable::ObservableItem;
use rxrust::subscription::Subscription;

pub trait Cleanup: gtkrs::prelude::ObjectType {
    fn cleanup(&self, f: impl Fn() + 'static);
}

pub trait Handler<T>: Clone {
    fn handle(&self, obj: &(impl IsA<glib::Object> + Cleanup), prop: &str);
}

impl<T, S> Handler<T> for S
    where
        S: ObservableItem<T, Box<dyn Fn(T)>> + Clone + 'static,
        T: Into<Value> + 'static
{
    fn handle(&self, obj: &(impl IsA<glib::Object> + Cleanup), prop: &str) {
        let obj_clone = obj.clone();
        let prop = prop.to_string();
        let sub = self.clone().subscribe(Box::new(move |value| {
            obj_clone.set_property(&prop, value);
        }) as Box<dyn Fn(T)>);
        let sub = RefCell::new(Some(sub));
        obj.cleanup(move || {
            let sub = std::mem::take(&mut *sub.borrow_mut());
            sub.map(|s| s.unsubscribe());
        });
    }
}