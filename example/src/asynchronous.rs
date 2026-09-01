use ::ousia::prelude::use_callback::CallbackProvider;
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
    let state = Local::behavior_subject(0).r#use();

    let run = {
        let state = state().r#use();
        MainContext::default().callback(async move || {
            for i in (0..6).rev() {
                state().next(i);
                timeout_future_seconds(1).await;
            }
            state().next(0);
        })
    };

    Button! {
        hexpand: true,
        #label: &state().map(|value| match value {
            0 => "Wait for 6s".to_string(),
            n => format!("t - {}s", n)
        }),
        #sensitive: &state().map(|value| value == 0),
        @clicked: move |_| { run(); }
    }
}