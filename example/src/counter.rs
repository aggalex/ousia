use gtkrs::prelude::*;
use gtkrs::Widget;
use rxrust::prelude::*;
use ::ousia::{*, prelude::*};

pub fn counter() -> impl IsA<Widget> {
    let state = LocalBehaviorSubject::new(0);
    
    Box! {
        orientation: gtkrs::Orientation::Vertical,
        spacing: 6,
        margin_start: 12,
        margin_end: 12,
        margin_top: 12,
        margin_bottom: 12,
        append: &Label! {
            vexpand: true,
            #label: &state.clone().map(|n| n.to_string())
        },
        append: &Button! {
            label: "+1",
            vexpand: true,
            @clicked: move |_| {
                state.clone().next_by(|value| value + 1);
            }
        }
    }
}