#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpatialCompiledProductFamilyIdentity {
    EvidenceLookupDerivedSupport,
    RetainedCancellationDerivedSupport,
    RetainedReplayDerivedSupport,
}

impl SpatialCompiledProductFamilyIdentity {
    pub const REQUIRED: [Self; 3] = [
        Self::EvidenceLookupDerivedSupport,
        Self::RetainedCancellationDerivedSupport,
        Self::RetainedReplayDerivedSupport,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceLookupDerivedSupport => "spatial.evidence-lookup-derived-support",
            Self::RetainedCancellationDerivedSupport => {
                "spatial.retained-cancellation-derived-support"
            }
            Self::RetainedReplayDerivedSupport => "spatial.retained-replay-derived-support",
        }
    }
}
