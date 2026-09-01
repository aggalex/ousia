extern crate generation;

use std::path::PathBuf;
use generation::generate::Generator;

fn main() {
    let target = PathBuf::from("src/ousia");
    // if target.read_dir()
    //     .map(|mut dir| !dir.next().is_none())
    //     .unwrap_or(false)
    // {
    //     return;
    // }

    println!("cargo:note=Generating module ousia",);

    let context = Generator {
        target,
        source_crate: PathBuf::from(option_env!("SOURCE_CRATE")
            .unwrap_or("../generation/gtk4-rs/gtk4")),
        // included: Some(vec!["application.rs", "application_window.rs", "box_.rs", "button.rs", "label.rs"]),
        excluded_classes: vec!["ParamSpecExpression".to_string()],
        ..Default::default()
    }
        .parse()
        .unwrap()
        .populate();

    context.diagnostics.emit();

    if context.diagnostics.has_errors() {
        panic!("ousia generation failed with errors (see diagnostics above)");
    }

    context.generate().unwrap();
}