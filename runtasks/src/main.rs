pub mod config;
pub mod parser;

use config::PipelineConfig;
use std::path::Path;

fn main() {
    let config_path = Path::new("../deploy_example.yml");
    let mut config: PipelineConfig = PipelineConfig::default();

    if let Some(path) = config_path.to_str() {
        config = PipelineConfig::read_content(path);
    } else {
        println!("Error stringifying file_path");
    }

    println!("Successfully loaded pipeline");
}