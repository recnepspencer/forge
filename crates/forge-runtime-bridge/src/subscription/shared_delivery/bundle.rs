use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::subscription::{
    BridgeOrderedMixedCause, BridgeSubscriptionCounters,
    BridgeSubscriptionSharedDeliveryBundleDraftIdentity,
    BridgeSubscriptionSharedDeliveryBundleSealedIdentity,
};

use super::BridgeSharedConsumerDeliveryLayout;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSharedConsumerDeliveryBundleDraft {
    shared_delivery_bundle_draft_identity: BridgeSubscriptionSharedDeliveryBundleDraftIdentity,
    layout: BridgeSharedConsumerDeliveryLayout,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSharedConsumerDeliveryBundleSealed {
    shared_delivery_bundle_sealed_identity: BridgeSubscriptionSharedDeliveryBundleSealedIdentity,
    layout_identity: Arc<str>,
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

impl BridgeSharedConsumerDeliveryBundleDraft {
    pub(crate) fn draft(layout: &BridgeSharedConsumerDeliveryLayout) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-shared-consumer-delivery-bundle-draft|ordered={}|consumers={}",
            layout
                .ordered_causes()
                .iter()
                .map(BridgeOrderedMixedCause::digest)
                .collect::<Vec<_>>()
                .join(","),
            layout
                .consumer_contract_identities()
                .iter()
                .map(|identity| identity.as_ref())
                .collect::<Vec<_>>()
                .join(","),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            shared_delivery_bundle_draft_identity:
                BridgeSubscriptionSharedDeliveryBundleDraftIdentity::new(format!(
                    "bridge-shared-consumer-delivery-bundle-draft-id:sha256:{digest:x}"
                )),
            layout: layout.clone(),
            counters: BridgeSubscriptionCounters::from_shared_delivery_bundle_draft(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-shared-consumer-delivery-bundle-draft:sha256:{digest:x}"
            )),
        }
    }

    pub(crate) fn seal(self) -> BridgeSharedConsumerDeliveryBundleSealed {
        BridgeSharedConsumerDeliveryBundleSealed::seal(self)
    }

    pub fn shared_delivery_bundle_draft_identity(
        &self,
    ) -> &BridgeSubscriptionSharedDeliveryBundleDraftIdentity {
        &self.shared_delivery_bundle_draft_identity
    }

    pub fn layout(&self) -> &BridgeSharedConsumerDeliveryLayout {
        &self.layout
    }
}

impl BridgeSharedConsumerDeliveryBundleSealed {
    fn seal(draft: BridgeSharedConsumerDeliveryBundleDraft) -> Self {
        let layout = draft.layout;
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-shared-consumer-delivery-bundle-sealed|active={}|admitted={}|family={}|ordered={}|consumers={}",
            layout.active_subscription_identity(),
            layout.admitted_subscription_identity(),
            super::canonical_bundle_family_token(layout.delivery_family_identity()),
            layout
                .ordered_causes()
                .iter()
                .map(BridgeOrderedMixedCause::digest)
                .collect::<Vec<_>>()
                .join(","),
            layout.consumer_contract_identities().iter().map(|identity| identity.as_ref()).collect::<Vec<_>>().join(","),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            shared_delivery_bundle_sealed_identity:
                BridgeSubscriptionSharedDeliveryBundleSealedIdentity::new(format!(
                    "bridge-shared-consumer-delivery-bundle-sealed-id:sha256:{digest:x}"
                )),
            layout_identity: Arc::from(
                layout.shared_delivery_layout_identity().as_str().to_owned(),
            ),
            active_subscription_identity: Arc::from(
                layout.active_subscription_identity().to_owned(),
            ),
            admitted_subscription_identity: Arc::from(
                layout.admitted_subscription_identity().to_owned(),
            ),
            mixed_cause_delivery_window_identity: Arc::from(
                layout.mixed_cause_delivery_window_identity().to_owned(),
            ),
            fanout_layout_identity: Arc::from(layout.fanout_layout_identity().to_owned()),
            delivery_family_identity: Arc::from(layout.delivery_family_identity().to_owned()),
            ordered_causes: layout.ordered_causes().to_vec().into(),
            consumer_contract_identities: layout.consumer_contract_identities().to_vec().into(),
            counters: BridgeSubscriptionCounters::from_shared_delivery_bundle_sealed(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-shared-consumer-delivery-bundle-sealed:sha256:{digest:x}"
            )),
        }
    }

    pub fn shared_delivery_bundle_sealed_identity(
        &self,
    ) -> &BridgeSubscriptionSharedDeliveryBundleSealedIdentity {
        &self.shared_delivery_bundle_sealed_identity
    }

    pub fn delivery_family_identity(&self) -> &str {
        self.delivery_family_identity.as_ref()
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

    pub fn ordered_causes(&self) -> &[BridgeOrderedMixedCause] {
        &self.ordered_causes
    }

    pub fn consumer_contract_identities(&self) -> &[Arc<str>] {
        &self.consumer_contract_identities
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
