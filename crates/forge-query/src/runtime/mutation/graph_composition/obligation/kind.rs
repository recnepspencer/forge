#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryGraphObligationKind {
    BlockingInvariant,
    SchemaContractValidator,
    AdvisoryObligation,
    PreflightSequencingObligation,
    CapabilityGapScreen,
    OperatingContextGate,
}

impl ForgeQueryGraphObligationKind {
    pub const ALL: [Self; 6] = [
        Self::BlockingInvariant,
        Self::SchemaContractValidator,
        Self::AdvisoryObligation,
        Self::PreflightSequencingObligation,
        Self::CapabilityGapScreen,
        Self::OperatingContextGate,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::BlockingInvariant => "blocking-invariant",
            Self::SchemaContractValidator => "schema-contract-validator",
            Self::AdvisoryObligation => "advisory-obligation",
            Self::PreflightSequencingObligation => "preflight-sequencing-obligation",
            Self::CapabilityGapScreen => "capability-gap-screen",
            Self::OperatingContextGate => "operating-context-gate",
        }
    }
}
