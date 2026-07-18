use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::evidence::sha256_serialized;

use super::{
    BuildArtifactClass, BuildArtifactInventory, BuildArtifactKind, BuildArtifactRetentionPolicy,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BuildArtifactCleanupPlan {
    schema_version: u32,
    plan_identity: String,
    inventory_identity: String,
    filesystem_identity: String,
    workspace_root: String,
    target_root: String,
    retention_policy: BuildArtifactRetentionPolicy,
    targets: Vec<BuildArtifactCleanupTarget>,
    protected_artifacts: Vec<ProtectedDiagnosticArtifact>,
    selected_file_count: usize,
    selected_directory_count: usize,
    selected_logical_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BuildArtifactCleanupTarget {
    relative_path: String,
    absolute_path: String,
    class: BuildArtifactClass,
    kind: BuildArtifactKind,
    logical_bytes: u64,
    modified_unix_nanos: u128,
    content_sha256: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtectedDiagnosticArtifact {
    relative_path: String,
    class: BuildArtifactClass,
    reason: String,
}

impl BuildArtifactCleanupPlan {
    pub fn lower(
        inventory: &BuildArtifactInventory,
        retention_policy: BuildArtifactRetentionPolicy,
    ) -> Result<Self, String> {
        inventory.validate_integrity()?;
        retention_policy.validate_integrity()?;
        if retention_policy.requires_reuse_basis() && inventory.reuse_basis().is_none() {
            return Err(format!(
                "retention policy {} requires a bound proof-run reuse basis",
                retention_policy.policy_name()
            ));
        }
        let mut targets = Vec::new();
        let mut protected_artifacts = Vec::new();
        for artifact in inventory.artifacts() {
            if artifact.protected() {
                protected_artifacts.push(ProtectedDiagnosticArtifact {
                    relative_path: artifact.relative_path().to_owned(),
                    class: artifact.class(),
                    reason: artifact
                        .protection_reason()
                        .unwrap_or("protected artifact")
                        .to_owned(),
                });
            } else if retention_policy.removes(artifact.class()) {
                targets.push(BuildArtifactCleanupTarget {
                    relative_path: artifact.relative_path().to_owned(),
                    absolute_path: artifact.absolute_path().to_owned(),
                    class: artifact.class(),
                    kind: artifact.kind(),
                    logical_bytes: artifact.logical_bytes(),
                    modified_unix_nanos: artifact.modified_unix_nanos(),
                    content_sha256: artifact.content_sha256().map(str::to_owned),
                });
            }
        }
        targets.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
        protected_artifacts.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
        let selected_file_count = targets
            .iter()
            .filter(|target| target.kind == BuildArtifactKind::File)
            .count();
        let selected_directory_count = targets.len() - selected_file_count;
        let selected_logical_bytes = targets.iter().map(|target| target.logical_bytes).sum();
        let mut plan = Self {
            schema_version: 1,
            plan_identity: String::new(),
            inventory_identity: inventory.inventory_identity().to_owned(),
            filesystem_identity: inventory.filesystem_identity().to_owned(),
            workspace_root: inventory.workspace_root().to_owned(),
            target_root: inventory.target_root().to_owned(),
            retention_policy,
            targets,
            protected_artifacts,
            selected_file_count,
            selected_directory_count,
            selected_logical_bytes,
        };
        plan.plan_identity = sha256_serialized(&plan)?;
        plan.validate_integrity()?;
        Ok(plan)
    }

    pub fn validate_integrity(&self) -> Result<(), String> {
        if self.schema_version != 1 {
            return Err(format!(
                "unsupported artifact cleanup plan schema {}",
                self.schema_version
            ));
        }
        self.retention_policy.validate_integrity()?;
        let mut basis = self.clone();
        basis.plan_identity.clear();
        if sha256_serialized(&basis)? != self.plan_identity {
            return Err("artifact cleanup plan identity does not match its contents".to_owned());
        }
        let target_root = Path::new(&self.target_root);
        if !target_root.is_absolute() {
            return Err("artifact cleanup plan target root is not absolute".to_owned());
        }
        for target in &self.targets {
            validate_relative_path(&target.relative_path)?;
            let expected = normalized(&target_root.join(&target.relative_path));
            if expected != target.absolute_path {
                return Err(format!(
                    "cleanup target absolute path differs from admitted root: {}",
                    target.absolute_path
                ));
            }
            if target.kind == BuildArtifactKind::File && target.content_sha256.is_none() {
                return Err(format!(
                    "cleanup file target omits content identity: {}",
                    target.relative_path
                ));
            }
        }
        Ok(())
    }

    pub fn output_path(&self) -> PathBuf {
        Path::new(&self.workspace_root)
            .join(".store-proof/evidence/artifacts/cleanup-plans")
            .join(format!("{}.json", self.plan_identity))
    }

    pub fn plan_identity(&self) -> &str {
        &self.plan_identity
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

    pub fn targets(&self) -> &[BuildArtifactCleanupTarget] {
        &self.targets
    }

    pub fn protected_artifacts(&self) -> &[ProtectedDiagnosticArtifact] {
        &self.protected_artifacts
    }

    pub const fn selected_file_count(&self) -> usize {
        self.selected_file_count
    }

    pub const fn selected_directory_count(&self) -> usize {
        self.selected_directory_count
    }

    pub const fn selected_logical_bytes(&self) -> u64 {
        self.selected_logical_bytes
    }
}

impl BuildArtifactCleanupTarget {
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

    pub fn content_sha256(&self) -> Option<&str> {
        self.content_sha256.as_deref()
    }
}

impl ProtectedDiagnosticArtifact {
    pub fn relative_path(&self) -> &str {
        &self.relative_path
    }

    pub const fn class(&self) -> BuildArtifactClass {
        self.class
    }
}

fn validate_relative_path(path: &str) -> Result<(), String> {
    let path = Path::new(path);
    if path.is_absolute()
        || path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir
                    | Component::CurDir
                    | Component::RootDir
                    | Component::Prefix(_)
            )
        })
    {
        return Err(format!(
            "cleanup target path is not confined: {}",
            path.display()
        ));
    }
    Ok(())
}

fn normalized(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
