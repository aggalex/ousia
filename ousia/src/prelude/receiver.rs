use std::ops::ControlFlow;
use gtkrs::glib::{MainContext, Priority, Sender};
use gtkrs::glib::shared::{Shared, SharedMemoryManager};
use rxrust::prelude::*;

pub trait ToLocalGlib<N, E> {

    fn glib_context_local(self, priority: Priority) -> Subject<'static, N, E>;
}

static ERR_MSG: &'static str = "Unable to send to main context";

struct GLibSenderObserver<Item, Error> (pub Sender<Option<Result<Item, Error>>>);

impl<Item, Error> Observer<Item, Error> for GLibSenderObserver<Item, Error> {
    fn next(&mut self, value: Item) {
        self.0.send(Some(Ok(value))).expect(ERR_MSG)
    }

    fn error(self, err: Error) {
        self.0.send(Some(Err(err))).expect(ERR_MSG)
    }

    fn complete(self) {
        self.0.send(None).expect(ERR_MSG)
    }

    fn is_finished(&self) -> bool {
        false
    }
}

unsafe impl<Item, Error> Send for GLibSenderObserver<Item, Error> {}
unsafe impl<Item, Error> Sync for GLibSenderObserver<Item, Error> {}

impl<S, N: Clone + 'static, E: Clone + 'static> ToLocalGlib<N, E> for S
    where S: Observable<N, E, GLibSenderObserver<N, E>>
{
    fn glib_context_local(self, priority: Priority) -> Subject<'static, N, E> {
        let (sender, receiver) = MainContext::channel(priority);

        let out = Subject::<'static, _, _>::default();

        self.actual_subscribe(GLibSenderObserver(sender));

        let mut receiver_subject = out.clone();

        receiver.attach(
            None,
            move |value| {
                match value {
                    Some(Ok(value)) => {
                        receiver_subject.next(value);
                        gtkrs::glib::ControlFlow::Continue
                    },
                    Some(Err(err)) => {
                        receiver_subject.clone().error(err);
                        gtkrs::glib::ControlFlow::Break
                    },
                    None => {
                        receiver_subject.clone().complete();
                        gtkrs::glib::ControlFlow::Break
                    }
                }
            }
        );

        out
    }
}