use crate::input::envelope::{TruthBranchIdentity, TruthCommitIdentity, TruthPatchIdentity};
use crate::routing::lowering::{BridgeInvalidationIdentity, BridgeSubscriptionSliceIdentity};
use crate::routing::planning::{BridgeRouteIdentity, BridgeRouteSourceSummary};
use crate::snapshot::TruthSnapshotIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRouteOutcomeReference {
    route_identity: BridgeRouteIdentity,
    invalidation_identity: BridgeInvalidationIdentity,
    source: BridgeRouteSourceSummary,
    subscription_slice_identity: BridgeSubscriptionSliceIdentity,
}

impl BridgeRouteOutcomeReference {
    pub(crate) fn new(
        route_identity: BridgeRouteIdentity,
        invalidation_identity: BridgeInvalidationIdentity,
        source: BridgeRouteSourceSummary,
        subscription_slice_identity: BridgeSubscriptionSliceIdentity,
    ) -> Self {
        Self {
            route_identity,
            invalidation_identity,
            source,
            subscription_slice_identity,
        }
    }

    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        &self.route_identity
    }

    pub fn invalidation_identity(&self) -> &BridgeInvalidationIdentity {
        &self.invalidation_identity
    }

    pub fn source(&self) -> &BridgeRouteSourceSummary {
        &self.source
    }

    pub fn source_commit(&self) -> &TruthCommitIdentity {
        self.source.source_commit()
    }

    pub fn source_branch(&self) -> &TruthBranchIdentity {
        self.source.source_branch()
    }

    pub fn source_patch(&self) -> &TruthPatchIdentity {
        self.source.source_patch()
    }

    pub fn source_snapshot(&self) -> &TruthSnapshotIdentity {
        self.source.source_snapshot()
    }

    pub fn subscription_slice_identity(&self) -> &BridgeSubscriptionSliceIdentity {
        &self.subscription_slice_identity
    }
}
