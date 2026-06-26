use forge_store_contracts::PhysicalAuthorityScope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreAspectNativeDenial {
    IdentityMismatch,
    LocatorIdentityMismatch,
    PhysicalAuthorityScopeMismatch(PhysicalAuthorityScope),
}
