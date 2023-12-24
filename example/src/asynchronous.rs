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
    let shared_state = BehaviorSubject::<_, Subject<_, _>>::new(0i32);
    let state = shared_state.clone();

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
        #label: &state.clone().map(|value| match value {
            0 => "Wait for 6s".to_string(),
            n => format!("t - {}s", n)
        }),
        #sensitive: &state.clone().map(|value| value == 0),
        @clicked: move |_| { run(); }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        use rxrust::prelude::*;
        let mut behavior = BehaviorSubject::<i32, Subject::<_, _>>::new(0);
        behavior.clone()
            .subscribe(|value| println!("{value}"));
        behavior.next(7);
        for i in 0..10 {
            behavior.next_by(|n| n + 1);
        }
        println!("{}", behavior.peek());
    }
}