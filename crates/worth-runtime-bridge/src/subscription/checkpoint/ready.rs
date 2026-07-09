use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::super::{BridgeSubscriptionCheckpointReadyIdentity, BridgeSubscriptionCounters};
use super::BridgeSubscriptionAcknowledgementFrontier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCheckpointReady {
    checkpoint_ready_identity: BridgeSubscriptionCheckpointReadyIdentity,
    frontier: BridgeSubscriptionAcknowledgementFrontier,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCheckpointReady {
    pub(crate) fn prepare(frontier: BridgeSubscriptionAcknowledgementFrontier) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-checkpoint-ready|frontier={}",
            frontier.acknowledgement_frontier_identity().as_str()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            checkpoint_ready_identity:
                BridgeSubscriptionCheckpointReadyIdentity::admit_bridge_owned(format!(
                    "bridge-subscription-checkpoint-ready-id:sha256:{digest:x}"
                )),
            frontier,
            counters: BridgeSubscriptionCounters::from_checkpoint_ready(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-checkpoint-ready:sha256:{digest:x}"
            )),
        }
    }

    pub fn checkpoint_ready_identity(&self) -> &BridgeSubscriptionCheckpointReadyIdentity {
        &self.checkpoint_ready_identity
    }

    pub fn frontier(&self) -> &BridgeSubscriptionAcknowledgementFrontier {
        &self.frontier
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
