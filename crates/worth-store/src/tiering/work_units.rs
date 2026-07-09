#![allow(dead_code)]

use serde::Serialize;

use super::{
    PlacementArtifactFamily, PlacementExecutionOrigin, PlacementObservationScopeClass,
    RecallCoalescingKey, TierResidenceClass,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuthoritativeTierMoveUnit {
    artifact_key: String,
    target_residence: TierResidenceClass,
}

impl AuthoritativeTierMoveUnit {
    pub(crate) fn new(
        artifact_key: impl Into<String>,
        target_residence: TierResidenceClass,
    ) -> Self {
        Self {
            artifact_key: artifact_key.into(),
            target_residence,
        }
    }

    pub fn artifact_key(&self) -> &str {
        &self.artifact_key
    }

    pub fn target_residence(&self) -> TierResidenceClass {
        self.target_residence
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedTierMoveUnit {
    artifact_family: PlacementArtifactFamily,
    artifact_id: String,
    target_residence: TierResidenceClass,
}

impl DerivedTierMoveUnit {
    pub(crate) fn new(
        artifact_family: PlacementArtifactFamily,
        artifact_id: impl Into<String>,
        target_residence: TierResidenceClass,
    ) -> Self {
        Self {
            artifact_family,
            artifact_id: artifact_id.into(),
            target_residence,
        }
    }

    pub fn artifact_family(&self) -> PlacementArtifactFamily {
        self.artifact_family
    }

    pub fn artifact_id(&self) -> &str {
        &self.artifact_id
    }

    pub fn target_residence(&self) -> TierResidenceClass {
        self.target_residence
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SnapshotRecallUnit {
    snapshot_id: crate::SnapshotId,
}

impl SnapshotRecallUnit {
    pub(crate) fn new(snapshot_id: crate::SnapshotId) -> Self {
        Self { snapshot_id }
    }

    pub fn snapshot_id(&self) -> crate::SnapshotId {
        self.snapshot_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeltaRecallUnit {
    branch_id: worth_relational::facade::history::BranchId,
}

impl DeltaRecallUnit {
    pub(crate) fn new(branch_id: worth_relational::facade::history::BranchId) -> Self {
        Self { branch_id }
    }

    pub fn branch_id(&self) -> &worth_relational::facade::history::BranchId {
        &self.branch_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LayoutFamilyRecallUnit {
    family_label: PlacementArtifactFamily,
}

impl LayoutFamilyRecallUnit {
    pub(crate) fn new(family_label: PlacementArtifactFamily) -> Self {
        Self { family_label }
    }

    pub fn family_label(&self) -> PlacementArtifactFamily {
        self.family_label
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlacementObservationUnit {
    scope_class: PlacementObservationScopeClass,
    scope_key: String,
    execution_origin: PlacementExecutionOrigin,
}

impl PlacementObservationUnit {
    pub(crate) fn new(
        scope_class: PlacementObservationScopeClass,
        scope_key: impl Into<String>,
        execution_origin: PlacementExecutionOrigin,
    ) -> Self {
        Self {
            scope_class,
            scope_key: scope_key.into(),
            execution_origin,
        }
    }

    pub fn scope_class(&self) -> PlacementObservationScopeClass {
        self.scope_class
    }

    pub fn scope_key(&self) -> &str {
        &self.scope_key
    }

    pub fn execution_origin(&self) -> PlacementExecutionOrigin {
        self.execution_origin
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FamilyLocalRecallUnit {
    family_label: PlacementArtifactFamily,
    coalescing_key: RecallCoalescingKey,
}

impl FamilyLocalRecallUnit {
    pub(crate) fn new(
        family_label: PlacementArtifactFamily,
        coalescing_key: RecallCoalescingKey,
    ) -> Self {
        Self {
            family_label,
            coalescing_key,
        }
    }

    pub fn family_label(&self) -> PlacementArtifactFamily {
        self.family_label
    }

    pub fn coalescing_key(&self) -> &RecallCoalescingKey {
        &self.coalescing_key
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SchedulerPlacementWorkToken {
    work_label: String,
    execution_origin: PlacementExecutionOrigin,
}

impl SchedulerPlacementWorkToken {
    pub(crate) fn new(
        work_label: impl Into<String>,
        execution_origin: PlacementExecutionOrigin,
    ) -> Self {
        Self {
            work_label: work_label.into(),
            execution_origin,
        }
    }

    pub fn work_label(&self) -> &str {
        &self.work_label
    }

    pub fn execution_origin(&self) -> PlacementExecutionOrigin {
        self.execution_origin
    }
}
