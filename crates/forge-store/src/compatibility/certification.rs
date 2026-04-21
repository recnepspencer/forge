use std::collections::BTreeSet;

use serde::Serialize;

use super::admission::{
    CompatibilityAdmissionCounters, CompatibilityReadAdmissionOutcome, CompatibilityRejection,
    CompatibilityRejectionKind, CompatibilityRelation, CompatibilityWriteAdmissionOutcome,
};
use super::derived::{DerivedLaneCompatibilityPlan, DerivedLaneCompatibilityPosture};
use super::manifests::{ArtifactFamilyId, ArtifactSemanticVersion};
use super::restore::{DisasterRecoveryCompatibilityPlan, RestoreCompatibilityPlan};
use super::rolling::RollingUpgradeAdmissionPlan;
use crate::evidence::Milestone12AdmissionReport;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub enum Milestone12CertificationLaneKind {
    CatalogCompleteness,
    AuthoritativeNativeRead,
    AuthoritativeForwardRead,
    AuthoritativeBackwardRead,
    AuthoritativeMissingEdgeRejected,
    AuthoritativeIncompatibleEdgeRejected,
    DerivedSnapshotReuseAccepted,
    DerivedLayoutBasisRejected,
    DerivedBulkResumeRejected,
    TierManifestNonAuthorityPreserved,
    RollingTwoCapabilityAdmitted,
    RollingMultiWriterRejected,
    RollingMissingEdgeRejected,
    RollingAdapterEdgeRejected,
    RestoreScopedBackupAdmitted,
    RestoreOutOfScopeRejected,
    RestorePublicationConflictRejected,
    RestoreMissingEdgeRejected,
    DisasterRecoveryTruthWindow,
    DisasterRecoveryDerivedWindow,
}

impl Milestone12CertificationLaneKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::CatalogCompleteness => "catalog_completeness",
            Self::AuthoritativeNativeRead => "authoritative_native_read",
            Self::AuthoritativeForwardRead => "authoritative_forward_read",
            Self::AuthoritativeBackwardRead => "authoritative_backward_read",
            Self::AuthoritativeMissingEdgeRejected => "authoritative_missing_edge_rejected",
            Self::AuthoritativeIncompatibleEdgeRejected => {
                "authoritative_incompatible_edge_rejected"
            }
            Self::DerivedSnapshotReuseAccepted => "derived_snapshot_reuse_accepted",
            Self::DerivedLayoutBasisRejected => "derived_layout_basis_rejected",
            Self::DerivedBulkResumeRejected => "derived_bulk_resume_rejected",
            Self::TierManifestNonAuthorityPreserved => "tier_manifest_non_authority_preserved",
            Self::RollingTwoCapabilityAdmitted => "rolling_two_capability_admitted",
            Self::RollingMultiWriterRejected => "rolling_multi_writer_rejected",
            Self::RollingMissingEdgeRejected => "rolling_missing_edge_rejected",
            Self::RollingAdapterEdgeRejected => "rolling_adapter_edge_rejected",
            Self::RestoreScopedBackupAdmitted => "restore_scoped_backup_admitted",
            Self::RestoreOutOfScopeRejected => "restore_out_of_scope_rejected",
            Self::RestorePublicationConflictRejected => "restore_publication_conflict_rejected",
            Self::RestoreMissingEdgeRejected => "restore_missing_edge_rejected",
            Self::DisasterRecoveryTruthWindow => "disaster_recovery_truth_window",
            Self::DisasterRecoveryDerivedWindow => "disaster_recovery_derived_window",
        }
    }

    pub fn lane_id(self) -> Milestone12CertificationLaneId {
        Milestone12CertificationLaneId::new(self.label())
    }

    pub const fn mandatory_phase_5a() -> &'static [Self] {
        &[
            Self::CatalogCompleteness,
            Self::AuthoritativeNativeRead,
            Self::AuthoritativeForwardRead,
            Self::AuthoritativeBackwardRead,
            Self::AuthoritativeMissingEdgeRejected,
            Self::AuthoritativeIncompatibleEdgeRejected,
            Self::DerivedSnapshotReuseAccepted,
            Self::DerivedLayoutBasisRejected,
            Self::DerivedBulkResumeRejected,
            Self::TierManifestNonAuthorityPreserved,
            Self::RollingTwoCapabilityAdmitted,
            Self::RollingMultiWriterRejected,
            Self::RollingMissingEdgeRejected,
            Self::RollingAdapterEdgeRejected,
            Self::RestoreScopedBackupAdmitted,
            Self::RestoreOutOfScopeRejected,
            Self::RestorePublicationConflictRejected,
            Self::RestoreMissingEdgeRejected,
            Self::DisasterRecoveryTruthWindow,
            Self::DisasterRecoveryDerivedWindow,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Milestone12CertificationLaneId(String);

impl Milestone12CertificationLaneId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12CertificationLaneInput {
    family_id: ArtifactFamilyId,
    source_semantic_version: ArtifactSemanticVersion,
    target_semantic_version: ArtifactSemanticVersion,
    expected_relation: Option<CompatibilityRelation>,
    expected_rejection_kind: Option<CompatibilityRejectionKind>,
}

impl Milestone12CertificationLaneInput {
    pub fn new(
        family_id: ArtifactFamilyId,
        source_semantic_version: ArtifactSemanticVersion,
        target_semantic_version: ArtifactSemanticVersion,
        expected_relation: Option<CompatibilityRelation>,
        expected_rejection_kind: Option<CompatibilityRejectionKind>,
    ) -> Self {
        Self {
            family_id,
            source_semantic_version,
            target_semantic_version,
            expected_relation,
            expected_rejection_kind,
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Milestone12CertificationLaneStatus {
    Accepted,
    Rejected,
    Invalidated,
    RebuildRequired,
    EvidenceOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Milestone12CertificationLaneRejection {
    DuplicateLane,
    MissingMandatoryLane,
    MatrixLaneMismatch,
    OutcomeStatusMismatch,
    CounterEvidenceMissing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12CertificationLaneOutcome {
    lane_id: Milestone12CertificationLaneId,
    lane_kind: Milestone12CertificationLaneKind,
    input: Milestone12CertificationLaneInput,
    status: Milestone12CertificationLaneStatus,
    relation: Option<CompatibilityRelation>,
    rejection_kind: Option<CompatibilityRejectionKind>,
    counters: Milestone12AdmissionReport,
}

impl Milestone12CertificationLaneOutcome {
    pub(crate) fn accepted(
        lane_kind: Milestone12CertificationLaneKind,
        input: Milestone12CertificationLaneInput,
        relation: CompatibilityRelation,
        counters: &CompatibilityAdmissionCounters,
    ) -> Self {
        Self {
            lane_id: lane_kind.lane_id(),
            lane_kind,
            input,
            status: Milestone12CertificationLaneStatus::Accepted,
            relation: Some(relation),
            rejection_kind: None,
            counters: Milestone12AdmissionReport::from_admission_counters(counters),
        }
    }

    pub(crate) fn rejected(
        lane_kind: Milestone12CertificationLaneKind,
        input: Milestone12CertificationLaneInput,
        rejection_kind: CompatibilityRejectionKind,
        counters: &CompatibilityAdmissionCounters,
    ) -> Self {
        Self {
            lane_id: lane_kind.lane_id(),
            lane_kind,
            input,
            status: Milestone12CertificationLaneStatus::Rejected,
            relation: None,
            rejection_kind: Some(rejection_kind),
            counters: Milestone12AdmissionReport::from_admission_counters(counters),
        }
    }

    pub(crate) fn non_admitted(
        lane_kind: Milestone12CertificationLaneKind,
        input: Milestone12CertificationLaneInput,
        status: Milestone12CertificationLaneStatus,
        counters: &CompatibilityAdmissionCounters,
    ) -> Self {
        Self {
            lane_id: lane_kind.lane_id(),
            lane_kind,
            input,
            status,
            relation: None,
            rejection_kind: None,
            counters: Milestone12AdmissionReport::from_admission_counters(counters),
        }
    }

    pub fn from_read_outcome(
        lane_kind: Milestone12CertificationLaneKind,
        input: Milestone12CertificationLaneInput,
        outcome: &CompatibilityReadAdmissionOutcome,
    ) -> Result<Self, Milestone12CertificationLaneRejection> {
        outcome_from_relation_and_rejection(
            lane_kind,
            input,
            outcome.relation(),
            outcome.rejection_kind(),
            outcome.counters(),
        )
    }

    pub fn from_write_outcome(
        lane_kind: Milestone12CertificationLaneKind,
        input: Milestone12CertificationLaneInput,
        outcome: &CompatibilityWriteAdmissionOutcome,
    ) -> Result<Self, Milestone12CertificationLaneRejection> {
        outcome_from_relation_and_rejection(
            lane_kind,
            input,
            outcome.relation(),
            outcome.rejection_kind(),
            outcome.counters(),
        )
    }

    pub fn from_derived_plan(
        lane_kind: Milestone12CertificationLaneKind,
        input: Milestone12CertificationLaneInput,
        plan: &DerivedLaneCompatibilityPlan,
        counters: &CompatibilityAdmissionCounters,
    ) -> Self {
        let status = match plan.posture() {
            DerivedLaneCompatibilityPosture::ReuseAdmitted
            | DerivedLaneCompatibilityPosture::SupportAdmitted => {
                Milestone12CertificationLaneStatus::Accepted
            }
            DerivedLaneCompatibilityPosture::InvalidatedForRebuild => {
                Milestone12CertificationLaneStatus::Invalidated
            }
            DerivedLaneCompatibilityPosture::RebuildAdmitted => {
                Milestone12CertificationLaneStatus::RebuildRequired
            }
            DerivedLaneCompatibilityPosture::Rejected => {
                Milestone12CertificationLaneStatus::Rejected
            }
        };
        if status == Milestone12CertificationLaneStatus::Accepted {
            Self::accepted(lane_kind, input, CompatibilityRelation::Native, counters)
        } else {
            Self::non_admitted(lane_kind, input, status, counters)
        }
    }

    pub fn from_compatibility_rejection(
        lane_kind: Milestone12CertificationLaneKind,
        input: Milestone12CertificationLaneInput,
        rejection: &CompatibilityRejection,
        counters: &CompatibilityAdmissionCounters,
    ) -> Self {
        Self::rejected(lane_kind, input, rejection.kind(), counters)
    }

    pub fn from_rolling_plan(
        input: Milestone12CertificationLaneInput,
        plan: &RollingUpgradeAdmissionPlan,
        counters: &CompatibilityAdmissionCounters,
    ) -> Self {
        Self::accepted(
            Milestone12CertificationLaneKind::RollingTwoCapabilityAdmitted,
            input,
            plan.relation(),
            counters,
        )
    }

    pub fn from_restore_plan(
        input: Milestone12CertificationLaneInput,
        plan: &RestoreCompatibilityPlan,
        counters: &CompatibilityAdmissionCounters,
    ) -> Self {
        Self::accepted(
            Milestone12CertificationLaneKind::RestoreScopedBackupAdmitted,
            input,
            plan.relation(),
            counters,
        )
    }

    pub fn from_disaster_recovery_plan(
        lane_kind: Milestone12CertificationLaneKind,
        input: Milestone12CertificationLaneInput,
        _plan: &DisasterRecoveryCompatibilityPlan,
        counters: &CompatibilityAdmissionCounters,
    ) -> Self {
        Self::non_admitted(
            lane_kind,
            input,
            Milestone12CertificationLaneStatus::EvidenceOnly,
            counters,
        )
    }

    pub fn lane_id(&self) -> &Milestone12CertificationLaneId {
        &self.lane_id
    }

    pub fn lane_kind(&self) -> Milestone12CertificationLaneKind {
        self.lane_kind
    }

    pub fn status(&self) -> Milestone12CertificationLaneStatus {
        self.status
    }

    pub fn relation(&self) -> Option<CompatibilityRelation> {
        self.relation
    }

    pub fn rejection_kind(&self) -> Option<CompatibilityRejectionKind> {
        self.rejection_kind
    }

    pub fn counters(&self) -> &Milestone12AdmissionReport {
        &self.counters
    }
}

fn outcome_from_relation_and_rejection(
    lane_kind: Milestone12CertificationLaneKind,
    input: Milestone12CertificationLaneInput,
    relation: Option<CompatibilityRelation>,
    rejection_kind: Option<CompatibilityRejectionKind>,
    counters: &CompatibilityAdmissionCounters,
) -> Result<Milestone12CertificationLaneOutcome, Milestone12CertificationLaneRejection> {
    match (relation, rejection_kind) {
        (Some(relation), None) => Ok(Milestone12CertificationLaneOutcome::accepted(
            lane_kind, input, relation, counters,
        )),
        (None, Some(rejection_kind)) => Ok(Milestone12CertificationLaneOutcome::rejected(
            lane_kind,
            input,
            rejection_kind,
            counters,
        )),
        _ => Err(Milestone12CertificationLaneRejection::OutcomeStatusMismatch),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12CompatibilityMatrixEntry {
    lane_id: Milestone12CertificationLaneId,
    lane_kind: Milestone12CertificationLaneKind,
    status: Milestone12CertificationLaneStatus,
}

impl Milestone12CompatibilityMatrixEntry {
    fn from_outcome(outcome: &Milestone12CertificationLaneOutcome) -> Self {
        Self {
            lane_id: outcome.lane_id().clone(),
            lane_kind: outcome.lane_kind(),
            status: outcome.status(),
        }
    }

    pub fn lane_id(&self) -> &Milestone12CertificationLaneId {
        &self.lane_id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Milestone12CompatibilityMatrixStatus {
    Complete,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12CompatibilityMatrix {
    entries: Vec<Milestone12CompatibilityMatrixEntry>,
    status: Milestone12CompatibilityMatrixStatus,
}

impl Milestone12CompatibilityMatrix {
    pub fn from_lane_outcomes(
        outcomes: &[Milestone12CertificationLaneOutcome],
    ) -> Result<Self, Milestone12CertificationLaneRejection> {
        let mut seen = BTreeSet::new();
        for outcome in outcomes {
            if !seen.insert(outcome.lane_id().clone()) {
                return Err(Milestone12CertificationLaneRejection::DuplicateLane);
            }
        }
        for kind in Milestone12CertificationLaneKind::mandatory_phase_5a() {
            if !seen.contains(&kind.lane_id()) {
                return Err(Milestone12CertificationLaneRejection::MissingMandatoryLane);
            }
        }
        let mut entries = outcomes
            .iter()
            .map(Milestone12CompatibilityMatrixEntry::from_outcome)
            .collect::<Vec<_>>();
        entries.sort_by_key(|entry| entry.lane_id().clone());
        Ok(Self {
            entries,
            status: Milestone12CompatibilityMatrixStatus::Complete,
        })
    }

    pub fn entries(&self) -> &[Milestone12CompatibilityMatrixEntry] {
        &self.entries
    }

    pub fn status(&self) -> Milestone12CompatibilityMatrixStatus {
        self.status
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12CertificationRunSummary {
    accepted_lane_count: u64,
    rejected_lane_count: u64,
    invalidated_lane_count: u64,
    rebuild_required_lane_count: u64,
    evidence_only_lane_count: u64,
}

impl Milestone12CertificationRunSummary {
    pub fn from_outcomes(outcomes: &[Milestone12CertificationLaneOutcome]) -> Self {
        let mut summary = Self {
            accepted_lane_count: 0,
            rejected_lane_count: 0,
            invalidated_lane_count: 0,
            rebuild_required_lane_count: 0,
            evidence_only_lane_count: 0,
        };
        for outcome in outcomes {
            match outcome.status() {
                Milestone12CertificationLaneStatus::Accepted => {
                    summary.accepted_lane_count += 1;
                }
                Milestone12CertificationLaneStatus::Rejected => {
                    summary.rejected_lane_count += 1;
                }
                Milestone12CertificationLaneStatus::Invalidated => {
                    summary.invalidated_lane_count += 1;
                }
                Milestone12CertificationLaneStatus::RebuildRequired => {
                    summary.rebuild_required_lane_count += 1;
                }
                Milestone12CertificationLaneStatus::EvidenceOnly => {
                    summary.evidence_only_lane_count += 1;
                }
            }
        }
        summary
    }

    pub fn accepted_lane_count(&self) -> u64 {
        self.accepted_lane_count
    }

    pub fn rejected_lane_count(&self) -> u64 {
        self.rejected_lane_count
    }
}
