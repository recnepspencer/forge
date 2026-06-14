use crate::evidence_identity::ForgeQueryEvidenceIdentity;

use super::active_digest::ActiveSubscriptionLaneDigest;
use super::future_selection::QuerySubscriptionFutureSelection;

#[derive(Debug, Eq, PartialEq)]
pub struct ActiveSubscriptionLaneHandle {
    lane_digest: ActiveSubscriptionLaneDigest,
    future_selection: QuerySubscriptionFutureSelection,
    basis_binding_identity: ForgeQueryEvidenceIdentity,
    checkpoint_identity: ForgeQueryEvidenceIdentity,
    lane_index: u64,
    registry_generation: u64,
}

impl ActiveSubscriptionLaneHandle {
    pub(super) fn new(
        lane_digest: ActiveSubscriptionLaneDigest,
        future_selection: QuerySubscriptionFutureSelection,
        basis_binding_identity: ForgeQueryEvidenceIdentity,
        checkpoint_identity: ForgeQueryEvidenceIdentity,
        lane_index: u64,
        registry_generation: u64,
    ) -> Self {
        Self {
            lane_digest,
            future_selection,
            basis_binding_identity,
            checkpoint_identity,
            lane_index,
            registry_generation,
        }
    }

    pub fn lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        &self.lane_digest
    }

    pub fn future_selection(&self) -> &QuerySubscriptionFutureSelection {
        &self.future_selection
    }

    pub fn basis_binding_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.basis_binding_identity
    }

    pub fn basis_binding_for_reporting(&self) -> &str {
        self.basis_binding_identity.as_str()
    }

    pub fn checkpoint_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.checkpoint_identity
    }

    pub fn checkpoint_for_reporting(&self) -> &str {
        self.checkpoint_identity.as_str()
    }

    pub fn lane_index(&self) -> u64 {
        self.lane_index
    }

    pub fn registry_generation(&self) -> u64 {
        self.registry_generation
    }
}
