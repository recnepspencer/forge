use std::marker::PhantomData;
use std::time::Instant;

use worth_query_installation::facade::ApplicationSchemaBindingIdentity;
use worth_relational::facade::runtime::RelationalExecutionBasisIdentity;
#[cfg(test)]
use worth_runtime_bridge::facade::TruthBranchIdentity;
use worth_runtime_bridge::facade::{
    BridgeTruthViewEvaluationRequest, BridgeTruthViewSelector, TruthCommitIdentity,
};

use super::super::resource_lifecycle::WorthQueryApplicationBasisLease;
use crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity;
use crate::domain_computation::primary_graph::WorthQueryApplicationCommitReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationHistoricalRead {
    selector: BridgeTruthViewSelector,
    provider_runtime_instance_id: Option<u64>,
}

impl WorthQueryApplicationHistoricalRead {
    #[cfg(test)]
    pub(crate) fn at_commit(branch: TruthBranchIdentity, commit: TruthCommitIdentity) -> Self {
        Self {
            selector: BridgeTruthViewSelector::historical_commit(branch, commit),
            provider_runtime_instance_id: None,
        }
    }

    pub fn at_application_commit(receipt: &WorthQueryApplicationCommitReceipt) -> Self {
        Self {
            selector: BridgeTruthViewSelector::historical_commit(
                crate::domain_computation::primary_graph::primary_truth_branch_identity(),
                TruthCommitIdentity::from_relational_commit_id(receipt.commit_id().0),
            ),
            provider_runtime_instance_id: Some(receipt.provider_runtime_instance_id()),
        }
    }

    pub(super) const fn provider_runtime_instance_id(&self) -> Option<u64> {
        self.provider_runtime_instance_id
    }

    pub(super) fn into_evaluation_request(self) -> BridgeTruthViewEvaluationRequest {
        BridgeTruthViewEvaluationRequest::new(self.selector)
    }
}

/// Move-only Query authority for one exact historical application snapshot.
pub struct WorthQueryApplicationHistoricalBasis<Schema> {
    pub(super) runtime_authority: WorthQueryRuntimeAuthorityIdentity,
    pub(super) schema_binding: ApplicationSchemaBindingIdentity,
    pub(super) graph_authority_identity: String,
    pub(super) provider_identity: String,
    pub(super) expires_at: Instant,
    pub(super) lease: WorthQueryApplicationBasisLease,
    pub(super) _schema: PhantomData<fn() -> Schema>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationHistoricalBasisReleaseReceipt {
    basis_identity: RelationalExecutionBasisIdentity,
    released: bool,
}

impl<Schema> WorthQueryApplicationHistoricalBasis<Schema> {
    pub fn identity(&self) -> &RelationalExecutionBasisIdentity {
        self.lease.identity()
    }

    pub fn version_id(&self) -> worth_relational::facade::identity::VersionId {
        self.lease.version_id()
    }

    pub const fn expires_at(&self) -> Instant {
        self.expires_at
    }

    pub fn is_live(&self) -> bool {
        Instant::now() < self.expires_at && self.lease.is_live()
    }

    pub fn release(self) -> WorthQueryApplicationHistoricalBasisReleaseReceipt {
        let release = self.lease.release();
        WorthQueryApplicationHistoricalBasisReleaseReceipt {
            basis_identity: release.identity().clone(),
            released: release.released(),
        }
    }

    pub(super) fn into_lease(self) -> WorthQueryApplicationBasisLease {
        self.lease
    }
}

impl WorthQueryApplicationHistoricalBasisReleaseReceipt {
    pub fn basis_identity(&self) -> &RelationalExecutionBasisIdentity {
        &self.basis_identity
    }

    pub const fn released(&self) -> bool {
        self.released
    }
}
