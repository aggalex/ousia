use std::fmt::Debug;
use std::sync::Arc;
use gtkrs::prelude::ToValue;
use crate::prelude::{Handler, HandlerOf, HandlerWithType, State};

pub trait Notifier {
    type Argument;
    fn use_effect(self, arg: impl Fn(Self::Argument) + Send + Sync + Clone + 'static);

    fn combine<G>(self, arg: impl (Fn(Self::Argument) -> G) + Send + Sync + Clone + 'static) -> State<G>
    where G: ToValue + Clone + Sync + Send + PartialEq + 'static;
}

macro_rules! impl_notifier {
    ($($i:ident),*) => {
        impl<$($i),*> Notifier for ($(&$i),*)
        where $($i: HandlerWithType + Sync + Send),*
        {
            type Argument = ($($i::Type),*);

            #[allow(non_snake_case)]
            fn use_effect(self, f: impl Fn(Self::Argument) + Send + Sync + Clone + 'static) {
                let ($($i),*) = self;
                let this = ($( $i.clone() ),*);
                let closure = move || {
                    let ($($i),*) = &this;
                    let tuple = ($( $i.get() ),*);
                    f(tuple);
                };
                let ($($i),*) = self;
                $(
                    Handler::connect_notify($i, closure.clone());
                )*
            }

            #[allow(non_snake_case)]
            fn combine<G>(self, f: impl Fn(Self::Argument) -> G + Send + Sync + Clone + 'static) -> State<G>
            where G: ToValue + Clone + Sync + Send + PartialEq + 'static
            {
                let ($($i),*) = self;
                let this = ($( $i.clone() ),*);
                let closure = move || {
                    let ($($i),*) = &this;
                    let tuple = ($( $i.get() ),*);
                    f(tuple)
                };

                let state = State::new(closure.clone()());
                let ($($i),*) = self;
                $(
                    Handler::connect_notify($i, {
                        let closure = closure.clone();
                        let state = state.clone();
                        move || state.set(closure())
                    });
                )*

                state
            }

        }
    }
}

impl_notifier!(HT1);
impl_notifier!(HT1, HT2);
impl_notifier!(HT1, HT2, HT3);
impl_notifier!(HT1, HT2, HT3, HT4);
impl_notifier!(HT1, HT2, HT3, HT4, HT5);
impl_notifier!(HT1, HT2, HT3, HT4, HT5, HT6);
impl_notifier!(HT1, HT2, HT3, HT4, HT5, HT6, HT7);
impl_notifier!(HT1, HT2, HT3, HT4, HT5, HT6, HT7, HT8);
impl_notifier!(HT1, HT2, HT3, HT4, HT5, HT6, HT7, HT8, HT9);
impl_notifier!(HT1, HT2, HT3, HT4, HT5, HT6, HT7, HT8, HT9, HT10);
impl_notifier!(HT1, HT2, HT3, HT4, HT5, HT6, HT7, HT8, HT9, HT10, HT11);
impl_notifier!(HT1, HT2, HT3, HT4, HT5, HT6, HT7, HT8, HT9, HT10, HT11, HT12);