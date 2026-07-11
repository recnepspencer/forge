#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalRecoveryPosture {
    TrustedTruth,
    DegradedDerived,
    Quarantined,
    Unrecoverable,
}
