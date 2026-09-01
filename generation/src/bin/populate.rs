extern crate generation;

mod db;
mod schema;

use std::path::PathBuf;

use generation::generate::{Context, Generator};

const DEFAULT_DB: &str = "index.sqlite3";

fn source_crate_from_env() -> PathBuf {
    PathBuf::from(option_env!("SOURCE_CRATE").unwrap_or("gtk4-rs/gtk4"))
}

/// Index the gtk4-rs source tree into a SQLite database.
///
/// Usage:
///     populate index [db] [source-crate]
///
/// Both trailing arguments are optional: `db` defaults to `index.sqlite3` and
/// `source-crate` defaults to the `SOURCE_CRATE` env var (or `gtk4-rs/gtk4`).
fn index(db_path: &PathBuf, source_crate: PathBuf) {
    db::initialize(db_path);

    let context = Generator {
        target: PathBuf::from("target"),
        source_crate,
        ..Default::default()
    }
        .parse()
        .expect("Failed to parse gtk4-rs source");

    let count = context.classes.len();
    db::insert_classes(db_path, &context.classes)
        .expect("Failed to write classes to index");

    println!("Indexed {count} classes into {}", db_path.display());
}

/// Load generated code from a pre-populated SQLite index instead of parsing the
/// gtk4-rs source tree directly.
///
/// Usage:
///     populate generate [db] [target]
///
/// `db` defaults to `index.sqlite3`; `target` defaults to `ousia`.
fn generate(db_path: &PathBuf, target: PathBuf) {
    let mut generator = Generator::default();
    generator.target = target;

    if !generator.target.is_dir() {
        std::fs::create_dir(&generator.target)
            .expect("Failed to create target directory");
    }

    let classes = db::load_classes(db_path)
        .expect("Failed to load classes from index")
        .into_iter()
        .map(|(file_name, class)| {
            let mut output_file = generator.target.clone();
            output_file.push(file_name);
            (output_file, class)
        })
        .collect();

    let mut src = generator.source_crate.clone();
    src.push("src");
    src.push("auto");
    src.push("mod.rs");
    let file = syn::parse_file(
        &std::fs::read_to_string(&src)
            .expect("Failed to read source mod.rs"),
    )
        .expect("Failed to parse source mod.rs");

    let mut modrs = generator.target.clone();
    modrs.push("mod.rs");

    Context::new(generator, (modrs, file), classes)
        .populate()
        .generate()
        .expect("Failed to generate module from index");
}

fn usage() -> ! {
    eprintln!(
        "Usage:\n\
         \x20 populate index   [db] [source-crate]\n\
         \x20 populate generate [db] [target]\n"
    );
    std::process::exit(2);
}

fn main() {
    let mut args = std::env::args().skip(1);

    let command = args.next().unwrap_or_else(|| {
        eprintln!("No subcommand given.");
        usage();
    });

    match command.as_str() {
        "index" => {
            let db_path = args.next()
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(DEFAULT_DB));
            let source_crate = args.next()
                .map(PathBuf::from)
                .unwrap_or_else(source_crate_from_env);
            index(&db_path, source_crate);
        }
        "generate" => {
            let db_path = args.next()
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(DEFAULT_DB));
            let target = args.next()
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("ousia"));
            generate(&db_path, target);
        }
        other => {
            eprintln!("Unknown subcommand `{other}`.");
            usage();
        }
    }
}
