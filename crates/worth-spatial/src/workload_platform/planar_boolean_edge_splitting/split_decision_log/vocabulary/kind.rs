#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PlanarBooleanSplitDecisionKind {
    QueryDecisionLogDeclared,
    EndpointNoOpRecorded,
    IntervalSubdivisionRetained,
    MicroIntervalCollapsed,
    MicroIntervalPolicyRequired,
    SplitVertexCoalesced,
    SplitFragmentCreated,
    SplitFragmentCoverageValidated,
    OverlapChainCoverageValidated,
    PersistentNamePropagated,
    SplitPhaseDenied,
}

impl PlanarBooleanSplitDecisionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::QueryDecisionLogDeclared => "query_decision_log_declared",
            Self::EndpointNoOpRecorded => "endpoint_noop_recorded",
            Self::IntervalSubdivisionRetained => "interval_subdivision_retained",
            Self::MicroIntervalCollapsed => "micro_interval_collapsed",
            Self::MicroIntervalPolicyRequired => "micro_interval_policy_required",
            Self::SplitVertexCoalesced => "split_vertex_coalesced",
            Self::SplitFragmentCreated => "split_fragment_created",
            Self::SplitFragmentCoverageValidated => "split_fragment_coverage_validated",
            Self::OverlapChainCoverageValidated => "overlap_chain_coverage_validated",
            Self::PersistentNamePropagated => "persistent_name_propagated",
            Self::SplitPhaseDenied => "split_phase_denied",
        }
    }
}
