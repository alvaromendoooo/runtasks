//! Library root. `mod` declares a module and tells the compile to load the
//! file with that name. `pub` makes it reachable from
//! outside the crate: runtasks::config::Stage from `main.rs`
//! 

pub mod config;
pub mod error;
pub mod parser;
pub mod cli;
pub mod executor;

use cli::Command;
use config::PipelineConfig;
use config::VersionControl;
use error::Result;

/// Top level entry point: everythin fallible returns `Result`, and the `?`
/// operator propagates errors up
pub fn run(command: Command) -> Result<()> {
    match command {
        Command::Validate { file } => {
            let config = PipelineConfig::read_content(&file)?;
            let order = parser::execution_order(&config)?;
            println!("'{}' is valid ({} stages).", config.name, order.len());
        }
        Command::Run { file, dry_run } => {
            let config = PipelineConfig::read_content(&file)?;
            let order = parser::execution_order(&config)?;

            println!("Pipeline '{}' (version {})", config.name, config.version);
            executor::run_stages(&order, dry_run)?;

            let mut history = VersionControl::default();
            history.register_version(config)?;
        }
    }
    Ok(())
}