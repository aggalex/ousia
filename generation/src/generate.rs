use crate::class::Class;
use crate::diagnostics::{Diagnostic, Diagnostics, Severity};
use crate::module::Module;
use proc_macro2::TokenStream;
use quote::ToTokens;
use rust_format::{Formatter, RustFmt};
use std::path::{Path, PathBuf};
use syn::File;
use crate::alias::ClassOrAlias;

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

#[derive(Debug)]
enum GenerationError {
    IO(std::io::Error),
    Syn(syn::Error),
}

impl From<syn::Error> for GenerationError {
    fn from(err: syn::Error) -> Self {
        GenerationError::Syn(err)
    }
}

impl From<std::io::Error> for GenerationError {
    fn from(err: std::io::Error) -> Self {
        GenerationError::IO(err)
    }
}

impl std::fmt::Display for GenerationError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenerationError::IO(err) => write!(fmt, "Failed to read: {}", err),
            GenerationError::Syn(err) => write!(fmt, "Failed to parse: {}", err),
        }
    }
}

impl std::error::Error for GenerationError {}

struct DiagnosticsScope<'a> {
    path: &'a PathBuf,
    tag: &'a str,
}

impl<'a> DiagnosticsScope<'a> {
    fn new(path: &'a PathBuf, tag: &'a str) -> Self {
        DiagnosticsScope { path, tag }
    }

    fn try_or_log<T>(&self, f: impl FnOnce(&'a PathBuf) -> Result<T, GenerationError>) -> Result<T, Diagnostics> {
        let mut diagnostics = Diagnostics::new();
        let tag = self.tag;
        match f(self.path) {
            Ok(f) => Ok(f),
            Err(GenerationError::IO(e)) => {
                diagnostics.error_at(format!("Failed to parse {tag}: {e}"), self.path.clone());
                Err(diagnostics)
            }
            Err(GenerationError::Syn(e)) => {
                diagnostics.error_at(format!("Failed to read {tag}: {e}"), self.path.clone());
                Err(diagnostics)
            }
        }
    }
}

struct CollectedResult<Ok, Err> {
    ok: Ok,
    err: Err
}

impl<OkI, ErrI, Ok, Err> FromIterator<Result<OkI, ErrI>> for CollectedResult<Ok, Err>
    where Ok: FromIterator<OkI>, Err: FromIterator<ErrI>
{
    fn from_iter<I: IntoIterator<Item=Result<OkI, ErrI>>>(iter: I) -> Self {
        let mut ok = vec![];
        let mut err = vec![];
        for item in iter {
            match item {
                Ok(item) => ok.push(item),
                Err(item) => err.push(item)
            }
        }
        Self {
            ok: ok.into_iter().collect(),
            err: err.into_iter().collect(),
        }
    }
}

impl Generator {
    pub fn parse(self) -> Result<Context, Diagnostics> {
        let mut diagnostics = Diagnostics::new();

        if !self.target.is_dir() {
            match std::fs::create_dir(&self.target) {
                Ok(item) => item,
                Err(err) => {
                    diagnostics.error_at(format!("Failed to create directory: {err}"), self.target.clone());
                    return Err(diagnostics)
                }
            }
        }

        let mut src = self.source_crate.clone();
        src.push("src");
        src.push("auto");

        let files = match std::fs::read_dir(&src) {
            Ok(files) => files,
            Err(e) => {
                diagnostics.error_at(format!("Missing GTK4-rs submodule: {e}"), src.clone());
                return Err(diagnostics)
            }
        };

        let mut classes = vec![];
        let mut aliases = vec![];

        for path in files {
            let path = match path {
                Ok(path) => path.path(),
                Err(e) => {
                    diagnostics.error(format!("Invalid path: {}", e));
                    continue;
                }
            };

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

            let parse = |path: &PathBuf| -> Result<File, GenerationError> {
                Ok(syn::parse_file(&std::fs::read_to_string(&path)?)?)
            };

            let CollectedResult {
                ok: additional_builders,
                err: builder_diagnostics
            } = self
                .additional_builders
                .iter()
                .map(|p| DiagnosticsScope::new(p, "builders").try_or_log(parse))
                .collect::<CollectedResult<Vec<_>, Diagnostics>>();

            diagnostics.extend(builder_diagnostics);

            let CollectedResult {
                ok: additional_signals,
                err: signals_diagnostics
            } = self
                .additional_signals
                .iter()
                .map(|p| DiagnosticsScope::new(p, "signals").try_or_log(parse))
                .collect::<CollectedResult<Vec<_>, _>>();

            diagnostics.extend(signals_diagnostics);

            let class_scope = DiagnosticsScope::new(&path, "class");

            let file = match class_scope.try_or_log(parse) {
                Ok(file) => file,
                Err(e) => {
                    diagnostics.extend(e);
                    continue;
                }
            };

            let mut cls = match class_scope.try_or_log(|_| Ok(ClassOrAlias::try_from(file.clone())?)) {
                Ok(cls) => cls,
                Err(e) => {
                    diagnostics.extend_as(e, Severity::Warning);
                    continue;
                }
            };

            match cls {
                ClassOrAlias::Alias(alias) => {
                    aliases.push(alias);
                },
                ClassOrAlias::Class(mut cls) => {
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
            }
        }

        let mut modrs = self.target.clone();

        src.push("mod.rs");
        let module_res = DiagnosticsScope::new(&src, "module").try_or_log(|path| {
            let m = std::fs::read_to_string(&src)?;
            let file = syn::parse_file(&m)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            modrs.push("mod.rs");
            Ok((modrs, file))
        });

        let module = match module_res {
            Ok(module) => module,
            Err(err) => {
                diagnostics.extend(err);
                return Err(diagnostics);
            }
        };

        let context = Context {
            generator: self,
            module,
            classes,
            diagnostics,
        };

        Ok(context)
    }
}
