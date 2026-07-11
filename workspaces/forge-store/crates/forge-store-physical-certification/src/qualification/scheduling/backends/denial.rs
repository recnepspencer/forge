use forge_store_physical_backend::{
    BackendCapabilityKind, BackendCapabilitySupportPosture, BackendRebindTriggers,
    BackendTargetProfile, CapabilityEvidenceClass,
};

use super::QualificationHarnessProofStrength;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualificationPublicationShortcut {
    CopiedRow,
    LogOutput,
    EnvironmentName,
    TestOnlyBackendLabel,
    UnsupportedCapabilityClaim,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendQualificationMatrixDenial {
    MissingHarnessProof,
    ProfileMismatch {
        expected: BackendTargetProfile,
        actual: BackendTargetProfile,
    },
    DuplicateRow {
        profile: BackendTargetProfile,
        capability: BackendCapabilityKind,
        evidence_class: CapabilityEvidenceClass,
    },
    RowNotFound {
        profile: BackendTargetProfile,
        capability: BackendCapabilityKind,
    },
    EvidenceClassTooWeak {
        required: CapabilityEvidenceClass,
        actual: CapabilityEvidenceClass,
    },
    ConfidenceLimitTooWeak,
    MissingMediaAssumption {
        capability: BackendCapabilityKind,
    },
    UnsupportedCapability {
        capability: BackendCapabilityKind,
        posture: BackendCapabilitySupportPosture,
    },
    ResidualDebtPresent {
        capability: BackendCapabilityKind,
    },
    MissingResidualDebt {
        capability: BackendCapabilityKind,
        posture: BackendCapabilitySupportPosture,
    },
    ResidualDebtCapabilityMismatch {
        expected: BackendCapabilityKind,
        actual: BackendCapabilityKind,
    },
    HarnessProofCapabilityMismatch {
        capability: BackendCapabilityKind,
    },
    HarnessProofEvidenceMismatch {
        capability: BackendCapabilityKind,
    },
    HarnessProofStrengthTooWeak {
        required: QualificationHarnessProofStrength,
        actual: QualificationHarnessProofStrength,
    },
    StaleRow {
        capability: BackendCapabilityKind,
    },
    RebindRequired {
        capability: BackendCapabilityKind,
        triggers: BackendRebindTriggers,
    },
    CrossBackendEvidenceSubstitution {
        expected: BackendTargetProfile,
        actual: BackendTargetProfile,
    },
    PublicationShortcut(QualificationPublicationShortcut),
}

pub fn reject_copied_backend_qualification_row() -> Result<(), BackendQualificationMatrixDenial> {
    Err(BackendQualificationMatrixDenial::PublicationShortcut(
        QualificationPublicationShortcut::CopiedRow,
    ))
}

pub fn reject_log_output_backend_qualification() -> Result<(), BackendQualificationMatrixDenial> {
    Err(BackendQualificationMatrixDenial::PublicationShortcut(
        QualificationPublicationShortcut::LogOutput,
    ))
}

pub fn reject_environment_name_backend_qualification(
) -> Result<(), BackendQualificationMatrixDenial> {
    Err(BackendQualificationMatrixDenial::PublicationShortcut(
        QualificationPublicationShortcut::EnvironmentName,
    ))
}

pub fn reject_test_only_backend_label_qualification() -> Result<(), BackendQualificationMatrixDenial>
{
    Err(BackendQualificationMatrixDenial::PublicationShortcut(
        QualificationPublicationShortcut::TestOnlyBackendLabel,
    ))
}
