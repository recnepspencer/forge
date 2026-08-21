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
pub enum RetentionDeliveryProfile {
    Ephemeral,
    Retained,
    Durable,
}
