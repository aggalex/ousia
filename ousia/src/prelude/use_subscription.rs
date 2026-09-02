use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use delegate::delegate;
use gtkrs::glib::Object;
use gtkrs::glib::prelude::{IsA, ObjectExt, ToValue};
use gtkrs::glib::property::PropertySet;
use rxrust::observer::Observer;
use rxrust::observable::Observable;
use rxrust::ops::Map;
use rxrust::prelude::*;

pub trait Cleanup: gtkrs::prelude::ObjectType {
    fn cleanup(&self, f: impl Fn() + 'static);
}

pub trait Handler: Clone {
    fn handle(&self, obj: &(impl IsA<Object> + Cleanup), prop: &str);
}

pub trait HandlerOf<T>: Handler + Observable<Item<'static> = T> where Self: 'static {}

impl<T, O: Handler + Observable<Item<'static> = T> + 'static> HandlerOf<T> for O {}

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

pub trait ObservableTypes: Context {
    type Subject<'a, T, Err> = Self::With<Subject<SubjectPtr<'a, Self, T, Err>>>
    where Self: 'a;

    type BehaviorSubject<'a, T: Clone, Err> = Self::With<BehaviorSubject<T, SubjectPtr<'a, Self, T, Err>>>
    where Self: 'a;
}

impl ObservableTypes for Local<()> {

}

#[derive(Clone)]
pub struct State<T: Clone> {
    pub subject: <Local<()> as ObservableTypes>::BehaviorSubject<'static, Rc<RefCell<T>>, ()>,
}

impl<T: Clone> From<T> for State<T> {
    fn from(state: T) -> Self {
        Self {
            subject: Local::behavior_subject(Rc::new(RefCell::new(state))),
        }
    }
}

// This is ugly, but at least it's a type alias
pub type StateObservable<T: Clone + 'static> = LocalCtx<Map<
    BehaviorSubject<Rc<RefCell<T>>,
        MutRc<Subscribers<BoxedObserver<'static, Rc<RefCell<T>>, ()>>>
    >, fn(Rc<RefCell<T>>) -> T>, LocalScheduler>;

impl<T: ToValue + Clone + 'static> State<T> {
    #[inline]
    pub fn observe(&self) -> StateObservable<T> {
        self.subject.clone()
            .map(|subject| subject.borrow().clone())
    }
}

impl<T: Clone> Observer<T, ()> for State<T> {
    #[inline]
    fn next(&mut self, value: T) {
        let cell = self.subject.peek();
        cell.set(value);
        self.subject.next(cell);
    }

    delegate! {
        to self.subject {
            fn error(self, err: ());
            fn complete(self);
            fn is_closed(&self) -> bool;
        }
    }
}

impl<T: Clone> Behavior for State<T> {
    type Item = T;

    #[inline]
    fn peek(&self) -> Self::Item {
        self.subject
            .peek()
            .borrow()
            .clone()
    }

    fn next_by(&mut self, f: impl FnOnce(Self::Item) -> Self::Item) {
        self.subject
            .next_by(|value| {
                let new_value = f(value.borrow().clone());
                value.set(new_value);
                value
            });
    }
}