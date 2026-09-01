use std::future::Future;
use std::pin::Pin;
use gtkrs::glib;
use gtkrs::glib::{MainContext, Priority};
use rxrust::prelude::*;

#[derive(Clone)]
pub struct GlibScheduler {
    context: MainContext,
    priority: Priority,
}

impl Default for GlibScheduler {
    fn default() -> GlibScheduler {
        GlibScheduler::new(MainContext::default(), Priority::DEFAULT)
    }
}

impl GlibScheduler {
    pub fn new(context: MainContext, priority: Priority) -> Self {
        Self { context, priority }
    }
}

impl SleepProvider for GlibScheduler {
    type SleepFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

    fn sleep(&self, duration: Duration) -> Self::SleepFuture {
        glib::timeout_future(duration)
    }
}

impl<S> Scheduler<S> for GlibScheduler
    where
        S: Schedulable<Self> + Send + 'static,
        S::Future: Send + 'static,
{
    fn schedule(&self, source: S, delay: Option<Duration>) -> TaskHandle {
        let context = self.context.clone();
        let priority = self.priority;
        let future = source.into_future(self);
        let task = async move {
            if let Some(d) = delay {
                glib::timeout_future(d).await;
            }
            future.await;
        };
        context.spawn_with_priority(priority, task);
        TaskHandle::finished()
    }
}
