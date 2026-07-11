//! Milestone-scoped harness authority grouped by roadmap phase.

pub mod s8_layout_access;

pub mod s6 {
    pub use crate::s6_backend_qualification::{
        evaluate_row_rebind, reject_copied_backend_qualification_row,
        reject_environment_name_backend_qualification, reject_log_output_backend_qualification,
        reject_test_only_backend_label_qualification, require_profile_local_row,
        BackendQualificationMatrix, BackendQualificationMatrixDenial,
        BackendQualificationParityComparison, BackendQualificationRow,
        BackendQualificationRowIdentity, CertifiedBackendQualificationSupport,
        PublishedQualificationPosture, QualificationCapabilityProofAuthority,
        QualificationHarnessProof, QualificationHarnessProofClaim,
        QualificationHarnessProofStrength, QualificationMatrixPublisher,
        QualificationPublicationShortcut, QualificationRebindEvaluation, QualificationResidualDebt,
        QualificationResidualDebtReason,
    };
    pub use crate::s6_io_pressure_coverage::S6ExecutedIoPressureCoverageRows;
    pub use crate::s6_io_pressure_execution::S6IoPressureExecutionCounters;
    pub use crate::s6_io_pressure_harness::{
        PhysicalFaultEvidenceClass, S6BackendSafetyQualificationDenial, S6HarnessSecureIoPosture,
        S6IoPressureFaultKind, S6IoPressureHarnessEvidence, S6IoPressureHarnessScenario,
        S6IoPressureOracleObservation, S6PressureEvidenceMaturity,
        S6RealBackendSafetyQualification,
    };
    pub use crate::s6_io_pressure_replay::S6IoPressureHarnessEvidenceDenial;
    #[cfg(any(test, feature = "certification-test-support"))]
    pub use crate::s6_io_pressure_test_support::replay_bundle_for as test_replay_bundle_for;
    pub use crate::s6_io_pressure_vocab::{
        all_s6_fault_evidence_classes, all_s6_io_pressure_fault_kinds,
    };
}
