#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryLowerRuntimeCostPosture {
    AuthorityReuse,
    QueryBoundaryAdapter,
    CompatibilityDebt,
    DeferredNeighbor,
    ForbiddenDuplicate,
}

impl WorthQueryLowerRuntimeCostPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AuthorityReuse => "authority-reuse",
            Self::QueryBoundaryAdapter => "query-boundary-adapter",
            Self::CompatibilityDebt => "compatibility-debt",
            Self::DeferredNeighbor => "deferred-neighbor",
            Self::ForbiddenDuplicate => "forbidden-duplicate",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryLowerRuntimeFailureTopology {
    RoutePlanningBoundary,
    ReadmissionHandoffBoundary,
}

impl WorthQueryLowerRuntimeFailureTopology {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RoutePlanningBoundary => "route-planning-boundary",
            Self::ReadmissionHandoffBoundary => "readmission-handoff-boundary",
        }
    }
}
