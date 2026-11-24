use std::fmt::Debug;
use std::sync::{Arc, Mutex, PoisonError};
use std::sync::atomic::{AtomicBool, Ordering};
use gtkrs::glib::Object;
use gtkrs::glib::object::BindingBuilder;
use gtkrs::glib::property::{PropertyGet, PropertySet};
use gtkrs::glib::value::FromValue;
use gtkrs::prelude::*;
use crate::prelude::binding::{Cleanup, Handler};
use crate::prelude::{HandlerOf, HandlerWithType, State};

pub struct BidirectionalBinding<From, To> {
    original_state: State<From>,
    state: State<To>,
    update: Arc<Box<dyn (Fn(To) -> From) + Send + Sync + 'static>>,
}

impl<From, To> Clone for BidirectionalBinding<From, To> {
    fn clone(&self) -> Self {
        Self {
            original_state: self.original_state.clone(),
            state: self.state.clone(),
            update: self.update.clone(),
        }
    }
}

impl<From: ToValue + Send + Clone + PartialEq + 'static, To: ToValue + Send + Clone + PartialEq + for<'a> FromValue<'a> + 'static> Handler for BidirectionalBinding<From, To> {
    fn build_binding<'a>(&'a self, obj: &'a (impl IsA<Object> + Cleanup), prop: &'a str) -> BindingBuilder<'a, 'a, 'a> {
        let original_state = Mutex::new(self.original_state.clone());
        let update = self.update.clone();
        let update_flag = Arc::new(AtomicBool::new(true));
        let update_flag_clone = update_flag.clone();
        self.state.bind(obj, prop, move |item| {
            let update_flag_clone = update_flag_clone.clone();
            Some(item).filter(move |_| update_flag_clone.load(Ordering::SeqCst))
        })
            .transform_from(move |_, value: To| {
                update_flag.set(false);
                {
                    original_state.lock()
                        .unwrap_or_else(PoisonError::into_inner)
                        .set(update(value));
                }
                update_flag.set(true);
                Option::<bool>::None
            })
            .bidirectional()
    }

    fn connect_notify(&self, f: impl Fn() + Send + Sync + 'static) {
        self.state.connect_notify(f);
    }
}

impl<T: ToValue + Send + Clone + for<'a> FromValue<'a> + 'static> State<T> {
    pub fn bidirectional(&self) -> BidirectionalBinding<T, T> {
        BidirectionalBinding {
            original_state: self.clone(),
            state: self.clone(),
            update: Arc::new(Box::new(|item| item))
        }
    }
}

impl<T: ToValue + Send + Clone + PartialEq + 'static> State<T> {
    fn map_bidirectional<G: ToValue + Send + Clone + PartialEq + for<'a> FromValue<'a> + 'static>(
        &self,
        to: impl (Fn(T) -> G) + Send + Sync + 'static,
        from: impl (Fn(G) -> T) + Send + Sync + 'static,
    ) -> BidirectionalBinding<T, G> {
        let state = self.map(to);
        BidirectionalBinding {
            original_state: self.clone(),
            state,
            update: Arc::new(Box::new(from))
        }
    }
}

impl<
    From: ToValue + Send + Clone + PartialEq + 'static,
    To: ToValue + Send + Clone + PartialEq + for<'a> FromValue<'a> + 'static + Debug
> HandlerOf<To>
for BidirectionalBinding<From, To> {
    fn get(&self) -> To {
        self.state.get()
    }
}

impl<
    From: ToValue + Send + Clone + PartialEq + 'static,
    To: ToValue + Send + Clone + PartialEq + for<'a> FromValue<'a> + 'static + Debug
> HandlerWithType for BidirectionalBinding<From, To> {
    type Type = To;
}