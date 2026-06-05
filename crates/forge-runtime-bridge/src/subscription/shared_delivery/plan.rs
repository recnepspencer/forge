use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::subscription::{
    BridgeActiveSubscription, BridgeMixedCauseDeliveryWindowPlan, BridgeOrderedMixedCause,
    BridgeSubscriptionCounters, BridgeSubscriptionFanoutLayout,
    BridgeSubscriptionSharedDeliveryPlanIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSharedConsumerDeliveryPlanRejectionKind {
    PreviewLaneRequiresPreviewSurface,
    ActiveSubscriptionMismatch,
    DeliveryFamilyMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSharedConsumerDeliveryPlanRejection {
    rejection_kind: BridgeSharedConsumerDeliveryPlanRejectionKind,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSharedConsumerDeliveryPlanRejection {
    fn new(rejection_kind: BridgeSharedConsumerDeliveryPlanRejectionKind) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-shared-consumer-delivery-plan-rejection|kind={rejection_kind:?}"
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            counters: BridgeSubscriptionCounters::from_shared_delivery_plan_rejection(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-shared-consumer-delivery-plan-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSharedConsumerDeliveryPlanRejectionKind {
        self.rejection_kind
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSharedConsumerDeliveryPlan {
    shared_delivery_plan_identity: BridgeSubscriptionSharedDeliveryPlanIdentity,
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

impl BridgeSharedConsumerDeliveryPlan {
    pub(crate) fn plan(
        active_subscription: &BridgeActiveSubscription,
        mixed_cause_window: &BridgeMixedCauseDeliveryWindowPlan,
        fanout_layout: &BridgeSubscriptionFanoutLayout,
    ) -> Result<Self, BridgeSharedConsumerDeliveryPlanRejection> {
        if !matches!(
            mixed_cause_window.lane_kind(),
            crate::subscription::BridgeMixedCauseOrderingLaneKind::Authoritative
        ) {
            return Err(BridgeSharedConsumerDeliveryPlanRejection::new(
                BridgeSharedConsumerDeliveryPlanRejectionKind::PreviewLaneRequiresPreviewSurface,
            ));
        }
        if fanout_layout.active_subscription_identity()
            != active_subscription.active_subscription_identity()
        {
            return Err(BridgeSharedConsumerDeliveryPlanRejection::new(
                BridgeSharedConsumerDeliveryPlanRejectionKind::ActiveSubscriptionMismatch,
            ));
        }
        if fanout_layout.delivery_family().delivery_family_identity()
            != mixed_cause_window
                .delivery_family()
                .delivery_family_identity()
        {
            return Err(BridgeSharedConsumerDeliveryPlanRejection::new(
                BridgeSharedConsumerDeliveryPlanRejectionKind::DeliveryFamilyMismatch,
            ));
        }

        let ordered_causes = mixed_cause_window.ordered_causes().to_vec();
        let mut consumer_contract_identities = fanout_layout
            .consumer_bindings()
            .iter()
            .map(|binding| {
                Arc::<str>::from(binding.consumer_contract_identity().as_str().to_owned())
            })
            .collect::<Vec<_>>();
        consumer_contract_identities.sort();

        let admitted = active_subscription.activation_ready().admitted();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-shared-consumer-delivery-plan|active={}|admitted={}|family={}|ordered={}|consumers={}",
            active_subscription.active_subscription_identity().as_str(),
            admitted.admitted_subscription_identity().as_str(),
            super::canonical_bundle_family_token(
                mixed_cause_window.delivery_family().delivery_family_identity().as_str(),
            ),
            ordered_causes
                .iter()
                .map(BridgeOrderedMixedCause::digest)
                .collect::<Vec<_>>()
                .join(","),
            consumer_contract_identities.iter().map(|identity| identity.as_ref()).collect::<Vec<_>>().join(","),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            shared_delivery_plan_identity: BridgeSubscriptionSharedDeliveryPlanIdentity::new(
                format!("bridge-shared-consumer-delivery-plan-id:sha256:{digest:x}"),
            ),
            active_subscription_identity: Arc::from(
                active_subscription
                    .active_subscription_identity()
                    .as_str()
                    .to_owned(),
            ),
            admitted_subscription_identity: Arc::from(
                admitted
                    .admitted_subscription_identity()
                    .as_str()
                    .to_owned(),
            ),
            mixed_cause_delivery_window_identity: Arc::from(
                mixed_cause_window
                    .delivery_window_identity()
                    .as_str()
                    .to_owned(),
            ),
            fanout_layout_identity: Arc::from(
                fanout_layout.fanout_layout_identity().as_str().to_owned(),
            ),
            delivery_family_identity: Arc::from(
                mixed_cause_window
                    .delivery_family()
                    .delivery_family_identity()
                    .as_str()
                    .to_owned(),
            ),
            ordered_causes: ordered_causes.into(),
            consumer_contract_identities: consumer_contract_identities.into(),
            counters: BridgeSubscriptionCounters::from_shared_delivery_plan(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-shared-consumer-delivery-plan:sha256:{digest:x}"
            )),
        })
    }

    pub fn shared_delivery_plan_identity(&self) -> &BridgeSubscriptionSharedDeliveryPlanIdentity {
        &self.shared_delivery_plan_identity
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

    pub fn delivery_family_identity(&self) -> &str {
        self.delivery_family_identity.as_ref()
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
