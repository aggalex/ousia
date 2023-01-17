use std::marker::PhantomData;
use gtkrs::glib;
use gtkrs::glib::{IsA, ObjectExt, SignalHandlerId};
use gtkrs::glib::value::FromValue;
use crate::prelude::Prop;

pub struct Property<Object: IsA<glib::Object>, T: for<'a> FromValue<'a> + 'static> {
    object: Object,
    property_name: String,
    phantom: PhantomData<T>
}

impl<Object: IsA<glib::Object>, T: for<'a> FromValue<'a> + 'static> Property<Object, T> {
    pub fn new(object: Object, name: String) -> Self {
        Self {
            object,
            property_name: name,
            phantom: Default::default()
        }
    }

    pub fn get(&self) -> T {
        self.object.property(&self.property_name)
    }
}

impl<Object: IsA<glib::Object>, T: for<'a> FromValue<'a> + 'static> Clone for Property<Object, T> {
    fn clone(&self) -> Self {
        Self {
            object: self.object.clone(),
            property_name: self.property_name.clone(),
            phantom: Default::default()
        }
    }
}

impl<Object: IsA<glib::Object>, T: for<'a> FromValue<'a> + 'static> Prop<T> for Property<Object, T> {
    fn with<Out>(&self, f: impl FnOnce(&T) -> Out) -> Out {
        f(&self.get())
    }

    fn connect_update(&self, func: impl Fn(&T) + 'static) -> Option<SignalHandlerId> {
        let name = self.property_name.clone();
        let id = self.object.connect_notify_local(
            Some(&self.property_name),
            move |obj, _| func(&obj.property(&name))
        );
        Some(id)
    }
}