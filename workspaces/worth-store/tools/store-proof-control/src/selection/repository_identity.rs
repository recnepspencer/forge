use std::path::Path;
use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::evidence::{sha256_bytes, sha256_file};

use super::ProofProductUnavailable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryIdentity {
    pub source_revision: String,
    pub source_tree_digest: String,
    pub lockfile_digest: String,
    pub rustc_identity: String,
    pub operating_system: String,
    pub architecture: String,
}

pub fn observe_repository_identity(
    workspace_root: &Path,
) -> Result<RepositoryIdentity, ProofProductUnavailable> {
    let source_revision = command_output(workspace_root, "git", &["rev-parse", "HEAD"])?;
    let source_tree_digest = observe_source_tree_digest(workspace_root)?;
    let rustc_identity = command_output(workspace_root, "rustc", &["-Vv"])?;
    let lockfile_digest = sha256_file(&workspace_root.join("Cargo.lock"))
        .map_err(ProofProductUnavailable::RepositoryObservation)?;
    Ok(RepositoryIdentity {
        source_revision,
        source_tree_digest,
        lockfile_digest,
        rustc_identity,
        operating_system: std::env::consts::OS.to_owned(),
        architecture: std::env::consts::ARCH.to_owned(),
    })
}

fn observe_source_tree_digest(workspace_root: &Path) -> Result<String, ProofProductUnavailable> {
    let mut identity = command_bytes(
        workspace_root,
        "git",
        &["diff", "--binary", "HEAD", "--", "."],
    )?;
    let untracked = command_bytes(
        workspace_root,
        "git",
        &[
            "ls-files",
            "--others",
            "--exclude-standard",
            "-z",
            "--",
            ".",
        ],
    )?;
    for relative in untracked
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
    {
        let relative = String::from_utf8_lossy(relative);
        let path = workspace_root.join(relative.as_ref());
        identity.extend_from_slice(relative.as_bytes());
        identity.push(0);
        let bytes = std::fs::read(&path).map_err(|error| {
            ProofProductUnavailable::RepositoryObservation(format!(
                "could not read untracked source {}: {error}",
                path.display()
            ))
        })?;
        identity.extend_from_slice(&bytes);
        identity.push(0);
    }
    Ok(sha256_bytes(&identity))
}

fn command_output(
    current_dir: &Path,
    program: &str,
    arguments: &[&str],
) -> Result<String, ProofProductUnavailable> {
    let output = Command::new(program)
        .args(arguments)
        .current_dir(current_dir)
        .output()
        .map_err(|error| {
            ProofProductUnavailable::RepositoryObservation(format!(
                "could not launch {program}: {error}"
            ))
        })?;
    if !output.status.success() {
        return Err(ProofProductUnavailable::RepositoryObservation(format!(
            "{program} observation failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn command_bytes(
    current_dir: &Path,
    program: &str,
    arguments: &[&str],
) -> Result<Vec<u8>, ProofProductUnavailable> {
    let output = Command::new(program)
        .args(arguments)
        .current_dir(current_dir)
        .output()
        .map_err(|error| {
            ProofProductUnavailable::RepositoryObservation(format!(
                "could not launch {program}: {error}"
            ))
        })?;
    if !output.status.success() {
        return Err(ProofProductUnavailable::RepositoryObservation(format!(
            "{program} observation failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    Ok(output.stdout)
}
