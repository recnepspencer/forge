#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpatialCompiledProductConsumer {
    EvidenceLookupIndexProduct,
    EvidenceLookupPublicCloseout,
    RetainedCancellationChain,
    RetainedReplayParity,
}

impl SpatialCompiledProductConsumer {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceLookupIndexProduct => "evidence-lookup-index-product",
            Self::EvidenceLookupPublicCloseout => "evidence-lookup-public-closeout",
            Self::RetainedCancellationChain => "retained-cancellation-chain",
            Self::RetainedReplayParity => "retained-replay-parity",
        }
    }
}
