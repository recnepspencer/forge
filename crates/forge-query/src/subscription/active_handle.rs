use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::identity_authority::{QueryProjectionIdentity, QuerySubscriptionIdentityKind};

use super::active_digest::ActiveSubscriptionLaneDigest;
use super::evidence_projection::subscription_evidence_projection;
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

    pub(crate) fn lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        &self.lane_digest
    }

    pub fn future_selection(&self) -> &QuerySubscriptionFutureSelection {
        &self.future_selection
    }

    pub fn basis_binding_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.basis_binding_identity)
    }

    pub fn basis_binding_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.basis_binding_identity
    }

    pub fn checkpoint_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.checkpoint_identity)
    }

    pub fn checkpoint_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.checkpoint_identity
    }

    pub fn lane_index(&self) -> u64 {
        self.lane_index
    }

    pub fn registry_generation(&self) -> u64 {
        self.registry_generation
    }
}
