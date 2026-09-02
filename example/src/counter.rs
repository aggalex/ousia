use gtk::prelude::*;
use gtk::{Widget};
use rxrust::prelude::*;
use ::ousia::{*, prelude::*};

pub fn counter() -> impl IsA<Widget> {
    let state = State::from(0i32).r#use();

    let string_memo = state()
        .observe()
        .map(|value| format!("{}", value));

    let add = |i: i32| {
        let state = state.clone();
        move || state().next_by(|value| value + i)
    };

    let up = add(1);
    let down = add(-1);

    let reset = {
        let state = state.clone();
        move || state().next(0)
    };

    Box! {
        orientation: gtk::Orientation::Vertical,
        spacing: 6,
        margin_start: 12,
        margin_end: 12,
        margin_top: 12,
        margin_bottom: 12,
        append: &Label! {
            vexpand: true,
            #label: &string_memo
        },
        append: &Box! {
            orientation: gtk::Orientation::Horizontal,
            spacing: 6,
            append: &Button! {
                label: "-1",
                vexpand: true,
                @clicked: move |_| down()
            },
            append: &Button! {
                label: "+1",
                vexpand: true,
                @clicked: move |_| up()
            },
            append: &Button! {
                label: "Reset",
                vexpand: true,
                @clicked: move |_| reset()
            }
        }
    }
}