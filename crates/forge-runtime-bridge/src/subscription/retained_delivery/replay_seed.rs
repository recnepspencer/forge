use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::super::{
    BridgeSubscriptionCounters, BridgeSubscriptionDeliveryWindowIdentity,
    BridgeSubscriptionFanoutDeliveryProjectionSet,
    BridgeSubscriptionFanoutDeliveryProjectionSetIdentity,
    BridgeSubscriptionRetainedDeliveryReplaySeedIdentity,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionRetainedDeliveryReplaySeed {
    retained_delivery_replay_seed_identity: BridgeSubscriptionRetainedDeliveryReplaySeedIdentity,
    fanout_projection_set_identity: BridgeSubscriptionFanoutDeliveryProjectionSetIdentity,
    delivery_window_identity: BridgeSubscriptionDeliveryWindowIdentity,
    canonical_member_digest_basis: Arc<str>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionRetainedDeliveryReplaySeed {
    pub(crate) fn retain(projection_set: &BridgeSubscriptionFanoutDeliveryProjectionSet) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-retained-delivery-replay-seed|projection-set={}|layout={}|window={}|family={}|member-basis={}|projection-count={}",
            projection_set.fanout_delivery_projection_set_identity().as_str(),
            projection_set.fanout_layout_identity().as_str(),
            projection_set.delivery_window_identity().as_str(),
            projection_set.delivery_family_identity().as_str(),
            projection_set.canonical_member_digest_basis(),
            projection_set.len(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            retained_delivery_replay_seed_identity:
                BridgeSubscriptionRetainedDeliveryReplaySeedIdentity::new(format!(
                    "bridge-subscription-retained-delivery-replay-seed-id:sha256:{digest:x}"
                )),
            fanout_projection_set_identity: projection_set
                .fanout_delivery_projection_set_identity()
                .clone(),
            delivery_window_identity: projection_set.delivery_window_identity().clone(),
            canonical_member_digest_basis: Arc::from(
                projection_set.canonical_member_digest_basis().to_owned(),
            ),
            counters: BridgeSubscriptionCounters::from_delivery_replay_seed_retention(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-retained-delivery-replay-seed:sha256:{digest:x}"
            )),
        }
    }

    pub fn retained_delivery_replay_seed_identity(
        &self,
    ) -> &BridgeSubscriptionRetainedDeliveryReplaySeedIdentity {
        &self.retained_delivery_replay_seed_identity
    }

    pub fn fanout_projection_set_identity(
        &self,
    ) -> &BridgeSubscriptionFanoutDeliveryProjectionSetIdentity {
        &self.fanout_projection_set_identity
    }

    pub fn delivery_window_identity(&self) -> &BridgeSubscriptionDeliveryWindowIdentity {
        &self.delivery_window_identity
    }

    pub fn canonical_member_digest_basis(&self) -> &str {
        self.canonical_member_digest_basis.as_ref()
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
