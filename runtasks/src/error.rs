//! One error type for the hole crate.
//! 
//! Rust has no exceptions: fallible functions return `Result<T, E>`. Having a single
//! `E` lets every module use `?` to leveling up errors to `main`, which is
//! the only place that decides how to show them to users.

use std::fmt;
use std::io;
use std::path::PathBuf;

/// Convenient alias so signatures read `Result<Pipeline>` instead of 
/// `Result<Pipeline, RunTasksError>`.
pub type Result<T> = std::result::Result<T, RunTasksError>;

#[derive(Debug)]
pub enum RunTasksError {
    /// The command line was malformed. Carries the usage text to print.
    Usage(String),
    /// A field could not be openen/read.
    Io { path: PathBuf, source: io::Error },
    /// The YAML is not valid or doesn't match PipelineConfig.
    Yaml { path: PathBuf, source: serde_yaml::Error },
    /// The Yaml is well formed but semantically wrong.
    Validation(String),
    /// A pipeline version was already registered.
    DuplicateVersion(String),
    /// A command cold not be spawned.
    Spawn { command: String, source: io::Error },
    /// A command runned but exited with a non-zero status.
    CommandFailed { stage: String, command: String, code: Option<i32> },
}

impl fmt::Display for RunTasksError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage(msg) => write!(f, "{msg}"),
            Self::Io { path, source } => write!(f, "cannot read '{}': {source}", path.display()),
            Self::Yaml { path, source } => write!(f, "invalid YAML in '{}': {source}", path.display()),
            Self::Validation(msg) => write!(f, "invalid pipeline: {msg}"),
            Self::DuplicateVersion(v) => write!(f, "version '{v}' is already registered."),
            Self::Spawn { command, source } => write!(f, "could not start '{command}': {source}"),
            Self::CommandFailed { stage, command, code } => match code {
                Some(c) => write!(f, "stage '{stage}': `{command}` exited with code {c}"),
                None => write!(f, "stage '{stage}': `{command}` was terminated by a signal"),
            },
        }
    }
}

// Implementing std::error::Error for Box<dyn Error>
impl std::error::Error for RunTasksError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } | Self::Spawn { source , ..} => Some(source),
            Self::Yaml {source, .. } => Some(source),
            _ => None,
        }
    }
}