use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::subscription::{
    BridgeOrderedMixedCause, BridgeSubscriptionCounters,
    BridgeSubscriptionSharedDeliveryLayoutIdentity,
};

use super::BridgeSharedConsumerDeliveryPlan;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSharedConsumerDeliveryLayout {
    shared_delivery_layout_identity: BridgeSubscriptionSharedDeliveryLayoutIdentity,
    shared_delivery_plan_identity: Arc<str>,
    active_subscription_identity: Arc<str>,
    admitted_subscription_identity: Arc<str>,
    mixed_cause_delivery_window_identity: Arc<str>,
    fanout_layout_identity: Arc<str>,
    delivery_family_identity: Arc<str>,
    ordered_causes: Arc<[BridgeOrderedMixedCause]>,
    consumer_contract_identities: Arc<[Arc<str>]>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSharedConsumerDeliveryLayout {
    pub(crate) fn build(plan: &BridgeSharedConsumerDeliveryPlan) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-shared-consumer-delivery-layout|active={}|admitted={}|family={}|ordered={}|consumers={}",
            plan.active_subscription_identity(),
            plan.admitted_subscription_identity(),
            super::canonical_bundle_family_token(plan.delivery_family_identity()),
            plan.ordered_causes()
                .iter()
                .map(BridgeOrderedMixedCause::digest)
                .collect::<Vec<_>>()
                .join(","),
            plan.consumer_contract_identities().iter().map(|identity| identity.as_ref()).collect::<Vec<_>>().join(","),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            shared_delivery_layout_identity: BridgeSubscriptionSharedDeliveryLayoutIdentity::new(
                format!("bridge-shared-consumer-delivery-layout-id:sha256:{digest:x}"),
            ),
            shared_delivery_plan_identity: Arc::from(
                plan.shared_delivery_plan_identity().as_str().to_owned(),
            ),
            active_subscription_identity: Arc::from(plan.active_subscription_identity().to_owned()),
            admitted_subscription_identity: Arc::from(
                plan.admitted_subscription_identity().to_owned(),
            ),
            mixed_cause_delivery_window_identity: Arc::from(
                plan.mixed_cause_delivery_window_identity().to_owned(),
            ),
            fanout_layout_identity: Arc::from(plan.fanout_layout_identity().to_owned()),
            delivery_family_identity: Arc::from(plan.delivery_family_identity().to_owned()),
            ordered_causes: plan.ordered_causes().to_vec().into(),
            consumer_contract_identities: plan.consumer_contract_identities().to_vec().into(),
            counters: BridgeSubscriptionCounters::from_shared_delivery_layout(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-shared-consumer-delivery-layout:sha256:{digest:x}"
            )),
        }
    }

    pub fn shared_delivery_layout_identity(
        &self,
    ) -> &BridgeSubscriptionSharedDeliveryLayoutIdentity {
        &self.shared_delivery_layout_identity
    }

    pub fn delivery_family_identity(&self) -> &str {
        self.delivery_family_identity.as_ref()
    }

    pub fn ordered_causes(&self) -> &[BridgeOrderedMixedCause] {
        &self.ordered_causes
    }

    pub fn consumer_contract_identities(&self) -> &[Arc<str>] {
        &self.consumer_contract_identities
    }

    pub fn active_subscription_identity(&self) -> &str {
        self.active_subscription_identity.as_ref()
    }

    pub fn admitted_subscription_identity(&self) -> &str {
        self.admitted_subscription_identity.as_ref()
    }

    pub fn mixed_cause_delivery_window_identity(&self) -> &str {
        self.mixed_cause_delivery_window_identity.as_ref()
    }

    pub fn fanout_layout_identity(&self) -> &str {
        self.fanout_layout_identity.as_ref()
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
