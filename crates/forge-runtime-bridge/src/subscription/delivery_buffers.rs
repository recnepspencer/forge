use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeSubscriptionCounters, BridgeSubscriptionDeliveryBufferLifecycleIdentity,
    BridgeSubscriptionDeliveryCostProfile,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionDeliveryBufferPlan {
    buffer_lifecycle_identity: BridgeSubscriptionDeliveryBufferLifecycleIdentity,
    member_capacity: usize,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionDeliveryBufferPlan {
    pub(crate) fn from_cost_profile(cost_profile: &BridgeSubscriptionDeliveryCostProfile) -> Self {
        let member_capacity = cost_profile.max_member_count();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-delivery-buffer-plan|cost-profile={}|member-capacity={}",
            cost_profile.cost_profile_identity().as_str(),
            member_capacity,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            buffer_lifecycle_identity: BridgeSubscriptionDeliveryBufferLifecycleIdentity::new(
                format!("bridge-subscription-delivery-buffer-lifecycle-id:sha256:{digest:x}"),
            ),
            member_capacity,
            counters: BridgeSubscriptionCounters::from_delivery_buffer_plan(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-delivery-buffer-plan:sha256:{digest:x}"
            )),
        }
    }

    pub fn buffer_lifecycle_identity(&self) -> &BridgeSubscriptionDeliveryBufferLifecycleIdentity {
        &self.buffer_lifecycle_identity
    }

    pub fn member_capacity(&self) -> usize {
        self.member_capacity
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
