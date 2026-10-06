use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::fs::File;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VersionControl {
    pub records: HashMap<String, PipelineConfig>
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PipelineConfig {
    pub version: String,
    pub name: String,
    pub stages: Vec<Stage>
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Stage {
    pub name: String,
    pub commands: Vec<String>
}

impl PipelineConfig {
    pub fn read_content(file_path: &str) -> Self {
        let file = File::open(file_path).expect("Could not open this file.");
        let content_config: PipelineConfig =
            serde_yaml::from_reader(file).expect("Could not parse YAML content.");

        content_config
    }
}