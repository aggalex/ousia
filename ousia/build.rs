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

    let ungenerated = Generator {
        target,
        source_crate: PathBuf::from(option_env!("SOURCE_CRATE")
            .unwrap_or("../generation/gtk4-rs/gtk4")),
        // included: Some(vec!["application.rs", "application_window.rs", "box_.rs", "button.rs", "label.rs"]),
        excluded_classes: vec!["ParamSpecExpression".to_string()],
        ..Default::default()
    }
        .parse()
        .unwrap()
        .populate()
        .generate()
        .unwrap();

    if ungenerated.len() != 0 {
        println!("cargo:warning=Failed to generate bindings for {len} files: {files}",
                 len = ungenerated.len(),
                 files = ungenerated.into_iter()
                     .map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
                     .collect::<Vec<_>>()
                     .join(", ")
        )
    }
}