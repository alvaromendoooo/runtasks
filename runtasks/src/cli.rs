//! Command line parsing

use crate::error::{Result, RunTasksError};
use std::path::PathBuf;

const USAGE: &str = "\
Usage:
    runtasks run <pipeline.yml> [--dry-run] execute the pipeline
    runtasks validate <pipeline.yml>        check it without running anything";

#[derive(Debug, PartialEq)]
pub enum Command {
    Run { file: PathBuf, dry_run: bool },
    Validate { file: PathBuf },
}

impl Command {
    /// Parses arguments excluding the program name.
    pub fn parse<I>(args: I) -> Result<Self>
    where 
        I: IntoIterator<Item = String>,
    {
        let usage = || RunTasksError::Usage(USAGE.to_string());
        let mut args = args.into_iter();

        let subcommand = args.next().ok_or_else(usage)?;
        let mut file = None;
        let mut dry_run = false;

        for arg in args {
            match arg.as_str() {
                "--dry-run" => dry_run = true,
                flag if flag.starts_with('-') => {
                    return Err(RunTasksError::Usage(format!(
                        "unknown option '{flag}'\n\n{USAGE}"
                    )));
                }
                _ if file.is_none() => file = Some(PathBuf::from(arg)),
                _ => return Err(usage()),
            }
        }

        let file = file.ok_or_else(usage)?;
        match subcommand.as_str() {
            "run" => Ok(Self::Run { file, dry_run }),
            "validate" if !dry_run => Ok(Self::Validate { file }),
            _ => Err(usage()),
        }
    }
}