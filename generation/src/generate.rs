use crate::class::Class;
use crate::diagnostics::Diagnostics;
use crate::module::Module;
use proc_macro2::TokenStream;
use quote::ToTokens;
use rust_format::{Formatter, RustFmt};
use std::path::{Path, PathBuf};
use syn::File;

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
            source_crate: PathBuf::from(option_env!("SOURCE_CRATE").unwrap_or("gtk4-rs/gtk4")),
            additional_signals: vec![],
            additional_builders: vec![],
            formatter: Box::new(|tt| RustFmt::default().format_tokens(tt).unwrap()),
            excluded_classes: vec![],
            included: None,
        }
    }
}

pub struct Context {
    generator: Generator,
    pub module: (PathBuf, File),
    pub classes: Vec<(PathBuf, Class)>,
    pub diagnostics: Diagnostics,
}

impl Context {
    pub fn new(
        generator: Generator,
        module: (PathBuf, File),
        classes: Vec<(PathBuf, Class)>,
    ) -> Self {
        Context {
            generator,
            module,
            classes,
            diagnostics: Diagnostics::new(),
        }
    }

    pub fn populate(mut self) -> Self {
        self.classes = self
            .classes
            .iter()
            .cloned()
            .map(|(path, mut cls)| {
                let parents = cls
                    .inherits
                    .iter()
                    .filter_map(|(feat, inherits)| {
                        inherits
                            .iter()
                            .find(|name| {
                                self.classes.iter().any(|(_, cls)| &cls.name == *name)
                            })
                            .map(|name| {
                                let parent = self.classes
                                    .iter()
                                    .find(|(_, cls)| &cls.name == name)
                                    .unwrap()
                                    .1
                                    .clone();
                                (feat.clone(), parent)
                            })
                    })
                    .collect::<Vec<_>>();

                for (feat, parent) in &parents {
                    cls.add_signals_from_class(parent, feat);
                }

                (path, cls)
            })
            .collect();

        self
    }

    pub fn generate(self) -> Result<Vec<PathBuf>, std::io::Error> {
        let generated = self
            .classes
            .iter()
            .filter(|(_, class)| {
                class.constructible && !self.generator.excluded_classes.contains(&class.name)
            })
            .map(|(output_file, class)| {
                std::fs::write(
                    output_file,
                    (self.generator.formatter)(class.into_token_stream()),
                )?;
                Ok(output_file.clone())
            })
            .collect::<Result<Vec<PathBuf>, std::io::Error>>()?;

        let mod_code = (self.generator.formatter)(
            Module::from(generated.clone())
                .fill_features(self.module.1)
                .into_token_stream(),
        );

        std::fs::write(&self.module.0, mod_code)?;

        Ok(generated)
    }
}

impl Generator {
    pub fn parse(self) -> Result<Context, std::io::Error> {
        let mut diagnostics = Diagnostics::new();

        if !self.target.is_dir() {
            std::fs::create_dir(&self.target)?;
        }

        let mut src = self.source_crate.clone();
        src.push("src");
        src.push("auto");

        let files = std::fs::read_dir(&src).map_err(|e| {
            diagnostics.error_at(format!("Missing GTK4-rs submodule: {e}"), src.clone());
            e
        })?;

        let mut classes = vec![];

        for path in files {
            let path = path?.path();

            if path
                .file_name()
                .map(|name| {
                    name == "mod.rs"
                        || !name.to_str().unwrap().ends_with(".rs")
                        || !self
                            .included
                            .as_ref()
                            .map(|whitelist| whitelist.contains(&name.to_str().unwrap()))
                            .unwrap_or(true)
                })
                .unwrap_or(false)
            {
                continue;
            }

            let parse = |path: &Path| -> std::io::Result<syn::Result<File>> {
                std::fs::read_to_string(&path).map(|ref str| syn::parse_file(str))
            };

            let unpack = |(path, r): (&PathBuf, std::io::Result<syn::Result<File>>),
                          diagnostics: &mut Diagnostics|
             -> Option<Result<File, std::io::Error>> {
                match r {
                    Ok(Ok(f)) => Some(Ok(f)),
                    Ok(Err(e)) => {
                        diagnostics.error_at(format!("Failed to parse: {e}"), path.clone());
                        None
                    }
                    Err(e) => {
                        diagnostics.error_at(format!("Failed to read: {e}"), path.clone());
                        None
                    }
                    _ => None,
                }
            };

            let additional_builders = self
                .additional_builders
                .iter()
                .map(|p| (p, parse(p)))
                .filter_map(|pair| unpack(pair, &mut diagnostics))
                .collect::<Result<Vec<_>, _>>()?;

            let additional_signals = self
                .additional_signals
                .iter()
                .map(|p| (p, parse(p)))
                .filter_map(|pair| unpack(pair, &mut diagnostics))
                .collect::<Result<Vec<_>, _>>()?;

            let file = match parse(&path)? {
                Ok(file) => file,
                Err(e) => {
                    diagnostics.warning_at(format!("Failed to parse: {e}"), path.clone());
                    continue;
                }
            };

            let mut cls = match Class::try_from(file.clone()) {
                Ok(cls) => cls,
                Err(e) => {
                    diagnostics.warning_at(format!("Failed to parse: {e}"), path.clone());
                    continue;
                }
            };

            for warning in cls.populate_from_file(&file) {
                diagnostics.warning_at(warning.to_string(), path.clone());
            }

            for builder in &additional_builders {
                if let Err(e) = cls.add_builder_from_file(builder) {
                    diagnostics.warning_at(
                        format!("Failed to add builders from file: {e}"),
                        path.clone(),
                    );
                }
            }
            for signal in &additional_signals {
                if let Err(e) = cls.add_signals_from_file(signal) {
                    diagnostics.warning_at(
                        format!("Failed to add signals from file: {e}"),
                        path.clone(),
                    );
                }
            }

            let mut output_file = self.target.clone();
            output_file.push(path.file_name().unwrap());

            classes.push((output_file, cls));
        }

        src.push("mod.rs");
        let m = std::fs::read_to_string(&src)?;
        let file = syn::parse_file(&m)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        let mut modrs = self.target.clone();
        modrs.push("mod.rs");

        let context = Context {
            generator: self,
            module: (modrs, file),
            classes,
            diagnostics,
        };

        Ok(context)
    }
}
