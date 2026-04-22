use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeActiveSubscription, BridgeActiveSubscriptionIdentity, BridgeAdmittedSubscriptionIdentity,
    BridgeSubscriptionBasisIdentity, BridgeSubscriptionCheckpointIdentity,
    BridgeSubscriptionCounters, BridgeSubscriptionDeliveryCostProfileIdentity,
    BridgeSubscriptionDeliveryFamilyIdentity, BridgeSubscriptionDeliveryFamilyKind,
    BridgeSubscriptionDeliveryReplayPlanIdentity,
    BridgeSubscriptionDeliveryReplayReadinessIdentity, BridgeSubscriptionDeliveryWindowIdentity,
    BridgeSubscriptionDeliveryWindowSealed, BridgeSubscriptionDuplicateReplayPolicyKind,
    BridgeSubscriptionFanoutDeliveryProjectionSet, BridgeSubscriptionResumeAdmission,
    BridgeSubscriptionResumeAdmissionIdentity,
    BridgeSubscriptionRetainedDeliveryReplaySeedIdentity,
    BridgeSubscriptionRetainedDeliveryWindowSeedIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionDeliveryReplayReadinessClass {
    DescriptorOnlyReplayReady,
    CanonicalMemberReplayReady,
    ReplayBlockedByOmittedPayload,
    ReplayBlockedByDiagnosticsPolicy,
    ReplayBlockedByUnsupportedFamily,
}

impl BridgeSubscriptionDeliveryReplayReadinessClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DescriptorOnlyReplayReady => "descriptor_only_replay_ready",
            Self::CanonicalMemberReplayReady => "canonical_member_replay_ready",
            Self::ReplayBlockedByOmittedPayload => "replay_blocked_by_omitted_payload",
            Self::ReplayBlockedByDiagnosticsPolicy => "replay_blocked_by_diagnostics_policy",
            Self::ReplayBlockedByUnsupportedFamily => "replay_blocked_by_unsupported_family",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionDeliveryWindowReplayReadiness {
    delivery_replay_readiness_identity: BridgeSubscriptionDeliveryReplayReadinessIdentity,
    readiness_class: BridgeSubscriptionDeliveryReplayReadinessClass,
    delivery_window_identity: BridgeSubscriptionDeliveryWindowIdentity,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionDeliveryWindowReplayReadiness {
    fn classify(
        sealed_window: &BridgeSubscriptionDeliveryWindowSealed,
    ) -> BridgeSubscriptionDeliveryReplayReadinessClass {
        match sealed_window.delivery_family().family_kind() {
            BridgeSubscriptionDeliveryFamilyKind::ReplayAuditDescriptor
            | BridgeSubscriptionDeliveryFamilyKind::RouteFocusedDescriptor => {
                BridgeSubscriptionDeliveryReplayReadinessClass::DescriptorOnlyReplayReady
            }
            BridgeSubscriptionDeliveryFamilyKind::CanonicalMember
            | BridgeSubscriptionDeliveryFamilyKind::AdmittedCoalesced => {
                if sealed_window
                    .members()
                    .iter()
                    .any(|member| member.payload_omitted_reason().is_some())
                {
                    BridgeSubscriptionDeliveryReplayReadinessClass::ReplayBlockedByOmittedPayload
                } else {
                    BridgeSubscriptionDeliveryReplayReadinessClass::CanonicalMemberReplayReady
                }
            }
        }
    }

    pub(crate) fn inspect(sealed_window: &BridgeSubscriptionDeliveryWindowSealed) -> Self {
        let readiness_class = Self::classify(sealed_window);
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-delivery-window-replay-readiness|window={}|family={}|class={}",
            sealed_window.delivery_window_identity().as_str(),
            sealed_window
                .delivery_family()
                .delivery_family_identity()
                .as_str(),
            readiness_class.as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            delivery_replay_readiness_identity:
                BridgeSubscriptionDeliveryReplayReadinessIdentity::new(format!(
                    "bridge-subscription-delivery-replay-readiness-id:sha256:{digest:x}"
                )),
            readiness_class,
            delivery_window_identity: sealed_window.delivery_window_identity().clone(),
            counters: BridgeSubscriptionCounters::from_delivery_replay_readiness_inspection(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-delivery-window-replay-readiness:sha256:{digest:x}"
            )),
        }
    }

    pub fn delivery_replay_readiness_identity(
        &self,
    ) -> &BridgeSubscriptionDeliveryReplayReadinessIdentity {
        &self.delivery_replay_readiness_identity
    }

    pub fn readiness_class(&self) -> BridgeSubscriptionDeliveryReplayReadinessClass {
        self.readiness_class
    }

    pub fn delivery_window_identity(&self) -> &BridgeSubscriptionDeliveryWindowIdentity {
        &self.delivery_window_identity
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

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

    #[cfg(test)]
    pub(crate) fn with_canonical_member_count_for_test(
        mut self,
        canonical_member_count: usize,
    ) -> Self {
        self.canonical_member_count = canonical_member_count;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionDeliveryReplayPlanRejectionKind {
    EmptyRetainedWindowSet,
    ActiveSubscriptionMismatch,
    AdmittedSubscriptionMismatch,
    BasisMismatch,
    DeliveryFamilyMismatch,
    RetainedWindowReplayReadinessBlocked,
    RetainedWindowNotAfterCheckpoint,
    RetainedWindowSequenceAmbiguous,
    RetainedWindowExceedsCostProfile,
}

impl BridgeSubscriptionDeliveryReplayPlanRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyRetainedWindowSet => "empty_retained_window_set",
            Self::ActiveSubscriptionMismatch => "active_subscription_mismatch",
            Self::AdmittedSubscriptionMismatch => "admitted_subscription_mismatch",
            Self::BasisMismatch => "basis_mismatch",
            Self::DeliveryFamilyMismatch => "delivery_family_mismatch",
            Self::RetainedWindowReplayReadinessBlocked => {
                "retained_window_replay_readiness_blocked"
            }
            Self::RetainedWindowNotAfterCheckpoint => "retained_window_not_after_checkpoint",
            Self::RetainedWindowSequenceAmbiguous => "retained_window_sequence_ambiguous",
            Self::RetainedWindowExceedsCostProfile => "retained_window_exceeds_cost_profile",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionDeliveryReplayPlanRejection {
    rejection_kind: BridgeSubscriptionDeliveryReplayPlanRejectionKind,
    rejection_context: Arc<str>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionDeliveryReplayPlanRejection {
    fn new(
        rejection_kind: BridgeSubscriptionDeliveryReplayPlanRejectionKind,
        rejection_context: impl Into<Arc<str>>,
    ) -> Self {
        let rejection_context = rejection_context.into();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-delivery-replay-plan-rejection|kind={}|context={}",
            rejection_kind.as_str(),
            rejection_context.as_ref()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            rejection_context,
            counters: BridgeSubscriptionCounters::from_delivery_replay_plan_rejection(matches!(
                rejection_kind,
                BridgeSubscriptionDeliveryReplayPlanRejectionKind::RetainedWindowExceedsCostProfile
            )),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-delivery-replay-plan-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionDeliveryReplayPlanRejectionKind {
        self.rejection_kind
    }

    pub fn rejection_context(&self) -> &str {
        self.rejection_context.as_ref()
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

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
                BridgeSubscriptionDeliveryReplayReadinessClass::ReplayBlockedByOmittedPayload
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
            if seed.canonical_member_count() > active_subscription.cost_profile().max_member_count()
            {
                return Err(BridgeSubscriptionDeliveryReplayPlanRejection::new(
                    BridgeSubscriptionDeliveryReplayPlanRejectionKind::RetainedWindowExceedsCostProfile,
                    format!(
                        "seed={}|window={}|member-count={}|max-member-count={}",
                        seed.retained_delivery_window_seed_identity().as_str(),
                        seed.delivery_window_identity().as_str(),
                        seed.canonical_member_count(),
                        active_subscription.cost_profile().max_member_count()
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionRetainedDeliveryReplaySeed {
    retained_delivery_replay_seed_identity: BridgeSubscriptionRetainedDeliveryReplaySeedIdentity,
    fanout_projection_set_identity: super::BridgeSubscriptionFanoutDeliveryProjectionSetIdentity,
    delivery_window_identity: BridgeSubscriptionDeliveryWindowIdentity,
    canonical_member_digest_basis: Arc<str>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionRetainedDeliveryReplaySeed {
    pub(crate) fn retain(projection_set: &BridgeSubscriptionFanoutDeliveryProjectionSet) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-retained-delivery-replay-seed|projection-set={}|layout={}|window={}|family={}|member-basis={}|projection-count={}",
            projection_set.fanout_delivery_projection_set_identity().as_str(),
            projection_set.fanout_layout_identity().as_str(),
            projection_set.delivery_window_identity().as_str(),
            projection_set.delivery_family_identity().as_str(),
            projection_set.canonical_member_digest_basis(),
            projection_set.len(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            retained_delivery_replay_seed_identity:
                BridgeSubscriptionRetainedDeliveryReplaySeedIdentity::new(format!(
                    "bridge-subscription-retained-delivery-replay-seed-id:sha256:{digest:x}"
                )),
            fanout_projection_set_identity: projection_set
                .fanout_delivery_projection_set_identity()
                .clone(),
            delivery_window_identity: projection_set.delivery_window_identity().clone(),
            canonical_member_digest_basis: Arc::from(
                projection_set.canonical_member_digest_basis().to_owned(),
            ),
            counters: BridgeSubscriptionCounters::from_delivery_replay_seed_retention(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-retained-delivery-replay-seed:sha256:{digest:x}"
            )),
        }
    }

    pub fn retained_delivery_replay_seed_identity(
        &self,
    ) -> &BridgeSubscriptionRetainedDeliveryReplaySeedIdentity {
        &self.retained_delivery_replay_seed_identity
    }

    pub fn fanout_projection_set_identity(
        &self,
    ) -> &super::BridgeSubscriptionFanoutDeliveryProjectionSetIdentity {
        &self.fanout_projection_set_identity
    }

    pub fn delivery_window_identity(&self) -> &BridgeSubscriptionDeliveryWindowIdentity {
        &self.delivery_window_identity
    }

    pub fn canonical_member_digest_basis(&self) -> &str {
        self.canonical_member_digest_basis.as_ref()
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
