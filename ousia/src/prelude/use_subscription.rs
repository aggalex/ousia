use std::cell::RefCell;
use std::marker::PhantomData;
use gtkrs::glib::Object;
use gtkrs::glib::prelude::{IsA, ObjectExt, ToValue};
use rxrust::observer::Observer;
use rxrust::observable::Observable;
use rxrust::prelude::*;

pub trait Cleanup: gtkrs::prelude::ObjectType {
    fn cleanup(&self, f: impl Fn() + 'static);
}

pub trait Handler: Clone {
    fn handle(&self, obj: &(impl IsA<Object> + Cleanup), prop: &str);
}

struct PropertySetter<N> {
    obj: Object,
    prop: String,
    _marker: PhantomData<fn(N)>,
}

impl<N: ToValue> Observer<N, ()> for PropertySetter<N> {
    fn next(&mut self, value: N) {
        self.obj.set_property(&self.prop, &value);
    }

    fn error(self, _err: ()) {}

    fn complete(self) {}

    fn is_closed(&self) -> bool {
        false
    }
}

impl<S> Handler for S
    where
        S: Observable + Clone + 'static,
        for<'a> S::Item<'a>: ToValue,
        for<'a> S::Inner: CoreObservable<S::With<PropertySetter<S::Item<'a>>>>,
{
    fn handle(&self, obj: &(impl IsA<Object> + Cleanup), prop: &str) {
        let obj_clone: Object = obj.clone().into();
        let prop = prop.to_string();
        let sub = RefCell::new(Some(
            self.clone().subscribe_with(PropertySetter {
                obj: obj_clone,
                prop,
                _marker: PhantomData,
            })
        ));
        obj.cleanup(move || {
            if let Some(s) = sub.borrow_mut().take() {
                s.unsubscribe();
            }
        });
    }
}

pub trait Hook {
    fn r#use(self) -> impl Clone + Fn() -> Self;
}

impl<O: Clone + 'static> Hook for O {

    fn r#use(self) -> impl Clone + Fn() -> Self {
        move || self.clone()
    }
}