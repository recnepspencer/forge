//! Store-neutral immutable signed-envelope repository contract.

mod contract;
mod denial;
mod record;

pub use contract::{
    WorthQueryExactPackageArchiveRequest, WorthQueryPackageArchiveIdentityConflict,
    WorthQueryPackageArchiveLoadOutcome, WorthQueryPackageArchiveRepository,
    WorthQueryPackageArchiveStoreOutcome,
};
pub use denial::{
    WorthQueryPackageArchiveRepositoryDenial, WorthQueryPackageArchiveRepositoryDenialKind,
    WorthQueryPackageArchiveStoreIndeterminate, WorthQueryPackageArchiveStoreIndeterminateKind,
};
pub use record::{WorthQuerySignedPackageArchiveRecord, WorthQueryUntrustedLoadedPackageArchive};
