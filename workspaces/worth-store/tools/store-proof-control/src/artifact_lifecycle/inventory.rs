use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::evidence::sha256_serialized;
use crate::execution::ExecutedProofRun;

use super::inventory_observation::{filesystem_identity, observe_records};
use super::{AdmittedArtifactRoot, BuildArtifactClass, BuildArtifactKind};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BuildArtifactInventory {
    schema_version: u32,
    inventory_identity: String,
    filesystem_identity: String,
    workspace_root: String,
    target_root: String,
    reuse_basis: Option<BuildArtifactReuseBasis>,
    artifacts: Vec<BuildArtifactRecord>,
    file_count: usize,
    directory_count: usize,
    logical_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BuildArtifactReuseBasis {
    run_identity: String,
    plan_digest: String,
    artifact_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BuildArtifactRecord {
    pub(super) relative_path: String,
    pub(super) absolute_path: String,
    pub(super) class: BuildArtifactClass,
    pub(super) kind: BuildArtifactKind,
    pub(super) logical_bytes: u64,
    pub(super) modified_unix_nanos: u128,
    pub(super) content_sha256: Option<String>,
    pub(super) protected: bool,
    pub(super) protection_reason: Option<String>,
}

impl BuildArtifactInventory {
    pub fn inspect(
        workspace_root: &Path,
        target_root: &Path,
        protected_run: Option<&ExecutedProofRun>,
    ) -> Result<Self, String> {
        let admitted = AdmittedArtifactRoot::admit(workspace_root, target_root)?;
        let reuse = protected_run
            .map(|run| reuse_basis(admitted.workspace_root(), admitted.target_root(), run))
            .transpose()?;
        Self::lower(admitted, reuse)
    }

    fn lower(
        admitted: AdmittedArtifactRoot,
        reuse: Option<BuildArtifactReuseBasis>,
    ) -> Result<Self, String> {
        let current_paths: BTreeSet<_> = reuse
            .as_ref()
            .into_iter()
            .flat_map(|basis| &basis.artifact_paths)
            .cloned()
            .collect();
        let mut artifacts = observe_records(&admitted, &current_paths, reuse.is_some())?;
        artifacts.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
        let filesystem_identity = filesystem_identity(admitted.target_root(), &artifacts)?;
        let file_count = artifacts
            .iter()
            .filter(|artifact| artifact.kind == BuildArtifactKind::File)
            .count();
        let directory_count = artifacts.len() - file_count;
        let logical_bytes = artifacts
            .iter()
            .map(|artifact| artifact.logical_bytes)
            .sum();
        let mut inventory = Self {
            schema_version: 1,
            inventory_identity: String::new(),
            filesystem_identity,
            workspace_root: normalized(admitted.workspace_root()),
            target_root: normalized(admitted.target_root()),
            reuse_basis: reuse,
            artifacts,
            file_count,
            directory_count,
            logical_bytes,
        };
        inventory.inventory_identity = sha256_serialized(&inventory)?;
        Ok(inventory)
    }

    #[cfg(test)]
    pub(crate) fn inspect_with_test_reuse_paths(
        workspace_root: &Path,
        target_root: &Path,
        paths: &[PathBuf],
    ) -> Result<Self, String> {
        let admitted = AdmittedArtifactRoot::admit(workspace_root, target_root)?;
        let mut artifact_paths = BTreeSet::new();
        for path in paths {
            let canonical = path.canonicalize().map_err(|error| {
                format!(
                    "could not resolve test reuse path {}: {error}",
                    path.display()
                )
            })?;
            if !canonical.starts_with(admitted.target_root()) {
                return Err(format!(
                    "test reuse path escaped admitted root: {}",
                    canonical.display()
                ));
            }
            artifact_paths.insert(normalized(&canonical));
        }
        Self::lower(
            admitted,
            Some(BuildArtifactReuseBasis {
                run_identity: "artifact-lifecycle-test-run".to_owned(),
                plan_digest: "artifact-lifecycle-test-plan".to_owned(),
                artifact_paths: artifact_paths.into_iter().collect(),
            }),
        )
    }

    pub fn validate_integrity(&self) -> Result<(), String> {
        if self.schema_version != 1 {
            return Err(format!(
                "unsupported build artifact inventory schema {}",
                self.schema_version
            ));
        }
        let mut basis = self.clone();
        basis.inventory_identity.clear();
        if sha256_serialized(&basis)? != self.inventory_identity {
            return Err("build artifact inventory identity does not match its contents".to_owned());
        }
        Ok(())
    }

    pub fn inventory_identity(&self) -> &str {
        &self.inventory_identity
    }

    pub fn filesystem_identity(&self) -> &str {
        &self.filesystem_identity
    }

    pub fn workspace_root(&self) -> &str {
        &self.workspace_root
    }

    pub fn target_root(&self) -> &str {
        &self.target_root
    }

    pub fn reuse_basis(&self) -> Option<&BuildArtifactReuseBasis> {
        self.reuse_basis.as_ref()
    }

    pub fn artifacts(&self) -> &[BuildArtifactRecord] {
        &self.artifacts
    }

    pub fn output_path(&self) -> PathBuf {
        Path::new(&self.workspace_root)
            .join(".store-proof/evidence/artifacts/inventories")
            .join(format!("{}.json", self.inventory_identity))
    }

    pub const fn file_count(&self) -> usize {
        self.file_count
    }

    pub const fn directory_count(&self) -> usize {
        self.directory_count
    }

    pub const fn logical_bytes(&self) -> u64 {
        self.logical_bytes
    }
}

impl BuildArtifactReuseBasis {
    pub fn run_identity(&self) -> &str {
        &self.run_identity
    }
}

impl BuildArtifactRecord {
    pub fn relative_path(&self) -> &str {
        &self.relative_path
    }

    pub fn absolute_path(&self) -> &str {
        &self.absolute_path
    }

    pub const fn class(&self) -> BuildArtifactClass {
        self.class
    }

    pub const fn kind(&self) -> BuildArtifactKind {
        self.kind
    }

    pub const fn logical_bytes(&self) -> u64 {
        self.logical_bytes
    }

    pub const fn modified_unix_nanos(&self) -> u128 {
        self.modified_unix_nanos
    }

    pub fn content_sha256(&self) -> Option<&str> {
        self.content_sha256.as_deref()
    }

    pub const fn protected(&self) -> bool {
        self.protected
    }

    pub fn protection_reason(&self) -> Option<&str> {
        self.protection_reason.as_deref()
    }
}

fn reuse_basis(
    workspace_root: &Path,
    target_root: &Path,
    run: &ExecutedProofRun,
) -> Result<BuildArtifactReuseBasis, String> {
    let mut artifact_paths = BTreeSet::new();
    for artifact in run
        .attempts
        .iter()
        .flat_map(|attempt| &attempt.observed_cargo_artifacts)
    {
        for path in artifact.filenames.iter().chain(artifact.executable.iter()) {
            let path = PathBuf::from(path);
            let path = if path.is_absolute() {
                path
            } else {
                workspace_root.join(path)
            };
            if path.exists() {
                let canonical = path.canonicalize().map_err(|error| {
                    format!(
                        "could not resolve current artifact {}: {error}",
                        path.display()
                    )
                })?;
                if canonical.starts_with(target_root) {
                    artifact_paths.insert(normalized(&canonical));
                }
            }
        }
    }
    Ok(BuildArtifactReuseBasis {
        run_identity: run.run_identity.clone(),
        plan_digest: run.plan_digest.clone(),
        artifact_paths: artifact_paths.into_iter().collect(),
    })
}

fn normalized(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
