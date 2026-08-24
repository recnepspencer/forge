use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Instant;

use super::super::WorthQueryApplicationBasisIdentity;
use worth_query_installation::facade::ApplicationSchemaBindingIdentity;
use worth_runtime_bridge::facade::{
    BridgePreviewSessionLivenessObserver, BridgeSpeculativeSessionHandle,
};

use super::super::resource_lifecycle::WorthQueryApplicationBasisLease;
use super::preview_session_open::WorthQueryApplicationPreviewSessionDenial;
use crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryApplicationPreviewSessionIdentity(Arc<str>);

impl WorthQueryApplicationPreviewSessionIdentity {
    pub(super) fn mint(value: String) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

/// Active preview authority opened by one application runtime.
///
/// The underlying Bridge session cannot be extracted or substituted with its
/// copied identity. Dropping the wrapper performs terminal discard.
pub struct WorthQueryApplicationPreviewSession<Schema> {
    pub(super) runtime_authority: WorthQueryRuntimeAuthorityIdentity,
    pub(super) schema_binding: ApplicationSchemaBindingIdentity,
    pub(super) identity: WorthQueryApplicationPreviewSessionIdentity,
    pub(super) handle: Option<BridgeSpeculativeSessionHandle>,
    pub(super) source_basis: worth_relational::facade::branch::AdmittedRelationalBranchBasis,
    pub(super) source_observation:
        Option<worth_relational::facade::bridge::RelationalBridgeObservationLease>,
    pub(super) _schema: PhantomData<fn() -> Schema>,
}

impl<Schema> WorthQueryApplicationPreviewSession<Schema> {
    pub fn identity(&self) -> &WorthQueryApplicationPreviewSessionIdentity {
        &self.identity
    }

    pub fn discard(
        mut self,
    ) -> Result<
        WorthQueryApplicationPreviewSessionDiscardReceipt,
        WorthQueryApplicationPreviewSessionDenial,
    > {
        self.handle
            .take()
            .expect("an exposed preview session remains active")
            .discard(Vec::new())
            .map(|_| WorthQueryApplicationPreviewSessionDiscardReceipt {
                identity: self.identity.clone(),
                discarded: true,
            })
            .map_err(super::preview_session_open::bridge_denial)
    }

    pub(super) fn handle(&self) -> Option<&BridgeSpeculativeSessionHandle> {
        self.handle.as_ref()
    }

    pub(super) fn source_basis(
        &self,
    ) -> &worth_relational::facade::branch::AdmittedRelationalBranchBasis {
        &self.source_basis
    }
}

impl<Schema> Drop for WorthQueryApplicationPreviewSession<Schema> {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            let _ = handle.discard(Vec::new());
        }
        self.source_observation.take();
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationPreviewSessionDiscardReceipt {
    identity: WorthQueryApplicationPreviewSessionIdentity,
    discarded: bool,
}

impl WorthQueryApplicationPreviewSessionDiscardReceipt {
    pub fn identity(&self) -> &WorthQueryApplicationPreviewSessionIdentity {
        &self.identity
    }

    pub const fn discarded(&self) -> bool {
        self.discarded
    }
}

/// Move-only Query authority for one exact preview snapshot.
pub struct WorthQueryApplicationPreviewBasis<Schema> {
    pub(super) runtime_authority: WorthQueryRuntimeAuthorityIdentity,
    pub(super) schema_binding: ApplicationSchemaBindingIdentity,
    pub(super) graph_authority_identity: String,
    pub(super) provider_identity: String,
    pub(super) preview_session_identity: WorthQueryApplicationPreviewSessionIdentity,
    pub(super) preview_session_liveness: BridgePreviewSessionLivenessObserver,
    pub(super) expires_at: Instant,
    pub(super) lease: WorthQueryApplicationBasisLease,
    pub(super) _schema: PhantomData<fn() -> Schema>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationPreviewBasisReleaseReceipt {
    basis_identity: WorthQueryApplicationBasisIdentity,
    released: bool,
}

impl<Schema> WorthQueryApplicationPreviewBasis<Schema> {
    pub fn identity(&self) -> &WorthQueryApplicationBasisIdentity {
        self.lease.identity()
    }

    pub fn version_id(&self) -> worth_relational::facade::identity::VersionId {
        self.lease.version_id()
    }

    pub fn preview_session_identity(&self) -> &WorthQueryApplicationPreviewSessionIdentity {
        &self.preview_session_identity
    }

    pub const fn expires_at(&self) -> Instant {
        self.expires_at
    }

    pub fn is_live(&self) -> bool {
        Instant::now() < self.expires_at && self.lease.is_live()
    }

    pub fn release(self) -> WorthQueryApplicationPreviewBasisReleaseReceipt {
        let release = self.lease.release();
        WorthQueryApplicationPreviewBasisReleaseReceipt {
            basis_identity: release.identity().clone(),
            released: release.released(),
        }
    }

    pub(super) fn into_lease_and_liveness(
        self,
    ) -> (
        WorthQueryApplicationBasisLease,
        BridgePreviewSessionLivenessObserver,
    ) {
        (self.lease, self.preview_session_liveness)
    }
}

impl WorthQueryApplicationPreviewBasisReleaseReceipt {
    pub fn basis_identity(&self) -> &WorthQueryApplicationBasisIdentity {
        &self.basis_identity
    }

    pub const fn released(&self) -> bool {
        self.released
    }
}
