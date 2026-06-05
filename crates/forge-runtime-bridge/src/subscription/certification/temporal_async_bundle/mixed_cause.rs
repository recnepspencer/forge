use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::subscription::{
    BridgeActiveSubscription, BridgeMixedCauseDeliveryWindowPlan,
    BridgeSharedConsumerDeliveryBundleSealed,
};

use super::bundle::{
    BridgeTemporalAsyncCertificationBundleRejection,
    BridgeTemporalAsyncCertificationBundleRejectionKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTemporalAsyncCertificationMixedCauseSection {
    bridge_owner: Arc<str>,
    active_subscription_identity: Arc<str>,
    admitted_subscription_identity: Arc<str>,
    delivery_window_identity: Arc<str>,
    shared_delivery_bundle_identity: Arc<str>,
    shared_delivery_bundle_digest: Arc<str>,
    delivery_family_identity: Arc<str>,
    consumer_count: usize,
    consumer_contract_identities: Arc<[Arc<str>]>,
    ordered_cause_digests: Arc<[Arc<str>]>,
    semantic_digest: Arc<str>,
    digest: Arc<str>,
}

impl BridgeTemporalAsyncCertificationMixedCauseSection {
    pub(crate) fn collect(
        active_subscription: &BridgeActiveSubscription,
        mixed_cause_window: &BridgeMixedCauseDeliveryWindowPlan,
        shared_delivery_bundle: &BridgeSharedConsumerDeliveryBundleSealed,
    ) -> Result<Self, BridgeTemporalAsyncCertificationBundleRejection> {
        if shared_delivery_bundle.active_subscription_identity()
            != active_subscription.active_subscription_identity().as_str()
            || shared_delivery_bundle.admitted_subscription_identity()
                != active_subscription
                    .activation_ready()
                    .admitted()
                    .admitted_subscription_identity()
                    .as_str()
        {
            return Err(BridgeTemporalAsyncCertificationBundleRejection::new(
                BridgeTemporalAsyncCertificationBundleRejectionKind::SharedDeliverySubscriptionMismatch,
                "shared-delivery bundle must come from the same active subscription",
            ));
        }
        if shared_delivery_bundle.mixed_cause_delivery_window_identity()
            != mixed_cause_window.delivery_window_identity().as_str()
        {
            return Err(BridgeTemporalAsyncCertificationBundleRejection::new(
                BridgeTemporalAsyncCertificationBundleRejectionKind::SharedDeliveryWindowMismatch,
                "shared-delivery bundle must retain the exact mixed-cause delivery window identity",
            ));
        }
        let ordered_cause_digests = mixed_cause_window
            .ordered_causes()
            .iter()
            .map(|cause| Arc::from(cause.digest().to_owned()))
            .collect::<Vec<_>>();
        let consumer_contract_identities = shared_delivery_bundle
            .consumer_contract_identities()
            .iter()
            .map(|identity| Arc::from(identity.as_ref().to_owned()))
            .collect::<Vec<_>>();
        let semantic_basis = format!(
            "bridge-temporal-async-certification-mixed-cause-section|window={}|shared-bundle={}|family={}|ordered={}|consumers={}",
            mixed_cause_window.delivery_window_identity().as_str(),
            shared_delivery_bundle.digest(),
            shared_delivery_bundle.delivery_family_identity(),
            ordered_cause_digests
                .iter()
                .map(|digest: &Arc<str>| digest.as_ref())
                .collect::<Vec<_>>()
                .join(","),
            consumer_contract_identities
                .iter()
                .map(|identity: &Arc<str>| identity.as_ref())
                .collect::<Vec<_>>()
                .join(","),
        );
        let semantic_digest = Sha256::digest(semantic_basis.as_bytes());
        let digest = Sha256::digest(
            format!("{semantic_basis}|bridge-owner=forge-runtime-bridge",).as_bytes(),
        );
        Ok(Self {
            bridge_owner: Arc::from("forge-runtime-bridge"),
            active_subscription_identity: Arc::from(
                active_subscription.active_subscription_identity().as_str().to_owned(),
            ),
            admitted_subscription_identity: Arc::from(
                active_subscription
                    .activation_ready()
                    .admitted()
                    .admitted_subscription_identity()
                    .as_str()
                    .to_owned(),
            ),
            delivery_window_identity: Arc::from(
                mixed_cause_window.delivery_window_identity().as_str().to_owned(),
            ),
            shared_delivery_bundle_identity: Arc::from(
                shared_delivery_bundle
                    .shared_delivery_bundle_sealed_identity()
                    .as_str()
                    .to_owned(),
            ),
            shared_delivery_bundle_digest: Arc::from(shared_delivery_bundle.digest().to_owned()),
            delivery_family_identity: Arc::from(
                shared_delivery_bundle.delivery_family_identity().to_owned(),
            ),
            consumer_count: shared_delivery_bundle.consumer_contract_identities().len(),
            consumer_contract_identities: consumer_contract_identities.into(),
            ordered_cause_digests: ordered_cause_digests.into(),
            semantic_digest: Arc::from(format!(
                "bridge-temporal-async-certification-mixed-cause-section-semantic:sha256:{semantic_digest:x}"
            )),
            digest: Arc::from(format!(
                "bridge-temporal-async-certification-mixed-cause-section:sha256:{digest:x}"
            )),
        })
    }

    pub fn bridge_owner(&self) -> &str {
        self.bridge_owner.as_ref()
    }

    pub fn delivery_window_identity(&self) -> &str {
        self.delivery_window_identity.as_ref()
    }

    pub fn consumer_count(&self) -> usize {
        self.consumer_count
    }

    pub fn semantic_digest(&self) -> &str {
        self.semantic_digest.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
