use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::evidence::{sha256_file, sha256_serialized, write_new_json};

use super::inventory_observation::current_filesystem_identity;
use super::{
    AdmittedArtifactRoot, BuildArtifactCleanupPlan, BuildArtifactCleanupTarget, BuildArtifactKind,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BuildArtifactCleanupReceipt {
    schema_version: u32,
    receipt_identity: String,
    plan_identity: String,
    inventory_identity: String,
    target_root: String,
    started_unix_millis: u128,
    completed_unix_millis: u128,
    outcome: BuildArtifactCleanupOutcome,
    deleted_targets: Vec<BuildArtifactCleanupTarget>,
    remaining_targets: Vec<BuildArtifactCleanupTarget>,
    protected_artifact_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum BuildArtifactCleanupOutcome {
    Completed,
    PartiallyApplied { failure: String },
}

impl BuildArtifactCleanupReceipt {
    pub fn execute(plan: &BuildArtifactCleanupPlan) -> Result<Self, String> {
        plan.validate_integrity()?;
        let workspace_root = Path::new(plan.workspace_root());
        let target_root = Path::new(plan.target_root());
        let admitted = AdmittedArtifactRoot::admit(workspace_root, target_root)?;
        let observed_identity =
            current_filesystem_identity(admitted.workspace_root(), admitted.target_root())?;
        if observed_identity != plan.filesystem_identity() {
            return Err(format!(
                "artifact root changed after planning: planned={} observed={observed_identity}",
                plan.filesystem_identity()
            ));
        }
        validate_targets(plan)?;
        let started_unix_millis = unix_millis()?;
        let mut ordered_targets = plan.targets().to_vec();
        ordered_targets.sort_by(|left, right| execution_order(left, right));
        let mut deleted_targets = Vec::new();
        let mut failure = None;
        for target in &ordered_targets {
            let result = match target.kind() {
                BuildArtifactKind::File => std::fs::remove_file(target.absolute_path()),
                BuildArtifactKind::Directory => std::fs::remove_dir(target.absolute_path()),
            };
            match result {
                Ok(()) => deleted_targets.push(target.clone()),
                Err(error) => {
                    failure = Some(format!(
                        "could not delete planned {:?} {}: {error}",
                        target.kind(),
                        target.absolute_path()
                    ));
                    break;
                }
            }
        }
        let deleted_paths: std::collections::BTreeSet<_> = deleted_targets
            .iter()
            .map(|target| target.relative_path())
            .collect();
        let remaining_targets = ordered_targets
            .into_iter()
            .filter(|target| !deleted_paths.contains(target.relative_path()))
            .collect();
        let outcome = failure.map_or(BuildArtifactCleanupOutcome::Completed, |failure| {
            BuildArtifactCleanupOutcome::PartiallyApplied { failure }
        });
        let mut receipt = Self {
            schema_version: 1,
            receipt_identity: String::new(),
            plan_identity: plan.plan_identity().to_owned(),
            inventory_identity: plan.inventory_identity().to_owned(),
            target_root: plan.target_root().to_owned(),
            started_unix_millis,
            completed_unix_millis: unix_millis()?,
            outcome,
            deleted_targets,
            remaining_targets,
            protected_artifact_paths: plan
                .protected_artifacts()
                .iter()
                .map(|artifact| artifact.relative_path().to_owned())
                .collect(),
        };
        receipt.receipt_identity = sha256_serialized(&receipt)?;
        write_new_json(&receipt.output_path(workspace_root), &receipt)?;
        Ok(receipt)
    }

    pub fn validate_integrity(&self) -> Result<(), String> {
        if self.schema_version != 1 {
            return Err(format!(
                "unsupported artifact cleanup receipt schema {}",
                self.schema_version
            ));
        }
        let mut basis = self.clone();
        basis.receipt_identity.clear();
        if sha256_serialized(&basis)? != self.receipt_identity {
            return Err("artifact cleanup receipt identity does not match its contents".to_owned());
        }
        Ok(())
    }

    pub fn output_path(&self, workspace_root: &Path) -> PathBuf {
        workspace_root
            .join(".store-proof/evidence/artifacts/cleanup-receipts")
            .join(format!("{}.json", self.receipt_identity))
    }

    pub fn receipt_identity(&self) -> &str {
        &self.receipt_identity
    }

    pub const fn outcome(&self) -> &BuildArtifactCleanupOutcome {
        &self.outcome
    }

    pub fn deleted_targets(&self) -> &[BuildArtifactCleanupTarget] {
        &self.deleted_targets
    }

    pub fn remaining_targets(&self) -> &[BuildArtifactCleanupTarget] {
        &self.remaining_targets
    }

    pub fn protected_artifact_paths(&self) -> &[String] {
        &self.protected_artifact_paths
    }
}

fn validate_targets(plan: &BuildArtifactCleanupPlan) -> Result<(), String> {
    for target in plan.targets() {
        let path = Path::new(target.absolute_path());
        let metadata = std::fs::symlink_metadata(path).map_err(|error| {
            format!(
                "could not inspect planned target {}: {error}",
                path.display()
            )
        })?;
        if metadata.file_type().is_symlink() {
            return Err(format!(
                "cleanup execution denies symlink or junction {}",
                path.display()
            ));
        }
        let observed_kind = if metadata.is_dir() {
            BuildArtifactKind::Directory
        } else {
            BuildArtifactKind::File
        };
        if observed_kind != target.kind() {
            return Err(format!(
                "cleanup target kind changed after planning: {}",
                path.display()
            ));
        }
        if let Some(expected) = target.content_sha256() {
            let observed = sha256_file(path)?;
            if observed != expected {
                return Err(format!(
                    "cleanup target content changed after planning: {}",
                    path.display()
                ));
            }
        }
    }
    Ok(())
}

fn execution_order(
    left: &BuildArtifactCleanupTarget,
    right: &BuildArtifactCleanupTarget,
) -> std::cmp::Ordering {
    left.kind()
        .cmp(&right.kind())
        .then_with(|| {
            right
                .relative_path()
                .matches('/')
                .count()
                .cmp(&left.relative_path().matches('/').count())
        })
        .then_with(|| left.relative_path().cmp(right.relative_path()))
}

fn unix_millis() -> Result<u128, String> {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .map_err(|error| format!("system clock precedes Unix epoch: {error}"))
}
