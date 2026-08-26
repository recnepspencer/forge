//! Exact-identity repository interaction contract.

use worth_query_installation::facade::WorthQueryPortableDomainPackageIdentity;

use crate::envelope::WorthQueryPackageEnvelopeLimits;

use super::{
    WorthQueryPackageArchiveRepositoryDenial, WorthQueryPackageArchiveStoreIndeterminate,
    WorthQuerySignedPackageArchiveRecord, WorthQueryUntrustedLoadedPackageArchive,
};

/// Independently selected package identity and bound for one repository read.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryExactPackageArchiveRequest {
    expected_package_identity: WorthQueryPortableDomainPackageIdentity,
    envelope_limits: WorthQueryPackageEnvelopeLimits,
}

impl WorthQueryExactPackageArchiveRequest {
    pub(crate) fn new_narrowed(
        expected_package_identity: WorthQueryPortableDomainPackageIdentity,
        envelope_limits: WorthQueryPackageEnvelopeLimits,
    ) -> Self {
        Self {
            expected_package_identity,
            envelope_limits: envelope_limits.narrowed(),
        }
    }

    pub fn new(
        expected_package_identity: WorthQueryPortableDomainPackageIdentity,
        envelope_limits: WorthQueryPackageEnvelopeLimits,
    ) -> Self {
        Self::new_narrowed(expected_package_identity, envelope_limits)
    }

    pub const fn expected_package_identity(&self) -> &WorthQueryPortableDomainPackageIdentity {
        &self.expected_package_identity
    }

    pub const fn envelope_limits(&self) -> WorthQueryPackageEnvelopeLimits {
        self.envelope_limits
    }
}

/// Same claimed package identity was already bound to different exact bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPackageArchiveIdentityConflict {
    claimed_package_identity: WorthQueryPortableDomainPackageIdentity,
}

impl WorthQueryPackageArchiveIdentityConflict {
    pub fn new(claimed_package_identity: WorthQueryPortableDomainPackageIdentity) -> Self {
        Self {
            claimed_package_identity,
        }
    }

    pub const fn claimed_package_identity(&self) -> &WorthQueryPortableDomainPackageIdentity {
        &self.claimed_package_identity
    }
}

/// Typed result of one immutable exact-record store attempt.
#[must_use = "a repository store outcome must be handled"]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryPackageArchiveStoreOutcome {
    Stored,
    AlreadyStoredExact,
    IdentityConflict(WorthQueryPackageArchiveIdentityConflict),
    Denied(WorthQueryPackageArchiveRepositoryDenial),
    Indeterminate(WorthQueryPackageArchiveStoreIndeterminate),
}

/// Typed result of one bounded exact-identity repository read.
#[must_use = "a repository load outcome must be handled"]
#[derive(Debug, Eq, PartialEq)]
pub enum WorthQueryPackageArchiveLoadOutcome {
    Found(WorthQueryUntrustedLoadedPackageArchive),
    NotFound,
    Denied(WorthQueryPackageArchiveRepositoryDenial),
}

/// Descriptive package-envelope storage port implemented by a physical adapter.
///
/// An implementation is already bound to its adapter namespace and durable
/// runtime stream by the composition root. It must not select a release by a
/// mutable name, overwrite different bytes for the same claimed package
/// identity, or materialize more bytes than the exact read request permits.
/// Repository success grants no host trust, Query validity, or activation.
pub trait WorthQueryPackageArchiveRepository: Send + Sync + 'static {
    fn store_exact(
        &self,
        record: &WorthQuerySignedPackageArchiveRecord,
    ) -> WorthQueryPackageArchiveStoreOutcome;

    fn load_exact(
        &self,
        request: WorthQueryExactPackageArchiveRequest,
    ) -> WorthQueryPackageArchiveLoadOutcome;
}
