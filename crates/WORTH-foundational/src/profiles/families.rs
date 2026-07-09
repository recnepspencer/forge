#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagnosticRichnessProfile {
    OperationalMinimal,
    Standard,
    Forensic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SupportPostureProfile {
    InternalOnly,
    SupportReady,
    CertificationReady,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CompatibilityPostureProfile {
    NativeOnly,
    CompatibilityLowered,
    CompatibilityRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AdmissionReadinessProfile {
    CandidateOnly,
    Admitted,
    ProductionGateReady,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RetentionDeliveryProfile {
    Ephemeral,
    Retained,
    Durable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CertificationPostureProfile {
    Uncertified,
    EvidenceBacked,
    ProductionCertified,
}
