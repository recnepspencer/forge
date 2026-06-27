#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumCompatibilityPosture {
    SameCoverageReused,
    ExplicitReadmissionRequired,
}
