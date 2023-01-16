use std::path::PathBuf;
use quote::__private::TokenStream;
use quote::ToTokens;
use rust_format::{Formatter, RustFmt};
use crate::class::Class;
use crate::module::Module;

pub struct Generator {
    pub target: PathBuf,
    pub source_crate: PathBuf,
    pub formatter: Box<dyn Fn(TokenStream) -> String>,
    pub included: Option<Vec<&'static str>>
}

impl Default for Generator {
    fn default() -> Self {
        Generator {
            target: PathBuf::from("forte"),
            source_crate: PathBuf::from(option_env!("SOURCE_CRATE")
                .unwrap_or("gtk4-rs/gtk4")),
            formatter: Box::new(|tt| RustFmt::default()
                .format_tokens(tt)
                .unwrap()),
            included: None
        }
    }
}

impl Generator {
    pub fn generate(&self) -> Result<Vec<PathBuf>, std::io::Error> {

        if !self.target.is_dir() {
            std::fs::create_dir(&self.target)?;
        }

        let mut src = self.source_crate.clone();
        src.push("src");
        src.push("auto");

        let files = std::fs::read_dir(&src).unwrap();

        let mut ungenerated = vec![];
        let mut generated = vec![];

        for path in files {
            let path = path?.path();

            if path.file_name()
                .map(|name| name == "mod.rs"
                    || !name.to_str().unwrap().ends_with(".rs")
                    || !self.included.as_ref()
                        .map(|whitelist| whitelist.contains(&name.to_str().unwrap()))
                        .unwrap_or(true))
                .unwrap_or(false)
            {
                continue
            }

            let file = std::fs::read_to_string(&path)?;
            let class = match syn::parse_file(&file)
                .and_then(Class::try_from)
            {
                Ok(ast) => ast,
                Err(e) => {
                    eprintln!("Failed to read file {}: {e}", path.to_str().unwrap().to_string());
                    ungenerated.push(path);
                    continue
                }
            };
            let mut output_file = self.target.clone();
            output_file.push(path.file_name().unwrap());
            std::fs::write(output_file, (self.formatter)(class.into_token_stream()))?;
            generated.push(path);
        }

        src.push("mod.rs");
        let m = std::fs::read_to_string(&src)?;
        let file = syn::parse_file(&m).unwrap();
        let module = (self.formatter)(
            Module::from(generated)
                .fill_features(file)
                .into_token_stream()
        );
        let mut modrs = self.target.clone();
        modrs.push("mod.rs");
        std::fs::write(modrs, module)?;

        Ok(ungenerated)
    }
}