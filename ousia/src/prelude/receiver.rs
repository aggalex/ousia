use gtkrs::glib::{Continue, MainContext, Priority, PRIORITY_DEFAULT, Sender};
use rxrust::prelude::*;

pub trait ToLocalGlib<N, E> {

    fn glib_context_local(self, priority: Priority) -> LocalSubject<'static, N, E>;
}

static ERR_MSG: &'static str = "Unable to send to main context";

struct GLibSenderObserver<Item, Error> (pub Sender<Option<Result<Item, Error>>>);

impl<Item, Error> Observer for GLibSenderObserver<Item, Error> {
    type Item = Item;
    type Err = Error;

    fn next(&mut self, value: Self::Item) {
        self.0.send(Some(Ok(value))).expect(ERR_MSG)
    }

    fn error(&mut self, err: Self::Err) {
        self.0.send(Some(Err(err))).expect(ERR_MSG)
    }

    fn complete(&mut self) {
        self.0.send(None).expect(ERR_MSG)
    }
}

unsafe impl<Item, Error> Send for GLibSenderObserver<Item, Error> {}
unsafe impl<Item, Error> Sync for GLibSenderObserver<Item, Error> {}

impl<S, N: Clone + 'static, E: Clone + 'static> ToLocalGlib<N, E> for Shared<S>
    where S: SharedObservable<Item = N, Err = E>
{
    fn glib_context_local(self, priority: Priority) -> LocalSubject<'static, N, E> {
        let (sender, receiver) = MainContext::channel(priority);

        let out = LocalSubject::new();

        self.actual_subscribe(GLibSenderObserver(sender));

        let mut receiver_subject = out.clone();

        receiver.attach(
            None,
            move |value| {
                let out = Continue(value.is_some());
                match value {
                    Some(Ok(value)) => receiver_subject.next(value),
                    Some(Err(err)) => receiver_subject.error(err),
                    None => receiver_subject.complete()
                }
                out
            }
        );

        out
    }
}