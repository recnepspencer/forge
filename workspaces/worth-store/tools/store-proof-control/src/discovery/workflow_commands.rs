use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};

use super::cargo_surface::normalized;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorkflowProofCommand {
    pub workflow_path: String,
    pub source_line: usize,
    pub command: String,
}

pub(super) fn discover_workflow_commands(
    workspace_root: &Path,
) -> Result<Vec<WorkflowProofCommand>, String> {
    let repository_root = repository_root(workspace_root)?;
    let workflows = repository_root.join(".github/workflows");
    if !workflows.exists() {
        return Ok(Vec::new());
    }
    let mut paths: Vec<_> = fs::read_dir(&workflows)
        .map_err(|error| format!("could not inspect {}: {error}", workflows.display()))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| matches!(extension, "yml" | "yaml"))
        })
        .collect();
    paths.sort();
    let mut commands = Vec::new();
    for path in paths {
        commands.extend(commands_in_workflow(&path)?);
    }
    commands.sort();
    Ok(commands)
}

fn commands_in_workflow(path: &Path) -> Result<Vec<WorkflowProofCommand>, String> {
    let source = fs::read_to_string(path)
        .map_err(|error| format!("could not read {}: {error}", path.display()))?;
    let lines: Vec<_> = source.lines().collect();
    let mut commands = Vec::new();
    let mut index = 0;
    while index < lines.len() {
        let line = lines[index];
        let trimmed = line.trim_start();
        let Some(run) = trimmed.strip_prefix("run:") else {
            index += 1;
            continue;
        };
        let indentation = line.len() - trimmed.len();
        let source_line = index + 1;
        let mut command = run.trim().trim_matches('"').to_owned();
        if matches!(command.as_str(), "|" | "|-" | ">" | ">-") {
            command.clear();
            index += 1;
            while index < lines.len() {
                let continuation = lines[index];
                if !continuation.trim().is_empty()
                    && continuation.len() - continuation.trim_start().len() <= indentation
                {
                    break;
                }
                if !continuation.trim().is_empty() {
                    if !command.is_empty() {
                        command.push(' ');
                    }
                    command.push_str(continuation.trim());
                }
                index += 1;
            }
        } else {
            index += 1;
        }
        if is_worth_store_command(&command) {
            commands.push(WorkflowProofCommand {
                workflow_path: normalized(path),
                source_line,
                command,
            });
        }
    }
    Ok(commands)
}

fn is_worth_store_command(command: &str) -> bool {
    command.contains("workspaces/worth-store") || command.contains("store-ci")
}

fn repository_root(workspace_root: &Path) -> Result<PathBuf, String> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(workspace_root)
        .output()
        .map_err(|error| format!("could not locate repository root: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "git could not locate repository root: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(PathBuf::from(
        String::from_utf8_lossy(&output.stdout).trim(),
    ))
}
