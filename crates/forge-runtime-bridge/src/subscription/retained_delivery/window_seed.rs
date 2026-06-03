use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::super::{
    BridgeActiveSubscriptionIdentity, BridgeAdmittedSubscriptionIdentity,
    BridgeSubscriptionBasisIdentity, BridgeSubscriptionCounters,
    BridgeSubscriptionDeliveryFamilyIdentity, BridgeSubscriptionDeliveryWindowIdentity,
    BridgeSubscriptionDeliveryWindowSealed, BridgeSubscriptionRetainedDeliveryWindowSeedIdentity,
};
use super::{
    BridgeSubscriptionDeliveryReplayReadinessClass, BridgeSubscriptionDeliveryWindowReplayReadiness,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionRetainedDeliveryWindowSeed {
    retained_delivery_window_seed_identity: BridgeSubscriptionRetainedDeliveryWindowSeedIdentity,
    active_subscription_identity: BridgeActiveSubscriptionIdentity,
    admitted_subscription_identity: BridgeAdmittedSubscriptionIdentity,
    delivery_family_identity: BridgeSubscriptionDeliveryFamilyIdentity,
    delivery_window_identity: BridgeSubscriptionDeliveryWindowIdentity,
    delivery_window_sequence: u64,
    basis_identity: BridgeSubscriptionBasisIdentity,
    canonical_member_count: usize,
    canonical_member_digest_basis: Arc<str>,
    replay_readiness_class: BridgeSubscriptionDeliveryReplayReadinessClass,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionRetainedDeliveryWindowSeed {
    pub(crate) fn retain(sealed_window: &BridgeSubscriptionDeliveryWindowSealed) -> Self {
        let replay_readiness_class =
            BridgeSubscriptionDeliveryWindowReplayReadiness::classify(sealed_window);
        let canonical_member_digest_basis = Arc::<str>::from(
            sealed_window
                .members()
                .iter()
                .map(|member| member.digest())
                .collect::<Vec<_>>()
                .join(","),
        );
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-retained-delivery-window-seed|active={}|admitted={}|family={}|window={}|sequence={}|basis={}|member-count={}|members={}|diagnostics={}|counter={}|readiness={}",
            sealed_window.active_subscription_identity().as_str(),
            sealed_window.admitted_subscription_identity().as_str(),
            sealed_window.delivery_family().delivery_family_identity().as_str(),
            sealed_window.delivery_window_identity().as_str(),
            sealed_window.delivery_window_sequence(),
            sealed_window.basis_identity().as_str(),
            sealed_window.members().len(),
            canonical_member_digest_basis.as_ref(),
            sealed_window.diagnostics_reference().diagnostics_reference_identity().as_str(),
            sealed_window.diagnostics_reference().counter_digest(),
            replay_readiness_class.as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            retained_delivery_window_seed_identity:
                BridgeSubscriptionRetainedDeliveryWindowSeedIdentity::new(format!(
                    "bridge-subscription-retained-delivery-window-seed-id:sha256:{digest:x}"
                )),
            active_subscription_identity: sealed_window.active_subscription_identity().clone(),
            admitted_subscription_identity: sealed_window.admitted_subscription_identity().clone(),
            delivery_family_identity: sealed_window
                .delivery_family()
                .delivery_family_identity()
                .clone(),
            delivery_window_identity: sealed_window.delivery_window_identity().clone(),
            delivery_window_sequence: sealed_window.delivery_window_sequence(),
            basis_identity: sealed_window.basis_identity().clone(),
            canonical_member_count: sealed_window.members().len(),
            canonical_member_digest_basis,
            replay_readiness_class,
            counters: BridgeSubscriptionCounters::from_delivery_window_seed_retention(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-retained-delivery-window-seed:sha256:{digest:x}"
            )),
        }
    }

    pub fn retained_delivery_window_seed_identity(
        &self,
    ) -> &BridgeSubscriptionRetainedDeliveryWindowSeedIdentity {
        &self.retained_delivery_window_seed_identity
    }

    pub fn delivery_window_identity(&self) -> &BridgeSubscriptionDeliveryWindowIdentity {
        &self.delivery_window_identity
    }

    pub fn active_subscription_identity(&self) -> &BridgeActiveSubscriptionIdentity {
        &self.active_subscription_identity
    }

    pub fn admitted_subscription_identity(&self) -> &BridgeAdmittedSubscriptionIdentity {
        &self.admitted_subscription_identity
    }

    pub fn delivery_family_identity(&self) -> &BridgeSubscriptionDeliveryFamilyIdentity {
        &self.delivery_family_identity
    }

    pub fn basis_identity(&self) -> &BridgeSubscriptionBasisIdentity {
        &self.basis_identity
    }

    pub fn delivery_window_sequence(&self) -> u64 {
        self.delivery_window_sequence
    }

    pub fn canonical_member_digest_basis(&self) -> &str {
        self.canonical_member_digest_basis.as_ref()
    }

    pub fn canonical_member_count(&self) -> usize {
        self.canonical_member_count
    }

    pub fn replay_readiness_class(&self) -> BridgeSubscriptionDeliveryReplayReadinessClass {
        self.replay_readiness_class
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
