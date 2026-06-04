use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeActiveSubscription, BridgeActiveSubscriptionIdentity, BridgeAdmittedSubscriptionIdentity,
    BridgeSubscriptionBasisIdentity, BridgeSubscriptionCheckpoint,
    BridgeSubscriptionCheckpointIdentity, BridgeSubscriptionConsumerContractIdentity,
    BridgeSubscriptionCounters, BridgeSubscriptionDeliveryCostProfileIdentity,
    BridgeSubscriptionDeliveryFamilyIdentity, BridgeSubscriptionDeliveryWindowIdentity,
    BridgeSubscriptionDuplicateReplayPolicyKind, BridgeSubscriptionResumeAdmissionIdentity,
    BridgeSubscriptionResumePlanIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionResumeAdmissionRejectionKind {
    ActiveSubscriptionMismatch,
    AdmittedSubscriptionMismatch,
    BasisMismatch,
    CostProfileMismatch,
    ConsumerContractMismatch,
    UnsealedStreamCheckpointNotAccepted,
    CheckpointTruncated,
    PreviewDiscardedCheckpointNotAccepted,
}

impl BridgeSubscriptionResumeAdmissionRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveSubscriptionMismatch => "active_subscription_mismatch",
            Self::AdmittedSubscriptionMismatch => "admitted_subscription_mismatch",
            Self::BasisMismatch => "basis_mismatch",
            Self::CostProfileMismatch => "cost_profile_mismatch",
            Self::ConsumerContractMismatch => "consumer_contract_mismatch",
            Self::UnsealedStreamCheckpointNotAccepted => "unsealed_stream_checkpoint_not_accepted",
            Self::CheckpointTruncated => "checkpoint_truncated",
            Self::PreviewDiscardedCheckpointNotAccepted => {
                "preview_discarded_checkpoint_not_accepted"
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionResumeAdmissionRejection {
    rejection_kind: BridgeSubscriptionResumeAdmissionRejectionKind,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionResumeAdmissionRejection {
    fn new(rejection_kind: BridgeSubscriptionResumeAdmissionRejectionKind) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-resume-admission-rejection|kind={}",
            rejection_kind.as_str()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            counters: BridgeSubscriptionCounters::from_resume_admission_rejection(
                matches!(
                    rejection_kind,
                    BridgeSubscriptionResumeAdmissionRejectionKind::UnsealedStreamCheckpointNotAccepted
                ),
                matches!(
                    rejection_kind,
                    BridgeSubscriptionResumeAdmissionRejectionKind::CheckpointTruncated
                ),
            ),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-resume-admission-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionResumeAdmissionRejectionKind {
        self.rejection_kind
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionResumeAdmission {
    resume_admission_identity: BridgeSubscriptionResumeAdmissionIdentity,
    checkpoint_identity: BridgeSubscriptionCheckpointIdentity,
    active_subscription_identity: BridgeActiveSubscriptionIdentity,
    admitted_subscription_identity: BridgeAdmittedSubscriptionIdentity,
    basis_identity: BridgeSubscriptionBasisIdentity,
    delivery_family_identity: BridgeSubscriptionDeliveryFamilyIdentity,
    delivery_window_identity: BridgeSubscriptionDeliveryWindowIdentity,
    delivery_window_sequence: u64,
    cost_profile_identity: BridgeSubscriptionDeliveryCostProfileIdentity,
    consumer_contract_identity: BridgeSubscriptionConsumerContractIdentity,
    acknowledged_canonical_sequence: usize,
    expected_next_canonical_sequence: usize,
    acknowledged_prefix_digest: Arc<str>,
    duplicate_replay_policy_kind: BridgeSubscriptionDuplicateReplayPolicyKind,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionResumeAdmission {
    pub(crate) fn admit(
        active_subscription: &BridgeActiveSubscription,
        checkpoint: &BridgeSubscriptionCheckpoint,
    ) -> Result<Self, BridgeSubscriptionResumeAdmissionRejection> {
        if checkpoint.active_subscription_identity()
            != active_subscription.active_subscription_identity()
        {
            return Err(BridgeSubscriptionResumeAdmissionRejection::new(
                BridgeSubscriptionResumeAdmissionRejectionKind::ActiveSubscriptionMismatch,
            ));
        }
        let admitted = active_subscription.activation_ready().admitted();
        if checkpoint.admitted_subscription_identity() != admitted.admitted_subscription_identity()
        {
            return Err(BridgeSubscriptionResumeAdmissionRejection::new(
                BridgeSubscriptionResumeAdmissionRejectionKind::AdmittedSubscriptionMismatch,
            ));
        }
        if checkpoint.basis_identity() != admitted.basis_binding().basis_identity() {
            return Err(BridgeSubscriptionResumeAdmissionRejection::new(
                BridgeSubscriptionResumeAdmissionRejectionKind::BasisMismatch,
            ));
        }
        if checkpoint.cost_profile_identity()
            != active_subscription.cost_profile().cost_profile_identity()
        {
            return Err(BridgeSubscriptionResumeAdmissionRejection::new(
                BridgeSubscriptionResumeAdmissionRejectionKind::CostProfileMismatch,
            ));
        }
        if checkpoint.consumer_contract_identity()
            != active_subscription
                .consumer_contract()
                .consumer_contract_identity()
        {
            return Err(BridgeSubscriptionResumeAdmissionRejection::new(
                BridgeSubscriptionResumeAdmissionRejectionKind::ConsumerContractMismatch,
            ));
        }

        let expected_next_canonical_sequence = checkpoint
            .acknowledged_canonical_sequence()
            .saturating_add(1);
        let duplicate_replay_policy_kind = checkpoint.duplicate_replay_policy().policy_kind();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-resume-admission|checkpoint={}|active={}|admitted={}|basis={}|family={}|window={}|window-sequence={}|cost={}|consumer={}|ack-sequence={}|next-sequence={}|ack-prefix={}|duplicate-policy={}",
            checkpoint.checkpoint_identity().as_str(),
            checkpoint.active_subscription_identity().as_str(),
            checkpoint.admitted_subscription_identity().as_str(),
            checkpoint.basis_identity().as_str(),
            checkpoint.delivery_family_identity().as_str(),
            checkpoint.delivery_window_identity().as_str(),
            checkpoint.delivery_window_sequence(),
            checkpoint.cost_profile_identity().as_str(),
            checkpoint.consumer_contract_identity().as_str(),
            checkpoint.acknowledged_canonical_sequence(),
            expected_next_canonical_sequence,
            checkpoint.acknowledged_prefix_digest(),
            duplicate_replay_policy_kind.as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            resume_admission_identity: BridgeSubscriptionResumeAdmissionIdentity::new(format!(
                "bridge-subscription-resume-admission-id:sha256:{digest:x}"
            )),
            checkpoint_identity: checkpoint.checkpoint_identity().clone(),
            active_subscription_identity: checkpoint.active_subscription_identity().clone(),
            admitted_subscription_identity: checkpoint.admitted_subscription_identity().clone(),
            basis_identity: checkpoint.basis_identity().clone(),
            delivery_family_identity: checkpoint.delivery_family_identity().clone(),
            delivery_window_identity: checkpoint.delivery_window_identity().clone(),
            delivery_window_sequence: checkpoint.delivery_window_sequence(),
            cost_profile_identity: checkpoint.cost_profile_identity().clone(),
            consumer_contract_identity: checkpoint.consumer_contract_identity().clone(),
            acknowledged_canonical_sequence: checkpoint.acknowledged_canonical_sequence(),
            expected_next_canonical_sequence,
            acknowledged_prefix_digest: Arc::from(
                checkpoint.acknowledged_prefix_digest().to_owned(),
            ),
            duplicate_replay_policy_kind,
            counters: BridgeSubscriptionCounters::from_resume_admission(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-resume-admission:sha256:{digest:x}"
            )),
        })
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

    pub fn basis_identity(&self) -> &BridgeSubscriptionBasisIdentity {
        &self.basis_identity
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

    pub fn cost_profile_identity(&self) -> &BridgeSubscriptionDeliveryCostProfileIdentity {
        &self.cost_profile_identity
    }

    pub fn consumer_contract_identity(&self) -> &BridgeSubscriptionConsumerContractIdentity {
        &self.consumer_contract_identity
    }

    pub fn acknowledged_canonical_sequence(&self) -> usize {
        self.acknowledged_canonical_sequence
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
            resume_plan_identity: BridgeSubscriptionResumePlanIdentity::new(format!(
                "bridge-subscription-resume-plan-id:sha256:{digest:x}"
            )),
            resume_admission_identity: admission.resume_admission_identity,
            checkpoint_identity: admission.checkpoint_identity,
            active_subscription_identity: admission.active_subscription_identity,
            delivery_family_identity: admission.delivery_family_identity,
            delivery_window_identity: admission.delivery_window_identity,
            delivery_window_sequence: admission.delivery_window_sequence,
            resume_after_acknowledged_canonical_sequence: admission.acknowledged_canonical_sequence,
            expected_next_canonical_sequence: admission.expected_next_canonical_sequence,
            acknowledged_prefix_digest: admission.acknowledged_prefix_digest,
            duplicate_replay_policy_kind: admission.duplicate_replay_policy_kind,
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
