use std::fmt::Display;
use std::sync::{Weak, Arc, Mutex, PoisonError};
use gtkrs::glib;
use gtkrs::glib::Object;
use gtkrs::glib::clone::Downgrade;
use gtkrs::glib::prelude::{IsA};
use gtkrs::prelude::{Cast, FileExt, ObjectExt, ObjectType, ToValue, TreeModelExtManual};

pub trait Cleanup: ObjectType {
    fn cleanup(&self, f: impl Fn() + 'static);
}

pub trait Handler: Clone {
    fn handle(&self, obj: &(impl IsA<Object> + Cleanup), prop: &str);
}

pub trait HandlerOf<T>: Handler where Self: 'static {

}

mod imp {
    use std::cell::{Cell, RefCell};
    use gtkrs::glib;
    use gtkrs::glib::{Properties, Variant};
    use gtkrs::subclass::prelude::*;
    use gtkrs::glib::object::ObjectExt;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::ValueNotifier)]
    pub(super) struct ValueNotifier {
        #[property(get, set)]
        value: Cell<bool>,
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

    pub fn bind<'a, G: ToValue + 'static>(&'a self, obj: &'a impl ObjectType, prop: &'a str, get_value: impl (Fn() -> G) + Send + Sync + 'static) -> glib::object::BindingBuilder {
        self.bind_property("value", obj, prop)
            .transform_to(move |_, _: bool| Some(get_value().to_value()))
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

impl<T: Clone + ToValue + 'static> State<T> {
    pub fn new(value: T) -> Self {
        Self {
            delegate: ValueNotifier::new(),
            value: Arc::new(Mutex::new(value)),
        }
    }

    pub fn get(&self) -> T {
        self.value.lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    pub fn set(&self, value: T) {
        {
            let mut value_ref = self.value.lock()
                .unwrap_or_else(PoisonError::into_inner);
            *value_ref = value;
        }
        self.delegate.notify();
    }

    pub fn update(&self, value: impl FnOnce(T) -> T) {
        self.set(value(self.get()));
    }

    pub fn map<G>(&self, f: impl Fn(T) -> G + Send + Sync + 'static) -> State<G>
        where G: Clone + ToValue + 'static
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

impl<T: ToValue + 'static + Clone + Send> Handler for State<T> {
    fn handle(&self, obj: &(impl IsA<Object> + Cleanup), prop: &str) {
        let arc = self.value.clone();
        self.delegate.bind(obj.as_ref(), prop, move || {
            arc.lock().unwrap_or_else(PoisonError::into_inner).clone()
        })
            .sync_create()
            .build();
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

impl<T: ToValue + 'static + Clone + Send> HandlerOf<T> for State<T> {

}