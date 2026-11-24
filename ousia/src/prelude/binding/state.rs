use std::fmt::{Debug, Display};
use std::sync::{Weak, Arc, Mutex, PoisonError};
use gtkrs::glib;
use gtkrs::glib::Object;
use gtkrs::glib::clone::Downgrade;
use gtkrs::glib::object::BindingBuilder;
use gtkrs::glib::prelude::{IsA};
use gtkrs::prelude::{Cast, FileExt, ObjectExt, ObjectType, ToValue, TreeModelExtManual};
use crate::prelude::binding::{Cleanup, Handler, HandlerOf};
use crate::prelude::HandlerWithType;

mod imp {
    use std::cell::{Cell};
    use std::sync::atomic::AtomicBool;
    use gtkrs::glib;
    use gtkrs::glib::{Properties, Variant};
    use gtkrs::subclass::prelude::*;
    use gtkrs::glib::object::ObjectExt;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::ValueNotifier)]
    pub(super) struct ValueNotifier {
        #[property(get, set)]
        value: AtomicBool,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ValueNotifier {
        const NAME: &'static str = "ValueDelegate";
        type Type = super::ValueNotifier;
        type ParentType = glib::Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ValueNotifier {

    }
}

glib::wrapper! {
    pub(self) struct ValueNotifier(ObjectSubclass<imp::ValueNotifier>);
}

impl ValueNotifier {
    pub fn new() -> ValueNotifier {
        Object::builder().build()
    }

    pub fn notify(&self) {
        self.notify_value();
    }

    pub fn bind<'a, G: ToValue + 'static>(&'a self, obj: &'a impl ObjectType, prop: &'a str, get_value: impl (Fn() -> Option<G>) + Send + Sync + 'static) -> BindingBuilder<'a, 'a, 'a> {
        self.bind_property("value", obj, prop)
            .transform_to(move |_, _: bool| get_value().map(|g| g.to_value()))
    }
}

pub struct State<T> {
    pub value: Arc<Mutex<T>>,
    delegate: ValueNotifier,
}

struct WeakState<T> {
    pub value: Weak<Mutex<T>>,
    delegate: <ValueNotifier as Downgrade>::Weak,
}

impl<T: Clone + ToValue + Send + PartialEq + 'static> State<T> {
    pub fn new(value: T) -> Self {
        Self {
            delegate: ValueNotifier::new(),
            value: Arc::new(Mutex::new(value)),
        }
    }

    pub fn set(&self, value: T) {
        if {
            let mut guard = self.value.lock()
                .unwrap_or_else(PoisonError::into_inner);
            let changed = &*guard != &value;
            if changed {
                *guard = value;
            }
            changed
        } {
            self.delegate.notify();
        }
    }

    pub fn update(&self, value: impl FnOnce(T) -> T) {
        self.set(value(self.get()));
    }

    pub fn map<G>(&self, f: impl Fn(T) -> G + Send + Sync + 'static) -> State<G>
        where G: Clone + ToValue + Send + PartialEq + 'static
    {
        let clonable_f = Arc::new(f);
        let f = clonable_f.clone();
        let new_value = f(self.get());
        let new_state = State::new(new_value);
        let new_state_clone = new_state.clone();
        let self_weak = self.downgrade();
        let f = clonable_f.clone();
        self.delegate.connect_value_notify(move |_| {
            let Some(this) = self_weak.upgrade() else { return };
            let item = f(this.get());
            new_state_clone.set(item);
        });
        new_state
    }

    pub(super) fn bind<'a, G>(
        &'a self,
        obj: &'a (impl IsA<Object> + Cleanup),
        prop: &'a str,
        f: impl (Fn(T) -> Option<G>) + Send + Sync + 'static
    ) -> BindingBuilder<'a, 'a, 'a> where G: ToValue + 'static {
        let arc = self.value.clone();
        self.delegate.bind(obj.as_ref(), prop, move || {
            f(arc.lock().unwrap_or_else(PoisonError::into_inner).clone())
        })
            .sync_create()
    }

    fn downgrade(&self) -> WeakState<T> {
        WeakState {
            value: Arc::downgrade(&self.value),
            delegate: Downgrade::downgrade(&self.delegate),
        }
    }
}

impl<T> WeakState<T> {
    pub fn upgrade(&self) -> Option<State<T>> {
        Some(State {
            value: self.value.upgrade()?,
            delegate: self.delegate.upgrade()?
        })
    }
}

impl<T: ToValue + 'static + Clone + Send + PartialEq> Handler for State<T> {
    fn build_binding<'a>(&'a self, obj: &'a (impl IsA<Object> + Cleanup), prop: &'a str) -> BindingBuilder<'a, 'a, 'a> {
        self.bind(obj, prop, Some)
    }

    fn connect_notify(&self, f: impl Fn() + Send + Sync + 'static) {
        self.delegate.connect_value_notify(move |_| { f() });
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            delegate: self.delegate.clone(),
        }
    }
}

impl<T: ToValue + 'static + Clone + Send + PartialEq> HandlerOf<T> for State<T> {
    fn get(&self) -> T {
        self.value.lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}

impl<T: ToValue + 'static + Clone + Send + PartialEq> HandlerWithType for State<T> {
    type Type = T;
}