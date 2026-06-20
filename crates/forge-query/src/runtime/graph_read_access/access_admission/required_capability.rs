#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphReadRequiredCapabilityOwner {
    QueryRuntime,
    LowerRuntime,
    PersistentStore,
    DomainRegistration,
    AsyncMaterializer,
}

impl ForgeQueryGraphReadRequiredCapabilityOwner {
    pub const ALL: [Self; 5] = [
        Self::QueryRuntime,
        Self::LowerRuntime,
        Self::PersistentStore,
        Self::DomainRegistration,
        Self::AsyncMaterializer,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::QueryRuntime => "query_runtime",
            Self::LowerRuntime => "lower_runtime",
            Self::PersistentStore => "persistent_store",
            Self::DomainRegistration => "domain_registration",
            Self::AsyncMaterializer => "async_materializer",
        }
    }
}
