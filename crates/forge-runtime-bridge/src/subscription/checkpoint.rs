use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeActiveSubscription, BridgeActiveSubscriptionIdentity, BridgeAdmittedSubscriptionIdentity,
    BridgeSubscriptionAcknowledgementFrontierIdentity, BridgeSubscriptionBasisIdentity,
    BridgeSubscriptionCheckpointIdentity, BridgeSubscriptionCheckpointReadyIdentity,
    BridgeSubscriptionConsumerContractIdentity, BridgeSubscriptionCounters,
    BridgeSubscriptionDeliveryCostProfileIdentity, BridgeSubscriptionDeliveryFamilyIdentity,
    BridgeSubscriptionDeliveryFamilyKind, BridgeSubscriptionDeliveryMemberIdentity,
    BridgeSubscriptionDeliveryWindowIdentity, BridgeSubscriptionDeliveryWindowSealed,
    BridgeSubscriptionDuplicateReplayPolicyIdentity, BridgeSubscriptionFanoutLayout,
    BridgeSubscriptionFanoutLayoutIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionAcknowledgementFrontierRejectionKind {
    EmptyWindow,
    AcknowledgedSequenceOutOfRange,
    AcknowledgedMemberIdentityMismatch,
    AcknowledgedMemberDigestMismatch,
    DescriptorOnlyFamilyCannotPublishCanonicalCheckpoint,
}

impl BridgeSubscriptionAcknowledgementFrontierRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyWindow => "empty_window",
            Self::AcknowledgedSequenceOutOfRange => "acknowledged_sequence_out_of_range",
            Self::AcknowledgedMemberIdentityMismatch => "acknowledged_member_identity_mismatch",
            Self::AcknowledgedMemberDigestMismatch => "acknowledged_member_digest_mismatch",
            Self::DescriptorOnlyFamilyCannotPublishCanonicalCheckpoint => {
                "descriptor_only_family_cannot_publish_canonical_checkpoint"
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionAcknowledgementFrontierRejection {
    rejection_kind: BridgeSubscriptionAcknowledgementFrontierRejectionKind,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionAcknowledgementFrontierRejection {
    fn new(rejection_kind: BridgeSubscriptionAcknowledgementFrontierRejectionKind) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-acknowledgement-frontier-rejection|kind={}",
            rejection_kind.as_str()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            counters: BridgeSubscriptionCounters::from_acknowledgement_frontier_rejection(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-acknowledgement-frontier-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionAcknowledgementFrontierRejectionKind {
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
pub struct BridgeSubscriptionAcknowledgementFrontier {
    acknowledgement_frontier_identity: BridgeSubscriptionAcknowledgementFrontierIdentity,
    active_subscription_identity: BridgeActiveSubscriptionIdentity,
    admitted_subscription_identity: BridgeAdmittedSubscriptionIdentity,
    delivery_family_identity: BridgeSubscriptionDeliveryFamilyIdentity,
    delivery_window_identity: BridgeSubscriptionDeliveryWindowIdentity,
    delivery_window_sequence: u64,
    basis_identity: BridgeSubscriptionBasisIdentity,
    acknowledged_canonical_sequence: usize,
    acknowledged_member_identity: BridgeSubscriptionDeliveryMemberIdentity,
    acknowledged_member_digest: Arc<str>,
    acknowledged_prefix_digest: Arc<str>,
    diagnostics_reference_identity: super::BridgeSubscriptionDeliveryDiagnosticsReferenceIdentity,
    counter_digest: Arc<str>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionAcknowledgementFrontier {
    pub(crate) fn admit(
        sealed_window: &BridgeSubscriptionDeliveryWindowSealed,
        acknowledged_sequence: usize,
        acknowledged_member_identity: &BridgeSubscriptionDeliveryMemberIdentity,
        acknowledged_member_digest: &str,
    ) -> Result<Self, BridgeSubscriptionAcknowledgementFrontierRejection> {
        if matches!(
            sealed_window.delivery_family().family_kind(),
            BridgeSubscriptionDeliveryFamilyKind::ReplayAuditDescriptor
                | BridgeSubscriptionDeliveryFamilyKind::RouteFocusedDescriptor
        ) {
            return Err(BridgeSubscriptionAcknowledgementFrontierRejection::new(
                BridgeSubscriptionAcknowledgementFrontierRejectionKind::DescriptorOnlyFamilyCannotPublishCanonicalCheckpoint,
            ));
        }
        if sealed_window.members().is_empty() {
            return Err(BridgeSubscriptionAcknowledgementFrontierRejection::new(
                BridgeSubscriptionAcknowledgementFrontierRejectionKind::EmptyWindow,
            ));
        }
        let Some(member) = sealed_window.members().get(acknowledged_sequence) else {
            return Err(BridgeSubscriptionAcknowledgementFrontierRejection::new(
                BridgeSubscriptionAcknowledgementFrontierRejectionKind::AcknowledgedSequenceOutOfRange,
            ));
        };
        if member.delivery_member_identity() != acknowledged_member_identity {
            return Err(BridgeSubscriptionAcknowledgementFrontierRejection::new(
                BridgeSubscriptionAcknowledgementFrontierRejectionKind::AcknowledgedMemberIdentityMismatch,
            ));
        }
        if member.digest() != acknowledged_member_digest {
            return Err(BridgeSubscriptionAcknowledgementFrontierRejection::new(
                BridgeSubscriptionAcknowledgementFrontierRejectionKind::AcknowledgedMemberDigestMismatch,
            ));
        }
        let prefix_basis = sealed_window
            .members()
            .iter()
            .take(acknowledged_sequence + 1)
            .map(|member| member.digest())
            .collect::<Vec<_>>()
            .join(",");
        let acknowledged_prefix_digest = Arc::<str>::from(format!(
            "bridge-subscription-acknowledged-prefix:sha256:{:x}",
            Sha256::digest(prefix_basis.as_bytes())
        ));
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-acknowledgement-frontier|active={}|admitted={}|family={}|window={}|sequence={}|basis={}|ack-sequence={}|ack-member={}|ack-digest={}|prefix={}|diagnostics={}|counter={}",
            sealed_window.active_subscription_identity().as_str(),
            sealed_window.admitted_subscription_identity().as_str(),
            sealed_window.delivery_family().delivery_family_identity().as_str(),
            sealed_window.delivery_window_identity().as_str(),
            sealed_window.delivery_window_sequence(),
            sealed_window.basis_identity().as_str(),
            acknowledged_sequence,
            member.delivery_member_identity().as_str(),
            member.digest(),
            acknowledged_prefix_digest.as_ref(),
            sealed_window.diagnostics_reference().diagnostics_reference_identity().as_str(),
            sealed_window.diagnostics_reference().counter_digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            acknowledgement_frontier_identity:
                BridgeSubscriptionAcknowledgementFrontierIdentity::new(format!(
                    "bridge-subscription-acknowledgement-frontier-id:sha256:{digest:x}"
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
            acknowledged_canonical_sequence: acknowledged_sequence,
            acknowledged_member_identity: member.delivery_member_identity().clone(),
            acknowledged_member_digest: Arc::from(member.digest().to_owned()),
            acknowledged_prefix_digest,
            diagnostics_reference_identity: sealed_window
                .diagnostics_reference()
                .diagnostics_reference_identity()
                .clone(),
            counter_digest: Arc::from(
                sealed_window
                    .diagnostics_reference()
                    .counter_digest()
                    .to_owned(),
            ),
            counters: BridgeSubscriptionCounters::from_acknowledgement_frontier_admission(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-acknowledgement-frontier:sha256:{digest:x}"
            )),
        })
    }

    pub fn acknowledgement_frontier_identity(
        &self,
    ) -> &BridgeSubscriptionAcknowledgementFrontierIdentity {
        &self.acknowledgement_frontier_identity
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

    pub fn delivery_window_identity(&self) -> &BridgeSubscriptionDeliveryWindowIdentity {
        &self.delivery_window_identity
    }

    pub fn delivery_window_sequence(&self) -> u64 {
        self.delivery_window_sequence
    }

    pub fn basis_identity(&self) -> &BridgeSubscriptionBasisIdentity {
        &self.basis_identity
    }

    pub fn acknowledged_canonical_sequence(&self) -> usize {
        self.acknowledged_canonical_sequence
    }

    pub fn acknowledged_member_identity(&self) -> &BridgeSubscriptionDeliveryMemberIdentity {
        &self.acknowledged_member_identity
    }

    pub fn acknowledged_member_digest(&self) -> &str {
        self.acknowledged_member_digest.as_ref()
    }

    pub fn acknowledged_prefix_digest(&self) -> &str {
        self.acknowledged_prefix_digest.as_ref()
    }

    pub fn diagnostics_reference_identity(
        &self,
    ) -> &super::BridgeSubscriptionDeliveryDiagnosticsReferenceIdentity {
        &self.diagnostics_reference_identity
    }

    pub fn counter_digest(&self) -> &str {
        self.counter_digest.as_ref()
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCheckpointReady {
    checkpoint_ready_identity: BridgeSubscriptionCheckpointReadyIdentity,
    frontier: BridgeSubscriptionAcknowledgementFrontier,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCheckpointReady {
    pub(crate) fn prepare(frontier: BridgeSubscriptionAcknowledgementFrontier) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-checkpoint-ready|frontier={}",
            frontier.acknowledgement_frontier_identity().as_str()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            checkpoint_ready_identity: BridgeSubscriptionCheckpointReadyIdentity::new(format!(
                "bridge-subscription-checkpoint-ready-id:sha256:{digest:x}"
            )),
            frontier,
            counters: BridgeSubscriptionCounters::from_checkpoint_ready(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-checkpoint-ready:sha256:{digest:x}"
            )),
        }
    }

    pub fn checkpoint_ready_identity(&self) -> &BridgeSubscriptionCheckpointReadyIdentity {
        &self.checkpoint_ready_identity
    }

    pub fn frontier(&self) -> &BridgeSubscriptionAcknowledgementFrontier {
        &self.frontier
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionDuplicateReplayPolicyKind {
    SuppressAcknowledgedMembers,
    RedeliverAcknowledgedMembersWhenIdempotent,
    RejectDuplicateReplay,
}

impl BridgeSubscriptionDuplicateReplayPolicyKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SuppressAcknowledgedMembers => "suppress_acknowledged_members",
            Self::RedeliverAcknowledgedMembersWhenIdempotent => {
                "redeliver_acknowledged_members_when_idempotent"
            }
            Self::RejectDuplicateReplay => "reject_duplicate_replay",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionDuplicateReplayPolicy {
    duplicate_replay_policy_identity: BridgeSubscriptionDuplicateReplayPolicyIdentity,
    policy_kind: BridgeSubscriptionDuplicateReplayPolicyKind,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionDuplicateReplayPolicy {
    pub(crate) fn select(policy_kind: BridgeSubscriptionDuplicateReplayPolicyKind) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-duplicate-replay-policy|kind={}",
            policy_kind.as_str()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            duplicate_replay_policy_identity: BridgeSubscriptionDuplicateReplayPolicyIdentity::new(
                format!("bridge-subscription-duplicate-replay-policy-id:sha256:{digest:x}"),
            ),
            policy_kind,
            counters: BridgeSubscriptionCounters::from_duplicate_replay_policy_selection(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-duplicate-replay-policy:sha256:{digest:x}"
            )),
        }
    }

    pub fn duplicate_replay_policy_identity(
        &self,
    ) -> &BridgeSubscriptionDuplicateReplayPolicyIdentity {
        &self.duplicate_replay_policy_identity
    }

    pub fn policy_kind(&self) -> BridgeSubscriptionDuplicateReplayPolicyKind {
        self.policy_kind
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionCheckpointRejectionKind {
    ActiveSubscriptionMismatch,
    AdmittedSubscriptionMismatch,
    BasisMismatch,
    FanoutLayoutActiveSubscriptionMismatch,
    FanoutLayoutCostProfileMismatch,
    FanoutLayoutDeliveryFamilyMismatch,
}

impl BridgeSubscriptionCheckpointRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveSubscriptionMismatch => "active_subscription_mismatch",
            Self::AdmittedSubscriptionMismatch => "admitted_subscription_mismatch",
            Self::BasisMismatch => "basis_mismatch",
            Self::FanoutLayoutActiveSubscriptionMismatch => {
                "fanout_layout_active_subscription_mismatch"
            }
            Self::FanoutLayoutCostProfileMismatch => "fanout_layout_cost_profile_mismatch",
            Self::FanoutLayoutDeliveryFamilyMismatch => "fanout_layout_delivery_family_mismatch",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCheckpointRejection {
    rejection_kind: BridgeSubscriptionCheckpointRejectionKind,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCheckpointRejection {
    fn new(rejection_kind: BridgeSubscriptionCheckpointRejectionKind) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-checkpoint-rejection|kind={}",
            rejection_kind.as_str()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            counters: BridgeSubscriptionCounters::from_checkpoint_publication_rejection(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-checkpoint-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionCheckpointRejectionKind {
        self.rejection_kind
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCheckpoint {
    checkpoint_identity: BridgeSubscriptionCheckpointIdentity,
    checkpoint_ready_identity: BridgeSubscriptionCheckpointReadyIdentity,
    active_subscription_identity: BridgeActiveSubscriptionIdentity,
    admitted_subscription_identity: BridgeAdmittedSubscriptionIdentity,
    delivery_family_identity: BridgeSubscriptionDeliveryFamilyIdentity,
    delivery_window_identity: BridgeSubscriptionDeliveryWindowIdentity,
    delivery_window_sequence: u64,
    basis_identity: BridgeSubscriptionBasisIdentity,
    acknowledged_canonical_sequence: usize,
    acknowledged_member_identity: BridgeSubscriptionDeliveryMemberIdentity,
    acknowledged_member_digest: Arc<str>,
    acknowledged_prefix_digest: Arc<str>,
    cost_profile_identity: BridgeSubscriptionDeliveryCostProfileIdentity,
    consumer_contract_identity: BridgeSubscriptionConsumerContractIdentity,
    fanout_layout_identity: Option<BridgeSubscriptionFanoutLayoutIdentity>,
    duplicate_replay_policy: BridgeSubscriptionDuplicateReplayPolicy,
    diagnostics_reference_identity: super::BridgeSubscriptionDeliveryDiagnosticsReferenceIdentity,
    counter_digest: Arc<str>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCheckpoint {
    pub(crate) fn publish(
        ready: BridgeSubscriptionCheckpointReady,
        active_subscription: &BridgeActiveSubscription,
        duplicate_replay_policy_kind: BridgeSubscriptionDuplicateReplayPolicyKind,
        fanout_layout: Option<&BridgeSubscriptionFanoutLayout>,
    ) -> Result<Self, BridgeSubscriptionCheckpointRejection> {
        let frontier = ready.frontier();
        if frontier.active_subscription_identity()
            != active_subscription.active_subscription_identity()
        {
            return Err(BridgeSubscriptionCheckpointRejection::new(
                BridgeSubscriptionCheckpointRejectionKind::ActiveSubscriptionMismatch,
            ));
        }
        let admitted = active_subscription.activation_ready().admitted();
        if frontier.admitted_subscription_identity() != admitted.admitted_subscription_identity() {
            return Err(BridgeSubscriptionCheckpointRejection::new(
                BridgeSubscriptionCheckpointRejectionKind::AdmittedSubscriptionMismatch,
            ));
        }
        if frontier.basis_identity() != admitted.basis_binding().basis_identity() {
            return Err(BridgeSubscriptionCheckpointRejection::new(
                BridgeSubscriptionCheckpointRejectionKind::BasisMismatch,
            ));
        }
        if let Some(layout) = fanout_layout {
            if layout.active_subscription_identity()
                != active_subscription.active_subscription_identity()
            {
                return Err(BridgeSubscriptionCheckpointRejection::new(
                    BridgeSubscriptionCheckpointRejectionKind::FanoutLayoutActiveSubscriptionMismatch,
                ));
            }
            if layout.cost_profile_identity()
                != active_subscription.cost_profile().cost_profile_identity()
            {
                return Err(BridgeSubscriptionCheckpointRejection::new(
                    BridgeSubscriptionCheckpointRejectionKind::FanoutLayoutCostProfileMismatch,
                ));
            }
            if layout.delivery_family().delivery_family_identity()
                != frontier.delivery_family_identity()
            {
                return Err(BridgeSubscriptionCheckpointRejection::new(
                    BridgeSubscriptionCheckpointRejectionKind::FanoutLayoutDeliveryFamilyMismatch,
                ));
            }
        }
        let duplicate_replay_policy =
            BridgeSubscriptionDuplicateReplayPolicy::select(duplicate_replay_policy_kind);
        let fanout_layout_identity =
            fanout_layout.map(|layout| layout.fanout_layout_identity().clone());
        let fanout_basis = fanout_layout_identity
            .as_ref()
            .map(BridgeSubscriptionFanoutLayoutIdentity::as_str)
            .unwrap_or("none");
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-checkpoint|ready={}|active={}|admitted={}|family={}|window={}|sequence={}|basis={}|ack-sequence={}|ack-member={}|ack-digest={}|ack-prefix={}|cost={}|consumer={}|fanout={}|duplicate-policy={}|diagnostics={}|counter={}",
            ready.checkpoint_ready_identity().as_str(),
            frontier.active_subscription_identity().as_str(),
            frontier.admitted_subscription_identity().as_str(),
            frontier.delivery_family_identity().as_str(),
            frontier.delivery_window_identity().as_str(),
            frontier.delivery_window_sequence(),
            frontier.basis_identity().as_str(),
            frontier.acknowledged_canonical_sequence(),
            frontier.acknowledged_member_identity().as_str(),
            frontier.acknowledged_member_digest(),
            frontier.acknowledged_prefix_digest(),
            active_subscription.cost_profile().cost_profile_identity().as_str(),
            active_subscription.consumer_contract().consumer_contract_identity().as_str(),
            fanout_basis,
            duplicate_replay_policy.duplicate_replay_policy_identity().as_str(),
            frontier.diagnostics_reference_identity().as_str(),
            frontier.counter_digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            checkpoint_identity: BridgeSubscriptionCheckpointIdentity::new(format!(
                "bridge-subscription-checkpoint-id:sha256:{digest:x}"
            )),
            checkpoint_ready_identity: ready.checkpoint_ready_identity().clone(),
            active_subscription_identity: frontier.active_subscription_identity().clone(),
            admitted_subscription_identity: frontier.admitted_subscription_identity().clone(),
            delivery_family_identity: frontier.delivery_family_identity().clone(),
            delivery_window_identity: frontier.delivery_window_identity().clone(),
            delivery_window_sequence: frontier.delivery_window_sequence(),
            basis_identity: frontier.basis_identity().clone(),
            acknowledged_canonical_sequence: frontier.acknowledged_canonical_sequence(),
            acknowledged_member_identity: frontier.acknowledged_member_identity().clone(),
            acknowledged_member_digest: Arc::from(frontier.acknowledged_member_digest().to_owned()),
            acknowledged_prefix_digest: Arc::from(frontier.acknowledged_prefix_digest().to_owned()),
            cost_profile_identity: active_subscription
                .cost_profile()
                .cost_profile_identity()
                .clone(),
            consumer_contract_identity: active_subscription
                .consumer_contract()
                .consumer_contract_identity()
                .clone(),
            fanout_layout_identity,
            duplicate_replay_policy,
            diagnostics_reference_identity: frontier.diagnostics_reference_identity().clone(),
            counter_digest: Arc::from(frontier.counter_digest().to_owned()),
            counters: BridgeSubscriptionCounters::from_checkpoint_publication(),
            canonical_basis,
            digest: Arc::from(format!("bridge-subscription-checkpoint:sha256:{digest:x}")),
        })
    }

    pub fn checkpoint_identity(&self) -> &BridgeSubscriptionCheckpointIdentity {
        &self.checkpoint_identity
    }

    pub fn checkpoint_ready_identity(&self) -> &BridgeSubscriptionCheckpointReadyIdentity {
        &self.checkpoint_ready_identity
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

    pub fn delivery_window_identity(&self) -> &BridgeSubscriptionDeliveryWindowIdentity {
        &self.delivery_window_identity
    }

    pub fn delivery_window_sequence(&self) -> u64 {
        self.delivery_window_sequence
    }

    pub fn basis_identity(&self) -> &BridgeSubscriptionBasisIdentity {
        &self.basis_identity
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

    pub fn acknowledged_member_identity(&self) -> &BridgeSubscriptionDeliveryMemberIdentity {
        &self.acknowledged_member_identity
    }

    pub fn acknowledged_member_digest(&self) -> &str {
        self.acknowledged_member_digest.as_ref()
    }

    pub fn acknowledged_prefix_digest(&self) -> &str {
        self.acknowledged_prefix_digest.as_ref()
    }

    pub fn duplicate_replay_policy(&self) -> &BridgeSubscriptionDuplicateReplayPolicy {
        &self.duplicate_replay_policy
    }

    pub fn fanout_layout_identity(&self) -> Option<&BridgeSubscriptionFanoutLayoutIdentity> {
        self.fanout_layout_identity.as_ref()
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
