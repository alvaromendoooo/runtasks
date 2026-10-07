//! Run stages by handing each command to the system shell.

use crate::config::Stage;
use crate::error::{Result, RunTasksError};
use std::process::Command;
use std::time::Instant;


/// Builds the OS-appropiate "run this string in a shell" command.
fn shell_command(line: &str) -> Command {
    if cfg!(windows) { // Evaluated at compiled time
        let mut cmd = Command::new("cmd");
        cmd.args(["/C", line]);
        cmd
    } else {
        let mut cmd = Command::new("sh");
        cmd.args(["-c", line]);
        cmd
    }
}

/// Runs every stage in order, stopping at the first failing command.
/// With 'dry_run' it only prints what would happend.
pub fn run_stages(stages: &[&Stage], dry_run: bool) -> Result<()> {
    let total = stages.len();
    let started = Instant::now();

    for (i, stage) in stages.iter().enumerate() {
        println!("\n==> [{}/{}] stage '{}'", i + 1, total, stage.name);
        let stage_start = Instant::now();

        for command in &stage.commands {
            println!("  $ {command}");
            if dry_run {
                continue;
            }

            let status = shell_command(command).status().map_err(|source| {
                RunTasksError::Spawn { 
                    command: command.clone(), 
                    source,
                }
            })?;

            if !status.success() {
                return Err(RunTasksError::CommandFailed { 
                    stage: stage.name.clone(), 
                    command: command.clone(), 
                    code: status.code(), 
                });
            }
        }
        println!("    ok ({:.1?})", stage_start.elapsed());
    }
    println!("\nPipeline finished in {:.1?}", started.elapsed());
    Ok(())
}