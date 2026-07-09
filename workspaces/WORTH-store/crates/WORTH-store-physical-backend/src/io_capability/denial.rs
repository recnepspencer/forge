use super::{
    BackendCapabilityKind, BackendCapabilitySupportPosture, BackendRebindTriggers,
    CapabilityEvidenceClass,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackendCapabilityAdmissionDenial {
    MissingMediaAssumption(BackendCapabilityKind),
    UnsupportedCapability {
        kind: BackendCapabilityKind,
        posture: BackendCapabilitySupportPosture,
    },
    UnavailableCapability {
        kind: BackendCapabilityKind,
        posture: BackendCapabilitySupportPosture,
    },
    UnknownCapability {
        kind: BackendCapabilityKind,
        posture: BackendCapabilitySupportPosture,
    },
    StaleCapability {
        kind: BackendCapabilityKind,
        posture: BackendCapabilitySupportPosture,
    },
    RebindRequired {
        kind: BackendCapabilityKind,
        triggers: BackendRebindTriggers,
    },
    EvidenceClassTooWeak {
        required: CapabilityEvidenceClass,
        actual: CapabilityEvidenceClass,
    },
    ConfidenceLimitTooWeak,
    RawBackendLabel,
    RawConfigString,
    RawOsName,
    RawProbeObservation,
    SameProcessMetricProjection,
    EnvironmentVariable,
    TerminalProjection,
    CopiedQualificationRow,
    CertificationOnlyEvidence,
}

pub fn reject_raw_backend_label() -> Result<(), BackendCapabilityAdmissionDenial> {
    Err(BackendCapabilityAdmissionDenial::RawBackendLabel)
}

pub fn reject_raw_config_string() -> Result<(), BackendCapabilityAdmissionDenial> {
    Err(BackendCapabilityAdmissionDenial::RawConfigString)
}

pub fn reject_raw_os_name() -> Result<(), BackendCapabilityAdmissionDenial> {
    Err(BackendCapabilityAdmissionDenial::RawOsName)
}

pub fn reject_raw_probe_observation() -> Result<(), BackendCapabilityAdmissionDenial> {
    Err(BackendCapabilityAdmissionDenial::RawProbeObservation)
}

pub fn reject_same_process_metric_projection() -> Result<(), BackendCapabilityAdmissionDenial> {
    Err(BackendCapabilityAdmissionDenial::SameProcessMetricProjection)
}

pub fn reject_environment_variable() -> Result<(), BackendCapabilityAdmissionDenial> {
    Err(BackendCapabilityAdmissionDenial::EnvironmentVariable)
}

pub fn reject_terminal_projection() -> Result<(), BackendCapabilityAdmissionDenial> {
    Err(BackendCapabilityAdmissionDenial::TerminalProjection)
}

pub fn reject_copied_qualification_row() -> Result<(), BackendCapabilityAdmissionDenial> {
    Err(BackendCapabilityAdmissionDenial::CopiedQualificationRow)
}

pub fn reject_certification_only_evidence() -> Result<(), BackendCapabilityAdmissionDenial> {
    Err(BackendCapabilityAdmissionDenial::CertificationOnlyEvidence)
}
