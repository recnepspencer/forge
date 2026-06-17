#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanarBooleanSplitDecisionReason {
    QueryDecisionLogDeclared,
    EndpointContactDecision,
    IntervalSubdivisionRetained,
    MicroIntervalCollapsed,
    MicroIntervalPolicyRequired,
    SplitVertexCoalesced(String),
    SplitFragmentCreated,
    SplitFragmentCoverageValidated,
    OverlapChainCoverageValidated,
    PersistentNamePropagated,
    SplitPhaseDenied(String),
}

impl PlanarBooleanSplitDecisionReason {
    pub fn reason_name(&self) -> &str {
        match self {
            Self::QueryDecisionLogDeclared => "query_decision_log_declared",
            Self::EndpointContactDecision => "endpoint_contact_decision",
            Self::IntervalSubdivisionRetained => "interval_subdivision_retained",
            Self::MicroIntervalCollapsed => "micro_interval_collapsed",
            Self::MicroIntervalPolicyRequired => "micro_interval_policy_required",
            Self::SplitVertexCoalesced(_) => "split_vertex_coalesced",
            Self::SplitFragmentCreated => "split_fragment_created",
            Self::SplitFragmentCoverageValidated => "split_fragment_coverage_validated",
            Self::OverlapChainCoverageValidated => "overlap_chain_coverage_validated",
            Self::PersistentNamePropagated => "persistent_name_propagated",
            Self::SplitPhaseDenied(_) => "split_phase_denied",
        }
    }

    pub fn detail(&self) -> Option<&str> {
        match self {
            Self::SplitVertexCoalesced(detail) | Self::SplitPhaseDenied(detail) => Some(detail),
            _ => None,
        }
    }
}
