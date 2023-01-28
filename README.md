# Ousia

#### Write declarative UIs in Gtk4 and Rust

Ousia provides an easy way to write UIs in Rust without the hussle of creating 
GObject classes and tracking variables all around.

## IN ALPHA

This crate is not ready for release yet.

## Choose your style: macro or builder

Ousia provides two choices for building UIs: either use the macros which will 
build everything for you in a sleek struct-like syntax, or the builders which
do not require any macro invocation and thus might play a bit nicer with code
intelligence. 

### Macros

The macros are all syntax sugar on top of builders. All the statements in the 
macro are translated into builder method calls with one-to-one relationship, so
that error messages are as simple and understandable as possible.

#### Features:

- initialize a property using `property_name: value`
- track a property using the `#` prefix and a reactive monad as value
- connect to a gtk signal using the `@` prefix and a `'static` closure as value

```rust
fn build_ui_with_macros(app: &gtkrs::Application) {
    let state = LocalBehaviorSubject::new(0);

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
                #label: &state.clone().map(ToString::to_string)
            },
            append: &Button! {
                label: "+1",
                vexpand: true,
                @clicked: move |_| state.clone().next_by(|n| n + 1)
            }
        }
    };
    window.present();
}
```

### Builders

Builders form the backbone of the crate. They are automatically generated based on
the [gtk4-rs](https://github.com/gtk-rs/gtk4-rs) crate itself, which is a dependency
to this crate, and make use of its own builders on their implementations. ousia builders
provide a few extra features on top of [gtk4-rs](https://github.com/gtk-rs/gtk4-rs)'s
builders, namely in supporting ousia's Rx-like reactivity structs, and also signals.

#### Features:
- Get a ousia builder using the `::ousia()` method on your desired gtk4-rs widget
- initialize a property using the corresponding method, like `.property_name(value)`
- track a property using the `.bind()` builder provider and a reactive monad as value:
`.bind().property_name(reactive)`
- connect to a gtk signal using the `.connect()` builder provider
and a `'static` closure as value: `.connect().signal_name(move |_| do_something())`

```rust
fn build_ui_with_builders(app: &gtkrs::Application) {
    let state = LocalBehaviorSubject::new(0);

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
                .bind().label(&state.clone().map(ToString::to_string))
                .create()
            )
            .append( &gtkrs::Button::forte()
                .label("+1")
                .vexpand(true)
                .connect().clicked(move |_| state.clone().next_by(|n| n + 1))
                .create()
            )
            .create()
        )
        .create();
    window.present();
}

```

## Rx development with `rxrust`

The `rxrust` crate provides an Rx-compliant API for writing Observables and using subscriptions
to pass messages throughout your program. Ousia uses rxrust in order to provide the best 
asynchronous MVVM experience in the safety and swiftness of Rust.

### Features

- A respectable amount of Rx types and operators are implemented in rxRust. 
Choose the one that matches your needs!
- Use a `LocalBehaviorSubject` for a reactive-like widget state, that works just like 
`react.js`'s `useState` or `vue.js`'s `reactive`
- Use it in gtk objects with the `#` prefix in macros or the `.bind()` builder provider. Now
every time the observable casts to subscribers, the UI is updated accordingly.
- Use asynchronous programming using Shared Observables, and use them in your ui using the 
`.glib_context_local()` operator, that translates a shared observable into a local one using
glib's `MainContext` sender and receiver

## Examples

More examples can be found in the `examples` subcrate, where you'll find an application written in
ousia with multiple widgets showcasing its abilities, including asynchronous programming.