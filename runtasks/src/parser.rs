//! Turns a raw `PipelineConfig` into an ordered execution plan.
//! 
//! Steps: validate (name, dependencies).

use crate::config::{PipelineConfig, Stage};
use crate::error::{Result, RunTasksError};
use std::collections::{HashMap, HashSet};

/// Returns the stages in an order where every stage comes after its dependencies.
/// Stages without ordering constraints keep file order.
pub fn execution_order(config: &PipelineConfig) -> Result<Vec<&Stage>> {
    validate(config)?;

    let by_name: HashMap<&str, &Stage> = 
        config.stages.iter().map(|s| (s.name.as_str(), s)).collect();

    let mut done: HashSet<&str> = HashSet::new();
    let mut in_progress: Vec<&str> = Vec::new();
    let mut order = Vec::with_capacity(config.stages.len());

    for stage in &config.stages {
        visit(stage, &by_name, &mut done, &mut in_progress, &mut order)?;
    }
    Ok(order)
}

// Emit dependencies first, then the stage itself.
fn visit<'a>(
    stage: &'a Stage,
    by_name: &HashMap<&str, &'a Stage>,
    done: &mut HashSet<&'a str>,
    in_progress: &mut Vec<&'a str>,
    order: &mut Vec<&'a Stage>,
) -> Result<()> {
    let name = stage.name.as_str();
    if done.contains(name) {
        return Ok(());
    }
    // Check for dependency loops errors
    if let Some(pos) = in_progress.iter().position(|n| *n == name) {
        let cycle = in_progress[pos..]
            .iter()
            .copied()
            .chain(std::iter::once(name))
            .collect::<Vec<_>>()
            .join(" -> ");

        return Err(RunTasksError::Validation(format!(
            "dependency cycle: {}",
            cycle
        )));
    }

    in_progress.push(name);
    for dep in stage.dependencies() {
        visit(by_name[dep.as_str()], by_name, done, in_progress, order)?;
    }
    in_progress.pop();

    done.insert(name);
    order.push(stage);
    Ok(())
}

/// Checks the rules the type system can't express.
pub fn validate(config: &PipelineConfig) -> Result<()> {
    // Helper that improve the coding verbosity by replacing Err(RunTasksError::Validation("..."))
    // with invalid("...".into())
    // .into() in this case converts &str into String. From<A> for B.
    let invalid = |msg: String| Err(RunTasksError::Validation(msg));

    if config.stages.is_empty() { // No stages in pipeline, error.
        return invalid("the pipeline has no stages".into());
    }

    let mut seen = HashSet::new(); // Keeps track of stages name, controlling stages duplicated
    for stage in &config.stages {
        if !seen.insert(stage.name.as_str()) { // Returns false if the set already contains stage.name.
            return invalid(format!("duplicate stage name '{}'", stage.name)); // No need of .into(), format! creates a String.
        }
        if stage.commands.is_empty() { // Cannot process stages without commands
            return invalid(format!("stage '{}' has no commands", stage.name));
        }
    }

    // Depends_on pipeline stages validation
    for stage in &config.stages {
        for dep in stage.dependencies() {
            if dep == &stage.name {
                return invalid(format!("stage '{}' depends on itself", stage.name));
            }
            if !seen.contains(dep.as_str()) {
                return invalid(format!(
                    "stage '{}' depends on unknown stage '{dep}'",
                    stage.name
                ));
            }
        }
    }
    Ok(())
}