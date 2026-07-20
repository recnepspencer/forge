#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphIndexLifecycleOwner {
    QueryRuntime,
    LowerRuntime,
    StoreOwned,
    DomainRegistration,
}

impl WorthQueryGraphIndexLifecycleOwner {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::QueryRuntime => "query_runtime",
            Self::LowerRuntime => "lower_runtime",
            Self::StoreOwned => "store_owned",
            Self::DomainRegistration => "domain_registration",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphIndexLifecycleClass {
    RuntimeMaintained,
    LowerRuntimeOwned,
    PersistentStoreOwned,
    EphemeralRuntimeOwned,
    StoreOwnedRequired,
    AccessCapabilityRegistrationRequired,
    TemporarilyUnavailable,
    Unsupported,
}

impl WorthQueryGraphIndexLifecycleClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RuntimeMaintained => "runtime_maintained",
            Self::LowerRuntimeOwned => "lower_runtime_owned",
            Self::PersistentStoreOwned => "persistent_store_owned",
            Self::EphemeralRuntimeOwned => "ephemeral_runtime_owned",
            Self::StoreOwnedRequired => "store_owned_required",
            Self::AccessCapabilityRegistrationRequired => "access_capability_registration_required",
            Self::TemporarilyUnavailable => "temporarily_unavailable",
            Self::Unsupported => "unsupported",
        }
    }
}
