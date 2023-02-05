use std::path::{Path, PathBuf};
use quote::__private::TokenStream;
use quote::ToTokens;
use rust_format::{Formatter, RustFmt};
use syn::{File, Pat};
use crate::class::Class;
use crate::module::Module;

pub struct Generator {
    pub target: PathBuf,
    pub source_crate: PathBuf,
    pub additional_builders: Vec<PathBuf>,
    pub additional_signals: Vec<PathBuf>,
    pub formatter: Box<dyn Fn(TokenStream) -> String>,
    pub excluded_classes: Vec<String>,
    pub included: Option<Vec<&'static str>>,
}

impl Default for Generator {
    fn default() -> Self {
        Generator {
            target: PathBuf::from("ousia"),
            source_crate: PathBuf::from(option_env!("SOURCE_CRATE")
                .unwrap_or("gtk4-rs/gtk4")),
            additional_signals: vec![],
            additional_builders: vec![],
            formatter: Box::new(|tt| RustFmt::default()
                .format_tokens(tt)
                .unwrap()),
            excluded_classes: vec![],
            included: None,
        }
    }
}

pub struct Context {
    generator: Generator,
    pub module: (PathBuf, File),
    pub classes: Vec<(PathBuf, Class)>,
    pub not_generated: Vec<PathBuf>
}

impl Context {
    pub fn populate(mut self) -> Self {
        self.classes = self.classes.iter()
            .cloned()
            .map(|(path, mut cls)| {
                for (_, parent) in cls.inherits.iter()
                    .filter_map(|p| self.classes.iter()
                        .find(|(_, cls)| &cls.name == p))
                    .collect::<Vec<_>>()
                {
                    cls.add_signals_from_class(parent);
                }

                (path, cls)
            })
            .collect();

        self
    }

    pub fn generate(self) -> Result<Vec<PathBuf>, std::io::Error> {
        let generated = self.classes.iter()
            .filter(|(_, class)| class.constructible && !self.generator.excluded_classes.contains(&class.name))
            .map(|(output_file, class)| {
                std::fs::write(output_file, (self.generator.formatter)(class.into_token_stream()))?;
                Ok(output_file.clone())
            })
            .collect::<Result<Vec<PathBuf>, std::io::Error>>()?;

        let mod_code = (self.generator.formatter)(
            Module::from(generated.clone())
                .fill_features(self.module.1)
                .into_token_stream()
        );

        std::fs::write(&self.module.0, mod_code)?;

        Ok(generated)
    }
}

impl Generator {
    pub fn parse(self) -> Result<Context, std::io::Error> {

        if !self.target.is_dir() {
            std::fs::create_dir(&self.target)?;
        }

        let mut src = self.source_crate.clone();
        src.push("src");
        src.push("auto");

        let files = std::fs::read_dir(&src).unwrap();

        let mut ungenerated = vec![];

        let mut classes = vec![];

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

            let mut handle_err = |path: PathBuf, err: syn::Error| {
                eprintln!("Failed to read file {}: {err}", path.to_str().unwrap().to_string());
                ungenerated.push(path);
            };

            let parse = |path: &Path| -> std::io::Result<syn::Result<File>> {
                std::fs::read_to_string(&path)
                    .map(|ref str| syn::parse_file(str))
            };

            let mut unpack = |(path, r): (&PathBuf, _)| match r {
                Ok(Ok(f)) => Some(Ok(f)),
                Ok(Err(e)) => {
                    handle_err(path.clone(), e);
                    None
                },
                Err(e) => Some(Err(e)),
                _ => None
            };

            let additional_builders = self.additional_builders.iter()
                .map(|p| (p, parse(p)))
                .filter_map(&mut unpack)
                .collect::<Result<Vec<_>, _>>()?;

            let additional_signals = self.additional_signals.iter()
                .map(|p| (p, parse(p)))
                .filter_map(&mut unpack)
                .collect::<Result<Vec<_>, _>>()?;

            let class = match parse(&path)?
                .and_then(Class::try_from)
                .map(|mut cls| {
                    for builder in &additional_builders {
                        let _ = cls.add_builder_from_file(builder);
                    }
                    for signal in &additional_signals {
                        let _ = cls.add_signals_from_file(signal);
                    }
                    cls
                })
                .map_err(|err| handle_err(path.clone(), err))
            {
                Ok(ast) => ast,
                Err(_) => continue
            };
            let mut output_file = self.target.clone();
            output_file.push(path.file_name().unwrap());

            classes.push((output_file, class));
        }

        src.push("mod.rs");
        let m = std::fs::read_to_string(&src)?;
        let file = syn::parse_file(&m).unwrap();
        let mut modrs = self.target.clone();
        modrs.push("mod.rs");

        let context = Context {
            generator: self,
            module: (modrs, file),
            classes,
            not_generated: ungenerated,
        };

        Ok(context)
    }
}