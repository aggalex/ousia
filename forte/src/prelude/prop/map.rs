use std::marker::PhantomData;
use gtkrs::glib::SignalHandlerId;
use crate::prelude::Prop;

pub struct Map<
    From: ?Sized,
    To,
    F: Fn(&From) -> To + Clone + 'static,
    P: Prop<From> + Clone
> {
    prop: P,
    operator: F,
    phantom: PhantomData<(Box<From>, To)>
}

impl<
    From: ?Sized,
    To,
    F: Fn(&From) -> To + Clone + 'static,
    P: Prop<From> + Clone
> Clone for Map<From, To, F, P> {
    fn clone(&self) -> Self {
        Self {
            prop: self.prop.clone(),
            operator: self.operator.clone(),
            phantom: Default::default()
        }
    }
}

impl<
    From: ?Sized,
    To,
    F: Fn(&From) -> To + Clone + 'static,
    P: Prop<From> + Clone
> Map<From, To, F, P> {
    pub fn new (prop: P, operator: F) -> Self {
        Map {
            prop,
            operator,
            phantom: Default::default()
        }
    }

}

impl<
    From: ?Sized,
    To,
    F: Fn(&From) -> To + Clone + 'static,
    P: Prop<From> + Clone
> Prop<To> for Map<From, To, F, P> {
    fn with<Out>(&self, f: impl FnOnce(&To) -> Out) -> Out {
        self.prop.with(|value| {
            f(&(self.operator)(value))
        })
    }

    fn connect_update(&self, func: impl Fn(&To) + 'static) -> Option<SignalHandlerId> {
        let operator = self.operator.clone();
        self.prop.connect_update(move |value| func(&operator(value)))
    }
}