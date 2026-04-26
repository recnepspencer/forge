mod actions;
mod artifacts;
mod catalog;
mod classification;
mod compatibility;
mod declaration;
mod digest;
mod evidence;
mod fetch;
mod handles;
mod identity;
mod maintenance;
mod participation;
mod performance;
mod persistence;
mod pipeline;
mod portability;
mod records;
mod restart;
mod resume;
mod retention;
#[cfg(test)]
mod tests;
mod trust;
mod witnesses;

use crate::failure::{StoreError, StoreErrorKind};

pub const SUBSCRIPTION_SUPPORT_FAMILY_VERSION: u16 = 1;

pub use actions::{
    CompletedSupportProgramAction, ExecutedSupportAction, PlannedSupportAction,
    ProofCheckedSupportAction, PublishedSupportConsequence, RawSupportProgramAction,
    SubscriptionSupportActionPublicationRecoveryReport, SupportActionDurableRecord,
    SupportActionId, SupportActionPublicationState, SupportActionPublicationWitness,
    SupportActionRecoveryDisposition, SupportConsequenceEnvelope,
};
pub use artifacts::{PublishableSubscriptionSupportArtifact, PublishedSubscriptionSupportArtifact};
pub use catalog::{
    SubscriptionSupportAccessStructure, SubscriptionSupportAccessStructureReport,
    SubscriptionSupportCatalog,
};
pub(crate) use classification::classify_causes;
pub use classification::{
    SubscriptionResumeClassification, SubscriptionSupportAllocationScope,
    SubscriptionSupportClassificationPlan, SubscriptionSupportClassificationReport,
    SubscriptionSupportDensityClass, SubscriptionSupportDriftCause,
    SubscriptionSupportPayloadBudget, SubscriptionSupportPlanFamily,
    SubscriptionSupportResultCostSurface,
};
pub use compatibility::{
    DegradedCompatibleSupportPosture, ExactCompatibleSupportMigration,
    SubscriptionSupportCompatibilityDecision, SubscriptionSupportCompatibilityDecisionKind,
    SubscriptionSupportCompatibilityOutcome, SubscriptionSupportCompatibilityReport,
    SupportCompatibilityAffectedSet, SupportCompatibilityBatchPlan,
    SupportCompatibilityParticipationRecord, SupportCompatibilityReceiptWitness,
    SupportDecodedRowSemanticAccess, SupportFamilyVersionWindow, SupportManifestAdmissionWitness,
    SupportVersionSkewRejection,
};
pub use declaration::{
    AdmittedSubscriptionSupportDeclaration, RawSubscriptionSupportDeclaration,
    SubscriptionSupportAuthority, SubscriptionSupportPayloadDigest, SubscriptionSupportScope,
};
pub use evidence::{
    SubscriptionSupportCertificationBundle, SubscriptionSupportCertificationLaneKind,
    SubscriptionSupportCertificationLaneOutcome, SubscriptionSupportCertificationMatrix,
    SubscriptionSupportCertificationMatrixStatus, SubscriptionSupportCounterSnapshot,
};
pub use fetch::{
    FetchedSubscriptionSupportArtifact, SubscriptionSupportFetchReport,
    SubscriptionSupportFetchRequest,
};
pub(crate) use handles::{ensure_classification, ensure_report_matches_artifact};
pub use handles::{
    DegradedSubscriptionResumeHandle, ExactSubscriptionResumeHandle,
    SubscriptionResumeDeniedReport, SubscriptionSupportRebuildPlanHandle,
};
pub use identity::{
    SubscriptionSupportArtifactId, SubscriptionSupportDeclarationDigest,
    SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind, SubscriptionSupportRole,
};
pub(crate) use maintenance::{support_maintenance_batch, synthetic_support_maintenance_receipt};
pub use maintenance::{
    SubscriptionSupportMaintenanceDebtReport, SubscriptionSupportMaintenanceDecision,
    SubscriptionSupportMaintenanceDecisionKind, SubscriptionSupportMaintenanceReport,
    SupportMaintenanceAdmissionWitness, SupportMaintenanceAffectedSet, SupportMaintenanceBatchPlan,
    SupportMaintenanceDebtRecord, SupportMaintenanceDebtSummary, SupportMaintenanceDescriptor,
    SupportMaintenanceDescriptorRecord, SupportMaintenanceParticipationRecord,
    SupportMaintenanceWorkKind,
};
pub use participation::{
    DegradedResumePreservationWitness, ExactResumePreservationWitness,
    PostActionResumeClassificationInput, ResumeClassificationTranslationPlan,
    SubscriptionSupportActionOrigin, SubscriptionSupportOperationalBasis,
    SubscriptionSupportOperationalVerdict, SubscriptionSupportOperationalVerdictTranslationRequest,
    SupportNonResumableWitness, SupportPolicyRejectionWitness, SupportRebuildAdmissionWitness,
};
pub(crate) use performance::cost_surface_for_program_path;
pub use performance::{
    SupportActionBreadthBudget, SupportAllocationScope, SupportBatchAdmissionReceipt,
    SupportBatchProofKind, SupportBatchReceiptReuseReport, SupportPathClass,
    SupportProgramDensityClass, SupportProgramPathPlan,
};
pub use persistence::{SubscriptionSupportStoredRecordKey, SubscriptionSupportStoredRecordSet};
pub use pipeline::SubscriptionSupportPublicationPipeline;
pub use portability::{
    CapsuleSupportManifest, ImportedSupportNotResumableReport, ImportedSupportSemanticAccess,
    PartialSupportOmissionReport, ReplicatedSupportBundle, SubscriptionSupportPortabilityDecision,
    SubscriptionSupportPortabilityDecisionKind, SubscriptionSupportPortabilityOutcome,
    SubscriptionSupportPortabilityReport, SupportImportAdmissionWitness,
    SupportPortabilityAffectedSet, SupportPortabilityBatchPlan, SupportPortabilityManifestBudget,
    SupportPortabilityParticipationRecord, SupportPortabilityRejection,
    SupportPortabilityScopeFootprint,
};
pub use records::{
    SubscriptionSupportArtifactRecord, SubscriptionSupportClassificationRecord,
    SubscriptionSupportLinkageRecord, SubscriptionSupportRestartRecord,
};
pub use restart::{
    SubscriptionSupportMissingSupportMaintenanceAdmission,
    SubscriptionSupportMissingSupportRecoveryReport,
    SubscriptionSupportMissingSupportRecoveryRequest,
    SubscriptionSupportRestartReconstructionReport,
    SubscriptionSupportRestartReconstructionRequest, SubscriptionSupportRestartShard,
    SubscriptionSupportRuntimeHandoffReport, SubscriptionSupportRuntimeHandoffRequest,
};
pub use resume::{SubscriptionSupportResumeEvidence, SubscriptionSupportResumeRequest};
pub use retention::{
    CompactedSupportBasis, ExpiredSupportArtifactSet, ReclaimedSupportArtifactSet,
    RetainedSupportArtifactSet, SubscriptionSupportPostActionReport,
    SubscriptionSupportRetentionDecision, SubscriptionSupportRetentionDecisionKind,
    SubscriptionSupportRetentionMaterialization, SubscriptionSupportRetentionPlan,
    SupportAffectedSet, SupportAffectedSetDigest, SupportReclaimConsequence,
    SupportRetentionBatchPlan, SupportRetentionParticipationRecord,
    SupportRetentionSurvivalWitness,
};
pub use trust::{
    admit_support_trust_request, check_support_trust_coverage, check_support_trust_drift,
    check_support_trust_equivalence, classify_certified_support_trust,
    classify_operational_support_trust, translate_support_trust_inputs,
    CertifiedSupportTrustClassified, CertifiedSupportTrustReport, CertifiedSupportTrustWitness,
    DegradedSupportTrustWitness, ExactSupportTrustWitness, OperationalSupportTrustClassified,
    OperationalSupportTrustReport, RawSupportTrustRequest, RebuildDerivedSupportTrustWitness,
    RejectedSupportTrustWitness, SubscriptionSupportAccuracyAccessCloseout,
    SubscriptionSupportAccuracyCertificationCounterSnapshot,
    SubscriptionSupportAccuracyCertificationOutputs, SubscriptionSupportAccuracyCertificationRow,
    SubscriptionSupportAccuracyCertificationRowKind, SubscriptionSupportAccuracyCertificationRun,
    SubscriptionSupportAccuracyCertificationRunner, SubscriptionSupportAccuracyCertificationSuite,
    SubscriptionSupportAccuracyLaneEvidence, SubscriptionSupportAccuracyLaneEvidenceSet,
    SubscriptionSupportAccuracyLaneOutcome, SubscriptionSupportAccuracyPerformanceCloseout,
    SubscriptionSupportAccuracyPersistencePosture, SubscriptionSupportCertificationCoveragePlan,
    SubscriptionSupportTrustClass, SupportBasisReceipt, SupportCatalogEpoch,
    SupportCertificationBatchScope, SupportCertificationBatchScopeKind,
    SupportCertificationCorpusVersion, SupportCertificationCounterSnapshot,
    SupportCertificationCoverageMatrix, SupportCertificationCoverageWitness,
    SupportCertificationEpoch, SupportCertificationEvidenceBundle, SupportCertificationGapReport,
    SupportCertificationHandoffReport, SupportCertificationLaneDigestSet, SupportCertificationRow,
    SupportCertificationRowEvidence, SupportCertificationRowRequirement,
    SupportCertificationSummary, SupportCompatibilityEpoch, SupportCompatibilityReceipt,
    SupportCursorCheckpointReceipt, SupportDomainCertificationBatchPlan,
    SupportDomainCertificationBundle, SupportDomainCertificationCounterSnapshot,
    SupportDomainCertificationDebtOwner, SupportDomainCertificationDebtReason,
    SupportDomainCertificationRow, SupportDomainCertificationRowStatus,
    SupportDomainCertificationScenario, SupportExactTrustTranslation, SupportFamilyRoleReceipt,
    SupportGenericCertificationCounterSnapshot, SupportGenericCertificationReport,
    SupportImportAdmissionReceipt, SupportImportEquivalenceWitness, SupportMaintenanceReceipt,
    SupportMigrationEquivalenceWitness, SupportOperationalLedgerEpoch,
    SupportOperationalVerdictReceipt, SupportPortabilityReceipt, SupportRebuildEquivalenceWitness,
    SupportReplicationEquivalenceWitness, SupportResumeClassificationReceipt,
    SupportRetentionReceipt, SupportRoadmapPhysicalReadinessPosture, SupportRoleTrustPosture,
    SupportStalenessVerdict, SupportTrustAccessIndexKind, SupportTrustAccessPath,
    SupportTrustAccessStructurePlan, SupportTrustAllocationScope, SupportTrustBatchCardinality,
    SupportTrustCertificationStamp, SupportTrustClass, SupportTrustClassificationCostSurface,
    SupportTrustClassificationCounterSnapshot, SupportTrustClassificationPlan,
    SupportTrustClassificationReport, SupportTrustClassificationWitness, SupportTrustCloneBoundary,
    SupportTrustComplexityContract, SupportTrustComplexityStatus, SupportTrustCoverageChecked,
    SupportTrustDensityClass, SupportTrustDowngradeReason, SupportTrustDriftCause,
    SupportTrustDriftChecked, SupportTrustDriftLocality, SupportTrustDriftReport,
    SupportTrustDriftScanPlan, SupportTrustEpoch, SupportTrustEquivalenceChecked,
    SupportTrustEquivalenceContract, SupportTrustEquivalenceEvidence, SupportTrustEquivalenceLane,
    SupportTrustEquivalenceWitness, SupportTrustEvidenceBudget, SupportTrustExpiredReport,
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustFreshnessWitness,
    SupportTrustOperationalWitness, SupportTrustPathClass, SupportTrustPerformancePlan,
    SupportTrustProvenance, SupportTrustReceiptBundle, SupportTrustReceiptStatus,
    SupportTrustRecoveryPosture, SupportTrustRequestAdmitted, SupportTrustRequestedUse,
    SupportTrustStrength, SupportTrustStrengthProvenance, SupportTrustSuppressedCause,
    SupportTrustTranslatedInputs, SupportTrustTranslationPlan, SupportTrustUseBoundary,
    UncertifiedSupportTrustPosture, SUBSCRIPTION_SUPPORT_ACCURACY_CERTIFICATION_SUITE_NAME,
};

use digest::stable_digest;

pub(crate) fn admission_error(message: impl Into<String>) -> StoreError {
    StoreError::new(
        StoreErrorKind::SubscriptionSupportAdmissionViolation,
        message,
    )
}

pub(crate) fn classification_error(message: impl Into<String>) -> StoreError {
    StoreError::new(
        StoreErrorKind::SubscriptionSupportClassificationViolation,
        message,
    )
}

pub(crate) fn publication_error(message: impl Into<String>) -> StoreError {
    StoreError::new(
        StoreErrorKind::SubscriptionSupportPublicationViolation,
        message,
    )
}
