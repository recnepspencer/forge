use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::super::{
    BridgeActiveSubscription, BridgeActiveSubscriptionIdentity, BridgeSubscriptionConsumerContract,
    BridgeSubscriptionConsumerContractIdentity, BridgeSubscriptionCounters,
    BridgeSubscriptionDeliveryCostProfileIdentity, BridgeSubscriptionFanoutPlanIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionFanoutPlanRejectionKind {
    NoAdditionalConsumer,
    ContractFamilyMismatch,
    PacingCapabilityMismatch,
    BackpressurePostureMismatch,
    CoalescingMismatch,
    DiagnosticsRetentionMismatch,
    SharingEligibilityMismatch,
    FanoutWidthExceedsCostProfile,
}

impl BridgeSubscriptionFanoutPlanRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAdditionalConsumer => "no_additional_consumer",
            Self::ContractFamilyMismatch => "contract_family_mismatch",
            Self::PacingCapabilityMismatch => "pacing_capability_mismatch",
            Self::BackpressurePostureMismatch => "backpressure_posture_mismatch",
            Self::CoalescingMismatch => "coalescing_mismatch",
            Self::DiagnosticsRetentionMismatch => "diagnostics_retention_mismatch",
            Self::SharingEligibilityMismatch => "sharing_eligibility_mismatch",
            Self::FanoutWidthExceedsCostProfile => "fanout_width_exceeds_cost_profile",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionFanoutPlanRejection {
    rejection_kind: BridgeSubscriptionFanoutPlanRejectionKind,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionFanoutPlanRejection {
    fn new(rejection_kind: BridgeSubscriptionFanoutPlanRejectionKind) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-fanout-plan-rejection|kind={}",
            rejection_kind.as_str()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            counters: BridgeSubscriptionCounters::from_fanout_plan_rejection(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-fanout-plan-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionFanoutPlanRejectionKind {
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
pub struct BridgeSubscriptionFanoutPlan {
    pub(super) fanout_plan_identity: BridgeSubscriptionFanoutPlanIdentity,
    pub(super) active_subscription_identity: BridgeActiveSubscriptionIdentity,
    pub(super) cost_profile_identity: BridgeSubscriptionDeliveryCostProfileIdentity,
    pub(super) sharing_eligibility_digest: Arc<str>,
    pub(super) primary_consumer_contract_identity: BridgeSubscriptionConsumerContractIdentity,
    pub(super) additional_consumer_contract_identities:
        Arc<[BridgeSubscriptionConsumerContractIdentity]>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionFanoutPlan {
    pub(crate) fn plan(
        active_subscription: &BridgeActiveSubscription,
        additional_consumers: Vec<BridgeSubscriptionConsumerContract>,
    ) -> Result<Self, BridgeSubscriptionFanoutPlanRejection> {
        if additional_consumers.is_empty() {
            return Err(BridgeSubscriptionFanoutPlanRejection::new(
                BridgeSubscriptionFanoutPlanRejectionKind::NoAdditionalConsumer,
            ));
        }
        let total_consumer_width = additional_consumers.len() + 1;
        if total_consumer_width > active_subscription.cost_profile().max_fanout_width() {
            return Err(BridgeSubscriptionFanoutPlanRejection::new(
                BridgeSubscriptionFanoutPlanRejectionKind::FanoutWidthExceedsCostProfile,
            ));
        }

        let primary = active_subscription.consumer_contract();
        for consumer in &additional_consumers {
            if consumer.family() != primary.family() {
                return Err(BridgeSubscriptionFanoutPlanRejection::new(
                    BridgeSubscriptionFanoutPlanRejectionKind::ContractFamilyMismatch,
                ));
            }
            if consumer.pacing_capability() != primary.pacing_capability() {
                return Err(BridgeSubscriptionFanoutPlanRejection::new(
                    BridgeSubscriptionFanoutPlanRejectionKind::PacingCapabilityMismatch,
                ));
            }
            if consumer.backpressure_posture() != primary.backpressure_posture() {
                return Err(BridgeSubscriptionFanoutPlanRejection::new(
                    BridgeSubscriptionFanoutPlanRejectionKind::BackpressurePostureMismatch,
                ));
            }
            if consumer.coalescing_admitted() != primary.coalescing_admitted() {
                return Err(BridgeSubscriptionFanoutPlanRejection::new(
                    BridgeSubscriptionFanoutPlanRejectionKind::CoalescingMismatch,
                ));
            }
            if consumer.diagnostics_retention() != primary.diagnostics_retention() {
                return Err(BridgeSubscriptionFanoutPlanRejection::new(
                    BridgeSubscriptionFanoutPlanRejectionKind::DiagnosticsRetentionMismatch,
                ));
            }
            if consumer.sharing_eligibility().digest() != primary.sharing_eligibility().digest() {
                return Err(BridgeSubscriptionFanoutPlanRejection::new(
                    BridgeSubscriptionFanoutPlanRejectionKind::SharingEligibilityMismatch,
                ));
            }
        }

        let additional_consumer_contract_identities = additional_consumers
            .iter()
            .map(|consumer| consumer.consumer_contract_identity().clone())
            .collect::<Vec<_>>();
        let additional_basis = additional_consumer_contract_identities
            .iter()
            .map(|identity| identity.as_str())
            .collect::<Vec<_>>()
            .join(",");
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-fanout-plan|active={}|cost-profile={}|sharing={}|primary-consumer={}|additional-consumers={}",
            active_subscription.active_subscription_identity().as_str(),
            active_subscription.cost_profile().cost_profile_identity().as_str(),
            primary.sharing_eligibility().digest(),
            primary.consumer_contract_identity().as_str(),
            additional_basis,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            fanout_plan_identity: BridgeSubscriptionFanoutPlanIdentity::admit_bridge_owned(
                format!("bridge-subscription-fanout-plan-id:sha256:{digest:x}"),
            ),
            active_subscription_identity: active_subscription
                .active_subscription_identity()
                .clone(),
            cost_profile_identity: active_subscription
                .cost_profile()
                .cost_profile_identity()
                .clone(),
            sharing_eligibility_digest: Arc::from(
                primary.sharing_eligibility().digest().to_owned(),
            ),
            primary_consumer_contract_identity: primary.consumer_contract_identity().clone(),
            additional_consumer_contract_identities: additional_consumer_contract_identities.into(),
            counters: BridgeSubscriptionCounters::from_fanout_plan_admission(),
            canonical_basis,
            digest: Arc::from(format!("bridge-subscription-fanout-plan:sha256:{digest:x}")),
        })
    }

    pub fn fanout_plan_identity(&self) -> &BridgeSubscriptionFanoutPlanIdentity {
        &self.fanout_plan_identity
    }

    pub fn active_subscription_identity(&self) -> &BridgeActiveSubscriptionIdentity {
        &self.active_subscription_identity
    }

    pub fn cost_profile_identity(&self) -> &BridgeSubscriptionDeliveryCostProfileIdentity {
        &self.cost_profile_identity
    }

    pub fn sharing_eligibility_digest(&self) -> &str {
        self.sharing_eligibility_digest.as_ref()
    }

    pub(super) fn ordered_consumer_contract_identities(
        &self,
    ) -> impl Iterator<Item = &BridgeSubscriptionConsumerContractIdentity> {
        std::iter::once(&self.primary_consumer_contract_identity)
            .chain(self.additional_consumer_contract_identities.iter())
    }

    pub fn primary_consumer_contract_identity(
        &self,
    ) -> &BridgeSubscriptionConsumerContractIdentity {
        &self.primary_consumer_contract_identity
    }

    pub fn additional_consumer_contract_identities(
        &self,
    ) -> &[BridgeSubscriptionConsumerContractIdentity] {
        &self.additional_consumer_contract_identities
    }

    pub fn consumer_contract_identity_count(&self) -> usize {
        self.additional_consumer_contract_identities.len() + 1
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
