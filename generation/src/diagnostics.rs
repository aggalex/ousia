use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Warning,
    Error,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Warning => write!(f, "warning"),
            Severity::Error => write!(f, "error"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub file: Option<PathBuf>,
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.file {
            Some(path) => write!(f, "[{}] {}: {}", self.severity, path.display(), self.message),
            None => write!(f, "[{}] {}", self.severity, self.message),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Diagnostics {
    diagnostics: Vec<Diagnostic>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn warning(&mut self, msg: impl Into<String>) {
        self.push(Diagnostic {
            severity: Severity::Warning,
            message: msg.into(),
            file: None,
        });
    }

    pub fn warning_at(&mut self, msg: impl Into<String>, file: PathBuf) {
        self.push(Diagnostic {
            severity: Severity::Warning,
            message: msg.into(),
            file: Some(file),
        });
    }

    pub fn error(&mut self, msg: impl Into<String>) {
        self.push(Diagnostic {
            severity: Severity::Error,
            message: msg.into(),
            file: None,
        });
    }

    pub fn error_at(&mut self, msg: impl Into<String>, file: PathBuf) {
        self.push(Diagnostic {
            severity: Severity::Error,
            message: msg.into(),
            file: Some(file),
        });
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.severity == Severity::Error)
    }

    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Emit all diagnostics as `cargo:warning=` / `cargo:error=` lines.
    pub fn emit(&self) {
        for diag in &self.diagnostics {
            match diag.severity {
                Severity::Warning => println!("cargo:warning={diag}"),
                Severity::Error => println!("cargo:error={diag}"),
            }
        }
    }

    /// Emit diagnostics and return `Ok(ok)` if no errors, or `Err(())` if any errors exist.
    pub fn emit_or_ok<T>(&self, ok: T) -> Result<T, ()> {
        self.emit();
        if self.has_errors() {
            Err(())
        } else {
            Ok(ok)
        }
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
}
