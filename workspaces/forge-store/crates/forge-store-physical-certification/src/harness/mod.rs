//! Harness authority for physical simulation certification scenarios.

pub mod by_milestone;

pub use by_milestone::s6::{
    all_s6_fault_evidence_classes, all_s6_io_pressure_fault_kinds, evaluate_row_rebind,
    reject_copied_backend_qualification_row, reject_environment_name_backend_qualification,
    reject_log_output_backend_qualification, reject_test_only_backend_label_qualification,
    require_profile_local_row, BackendQualificationMatrix, BackendQualificationMatrixDenial,
    BackendQualificationParityComparison, BackendQualificationRow, BackendQualificationRowIdentity,
    CertifiedBackendQualificationSupport, PhysicalFaultEvidenceClass,
    PublishedQualificationPosture, QualificationCapabilityProofAuthority,
    QualificationHarnessProof, QualificationHarnessProofClaim, QualificationHarnessProofStrength,
    QualificationMatrixPublisher, QualificationPublicationShortcut, QualificationRebindEvaluation,
    QualificationResidualDebt, QualificationResidualDebtReason, S6BackendSafetyQualificationDenial,
    S6ExecutedIoPressureCoverageRows, S6HarnessSecureIoPosture, S6IoPressureExecutionCounters,
    S6IoPressureFaultKind, S6IoPressureHarnessEvidence, S6IoPressureHarnessEvidenceDenial,
    S6IoPressureHarnessScenario, S6IoPressureOracleObservation, S6PressureEvidenceMaturity,
    S6RealBackendSafetyQualification,
};
pub use by_milestone::s7_blob_harness::{
    lower_blob_simulation_seed_plan, BlobHarnessLoweredSeedPlan, BlobHarnessLoweringDenial,
    BlobHarnessMaterializedProfile, BlobHarnessProfile, BlobHarnessProfileSet,
    BlobHarnessScenarioSeed, BlobHarnessScenarioSeedBuilder, BlobHarnessShortcutAttempt,
    BlobHarnessShortcutDenial, S7BlobResumeCrashPoint, S7BlobResumeExpectedOutcome,
    S7BlobResumeRecoveryScenario,
};
