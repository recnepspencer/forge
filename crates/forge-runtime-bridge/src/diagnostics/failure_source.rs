use crate::routing::{
    BridgeInvalidationIdentity, BridgeRouteContractProof, BridgeRouteIdentity,
    BridgeRoutingCounters, BridgeSubscriptionSliceIdentity,
};
use crate::snapshot::TruthSnapshotIdentity;

#[derive(Debug, Clone)]
pub(crate) struct BridgeFailureSource {
    pub(crate) source_commit: crate::input::envelope::TruthCommitIdentity,
    pub(crate) source_patch: crate::input::envelope::TruthPatchIdentity,
    pub(crate) source_snapshot: TruthSnapshotIdentity,
    pub(crate) route_identity: Option<BridgeRouteIdentity>,
    pub(crate) invalidation_identity: Option<BridgeInvalidationIdentity>,
    pub(crate) subscription_slice_identity: Option<BridgeSubscriptionSliceIdentity>,
    pub(crate) contract_proof: Option<BridgeRouteContractProof>,
    pub(crate) counters: BridgeRoutingCounters,
}

impl BridgeFailureSource {
    pub(crate) fn new(
        source_commit: crate::input::envelope::TruthCommitIdentity,
        source_patch: crate::input::envelope::TruthPatchIdentity,
        source_snapshot: TruthSnapshotIdentity,
        counters: BridgeRoutingCounters,
    ) -> Self {
        Self {
            source_commit,
            source_patch,
            source_snapshot,
            route_identity: None,
            invalidation_identity: None,
            subscription_slice_identity: None,
            contract_proof: None,
            counters,
        }
    }

    pub(crate) fn with_route_identity(mut self, route_identity: BridgeRouteIdentity) -> Self {
        self.route_identity = Some(route_identity);
        self
    }

    pub(crate) fn with_invalidation_identity(
        mut self,
        invalidation_identity: BridgeInvalidationIdentity,
    ) -> Self {
        self.invalidation_identity = Some(invalidation_identity);
        self
    }

    pub(crate) fn with_subscription_slice_identity(
        mut self,
        subscription_slice_identity: BridgeSubscriptionSliceIdentity,
    ) -> Self {
        self.subscription_slice_identity = Some(subscription_slice_identity);
        self
    }

    pub(crate) fn with_contract_proof(mut self, contract_proof: BridgeRouteContractProof) -> Self {
        self.contract_proof = Some(contract_proof);
        self
    }

    pub(crate) fn with_counters(mut self, counters: BridgeRoutingCounters) -> Self {
        self.counters = counters;
        self
    }
}
