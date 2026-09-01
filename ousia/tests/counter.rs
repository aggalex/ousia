use gtkrs::prelude::*;
use ::ousia::prelude::*;
use ::ousia::*;
use gtkrs::glib;
use rxrust::prelude::*;

fn main() {
    // Create a new application
    let app = gtkrs::Application::builder().build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &gtkrs::Application) {
    let state = Local::behavior_subject(0);

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
                        state.clone().next_by(|v| v + 1);
                    },
                )
            }
        }
    };
    window.present();
}