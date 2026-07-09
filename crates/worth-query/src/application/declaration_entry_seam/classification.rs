#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationEntrySeamClassification {
    CanonicalReuse,
    QueryBoundaryAdapter,
    CompatibilityDebt,
    DeferredNeighbor,
    ForbiddenDuplicate,
}

impl WorthQueryDeclarationEntrySeamClassification {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalReuse => "canonical_reuse",
            Self::QueryBoundaryAdapter => "query_boundary_adapter",
            Self::CompatibilityDebt => "compatibility_debt",
            Self::DeferredNeighbor => "deferred_neighbor",
            Self::ForbiddenDuplicate => "forbidden_duplicate",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationEntryLowerOwnerCrate {
    Query,
    WorthRelational,
    WorthRuntimeBridge,
    WorthSignal,
}

impl WorthQueryDeclarationEntryLowerOwnerCrate {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Query => "worth_query",
            Self::WorthRelational => "worth_relational",
            Self::WorthRuntimeBridge => "worth_runtime_bridge",
            Self::WorthSignal => "worth_signal",
        }
    }
}
