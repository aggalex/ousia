use std::path::PathBuf;
use quote::ToTokens;
use rust_format::{Formatter, RustFmt};
use syn::parse::Parse;
use syn::UseTree;
use crate::class::Class;
use crate::generate::Generator;

fn print_use_tree(use_tree: &UseTree) {
    match use_tree {
        UseTree::Path(p) => {
            println!("{}", p.ident);
            print_use_tree(&p.tree)
        },
        UseTree::Name(name) => println!("{}", name.ident),
        UseTree::Rename(name) => println!("as {}", name.ident),
        UseTree::Glob(glob) => println!("*"),
        UseTree::Group(group) => {
            group.items.iter()
                .for_each(print_use_tree)
        }
    }
}

#[test]
fn test_box() {
    let gtk_path = option_env!("SOURCE_CRATE").unwrap_or("gtk4-rs/gtk4");

    let mut gtk_path = PathBuf::from(gtk_path);
    gtk_path.push("src");
    gtk_path.push("auto");
    gtk_path.push("box_.rs");

    let file = std::fs::read_to_string(gtk_path).unwrap();
    let file = syn::parse_file(&file).unwrap();

    let class = Class::try_from(file).unwrap();
    // println!("{:#?}", class);

    println!("{}", RustFmt::new().format_tokens(class.into_token_stream()).unwrap());
}

#[test]
fn test_generate() {
    let ungenerated = Generator {
        included: Some(vec!["application.rs", "application_window.rs", "box_.rs", "button.rs", "label.rs"]),
        ..Generator::default()
    }
        .parse()
        .unwrap()
        .populate()
        .generate()
        .unwrap();

    println!("{}", ungenerated.into_iter()
        .map(|buf| buf.as_os_str()
            .to_str()
            .unwrap()
            .to_string())
        .collect::<Vec<_>>().join(", "));

}