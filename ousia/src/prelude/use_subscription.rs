use std::marker::PhantomData;
use gtkrs::glib::{IsA, Object, ObjectExt, ObjectType, ToValue};
use gtkrs::prelude::WidgetExt;
use gtkrs::{Application, glib, Widget};
use gtkrs::pango::Item;
use gtkrs::SortColumn::Default;
use rxrust::observable::Observable;
use rxrust::prelude::{BehaviorSubject, LocalBehaviorSubject, LocalObservable, Observer, SubscribeNext, SubscriptionLike};
use rxrust::shared::SharedObservable;

pub trait Cleanup: gtkrs::prelude::ObjectType {
    fn cleanup(&self, f: impl FnMut() + 'static);
}

pub trait Handler<T>: Clone {
    fn handle(&self, obj: &(impl IsA<glib::Object> + Cleanup), prop: &str);
}

impl<T, S> Handler<T> for S
    where
        S: LocalObservable<'static, Err = (), Item = T> + Clone + 'static,
        T: ToValue + 'static
{
    fn handle(&self, obj: &(impl IsA<glib::Object> + Cleanup), prop: &str) {
        let obj_clone = obj.clone();
        let prop = prop.to_string();
        let mut sub = self.clone().subscribe(move |value|
            obj_clone.set_property(&prop, value)
        );
        obj.cleanup(move || sub.unsubscribe());
    }
}