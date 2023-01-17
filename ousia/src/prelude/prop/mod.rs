mod glib_state;
mod state;
mod map;
mod property;
// mod as_ref;

pub use state::*;

use gtkrs::glib::SignalHandlerId;
use crate::prelude::prop::map::Map;

pub trait Prop<T: ?Sized>: Clone {
    fn with<Out>(&self, f: impl FnOnce(&T) -> Out) -> Out;

    fn connect_update(&self, func: impl Fn (&T) + 'static) -> Option<SignalHandlerId>;

    fn map<New, F: Fn(&T) -> New + Clone + 'static>(&self, f: F) -> Map<T, New, F, Self> where Self: Sized + Clone {
        Map::new(self.clone(), f)
    }
}

impl<P: Prop<String>> Prop<str> for P {
    fn with<Out>(&self, f: impl FnOnce(&str) -> Out) -> Out {
        self.with(|t| f(&t[..]))
    }

    fn connect_update(&self, func: impl Fn(&str) + 'static) -> Option<SignalHandlerId> {
        self.connect_update(move |val| func(&val[..]))
    }
}

impl<T, P: Prop<Vec<T>>> Prop<[T]> for P {
    fn with<Out>(&self, f: impl FnOnce(&[T]) -> Out) -> Out {
        self.with(|t| f(&t[..]))
    }

    fn connect_update(&self, func: impl Fn(&[T]) + 'static) -> Option<SignalHandlerId> {
        self.connect_update(move |val| func(&val[..]))
    }
}

