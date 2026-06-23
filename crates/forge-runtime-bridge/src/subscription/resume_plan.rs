use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeActiveSubscriptionIdentity, BridgeSubscriptionCheckpointIdentity,
    BridgeSubscriptionCounters, BridgeSubscriptionDeliveryFamilyIdentity,
    BridgeSubscriptionDeliveryWindowIdentity, BridgeSubscriptionDuplicateReplayPolicyKind,
    BridgeSubscriptionResumeAdmission, BridgeSubscriptionResumeAdmissionIdentity,
    BridgeSubscriptionResumePlanIdentity,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionResumePlan {
    resume_plan_identity: BridgeSubscriptionResumePlanIdentity,
    resume_admission_identity: BridgeSubscriptionResumeAdmissionIdentity,
    checkpoint_identity: BridgeSubscriptionCheckpointIdentity,
    active_subscription_identity: BridgeActiveSubscriptionIdentity,
    delivery_family_identity: BridgeSubscriptionDeliveryFamilyIdentity,
    delivery_window_identity: BridgeSubscriptionDeliveryWindowIdentity,
    delivery_window_sequence: u64,
    resume_after_acknowledged_canonical_sequence: usize,
    expected_next_canonical_sequence: usize,
    acknowledged_prefix_digest: Arc<str>,
    duplicate_replay_policy_kind: BridgeSubscriptionDuplicateReplayPolicyKind,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionResumePlan {
    pub(crate) fn plan(admission: BridgeSubscriptionResumeAdmission) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-resume-plan|admission={}|checkpoint={}|active={}|family={}|window={}|window-sequence={}|resume-after={}|next-sequence={}|ack-prefix={}|duplicate-policy={}",
            admission.resume_admission_identity().as_str(),
            admission.checkpoint_identity().as_str(),
            admission.active_subscription_identity().as_str(),
            admission.delivery_family_identity().as_str(),
            admission.delivery_window_identity().as_str(),
            admission.delivery_window_sequence(),
            admission.acknowledged_canonical_sequence(),
            admission.expected_next_canonical_sequence(),
            admission.acknowledged_prefix_digest(),
            admission.duplicate_replay_policy_kind().as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            resume_plan_identity: BridgeSubscriptionResumePlanIdentity::admit_bridge_owned(
                format!("bridge-subscription-resume-plan-id:sha256:{digest:x}"),
            ),
            resume_admission_identity: admission.resume_admission_identity().clone(),
            checkpoint_identity: admission.checkpoint_identity().clone(),
            active_subscription_identity: admission.active_subscription_identity().clone(),
            delivery_family_identity: admission.delivery_family_identity().clone(),
            delivery_window_identity: admission.delivery_window_identity().clone(),
            delivery_window_sequence: admission.delivery_window_sequence(),
            resume_after_acknowledged_canonical_sequence: admission
                .acknowledged_canonical_sequence(),
            expected_next_canonical_sequence: admission.expected_next_canonical_sequence(),
            acknowledged_prefix_digest: Arc::from(admission.acknowledged_prefix_digest()),
            duplicate_replay_policy_kind: admission.duplicate_replay_policy_kind(),
            counters: BridgeSubscriptionCounters::from_resume_plan(),
            canonical_basis,
            digest: Arc::from(format!("bridge-subscription-resume-plan:sha256:{digest:x}")),
        }
    }

    pub fn resume_plan_identity(&self) -> &BridgeSubscriptionResumePlanIdentity {
        &self.resume_plan_identity
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

    pub fn delivery_family_identity(&self) -> &BridgeSubscriptionDeliveryFamilyIdentity {
        &self.delivery_family_identity
    }

    pub fn delivery_window_identity(&self) -> &BridgeSubscriptionDeliveryWindowIdentity {
        &self.delivery_window_identity
    }

    pub fn delivery_window_sequence(&self) -> u64 {
        self.delivery_window_sequence
    }

    pub fn resume_after_acknowledged_canonical_sequence(&self) -> usize {
        self.resume_after_acknowledged_canonical_sequence
    }

    pub fn expected_next_canonical_sequence(&self) -> usize {
        self.expected_next_canonical_sequence
    }

    pub fn acknowledged_prefix_digest(&self) -> &str {
        self.acknowledged_prefix_digest.as_ref()
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
