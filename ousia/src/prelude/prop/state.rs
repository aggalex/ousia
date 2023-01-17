use std::cell::{Ref, RefMut};
use std::default::Default;
use gtkrs::glib::{BoxedAnyObject, ObjectExt, SignalHandlerId};
use std::marker::PhantomData;
use gtkrs::glib;
use crate::prelude::{Prop};
use crate::prelude::prop::glib_state;

pub struct Reactive<T: 'static> {
    object: glib_state::State,
    phantom: PhantomData<T>
}

impl<T: 'static> Clone for Reactive<T> {
    fn clone(&self) -> Self {
        Self {
            object: self.object.clone(),
            phantom: Default::default()
        }
    }
}

impl<T: 'static> Reactive<T> {
    pub fn new (data: T) -> Self {
        let glib = glib_state::State::new();
        glib.set_property("value", BoxedAnyObject::new(data));
        Self {
            object: glib,
            phantom: PhantomData
        }
    }

    pub fn set(&self, data: T) {
        self.object.set_property("value", BoxedAnyObject::new(data));
    }

    pub fn with<Out>(&self, f: impl FnOnce(&T) -> Out) -> Out {
        let value = self.object.property::<BoxedAnyObject>("value");
        let x = f(&*value.borrow());
        x
    }

    pub fn get(&self) -> StateCell<T> {
        let value = self.object.property::<BoxedAnyObject>("value");
        StateCell {
            boxed: value,
            phantom: PhantomData::default()
        }
    }
}

impl<T: 'static> Prop<T> for Reactive<T> {
    fn with<Out>(&self, f: impl FnOnce(&T) -> Out) -> Out {
        Reactive::with(self, f)
    }

    fn connect_update(&self, func: impl Fn(&T) + 'static) -> Option<SignalHandlerId> {
        let out = self.object
            .connect_updated(move |value| func(&*value.borrow::<T>()));
        Some(out)
    }
}

pub struct StateCell<T> {
    boxed: BoxedAnyObject,
    phantom: PhantomData<T>
}

impl<T: 'static> StateCell<T> {

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.boxed.borrow_mut()
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        self.boxed.borrow()
    }
}
