use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::subscription::{
    BridgeActiveSubscription, BridgeSubscriptionCheckpoint,
    BridgeSubscriptionDuplicateReplayPolicyKind, BridgeSubscriptionRetainedResumeBasisIdentity,
};

use super::{
    async_inflight::BridgeRetainedInflightAsyncResumeBasis,
    delivery::BridgeRetainedDeliveryResumeBasis, temporal::BridgeRetainedTemporalResumeBasis,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRetainedSubscriptionResumeBasis {
    retained_resume_basis_identity: BridgeSubscriptionRetainedResumeBasisIdentity,
    active_subscription_identity: Arc<str>,
    admitted_subscription_identity: Arc<str>,
    basis_identity: Arc<str>,
    checkpoint_identity: Arc<str>,
    delivery_family_identity: Arc<str>,
    delivery_window_identity: Arc<str>,
    delivery_window_sequence: u64,
    cost_profile_identity: Arc<str>,
    consumer_contract_identity: Arc<str>,
    acknowledged_prefix_digest: Arc<str>,
    duplicate_replay_policy_kind: BridgeSubscriptionDuplicateReplayPolicyKind,
    fanout_layout_identity: Option<Arc<str>>,
    acknowledged_canonical_sequence: usize,
    expected_next_canonical_sequence: usize,
    temporal_resume_basis: Option<BridgeRetainedTemporalResumeBasis>,
    inflight_async_resume_basis: Option<BridgeRetainedInflightAsyncResumeBasis>,
    delivery_resume_basis: Option<BridgeRetainedDeliveryResumeBasis>,
    retention_complete: bool,
    counters: crate::subscription::BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeRetainedSubscriptionResumeBasis {
    pub(crate) fn capture(
        active_subscription: &BridgeActiveSubscription,
        checkpoint: &BridgeSubscriptionCheckpoint,
        temporal_resume_basis: Option<BridgeRetainedTemporalResumeBasis>,
        inflight_async_resume_basis: Option<BridgeRetainedInflightAsyncResumeBasis>,
        delivery_resume_basis: Option<BridgeRetainedDeliveryResumeBasis>,
        retention_complete: bool,
    ) -> Self {
        let expected_next_canonical_sequence = checkpoint
            .acknowledged_canonical_sequence()
            .saturating_add(1);
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-retained-subscription-resume-basis|active={}|admitted={}|basis={}|checkpoint={}|family={}|window={}|window-sequence={}|cost={}|consumer={}|ack-prefix={}|duplicate-policy={}|fanout={}|ack-sequence={}|next-sequence={}|temporal={}|async={}|delivery={}|retention-complete={retention_complete}",
            active_subscription.active_subscription_identity().as_str(),
            active_subscription
                .activation_ready()
                .admitted()
                .admitted_subscription_identity()
                .as_str(),
            active_subscription
                .activation_ready()
                .admitted()
                .basis_binding()
                .basis_identity()
                .as_str(),
            checkpoint.checkpoint_identity().as_str(),
            checkpoint.delivery_family_identity().as_str(),
            checkpoint.delivery_window_identity().as_str(),
            checkpoint.delivery_window_sequence(),
            active_subscription.cost_profile().cost_profile_identity().as_str(),
            active_subscription
                .consumer_contract()
                .consumer_contract_identity()
                .as_str(),
            checkpoint.acknowledged_prefix_digest(),
            checkpoint.duplicate_replay_policy().policy_kind().as_str(),
            checkpoint
                .fanout_layout_identity()
                .map(|identity| identity.as_str())
                .unwrap_or("-"),
            checkpoint.acknowledged_canonical_sequence(),
            expected_next_canonical_sequence,
            temporal_resume_basis
                .as_ref()
                .map(BridgeRetainedTemporalResumeBasis::digest)
                .unwrap_or("-"),
            inflight_async_resume_basis
                .as_ref()
                .map(BridgeRetainedInflightAsyncResumeBasis::digest)
                .unwrap_or("-"),
            delivery_resume_basis
                .as_ref()
                .map(BridgeRetainedDeliveryResumeBasis::digest)
                .unwrap_or("-"),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            retained_resume_basis_identity: BridgeSubscriptionRetainedResumeBasisIdentity::new(
                format!("bridge-retained-subscription-resume-basis-id:sha256:{digest:x}"),
            ),
            active_subscription_identity: Arc::from(
                active_subscription
                    .active_subscription_identity()
                    .as_str()
                    .to_owned(),
            ),
            admitted_subscription_identity: Arc::from(
                active_subscription
                    .activation_ready()
                    .admitted()
                    .admitted_subscription_identity()
                    .as_str()
                    .to_owned(),
            ),
            basis_identity: Arc::from(
                active_subscription
                    .activation_ready()
                    .admitted()
                    .basis_binding()
                    .basis_identity()
                    .as_str()
                    .to_owned(),
            ),
            checkpoint_identity: Arc::from(checkpoint.checkpoint_identity().as_str().to_owned()),
            delivery_family_identity: Arc::from(
                checkpoint.delivery_family_identity().as_str().to_owned(),
            ),
            delivery_window_identity: Arc::from(
                checkpoint.delivery_window_identity().as_str().to_owned(),
            ),
            delivery_window_sequence: checkpoint.delivery_window_sequence(),
            cost_profile_identity: Arc::from(
                checkpoint.cost_profile_identity().as_str().to_owned(),
            ),
            consumer_contract_identity: Arc::from(
                checkpoint.consumer_contract_identity().as_str().to_owned(),
            ),
            acknowledged_prefix_digest: Arc::from(
                checkpoint.acknowledged_prefix_digest().to_owned(),
            ),
            duplicate_replay_policy_kind: checkpoint.duplicate_replay_policy().policy_kind(),
            fanout_layout_identity: checkpoint
                .fanout_layout_identity()
                .map(|identity| Arc::from(identity.as_str().to_owned())),
            acknowledged_canonical_sequence: checkpoint.acknowledged_canonical_sequence(),
            expected_next_canonical_sequence,
            temporal_resume_basis,
            inflight_async_resume_basis,
            delivery_resume_basis,
            retention_complete,
            counters: crate::subscription::BridgeSubscriptionCounters::from_resume_basis_capture(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-retained-subscription-resume-basis:sha256:{digest:x}"
            )),
        }
    }

    pub fn admitted_subscription_identity(&self) -> &str {
        self.admitted_subscription_identity.as_ref()
    }

    pub fn basis_identity(&self) -> &str {
        self.basis_identity.as_ref()
    }

    pub fn active_subscription_identity(&self) -> &str {
        self.active_subscription_identity.as_ref()
    }

    pub fn checkpoint_identity(&self) -> &str {
        self.checkpoint_identity.as_ref()
    }

    pub fn delivery_family_identity(&self) -> &str {
        self.delivery_family_identity.as_ref()
    }

    pub fn delivery_window_identity(&self) -> &str {
        self.delivery_window_identity.as_ref()
    }

    pub fn delivery_window_sequence(&self) -> u64 {
        self.delivery_window_sequence
    }

    pub fn cost_profile_identity(&self) -> &str {
        self.cost_profile_identity.as_ref()
    }

    pub fn consumer_contract_identity(&self) -> &str {
        self.consumer_contract_identity.as_ref()
    }

    pub fn acknowledged_prefix_digest(&self) -> &str {
        self.acknowledged_prefix_digest.as_ref()
    }

    pub fn duplicate_replay_policy_kind(&self) -> BridgeSubscriptionDuplicateReplayPolicyKind {
        self.duplicate_replay_policy_kind
    }

    pub fn acknowledged_canonical_sequence(&self) -> usize {
        self.acknowledged_canonical_sequence
    }

    pub fn fanout_layout_identity(&self) -> Option<&str> {
        self.fanout_layout_identity.as_deref()
    }

    pub fn temporal_resume_basis(&self) -> Option<&BridgeRetainedTemporalResumeBasis> {
        self.temporal_resume_basis.as_ref()
    }

    pub fn inflight_async_resume_basis(&self) -> Option<&BridgeRetainedInflightAsyncResumeBasis> {
        self.inflight_async_resume_basis.as_ref()
    }

    pub fn delivery_resume_basis(&self) -> Option<&BridgeRetainedDeliveryResumeBasis> {
        self.delivery_resume_basis.as_ref()
    }

    pub fn expected_next_canonical_sequence(&self) -> usize {
        self.expected_next_canonical_sequence
    }

    pub fn retention_complete(&self) -> bool {
        self.retention_complete
    }

    pub fn counters(&self) -> &crate::subscription::BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
