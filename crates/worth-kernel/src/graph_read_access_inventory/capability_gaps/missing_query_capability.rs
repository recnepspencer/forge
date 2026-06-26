#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadMissingQueryCapability {
    PersistentContinuationIndex,
    AsyncMaterializedGraphRead,
    StoreBackedGraphIndex,
    DomainOperationRegistration,
}

impl WorthGraphReadMissingQueryCapability {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PersistentContinuationIndex => "persistent_continuation_index",
            Self::AsyncMaterializedGraphRead => "async_materialized_graph_read",
            Self::StoreBackedGraphIndex => "store_backed_graph_index",
            Self::DomainOperationRegistration => "domain_operation_registration",
        }
    }
}
