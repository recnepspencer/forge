use std::marker::PhantomData;
use std::time::Instant;

use worth_query_installation::facade::ApplicationSchemaBindingIdentity;
use worth_relational::facade::runtime::RelationalExecutionBasisIdentity;

use crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity;
use crate::domain_computation::primary_graph::application_query::resource_lifecycle::WorthQueryApplicationBasisLease;

/// Move-only authority retaining one exact primary-graph version for a
/// bounded application-query execution.
///
/// The proof is minted only by its owning application runtime. Descriptive
/// version, snapshot, provider, or schema identities cannot construct it.
pub struct WorthQueryApplicationPinnedBasis<Schema> {
    runtime_authority: WorthQueryRuntimeAuthorityIdentity,
    schema_binding: ApplicationSchemaBindingIdentity,
    graph_authority_identity: String,
    provider_identity: String,
    expires_at: Instant,
    lease: WorthQueryApplicationBasisLease,
    _schema: PhantomData<fn() -> Schema>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationPinnedBasisReleaseReceipt {
    basis_identity: RelationalExecutionBasisIdentity,
    released: bool,
}

pub(super) struct WorthQueryApplicationPinnedBasisParts<Schema> {
    pub(super) runtime_authority: WorthQueryRuntimeAuthorityIdentity,
    pub(super) schema_binding: ApplicationSchemaBindingIdentity,
    pub(super) graph_authority_identity: String,
    pub(super) provider_identity: String,
    pub(super) expires_at: Instant,
    pub(super) lease: WorthQueryApplicationBasisLease,
    pub(super) _schema: PhantomData<fn() -> Schema>,
}

impl<Schema> WorthQueryApplicationPinnedBasis<Schema> {
    pub(super) fn new(parts: WorthQueryApplicationPinnedBasisParts<Schema>) -> Self {
        Self {
            runtime_authority: parts.runtime_authority,
            schema_binding: parts.schema_binding,
            graph_authority_identity: parts.graph_authority_identity,
            provider_identity: parts.provider_identity,
            expires_at: parts.expires_at,
            lease: parts.lease,
            _schema: parts._schema,
        }
    }

    pub fn identity(&self) -> &RelationalExecutionBasisIdentity {
        self.lease.identity()
    }

    pub const fn expires_at(&self) -> Instant {
        self.expires_at
    }

    pub fn is_live(&self) -> bool {
        Instant::now() < self.expires_at && self.lease.is_live()
    }

    pub fn release(self) -> WorthQueryApplicationPinnedBasisReleaseReceipt {
        let release = self.lease.release();
        WorthQueryApplicationPinnedBasisReleaseReceipt {
            basis_identity: release.identity().clone(),
            released: release.released(),
        }
    }

    pub(super) const fn runtime_authority(&self) -> WorthQueryRuntimeAuthorityIdentity {
        self.runtime_authority
    }

    pub(super) fn schema_binding(&self) -> &ApplicationSchemaBindingIdentity {
        &self.schema_binding
    }

    pub(super) fn graph_authority_identity(&self) -> &str {
        &self.graph_authority_identity
    }

    pub(super) fn provider_identity(&self) -> &str {
        &self.provider_identity
    }

    pub(super) fn into_lease(self) -> WorthQueryApplicationBasisLease {
        self.lease
    }
}

impl WorthQueryApplicationPinnedBasisReleaseReceipt {
    pub fn basis_identity(&self) -> &RelationalExecutionBasisIdentity {
        &self.basis_identity
    }

    pub const fn released(&self) -> bool {
        self.released
    }
}
