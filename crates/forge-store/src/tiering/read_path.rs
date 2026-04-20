#![allow(dead_code)]

use serde::Serialize;

use super::{
    ColdRecallTierPath, PlacementArtifactFamily, PlacementBudgetClass,
    PlacementExecutionOrigin, RecallAmplificationBudget, RecallCostClass, TierResidenceClass,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlacementBoundArtifactRef {
    artifact_family: PlacementArtifactFamily,
    artifact_id: String,
    retained_basis_label: Option<String>,
}

impl PlacementBoundArtifactRef {
    pub fn authoritative_branch_head(branch_id: impl Into<String>) -> Self {
        Self {
            artifact_family: PlacementArtifactFamily::AuthoritativeBranchHead,
            artifact_id: branch_id.into(),
            retained_basis_label: None,
        }
    }

    pub fn stable_basis(
        stable_basis_id: impl Into<String>,
        retained_basis_label: Option<String>,
    ) -> Self {
        Self {
            artifact_family: PlacementArtifactFamily::StableBasis,
            artifact_id: stable_basis_id.into(),
            retained_basis_label,
        }
    }

    pub fn snapshot_family(snapshot_id: impl Into<String>) -> Self {
        Self {
            artifact_family: PlacementArtifactFamily::SnapshotFamily,
            artifact_id: snapshot_id.into(),
            retained_basis_label: None,
        }
    }

    pub fn branch_delta_family(layer_id: impl Into<String>) -> Self {
        Self {
            artifact_family: PlacementArtifactFamily::BranchDeltaFamily,
            artifact_id: layer_id.into(),
            retained_basis_label: None,
        }
    }

    pub fn milestone6_layout_family(artifact_id: impl Into<String>) -> Self {
        Self {
            artifact_family: PlacementArtifactFamily::Milestone6LayoutFamily,
            artifact_id: artifact_id.into(),
            retained_basis_label: None,
        }
    }

    pub(crate) fn new(
        artifact_family: PlacementArtifactFamily,
        artifact_id: impl Into<String>,
        retained_basis_label: Option<String>,
    ) -> Self {
        Self {
            artifact_family,
            artifact_id: artifact_id.into(),
            retained_basis_label,
        }
    }

    pub fn artifact_family(&self) -> PlacementArtifactFamily {
        self.artifact_family
    }

    pub fn artifact_id(&self) -> &str {
        &self.artifact_id
    }

    pub fn retained_basis_label(&self) -> Option<&str> {
        self.retained_basis_label.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResidentReadLease {
    artifact_ref: PlacementBoundArtifactRef,
    residence_class: TierResidenceClass,
    budget_class: PlacementBudgetClass,
    execution_origin: PlacementExecutionOrigin,
}

impl ResidentReadLease {
    pub(crate) fn new(
        artifact_ref: PlacementBoundArtifactRef,
        residence_class: TierResidenceClass,
        budget_class: PlacementBudgetClass,
        execution_origin: PlacementExecutionOrigin,
    ) -> Self {
        Self {
            artifact_ref,
            residence_class,
            budget_class,
            execution_origin,
        }
    }

    pub fn artifact_ref(&self) -> &PlacementBoundArtifactRef {
        &self.artifact_ref
    }

    pub fn residence_class(&self) -> TierResidenceClass {
        self.residence_class
    }

    pub fn budget_class(&self) -> PlacementBudgetClass {
        self.budget_class
    }

    pub fn execution_origin(&self) -> PlacementExecutionOrigin {
        self.execution_origin
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ColdRecallLease {
    artifact_ref: PlacementBoundArtifactRef,
    recall_cost_class: RecallCostClass,
    amplification_budget: RecallAmplificationBudget,
    execution_origin: PlacementExecutionOrigin,
}

impl ColdRecallLease {
    pub(crate) fn new(
        artifact_ref: PlacementBoundArtifactRef,
        recall_cost_class: RecallCostClass,
        amplification_budget: RecallAmplificationBudget,
        execution_origin: PlacementExecutionOrigin,
    ) -> Self {
        Self {
            artifact_ref,
            recall_cost_class,
            amplification_budget,
            execution_origin,
        }
    }

    pub fn artifact_ref(&self) -> &PlacementBoundArtifactRef {
        &self.artifact_ref
    }

    pub fn recall_cost_class(&self) -> RecallCostClass {
        self.recall_cost_class
    }

    pub fn amplification_budget(&self) -> RecallAmplificationBudget {
        self.amplification_budget
    }

    pub fn execution_origin(&self) -> PlacementExecutionOrigin {
        self.execution_origin
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlacementResolvedReadHandle {
    artifact_ref: PlacementBoundArtifactRef,
    execution_origin: PlacementExecutionOrigin,
    resolved_path: ColdRecallTierPath,
}

impl PlacementResolvedReadHandle {
    pub(crate) fn from_resident_lease(lease: &ResidentReadLease) -> Self {
        let resolved_path = match lease.residence_class() {
            TierResidenceClass::Hot => ColdRecallTierPath::HotResident,
            TierResidenceClass::Warm => ColdRecallTierPath::WarmResident,
            TierResidenceClass::Cold => ColdRecallTierPath::ColdRecalled,
        };
        Self {
            artifact_ref: lease.artifact_ref().clone(),
            execution_origin: lease.execution_origin(),
            resolved_path,
        }
    }

    pub(crate) fn from_cold_recall_lease(lease: &ColdRecallLease) -> Self {
        Self {
            artifact_ref: lease.artifact_ref().clone(),
            execution_origin: lease.execution_origin(),
            resolved_path: ColdRecallTierPath::ColdRecalled,
        }
    }

    pub fn artifact_ref(&self) -> &PlacementBoundArtifactRef {
        &self.artifact_ref
    }

    pub fn execution_origin(&self) -> PlacementExecutionOrigin {
        self.execution_origin
    }

    pub fn resolved_path(&self) -> ColdRecallTierPath {
        self.resolved_path
    }
}
