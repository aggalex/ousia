use gtk::glib::{MainContext, timeout_future_seconds};
use ::ousia::{*, prelude::{*}};
use gtk::prelude::*;
use gtk::Widget;
use rxrust::prelude::*;

pub fn asynchronous() -> impl IsA<Widget> {
    Box! {
        append: &wait_button(),
        append: &wait_button(),
        append: &wait_button()
    }
}

fn wait_button() -> impl IsA<Widget> {
    let shared_state = BehaviorSubject::new(0);

    let run = move || {
        let state = shared_state.clone();
        MainContext::default().spawn_local(async move {
            for i in (0..6).rev() {
                state.clone().next(i);
                timeout_future_seconds(1).await;
            }
            state.clone().next(0);
        });
    };

    Button! {
        hexpand: true,
        #label: &shared_state.clone().map(|value| match value {
            0 => "Wait for 6s".to_string(),
            n => format!("t - {}s", n)
        }),
        #sensitive: &shared_state.clone().map(|value| value == 0),
        @clicked: move |_| { run(); }
    }
}