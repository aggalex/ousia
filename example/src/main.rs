use gtk::{Application, ApplicationWindow};
use gtk::prelude::*;
use ::ousia::{*, prelude::*};

mod asynchronous;
mod counter;

fn main() {
    // Create a new application
    let app = Application::builder().build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow! {
        application: app,
        title: "Ousia examples",
        child: &Box! {
            append: &asynchronous::asynchronous(),
            append: &counter::counter()
        }
    };

    window.present();
}