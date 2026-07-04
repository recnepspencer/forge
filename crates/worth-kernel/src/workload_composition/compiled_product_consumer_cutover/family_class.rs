#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum KernelCompiledProductFamilyClass {
    TopologyDerivedEquivalenceContract,
    TopologyDerivedInvalidationDisposition,
    SpatialEvidenceLookupIndex,
    SpatialRetainedReplayWorkload,
    ReplayUndoBoundaryProof,
    KernelOrdinaryConsumerCutoverSummary,
    KernelPublicCloseoutProofChain,
    KernelPublicCloseoutSeed,
    SpatialEvidenceLookupPublicCloseout,
    QueryProjectionConsumption,
    QueryLowerRuntimeBoundaryEnvelope,
}

impl KernelCompiledProductFamilyClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopologyDerivedEquivalenceContract => "topology-derived-equivalence-contract",
            Self::TopologyDerivedInvalidationDisposition => {
                "topology-derived-invalidation-disposition"
            }
            Self::SpatialEvidenceLookupIndex => "spatial-evidence-lookup-index",
            Self::SpatialRetainedReplayWorkload => "spatial-retained-replay-workload",
            Self::ReplayUndoBoundaryProof => "replay-undo-boundary-proof",
            Self::KernelOrdinaryConsumerCutoverSummary => {
                "kernel-ordinary-consumer-cutover-summary"
            }
            Self::KernelPublicCloseoutProofChain => "kernel-public-closeout-proof-chain",
            Self::KernelPublicCloseoutSeed => "kernel-public-closeout-seed",
            Self::SpatialEvidenceLookupPublicCloseout => "spatial-evidence-lookup-public-closeout",
            Self::QueryProjectionConsumption => "query-projection-consumption",
            Self::QueryLowerRuntimeBoundaryEnvelope => "query-lower-runtime-boundary-envelope",
        }
    }
}
