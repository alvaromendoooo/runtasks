//! Data model (the "shape" of the YAML) and loading from disk.

use crate::error::{Result, RunTasksError};
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VersionControl {
    pub records: HashMap<String, PipelineConfig>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PipelineConfig {
    pub version: String,
    pub name: String,
    pub stages: Vec<Stage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Stage {
    pub name: String,
    pub commands: Vec<String>,
    // A missing key in the YAML becomes `None` for `Option` fields.
    pub depends_on: Option<Vec<String>>,
}

impl Stage {
    /// Dependencies as a slice; empty when the key is absent.
    /// Returning `&[String]` (a borrow) avoids cloning the Vec.
    pub fn dependencies(&self) -> &[String] {
        self.depends_on.as_deref().unwrap_or(&[])
    }
}

impl PipelineConfig {
    /// Reads and deserializes a pipeline file.
    ///
    /// `AsRef<Path>` accepts `&str`, `String`, `&Path`, `PathBuf`... so
    /// callers don't need to convert first (the old code needed `to_str()`).
    pub fn read_content(file_path: impl AsRef<Path>) -> Result<Self> {
        let path = file_path.as_ref();
        let file = File::open(path).map_err(|source| RunTasksError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        serde_yaml::from_reader(file).map_err(|source| RunTasksError::Yaml {
            path: path.to_path_buf(),
            source,
        })
    }
}

impl VersionControl {
    /// Stores a pipeline under a timestamp key, refusing repeated versions.
    pub fn register_version(&mut self, version: PipelineConfig) -> Result<()> {
        if self.records.values().any(|r| r.version == version.version) {
            return Err(RunTasksError::DuplicateVersion(version.version));
        }

        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        self.records.insert(timestamp, version);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config(version: &str) -> PipelineConfig {
        PipelineConfig {
            version: version.into(),
            ..Default::default()
        }
    }

    #[test]
    fn rejects_duplicate_versions() {
        let mut vc = VersionControl::default();
        assert!(vc.register_version(config("1.0")).is_ok());
        assert!(matches!(
            vc.register_version(config("1.0")),
            Err(RunTasksError::DuplicateVersion(_))
        ));
        assert!(vc.register_version(config("2.0")).is_ok());
    }

    #[test]
    fn missing_depends_on_is_empty() {
        let stage: Stage = serde_yaml::from_str("name: a\ncommands: [echo hi]").unwrap();
        assert!(stage.dependencies().is_empty());
    }
}
