use std::sync::Arc;

use crate::routing::{
    BridgeInvalidationIdentity, BridgeInvalidationTarget, BridgeSubscriptionSlice,
    BridgeSubscriptionSliceIdentity,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeLoweringDiagnosticsRecord {
    invalidation_identity: BridgeInvalidationIdentity,
    subscription_slice_identity: BridgeSubscriptionSliceIdentity,
    subscription_slices: Arc<[BridgeSubscriptionSlice]>,
    invalidation_targets: Arc<[BridgeInvalidationTarget]>,
}

impl BridgeLoweringDiagnosticsRecord {
    pub(crate) fn new(
        invalidation_identity: BridgeInvalidationIdentity,
        subscription_slice_identity: BridgeSubscriptionSliceIdentity,
        subscription_slices: Arc<[BridgeSubscriptionSlice]>,
        invalidation_targets: Arc<[BridgeInvalidationTarget]>,
    ) -> Self {
        Self {
            invalidation_identity,
            subscription_slice_identity,
            subscription_slices,
            invalidation_targets,
        }
    }

    pub fn invalidation_identity(&self) -> &BridgeInvalidationIdentity {
        &self.invalidation_identity
    }

    pub fn subscription_slice_identity(&self) -> &BridgeSubscriptionSliceIdentity {
        &self.subscription_slice_identity
    }

    pub fn subscription_slices(&self) -> &[BridgeSubscriptionSlice] {
        &self.subscription_slices
    }

    pub fn invalidation_targets(&self) -> &[BridgeInvalidationTarget] {
        &self.invalidation_targets
    }
}
