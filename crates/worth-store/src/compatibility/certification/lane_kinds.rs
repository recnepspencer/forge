use serde::Serialize;

use super::super::admission::{CompatibilityRejectionKind, CompatibilityRelation};

use super::super::manifests::{ArtifactFamilyId, ArtifactSemanticVersion};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub enum Milestone12CertificationLaneKind {
    CatalogCompleteness,
    AuthoritativeNativeRead,
    AuthoritativeForwardRead,
    AuthoritativeBackwardRead,
    AuthoritativeMissingEdgeRejected,
    AuthoritativeIncompatibleEdgeRejected,
    DerivedSnapshotReuseAccepted,
    MaintenanceSummaryRebuildAdmitted,
    DerivedLayoutBasisRejected,
    DerivedBulkResumeRejected,
    TierManifestNonAuthorityPreserved,
    RollingTwoCapabilityAdmitted,
    RollingMultiWriterRejected,
    RollingMissingEdgeRejected,
    RollingAdapterEdgeRejected,
    AdapterParityAdmitted,
    AdapterParityDigestRejected,
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
            Self::MaintenanceSummaryRebuildAdmitted => "maintenance_summary_rebuild_admitted",
            Self::DerivedLayoutBasisRejected => "derived_layout_basis_rejected",
            Self::DerivedBulkResumeRejected => "derived_bulk_resume_rejected",
            Self::TierManifestNonAuthorityPreserved => "tier_manifest_non_authority_preserved",
            Self::RollingTwoCapabilityAdmitted => "rolling_two_capability_admitted",
            Self::RollingMultiWriterRejected => "rolling_multi_writer_rejected",
            Self::RollingMissingEdgeRejected => "rolling_missing_edge_rejected",
            Self::RollingAdapterEdgeRejected => "rolling_adapter_edge_rejected",
            Self::AdapterParityAdmitted => "adapter_parity_admitted",
            Self::AdapterParityDigestRejected => "adapter_parity_digest_rejected",
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
            Self::MaintenanceSummaryRebuildAdmitted,
            Self::DerivedLayoutBasisRejected,
            Self::DerivedBulkResumeRejected,
            Self::TierManifestNonAuthorityPreserved,
            Self::RollingTwoCapabilityAdmitted,
            Self::RollingMultiWriterRejected,
            Self::RollingMissingEdgeRejected,
            Self::RollingAdapterEdgeRejected,
            Self::AdapterParityAdmitted,
            Self::AdapterParityDigestRejected,
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
