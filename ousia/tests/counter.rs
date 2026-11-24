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
    let state = State::new(0).r#use();
    let text = State::new("Hello World!".to_string()).r#use();

    (&state(), &text()).use_effect(move |(state, text)| {
        println!("Count: {}", state);
    });

    let window = ApplicationWindow! {
        application: app,
        title: "My GTK App",
        default_width: 200,
        default_height: 200,
        child: &Box! {
            orientation: gtkrs::Orientation::Vertical,
            spacing: 6,
            append: &Label! {
                #label: &(&state(), &text()).combine(|(state, text)| {
                    format!("Count: {}, Message: {}", state, text)
                })
            },
            append: &Entry! {
                #text: &text().bidirectional()
            },
            append: &Button! {
                label: "+1",
                @clicked: glib::clone!(
                    #[strong] state,
                    move |_| {
                        state().update(|v| v + 1);
                    },
                )
            }
        }
    };
    window.present();
}