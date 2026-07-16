mod consistency_basis;
mod read_only_capability;
#[cfg(test)]
mod read_only_capability_tests;

pub use consistency_basis::{
    OfflineMediaClosureEntry, OfflineMediaConsistencyBasis, OfflineMediaConsistencyBasisDenial,
};
pub(crate) use read_only_capability::physical_file_identity;
pub use read_only_capability::{
    OfflineMediaFileIdentity, OfflineMediaReadDenial, OfflineMediaReadObservation,
    ReadOnlyOfflineMediaCapability,
};
