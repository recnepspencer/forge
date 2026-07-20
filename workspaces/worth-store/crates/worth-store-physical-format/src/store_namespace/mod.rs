mod classification;
mod identity_boundary;
mod identity_canonical_basis;
mod identity_record;
mod namespace_roles;
mod namespace_version;
mod staged_name;
mod store_identity;

pub use classification::{
    classify_store_namespace, NamespaceAmbiguity, NamespaceContention, NamespaceDamage,
    NamespaceEntryObservation, NamespaceEntryType, NamespaceRootObservation,
    StoreNamespaceClassification,
};
pub use identity_boundary::{
    BridgedStoreNamespaceIdentityBoundary, StoreNamespaceIdentityBoundary,
    StoreNamespaceIdentityReadmissionDenial,
};
pub use identity_canonical_basis::{
    prepare_external_store_namespace_identity_canonical_basis,
    prepare_store_namespace_identity_canonical_basis, ExternalStoreNamespaceIdentityMeaning,
    StoreNamespaceIdentityCanonicalBasisOutcome, StoreNamespaceIdentityCanonicalMeaning,
    StoreNamespaceIdentityPublicationPosture,
};
pub use identity_record::{
    StoreNamespaceIdentityDecodeError, StoreNamespaceIdentityRecord,
    STORE_NAMESPACE_IDENTITY_RECORD_LENGTH,
};
pub use namespace_roles::StoreNamespaceRelativeRole;
pub use namespace_version::StoreNamespaceVersion;
pub use staged_name::{NamespaceInitializationAttempt, StagedNamespaceName};
pub use store_identity::{ProposedStoreIdentity, StableStoreIdentity};

#[cfg(test)]
mod tests;
