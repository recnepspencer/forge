use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::super::{
    BridgeActiveSubscription, BridgeActiveSubscriptionIdentity, BridgeAdmittedSubscriptionIdentity,
    BridgeSubscriptionCheckpointIdentity, BridgeSubscriptionCounters,
    BridgeSubscriptionDeliveryCostProfileIdentity, BridgeSubscriptionDeliveryFamilyIdentity,
    BridgeSubscriptionDeliveryReplayPlanIdentity, BridgeSubscriptionDeliveryWindowIdentity,
    BridgeSubscriptionDuplicateReplayPolicyKind, BridgeSubscriptionResumeAdmission,
    BridgeSubscriptionResumeAdmissionIdentity,
};
use super::{
    BridgeSubscriptionDeliveryReplayPlanRejection,
    BridgeSubscriptionDeliveryReplayPlanRejectionKind,
    BridgeSubscriptionDeliveryReplayReadinessClass, BridgeSubscriptionRetainedDeliveryWindowSeed,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionDeliveryReplayPlan {
    delivery_replay_plan_identity: BridgeSubscriptionDeliveryReplayPlanIdentity,
    resume_admission_identity: BridgeSubscriptionResumeAdmissionIdentity,
    checkpoint_identity: BridgeSubscriptionCheckpointIdentity,
    active_subscription_identity: BridgeActiveSubscriptionIdentity,
    admitted_subscription_identity: BridgeAdmittedSubscriptionIdentity,
    cost_profile_identity: BridgeSubscriptionDeliveryCostProfileIdentity,
    delivery_family_identity: BridgeSubscriptionDeliveryFamilyIdentity,
    checkpoint_delivery_window_identity: BridgeSubscriptionDeliveryWindowIdentity,
    checkpoint_delivery_window_sequence: u64,
    retained_window_count: usize,
    retained_member_count: usize,
    retained_window_digest_basis: Arc<str>,
    duplicate_replay_policy_kind: BridgeSubscriptionDuplicateReplayPolicyKind,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionDeliveryReplayPlan {
    pub(crate) fn plan(
        active_subscription: &BridgeActiveSubscription,
        resume_admission: BridgeSubscriptionResumeAdmission,
        mut retained_window_seeds: Vec<BridgeSubscriptionRetainedDeliveryWindowSeed>,
    ) -> Result<Self, BridgeSubscriptionDeliveryReplayPlanRejection> {
        if retained_window_seeds.is_empty() {
            return Err(BridgeSubscriptionDeliveryReplayPlanRejection::new(
                BridgeSubscriptionDeliveryReplayPlanRejectionKind::EmptyRetainedWindowSet,
                format!(
                    "active={}|admission={}|checkpoint={}",
                    active_subscription.active_subscription_identity().as_str(),
                    resume_admission.resume_admission_identity().as_str(),
                    resume_admission.checkpoint_identity().as_str()
                ),
            ));
        }
        if active_subscription.active_subscription_identity()
            != resume_admission.active_subscription_identity()
        {
            return Err(BridgeSubscriptionDeliveryReplayPlanRejection::new(
                BridgeSubscriptionDeliveryReplayPlanRejectionKind::ActiveSubscriptionMismatch,
                format!(
                    "active={}|admission={}|admission-active={}",
                    active_subscription.active_subscription_identity().as_str(),
                    resume_admission.resume_admission_identity().as_str(),
                    resume_admission.active_subscription_identity().as_str()
                ),
            ));
        }

        retained_window_seeds.sort_by_key(|seed| seed.delivery_window_sequence());
        let mut previous_sequence = None;
        let mut retained_member_count = 0usize;
        for seed in &retained_window_seeds {
            if seed.active_subscription_identity()
                != resume_admission.active_subscription_identity()
            {
                return Err(BridgeSubscriptionDeliveryReplayPlanRejection::new(
                    BridgeSubscriptionDeliveryReplayPlanRejectionKind::ActiveSubscriptionMismatch,
                    format!(
                        "seed={}|seed-active={}|admission-active={}",
                        seed.retained_delivery_window_seed_identity().as_str(),
                        seed.active_subscription_identity().as_str(),
                        resume_admission.active_subscription_identity().as_str()
                    ),
                ));
            }
            if seed.admitted_subscription_identity()
                != resume_admission.admitted_subscription_identity()
            {
                return Err(BridgeSubscriptionDeliveryReplayPlanRejection::new(
                    BridgeSubscriptionDeliveryReplayPlanRejectionKind::AdmittedSubscriptionMismatch,
                    format!(
                        "seed={}|seed-admitted={}|admission-admitted={}",
                        seed.retained_delivery_window_seed_identity().as_str(),
                        seed.admitted_subscription_identity().as_str(),
                        resume_admission.admitted_subscription_identity().as_str()
                    ),
                ));
            }
            if seed.basis_identity() != resume_admission.basis_identity() {
                return Err(BridgeSubscriptionDeliveryReplayPlanRejection::new(
                    BridgeSubscriptionDeliveryReplayPlanRejectionKind::BasisMismatch,
                    format!(
                        "seed={}|seed-basis={}|admission-basis={}",
                        seed.retained_delivery_window_seed_identity().as_str(),
                        seed.basis_identity().as_str(),
                        resume_admission.basis_identity().as_str()
                    ),
                ));
            }
            if seed.delivery_family_identity() != resume_admission.delivery_family_identity() {
                return Err(BridgeSubscriptionDeliveryReplayPlanRejection::new(
                    BridgeSubscriptionDeliveryReplayPlanRejectionKind::DeliveryFamilyMismatch,
                    format!(
                        "seed={}|seed-family={}|admission-family={}",
                        seed.retained_delivery_window_seed_identity().as_str(),
                        seed.delivery_family_identity().as_str(),
                        resume_admission.delivery_family_identity().as_str()
                    ),
                ));
            }
            match seed.replay_readiness_class() {
                BridgeSubscriptionDeliveryReplayReadinessClass::DescriptorOnlyReplayReady
                | BridgeSubscriptionDeliveryReplayReadinessClass::CanonicalMemberReplayReady => {}
                BridgeSubscriptionDeliveryReplayReadinessClass::ReplayBlockedByOmittedContent
                | BridgeSubscriptionDeliveryReplayReadinessClass::ReplayBlockedByDiagnosticsPolicy
                | BridgeSubscriptionDeliveryReplayReadinessClass::ReplayBlockedByUnsupportedFamily => {
                    return Err(BridgeSubscriptionDeliveryReplayPlanRejection::new(
                        BridgeSubscriptionDeliveryReplayPlanRejectionKind::RetainedWindowReplayReadinessBlocked,
                        format!(
                            "seed={}|window={}|readiness={}",
                            seed.retained_delivery_window_seed_identity().as_str(),
                            seed.delivery_window_identity().as_str(),
                            seed.replay_readiness_class().as_str()
                        ),
                    ));
                }
            }
            if seed.delivery_window_sequence() <= resume_admission.delivery_window_sequence() {
                return Err(BridgeSubscriptionDeliveryReplayPlanRejection::new(
                    BridgeSubscriptionDeliveryReplayPlanRejectionKind::RetainedWindowNotAfterCheckpoint,
                    format!(
                        "seed={}|window={}|seed-sequence={}|checkpoint-window={}|checkpoint-sequence={}",
                        seed.retained_delivery_window_seed_identity().as_str(),
                        seed.delivery_window_identity().as_str(),
                        seed.delivery_window_sequence(),
                        resume_admission.delivery_window_identity().as_str(),
                        resume_admission.delivery_window_sequence()
                    ),
                ));
            }
            if previous_sequence == Some(seed.delivery_window_sequence()) {
                return Err(BridgeSubscriptionDeliveryReplayPlanRejection::new(
                    BridgeSubscriptionDeliveryReplayPlanRejectionKind::RetainedWindowSequenceAmbiguous,
                    format!(
                        "seed={}|window={}|sequence={}",
                        seed.retained_delivery_window_seed_identity().as_str(),
                        seed.delivery_window_identity().as_str(),
                        seed.delivery_window_sequence()
                    ),
                ));
            }
            retained_member_count =
                retained_member_count.saturating_add(seed.canonical_member_count());
            previous_sequence = Some(seed.delivery_window_sequence());
        }

        let retained_window_digest_basis = Arc::<str>::from(
            retained_window_seeds
                .iter()
                .map(|seed| {
                    format!(
                        "{}:{}:{}:{}:{}",
                        seed.delivery_window_sequence(),
                        seed.retained_delivery_window_seed_identity().as_str(),
                        seed.delivery_window_identity().as_str(),
                        seed.digest(),
                        seed.canonical_member_digest_basis()
                    )
                })
                .collect::<Vec<_>>()
                .join(","),
        );
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-delivery-replay-plan|admission={}|checkpoint={}|active={}|admitted={}|cost={}|family={}|checkpoint-window={}|checkpoint-sequence={}|window-count={}|member-count={}|windows={}|duplicate-policy={}",
            resume_admission.resume_admission_identity().as_str(),
            resume_admission.checkpoint_identity().as_str(),
            resume_admission.active_subscription_identity().as_str(),
            resume_admission.admitted_subscription_identity().as_str(),
            active_subscription.cost_profile().cost_profile_identity().as_str(),
            resume_admission.delivery_family_identity().as_str(),
            resume_admission.delivery_window_identity().as_str(),
            resume_admission.delivery_window_sequence(),
            retained_window_seeds.len(),
            retained_member_count,
            retained_window_digest_basis.as_ref(),
            resume_admission.duplicate_replay_policy_kind().as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            delivery_replay_plan_identity: BridgeSubscriptionDeliveryReplayPlanIdentity::new(
                format!("bridge-subscription-delivery-replay-plan-id:sha256:{digest:x}"),
            ),
            resume_admission_identity: resume_admission.resume_admission_identity().clone(),
            checkpoint_identity: resume_admission.checkpoint_identity().clone(),
            active_subscription_identity: resume_admission.active_subscription_identity().clone(),
            admitted_subscription_identity: resume_admission
                .admitted_subscription_identity()
                .clone(),
            cost_profile_identity: active_subscription
                .cost_profile()
                .cost_profile_identity()
                .clone(),
            delivery_family_identity: resume_admission.delivery_family_identity().clone(),
            checkpoint_delivery_window_identity: resume_admission
                .delivery_window_identity()
                .clone(),
            checkpoint_delivery_window_sequence: resume_admission.delivery_window_sequence(),
            retained_window_count: retained_window_seeds.len(),
            retained_member_count,
            retained_window_digest_basis,
            duplicate_replay_policy_kind: resume_admission.duplicate_replay_policy_kind(),
            counters: BridgeSubscriptionCounters::from_delivery_replay_plan(
                retained_window_seeds.len(),
                retained_member_count,
            ),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-delivery-replay-plan:sha256:{digest:x}"
            )),
        })
    }

    pub fn delivery_replay_plan_identity(&self) -> &BridgeSubscriptionDeliveryReplayPlanIdentity {
        &self.delivery_replay_plan_identity
    }

    pub fn resume_admission_identity(&self) -> &BridgeSubscriptionResumeAdmissionIdentity {
        &self.resume_admission_identity
    }

    pub fn checkpoint_identity(&self) -> &BridgeSubscriptionCheckpointIdentity {
        &self.checkpoint_identity
    }

    pub fn active_subscription_identity(&self) -> &BridgeActiveSubscriptionIdentity {
        &self.active_subscription_identity
    }

    pub fn admitted_subscription_identity(&self) -> &BridgeAdmittedSubscriptionIdentity {
        &self.admitted_subscription_identity
    }

    pub fn cost_profile_identity(&self) -> &BridgeSubscriptionDeliveryCostProfileIdentity {
        &self.cost_profile_identity
    }

    pub fn delivery_family_identity(&self) -> &BridgeSubscriptionDeliveryFamilyIdentity {
        &self.delivery_family_identity
    }

    pub fn checkpoint_delivery_window_identity(&self) -> &BridgeSubscriptionDeliveryWindowIdentity {
        &self.checkpoint_delivery_window_identity
    }

    pub fn checkpoint_delivery_window_sequence(&self) -> u64 {
        self.checkpoint_delivery_window_sequence
    }

    pub fn retained_window_count(&self) -> usize {
        self.retained_window_count
    }

    pub fn retained_member_count(&self) -> usize {
        self.retained_member_count
    }

    pub fn retained_window_digest_basis(&self) -> &str {
        self.retained_window_digest_basis.as_ref()
    }

    pub fn duplicate_replay_policy_kind(&self) -> BridgeSubscriptionDuplicateReplayPolicyKind {
        self.duplicate_replay_policy_kind
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
