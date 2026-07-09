use crate::error::{BridgeDeliveryErrorKind, BridgeErrorContext, BridgeReplayErrorKind};
use crate::input::envelope::{TruthCommitIdentity, TruthPatchIdentity};
use crate::routing::{
    BridgeInvalidationIdentity, BridgeRouteContractProof, BridgeRouteIdentity,
    BridgeRoutingCounters, BridgeSubscriptionSliceIdentity,
};
use crate::snapshot::TruthSnapshotIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeFailureClass {
    Delivery(BridgeDeliveryErrorKind),
    Replay(BridgeReplayErrorKind),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeFailureRecord {
    failure_class: BridgeFailureClass,
    source_commit: TruthCommitIdentity,
    source_patch: TruthPatchIdentity,
    source_snapshot: TruthSnapshotIdentity,
    route_identity: Option<BridgeRouteIdentity>,
    invalidation_identity: Option<BridgeInvalidationIdentity>,
    subscription_slice_identity: Option<BridgeSubscriptionSliceIdentity>,
    contract_proof: Option<BridgeRouteContractProof>,
    counters: BridgeRoutingCounters,
    detail: String,
    context: BridgeErrorContext,
}

impl BridgeFailureRecord {
    pub(crate) fn from_failure(
        source: crate::diagnostics::BridgeFailureSource,
        failure_class: BridgeFailureClass,
        detail: impl Into<String>,
        context: BridgeErrorContext,
    ) -> Self {
        Self {
            failure_class,
            source_commit: source.source_commit,
            source_patch: source.source_patch,
            source_snapshot: source.source_snapshot,
            route_identity: source.route_identity,
            invalidation_identity: source.invalidation_identity,
            subscription_slice_identity: source.subscription_slice_identity,
            contract_proof: source.contract_proof,
            counters: source.counters,
            detail: detail.into(),
            context,
        }
    }

    pub fn failure_class(&self) -> &BridgeFailureClass {
        &self.failure_class
    }

    pub fn source_commit(&self) -> &TruthCommitIdentity {
        &self.source_commit
    }

    pub fn source_patch(&self) -> &TruthPatchIdentity {
        &self.source_patch
    }

    pub fn source_snapshot(&self) -> &TruthSnapshotIdentity {
        &self.source_snapshot
    }

    pub fn route_identity(&self) -> Option<&BridgeRouteIdentity> {
        self.route_identity.as_ref()
    }

    pub fn invalidation_identity(&self) -> Option<&BridgeInvalidationIdentity> {
        self.invalidation_identity.as_ref()
    }

    pub fn subscription_slice_identity(&self) -> Option<&BridgeSubscriptionSliceIdentity> {
        self.subscription_slice_identity.as_ref()
    }

    pub fn contract_proof(&self) -> Option<&BridgeRouteContractProof> {
        self.contract_proof.as_ref()
    }

    pub fn counters(&self) -> &BridgeRoutingCounters {
        &self.counters
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn context(&self) -> &BridgeErrorContext {
        &self.context
    }
}
