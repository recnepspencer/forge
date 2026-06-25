#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorthTopologyDiagnosticProjectionPosture {
    Minimal,
    ViolationWitness,
    AdvisoryWitness,
    CertificationComparison,
}

impl WorthTopologyDiagnosticProjectionPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Minimal => "minimal",
            Self::ViolationWitness => "violation-witness",
            Self::AdvisoryWitness => "advisory-witness",
            Self::CertificationComparison => "certification-comparison",
        }
    }
}
