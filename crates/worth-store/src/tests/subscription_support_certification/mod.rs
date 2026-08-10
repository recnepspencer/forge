mod matrix;
mod matrix_validation;
mod runtime_handoff;
mod world;

pub(super) use crate::tests::harness::fixtures::stores::unique_test_sqlite_path;
pub(super) use crate::{
    ArtifactCompatibilityWindow, ArtifactFamilyId, ArtifactFormatVersion, ArtifactSemanticVersion,
    CompatibilityAdmissionCounters, CompatibilityAdmissionPath, CompatibilityAdmissionReceipt,
    CompatibilityManifestDigest, CompatibilityReadAdmissionOutcome, CompatibilityRejection,
    CompatibilityRejectionKind, CompatibilityRelation, QuarantinedDecodedArtifact,
    RawSubscriptionSupportDeclaration, RawSupportProgramAction, ReadCompatibilityReceipt,
    StoreErrorKind, SubscriptionResumeClassification, SubscriptionSupportActionOrigin,
    SubscriptionSupportAllocationScope, SubscriptionSupportArtifactId,
    SubscriptionSupportAuthority, SubscriptionSupportCatalog,
    SubscriptionSupportCertificationBundle, SubscriptionSupportCertificationLaneKind,
    SubscriptionSupportCertificationLaneOutcome, SubscriptionSupportCertificationMatrixStatus,
    SubscriptionSupportClassificationPlan, SubscriptionSupportClassificationReport,
    SubscriptionSupportCompatibilityBatchRequest, SubscriptionSupportCompatibilityDecision,
    SubscriptionSupportCounterSnapshot, SubscriptionSupportDensityClass,
    SubscriptionSupportDriftCause, SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportFetchRequest, SubscriptionSupportMaintenanceBatchRequest,
    SubscriptionSupportMaintenanceDecision, SubscriptionSupportMissingSupportMaintenanceAdmission,
    SubscriptionSupportMissingSupportRecoveryRequest, SubscriptionSupportOperationalBasis,
    SubscriptionSupportOperationalVerdictTranslationRequest, SubscriptionSupportPayloadBudget,
    SubscriptionSupportPayloadDigest, SubscriptionSupportPlanFamily,
    SubscriptionSupportPortabilityBatchRequest, SubscriptionSupportPortabilityDecision,
    SubscriptionSupportRestartReconstructionRequest, SubscriptionSupportRestartShard,
    SubscriptionSupportResumeEvidence, SubscriptionSupportResumeRequest,
    SubscriptionSupportRetentionBatchRequest, SubscriptionSupportRetentionDecision,
    SubscriptionSupportRole, SubscriptionSupportRuntimeHandoffRequest, SubscriptionSupportScope,
    SupportActionBreadthBudget, SupportActionId, SupportActionRecoveryDisposition,
    SupportAllocationScope, SupportBatchProofKind, SupportCompatibilityReceiptWitness,
    SupportFamilyVersionWindow, SupportPathClass, SupportPortabilityManifestBudget,
    SupportProgramDensityClass, SupportProgramPathAdmissionRequest, SupportProgramPathPolicy,
    WORTHStore, WORTHStoreBuilder,
};

pub(super) fn retention_batch_request(
    action_id: SupportActionId,
    affected_bases: Vec<SubscriptionSupportOperationalBasis>,
    decision: SubscriptionSupportRetentionDecision,
) -> SubscriptionSupportRetentionBatchRequest {
    SubscriptionSupportRetentionBatchRequest {
        action_id,
        affected_bases,
        decision,
        path: SupportProgramPathPolicy {
            path_class: SupportPathClass::OperationalPlanning,
            density_class: SupportProgramDensityClass::FamilyLocalBatch,
            allocation_scope: SupportAllocationScope::FamilyLocalBatch,
            budget: SupportActionBreadthBudget::new(4, 1024).unwrap(),
            payload_header_bytes: 128,
        },
    }
}

pub(super) fn compatibility_batch_request(
    action_id: SupportActionId,
    affected_bases: Vec<SubscriptionSupportOperationalBasis>,
    compatibility_receipt: SupportCompatibilityReceiptWitness,
    semantic_digest: impl Into<String>,
    decision: SubscriptionSupportCompatibilityDecision,
) -> SubscriptionSupportCompatibilityBatchRequest {
    SubscriptionSupportCompatibilityBatchRequest {
        action_id,
        affected_bases,
        compatibility_receipt,
        semantic_digest: semantic_digest.into(),
        decision,
        path: SupportProgramPathPolicy {
            path_class: SupportPathClass::OperationalPlanning,
            density_class: SupportProgramDensityClass::FamilyLocalBatch,
            allocation_scope: SupportAllocationScope::FamilyLocalBatch,
            budget: SupportActionBreadthBudget::new(4, 1024).unwrap(),
            payload_header_bytes: 128,
        },
    }
}

pub(super) use world::{
    compatibility_basis, compatibility_manifest_digest, fetched_exact_report, maintenance_basis,
    portability_basis, publish_exact, raw_degraded, raw_exact, raw_materialized,
    read_receipt_witness, rejected_read_outcome_witness, retention_basis, support_version_window,
};
