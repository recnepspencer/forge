#![allow(dead_code)]

use forge_relational::facade::history::CommitId;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SnapshotCompactionUnit {
    retained_basis_label: String,
    snapshot_artifact_id: String,
}

impl SnapshotCompactionUnit {
    pub(crate) fn new(
        retained_basis_label: impl Into<String>,
        snapshot_artifact_id: impl Into<String>,
    ) -> Self {
        Self {
            retained_basis_label: retained_basis_label.into(),
            snapshot_artifact_id: snapshot_artifact_id.into(),
        }
    }

    pub fn retained_basis_label(&self) -> &str {
        &self.retained_basis_label
    }

    pub fn snapshot_artifact_id(&self) -> &str {
        &self.snapshot_artifact_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeltaLayerCompactionUnit {
    retained_basis_label: String,
    branch_delta_layer_id: String,
}

impl DeltaLayerCompactionUnit {
    pub(crate) fn new(
        retained_basis_label: impl Into<String>,
        branch_delta_layer_id: impl Into<String>,
    ) -> Self {
        Self {
            retained_basis_label: retained_basis_label.into(),
            branch_delta_layer_id: branch_delta_layer_id.into(),
        }
    }

    pub fn retained_basis_label(&self) -> &str {
        &self.retained_basis_label
    }

    pub fn branch_delta_layer_id(&self) -> &str {
        &self.branch_delta_layer_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LayoutFamilyCompactionUnit {
    retained_basis_label: String,
    family_label: String,
    artifact_id: String,
}

impl LayoutFamilyCompactionUnit {
    pub(crate) fn new(
        retained_basis_label: impl Into<String>,
        family_label: impl Into<String>,
        artifact_id: impl Into<String>,
    ) -> Self {
        Self {
            retained_basis_label: retained_basis_label.into(),
            family_label: family_label.into(),
            artifact_id: artifact_id.into(),
        }
    }

    pub fn retained_basis_label(&self) -> &str {
        &self.retained_basis_label
    }

    pub fn family_label(&self) -> &str {
        &self.family_label
    }

    pub fn artifact_id(&self) -> &str {
        &self.artifact_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuthoritativeRangeReclaimUnit {
    branch_id: forge_relational::facade::history::BranchId,
    expired_commit_ids: Vec<CommitId>,
}

impl AuthoritativeRangeReclaimUnit {
    pub(crate) fn new(
        branch_id: forge_relational::facade::history::BranchId,
        expired_commit_ids: Vec<CommitId>,
    ) -> Self {
        Self {
            branch_id,
            expired_commit_ids,
        }
    }

    pub fn branch_id(&self) -> &forge_relational::facade::history::BranchId {
        &self.branch_id
    }

    pub fn expired_commit_ids(&self) -> &[CommitId] {
        &self.expired_commit_ids
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedFamilyReclaimUnit {
    retained_basis_label: String,
    family_label: String,
    artifact_id: String,
}

impl DerivedFamilyReclaimUnit {
    pub(crate) fn new(
        retained_basis_label: impl Into<String>,
        family_label: impl Into<String>,
        artifact_id: impl Into<String>,
    ) -> Self {
        Self {
            retained_basis_label: retained_basis_label.into(),
            family_label: family_label.into(),
            artifact_id: artifact_id.into(),
        }
    }

    pub fn retained_basis_label(&self) -> &str {
        &self.retained_basis_label
    }

    pub fn family_label(&self) -> &str {
        &self.family_label
    }

    pub fn artifact_id(&self) -> &str {
        &self.artifact_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RetainedRangeRebuildUnit {
    retained_basis_label: String,
    family_label: String,
    rebuild_target_id: String,
}

impl RetainedRangeRebuildUnit {
    pub(crate) fn new(
        retained_basis_label: impl Into<String>,
        family_label: impl Into<String>,
        rebuild_target_id: impl Into<String>,
    ) -> Self {
        Self {
            retained_basis_label: retained_basis_label.into(),
            family_label: family_label.into(),
            rebuild_target_id: rebuild_target_id.into(),
        }
    }

    pub fn retained_basis_label(&self) -> &str {
        &self.retained_basis_label
    }

    pub fn family_label(&self) -> &str {
        &self.family_label
    }

    pub fn rebuild_target_id(&self) -> &str {
        &self.rebuild_target_id
    }
}
