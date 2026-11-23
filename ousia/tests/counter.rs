use gtkrs::prelude::*;
use ::ousia::prelude::*;
use ::ousia::*;
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
    let state = State::new(0);

    let window = ApplicationWindow! {
        application: app,
        title: "My GTK App",
        default_width: 200,
        default_height: 200,
        child: &Box! {
            orientation: gtkrs::Orientation::Vertical,
            spacing: 6,
            append: &Label! {
                #label: &state.clone().map(|n| n.to_string())
            },
            append: &Button! {
                label: "+1",
                @clicked: glib::clone!(
                    #[strong] state,
                    move |_| {
                        state.clone().update(|v| v + 1);
                    },
                )
            }
        }
    };
    window.present();
}