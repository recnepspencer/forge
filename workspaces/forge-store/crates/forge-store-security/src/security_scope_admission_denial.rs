use crate::{StoreCustodyPosture, StoreKeyVersionPosture};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreSecurityScopeAdmissionDenial {
    DeserializedSecurityScopeRequiresReadmission,
    ReplayedAdmissionEvidence,
    WrongPhysicalSecurityScope,
    WrongKeyScope,
    WrongTenantScope,
    MissingAuthenticityRequirement,
    UnsupportedAuthenticityRequirement,
    UnsupportedKeyVersionPosture,
    UnavailableKeyVersionPosture,
    DeniedKeyVersionPosture,
    ExportedCustodyRequiresReadmission,
    ImportedCustodyRequiresReadmission,
    MissingCustodyPosture,
    UnavailableCustodyPosture,
    DeniedCustodyPosture,
    UnsupportedCustodyPosture,
    WrongCustodyPosture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreSecurityScopeAdmissionDeferred {
    CustodyEvidenceDeferred(StoreCustodyPosture),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreSecurityScopeAdmissionStale {
    StaleKeyVersionPosture(StoreKeyVersionPosture),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreSecurityScopeAdmissionRebindRequired {
    KeyVersionRebindRequired(StoreKeyVersionPosture),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreSecurityScopeAdmissionFailure {
    PhysicalAuthorityDrift,
}
