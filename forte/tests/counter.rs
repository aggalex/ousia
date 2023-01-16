use gtkrs::prelude::*;
use ::forte::prelude::*;
use ::forte::*;
use gtkrs::glib;

fn main() {
    // Create a new application
    let app = gtkrs::Application::builder().build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &gtkrs::Application) {
    let state = Reactive::new(0);

    let window = ApplicationWindow! {
        application: app,
        title: "My GTK App",
        default_width: 200,
        default_height: 200,
        child: &Box! {
            orientation: gtkrs::Orientation::Vertical,
            spacing: 6,
            append: &Label! {
                bind_label: &state.map(i32::to_string)
            },
            append: &Button! {
                label: "+1",
                @connect_clicked: glib::clone!{ @strong state =>
                    move |_| state.with(|v| state.set(v + 1))
                }
            }
        }
    };
    window.present();
}
