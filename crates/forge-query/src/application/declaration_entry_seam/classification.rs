#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationEntrySeamClassification {
    CanonicalReuse,
    QueryBoundaryAdapter,
    CompatibilityDebt,
    DeferredNeighbor,
    ForbiddenDuplicate,
}

impl ForgeQueryDeclarationEntrySeamClassification {
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
pub enum ForgeQueryDeclarationEntryLowerOwnerCrate {
    Query,
    ForgeRelational,
    ForgeRuntimeBridge,
    ForgeSignal,
}

impl ForgeQueryDeclarationEntryLowerOwnerCrate {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Query => "forge_query",
            Self::ForgeRelational => "forge_relational",
            Self::ForgeRuntimeBridge => "forge_runtime_bridge",
            Self::ForgeSignal => "forge_signal",
        }
    }
}
