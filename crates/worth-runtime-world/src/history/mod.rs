mod catalog;
mod commit;
mod parentage;
mod reclamation;
mod retention;

pub(crate) use catalog::{
    CompositeHistoryCatalog, CompositeHistoryCatalogDenial, ReservedCompositeCommitCapacity,
    RuntimeWorldHistoryCatalogContract,
};
#[allow(
    unused_imports,
    reason = "the internal denial is asserted by the real constructor contract test"
)]
pub(crate) use commit::CompositeCommitConstructionDenial;
pub use commit::{
    CompositeCallerCorrelation, CompositeCommitParent, CompositeCommitProvenance,
    CompositeComponentChangePosture, CompositeRuntimeWorldCommit,
    CompositeSignalPublicationIdentity,
};
pub use parentage::OrdinaryParent;
#[cfg(test)]
pub(crate) use reclamation::CompositeHistoryReclamationRequest;
pub(crate) use retention::{
    ExplicitCommitHistoryProtectionObligation, ProductHeadHistoryProtectionObligation,
    ProductUnpublishedHistoryProtectionObligation,
};
