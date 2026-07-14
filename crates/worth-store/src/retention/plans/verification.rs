#![allow(dead_code)]

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RetentionTargetStateVerification {
    family_label: String,
    target_id: String,
    expected_present: bool,
    observed_present: bool,
}

impl RetentionTargetStateVerification {
    pub(crate) fn new(
        family_label: impl Into<String>,
        target_id: impl Into<String>,
        expected_present: bool,
        observed_present: bool,
    ) -> Self {
        Self {
            family_label: family_label.into(),
            target_id: target_id.into(),
            expected_present,
            observed_present,
        }
    }

    pub fn family_label(&self) -> &str {
        &self.family_label
    }

    pub fn target_id(&self) -> &str {
        &self.target_id
    }

    pub fn expected_present(&self) -> bool {
        self.expected_present
    }

    pub fn observed_present(&self) -> bool {
        self.observed_present
    }

    pub fn matches_expectation(&self) -> bool {
        self.expected_present == self.observed_present
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RetentionMaintenanceVerification {
    operation_label: String,
    truth_digest: String,
    restore_digest: String,
    restore_truth_parity: bool,
    target_state: Option<RetentionTargetStateVerification>,
}

impl RetentionMaintenanceVerification {
    pub(crate) fn new(
        operation_label: impl Into<String>,
        truth_digest: impl Into<String>,
        restore_digest: impl Into<String>,
        restore_truth_parity: bool,
        target_state: Option<RetentionTargetStateVerification>,
    ) -> Self {
        Self {
            operation_label: operation_label.into(),
            truth_digest: truth_digest.into(),
            restore_digest: restore_digest.into(),
            restore_truth_parity,
            target_state,
        }
    }

    pub fn operation_label(&self) -> &str {
        &self.operation_label
    }

    pub fn truth_digest(&self) -> &str {
        &self.truth_digest
    }

    pub fn restore_digest(&self) -> &str {
        &self.restore_digest
    }

    pub fn restore_truth_parity(&self) -> bool {
        self.restore_truth_parity
    }

    pub fn target_state(&self) -> Option<&RetentionTargetStateVerification> {
        self.target_state.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReclaimExecutionReport {
    reclaim_unit: crate::DerivedFamilyReclaimUnit,
    rebuild_unit: crate::RetainedRangeRebuildUnit,
    deleted_artifact_count: u64,
    cost_surface: crate::RetainedReadCostSurface,
    verification: RetentionMaintenanceVerification,
}

impl ReclaimExecutionReport {
    pub(crate) fn new(
        reclaim_unit: crate::DerivedFamilyReclaimUnit,
        rebuild_unit: crate::RetainedRangeRebuildUnit,
        deleted_artifact_count: u64,
        cost_surface: crate::RetainedReadCostSurface,
        verification: RetentionMaintenanceVerification,
    ) -> Self {
        Self {
            reclaim_unit,
            rebuild_unit,
            deleted_artifact_count,
            cost_surface,
            verification,
        }
    }

    pub fn reclaim_unit(&self) -> &crate::DerivedFamilyReclaimUnit {
        &self.reclaim_unit
    }

    pub fn rebuild_unit(&self) -> &crate::RetainedRangeRebuildUnit {
        &self.rebuild_unit
    }

    pub fn deleted_artifact_count(&self) -> u64 {
        self.deleted_artifact_count
    }

    pub fn cost_surface(&self) -> &crate::RetainedReadCostSurface {
        &self.cost_surface
    }

    pub fn verification(&self) -> &RetentionMaintenanceVerification {
        &self.verification
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuthoritativeReclaimReport {
    reclaim_unit: crate::AuthoritativeRangeReclaimUnit,
    deleted_artifact_count: u64,
    cost_surface: crate::RetainedReadCostSurface,
    verification: RetentionMaintenanceVerification,
}

impl AuthoritativeReclaimReport {
    pub(crate) fn new(
        reclaim_unit: crate::AuthoritativeRangeReclaimUnit,
        deleted_artifact_count: u64,
        cost_surface: crate::RetainedReadCostSurface,
        verification: RetentionMaintenanceVerification,
    ) -> Self {
        Self {
            reclaim_unit,
            deleted_artifact_count,
            cost_surface,
            verification,
        }
    }

    pub fn reclaim_unit(&self) -> &crate::AuthoritativeRangeReclaimUnit {
        &self.reclaim_unit
    }

    pub fn deleted_artifact_count(&self) -> u64 {
        self.deleted_artifact_count
    }

    pub fn cost_surface(&self) -> &crate::RetainedReadCostSurface {
        &self.cost_surface
    }

    pub fn verification(&self) -> &RetentionMaintenanceVerification {
        &self.verification
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RetainedRangeRebuildReport {
    rebuild_unit: crate::RetainedRangeRebuildUnit,
    rebuilt_artifact_count: u64,
    cost_surface: crate::RetainedReadCostSurface,
    verification: RetentionMaintenanceVerification,
}

impl RetainedRangeRebuildReport {
    pub(crate) fn new(
        rebuild_unit: crate::RetainedRangeRebuildUnit,
        rebuilt_artifact_count: u64,
        cost_surface: crate::RetainedReadCostSurface,
        verification: RetentionMaintenanceVerification,
    ) -> Self {
        Self {
            rebuild_unit,
            rebuilt_artifact_count,
            cost_surface,
            verification,
        }
    }

    pub fn rebuild_unit(&self) -> &crate::RetainedRangeRebuildUnit {
        &self.rebuild_unit
    }

    pub fn rebuilt_artifact_count(&self) -> u64 {
        self.rebuilt_artifact_count
    }

    pub fn cost_surface(&self) -> &crate::RetainedReadCostSurface {
        &self.cost_surface
    }

    pub fn verification(&self) -> &RetentionMaintenanceVerification {
        &self.verification
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompactionCandidateRejection {
    family_label: String,
    artifact_id: Option<String>,
    reason: String,
}

impl CompactionCandidateRejection {
    pub(crate) fn new(
        family_label: impl Into<String>,
        artifact_id: Option<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            family_label: family_label.into(),
            artifact_id,
            reason: reason.into(),
        }
    }

    pub fn family_label(&self) -> &str {
        &self.family_label
    }

    pub fn artifact_id(&self) -> Option<&str> {
        self.artifact_id.as_deref()
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RebuildDebtSummary {
    family_label: String,
    retained_basis_label: String,
    rebuild_target_id: String,
    debt_reason: String,
}

impl RebuildDebtSummary {
    pub(crate) fn new(
        family_label: impl Into<String>,
        retained_basis_label: impl Into<String>,
        rebuild_target_id: impl Into<String>,
        debt_reason: impl Into<String>,
    ) -> Self {
        Self {
            family_label: family_label.into(),
            retained_basis_label: retained_basis_label.into(),
            rebuild_target_id: rebuild_target_id.into(),
            debt_reason: debt_reason.into(),
        }
    }

    pub fn family_label(&self) -> &str {
        &self.family_label
    }

    pub fn retained_basis_label(&self) -> &str {
        &self.retained_basis_label
    }

    pub fn rebuild_target_id(&self) -> &str {
        &self.rebuild_target_id
    }

    pub fn debt_reason(&self) -> &str {
        &self.debt_reason
    }
}
