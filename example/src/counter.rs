use gtk::prelude::*;
use gtk::{Widget};
use rxrust::prelude::*;
use ::ousia::{*, prelude::*};

pub fn counter() -> impl IsA<Widget> {
    let state = Local::behavior_subject(0).r#use();

    let string_memo = state()
        .map(|value| value.to_string())
        .r#use();

    Box! {
        orientation: gtk::Orientation::Vertical,
        spacing: 6,
        margin_start: 12,
        margin_end: 12,
        margin_top: 12,
        margin_bottom: 12,
        append: &Label! {
            vexpand: true,
            #label: &string_memo()
        },
        append: &Button! {
            label: "+1",
            vexpand: true,
            @clicked: move |_| {
                state().next_by(|value| value + 1);
            }
        }
    }
}