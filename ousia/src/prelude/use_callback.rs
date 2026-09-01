use std::future::Future;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use gtkrs::glib::{JoinHandle, MainContext};

pub trait CallbackProvider<'a> {
    type Handle<R: 'a>: 'a;
    fn callback<R: 'a>(self, f: impl AsyncFn() -> R + 'a) -> impl Fn() -> Self::Handle<R> + 'a;
}

impl CallbackProvider<'static> for MainContext {
    type Handle<R: 'static> = JoinHandle<R>;

    fn callback<R: 'static>(self, f: impl AsyncFn() -> R + 'static) -> impl Fn() -> Self::Handle<R> + 'static {
        let rcf = Arc::new(f);
        move || {
            let rcf = rcf.clone();
            self.spawn_local(async move {
                let f = rcf.deref();
                f().await
            })
        }
    }
}
