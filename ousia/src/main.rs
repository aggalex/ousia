use gtkrs::prelude::*;
use ::ousia::prelude::*;
use ::ousia::*;
use gtkrs::{glib, Widget};

fn main() {
    // Create a new application
    let app = gtkrs::Application::builder().build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui_with_macros);

    // Run the application
    app.run();
}

#[allow(dead_code)]
fn build_ui_with_macros(app: &gtkrs::Application) {
    let state = Reactive::new(0);

    let window = ApplicationWindow! {
        application: app,
        title: "My GTK App",
        default_width: 200,
        default_height: 200,
        child: &Box! {
            orientation: gtkrs::Orientation::Vertical,
            spacing: 6,
            margin_start: 12,
            margin_end: 12,
            margin_top: 12,
            margin_bottom: 12,
            append: &Label! {
                vexpand: true,
                #label: &state.map(ToString::to_string)
            },
            append: &Button! {
                label: "+1",
                vexpand: true,
                @clicked: glib::clone!{ @strong state =>
                    move |_| state.with(|v| state.set(v + 1))
                }
            }
        }
    };
    window.present();
}

#[allow(dead_code)]
fn build_ui_with_builders(app: &gtkrs::Application) {
    let state = Reactive::new(0);

    let window = gtkrs::ApplicationWindow::ousia()
        .application(app)
        .default_width(200)
        .default_height(200)
        .child( &gtkrs::Box::ousia()
            .orientation(gtkrs::Orientation::Vertical)
            .margin_start(12)
            .margin_end(12)
            .margin_top(12)
            .margin_bottom(12)
            .spacing(6)
            .append( &gtkrs::Label::ousia()
                .vexpand(true)
                .bind().label(&state.map(ToString::to_string))
                .create()
            )
            .append( &gtkrs::Button::ousia()
                .label("+1")
                .vexpand(true)
                .connect().clicked(glib::clone!{ @strong state =>
                    move |_| state.with(|v| state.set(v + 1))
                })
                .create()
            )
            .create()
        )
        .create();
    window.present();
}

