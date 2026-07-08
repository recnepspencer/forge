//! Milestone closeout surfaces: physical integrity, late milestones, and synthetic rejection.

pub mod s6;

pub use crate::physical_integrity_closeout_bundle::{
    close_s3_physical_integrity_from_executed_evidence, PhysicalIntegrityCertificationBundle,
};
pub use crate::physical_integrity_closeout_denial::{
    PhysicalIntegrityCloseoutDenial, S3CloseoutDenialBoundary,
};
pub use crate::physical_integrity_closeout_handoff::S3S4HandoffCloseoutEvidence;
pub use crate::physical_integrity_closeout_harness::S3HarnessTranscriptEvidence;
pub use crate::physical_integrity_closeout_harness_execution::{
    S3CloseoutExecutedOutputKind, S3CloseoutHarnessExecutionEvidence,
};
pub use crate::physical_integrity_closeout_line_cap::{
    S3CloseoutModuleKind, S3LineCapCompositionEvidence, S3LineCapModuleEvidence,
};
pub use crate::physical_integrity_closeout_owned_file::S3OwnedCloseoutFileEvidence;
pub use crate::physical_integrity_closeout_proof::{
    S3ExecutedBoundaryDenialEvidence, S3ExecutedCorruptionLocalizationEvidence,
};
pub use crate::physical_integrity_closeout_report::{
    PhysicalIntegrityCloseoutReport, S3CloseoutSuiteHarnessSummary,
};
pub use crate::physical_integrity_closeout_suite::{
    PhysicalIntegrityCloseoutSuite, PhysicalIntegrityCloseoutSuiteEvidence,
};
pub use crate::physical_integrity_closeout_suite_kind::{
    S3AcceptanceSuiteKind, S3CloseoutEvidenceFamily, S3CorruptionLocalizationBoundary,
};
pub use crate::physical_substrate_closeout::{
    PhysicalPageSegmentExtentSubstrateCloseout, PhysicalPageSegmentExtentSubstrateEvidence,
    PhysicalPageSegmentExtentSubstrateRun, PhysicalSubstrateCloseoutDenial,
};
pub use crate::physical_substrate_closeout_story::{
    PhysicalSubstrateCloseoutStoryDenial, PhysicalSubstrateCloseoutStoryReport,
    PhysicalSubstrateCloseoutStoryRow,
};
pub use crate::s4_recovery_harness::{
    RecoveryPhysicsCertificationDenial, RecoveryPhysicsCertificationMatrix,
    RecoveryPhysicsCertificationRow, RecoveryPhysicsCounterExpectation, RecoveryPhysicsCounterKind,
    RecoveryPhysicsCrashLane, RecoveryPhysicsCrashMatrix, RecoveryPhysicsCrashMatrixBuilder,
    RecoveryPhysicsCrashMatrixDenial, RecoveryPhysicsEvidenceBundle, RecoveryPhysicsMutant,
    RecoveryPhysicsMutationFailureEvidence, RecoveryPhysicsMutationSuiteEvidence,
    RecoveryPhysicsMutationSuiteEvidenceDenial, RecoveryPhysicsMutationSuiteLaneEvidence,
    RecoveryPhysicsMutationValidationDenial, RecoveryPhysicsMutationValidationMatrix,
    RecoveryPhysicsMutationValidationRow, RecoveryPhysicsObserverKind,
    RecoveryPhysicsOracleJudgment, RecoveryPhysicsOracleKind,
    RecoveryPhysicsRoadmap2HarnessCertification, RecoveryPhysicsRoadmap2HarnessDenial,
    RecoveryPhysicsScenarioDefinition, RecoveryPhysicsScenarioDefinitionBuilder,
    RecoveryPhysicsScenarioDefinitionDenial, RecoveryPhysicsScenarioDrivers,
    RecoveryPhysicsScenarioPlan, RecoveryPhysicsScenarioPlanDenial, RecoveryPhysicsShortcutAttempt,
    RecoveryPhysicsShortcutDenialBoundary, RecoveryPhysicsShortcutDenialReason,
    RecoveryPhysicsShortcutRejection, RecoveryPhysicsTranscript,
};
pub use crate::s5_1_closeout::{
    certify_s5_1_security_scope_closeout, S51CertificationCloseoutDenial,
    S51CertificationCloseoutEvidence, S51CertificationCloseoutInput,
    S51CertificationEvidencePolicy, S51CloseoutApiAdoptionEvidence,
    S51CloseoutBoundaryEvidencePublication, S51CloseoutCounterMatrix,
    S51CloseoutFoundationalBoundaryPackage, S51CloseoutFoundationalLane,
    S51CloseoutPerformanceReceipts, S51CloseoutPerformanceRows,
};
pub use crate::s5_evidence_materialization::{
    materialize_s5_executed_isolation_evidence, S5ExecutedIsolationEvidenceBundle,
    S5ExecutedIsolationMaterializationDenial, S5FoundationalCanonicalBasis,
    S5FoundationalDiagnostics, S5FoundationalPerformanceReceipts, S5PhysicalIsolationProofTrace,
    S5ProofProjectionArtifact,
};
pub use crate::s5_physical_isolation_closeout::{
    PhysicalIsolationCloseoutDenial, PhysicalIsolationCloseoutHandoffEvidence,
    PhysicalIsolationCloseoutLaneEvidence, PhysicalIsolationCloseoutSuite,
    S5CloseoutReservationSet, S5CloseoutReservedScope,
};
pub use crate::s5_physical_isolation_harness::{
    assemble_s5_physical_isolation_replay_bundle, observe_s5_physical_isolation_trace,
    s5_physical_isolation_ci_certification_context_without_lane_registration,
    s5_physical_isolation_ci_certification_planning_context,
    s5_physical_isolation_context_without_lane_registration, s5_physical_isolation_coverage_matrix,
    s5_physical_isolation_lanes, s5_physical_isolation_planning_context,
    S5PhysicalIsolationHarnessLane, S5PhysicalIsolationTraceFixtures,
};
pub use crate::s7_closeout::{
    admit_s7_backup_non_claim_handoff, admit_s7_full_certification_non_claim_handoff,
    admit_s7_key_lifecycle_non_claim_handoff, admit_s7_layout_readiness_handoff,
    certify_s7_native_blob_store_closeout, evaluate_s7_closeout_request,
    S10BlobBackupRepairNonClaimHandoff, S11KeyLifecycleNonClaimHandoff,
    S12FullCertificationNonClaimHandoff, S7CloseoutCertificationInput, S7CloseoutDenial,
    S7CloseoutEvidencePolicy, S7CloseoutRequest, S7CloseoutShortcutAttempt,
    S7CloseoutShortcutInput, S7CloseoutShortcutRejectionReport, S7NativeBlobStoreCloseout,
    S7S8LayoutReadinessHandoff,
};
pub use crate::synthetic_closeout_rejection::{
    SyntheticCloseoutRejectionDenial, SyntheticCloseoutShortcutAttempt,
    SyntheticCloseoutShortcutInput, SyntheticCloseoutShortcutRejectionReport,
};
pub use forge_store_physical_certification::{
    s5_physical_isolation_required_mutation_rows, S5EvidenceProfileCounterSet,
    S5ExecutedIsolationEvidenceSource, S5ExecutedIsolationFinding, S5ExecutedIsolationOutcome,
    S5ExecutedIsolationRequiredCounters, S5ExecutedIsolationSourceBasis,
    S5ExecutedIsolationSourceDenial, S5PhysicalIsolationMutationEvidence,
};
pub use s6::{
    adopt_materialized_s6_certification_evidence_for_closeout,
    materialize_s6_certification_evidence, S6CertificationEvidenceAdoptionReceipt,
};
