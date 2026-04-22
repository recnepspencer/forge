use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeActiveSubscription, BridgeActiveSubscriptionIdentity, BridgeSubscriptionConsumerContract,
    BridgeSubscriptionConsumerContractIdentity, BridgeSubscriptionCounters,
    BridgeSubscriptionDeliveryCostProfileIdentity, BridgeSubscriptionDeliveryFamily,
    BridgeSubscriptionDeliveryFamilyIdentity, BridgeSubscriptionDeliveryFamilyKind,
    BridgeSubscriptionDeliveryWindowIdentity, BridgeSubscriptionDeliveryWindowSealed,
    BridgeSubscriptionFanoutConsumerBindingIdentity,
    BridgeSubscriptionFanoutDeliveryProjectionIdentity,
    BridgeSubscriptionFanoutDeliveryProjectionSetIdentity, BridgeSubscriptionFanoutLayoutIdentity,
    BridgeSubscriptionFanoutPlanIdentity, BridgeSubscriptionFanoutProjectionValidationIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionFanoutAcknowledgementPolicyClass {
    CanonicalMemberAcknowledgement,
}

impl BridgeSubscriptionFanoutAcknowledgementPolicyClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalMemberAcknowledgement => "canonical_member_acknowledgement",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionFanoutDiagnosticsPolicyClass {
    MinimalReferenceOnly,
}

impl BridgeSubscriptionFanoutDiagnosticsPolicyClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MinimalReferenceOnly => "minimal_reference_only",
        }
    }
}

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
    fanout_plan_identity: BridgeSubscriptionFanoutPlanIdentity,
    active_subscription_identity: BridgeActiveSubscriptionIdentity,
    cost_profile_identity: BridgeSubscriptionDeliveryCostProfileIdentity,
    sharing_eligibility_digest: Arc<str>,
    primary_consumer_contract_identity: BridgeSubscriptionConsumerContractIdentity,
    additional_consumer_contract_identities: Arc<[BridgeSubscriptionConsumerContractIdentity]>,
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
            fanout_plan_identity: BridgeSubscriptionFanoutPlanIdentity::new(format!(
                "bridge-subscription-fanout-plan-id:sha256:{digest:x}"
            )),
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

    fn ordered_consumer_contract_identities(
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionFanoutConsumerBinding {
    fanout_consumer_binding_identity: BridgeSubscriptionFanoutConsumerBindingIdentity,
    slot_index: usize,
    frontier_slot_index: usize,
    consumer_contract_identity: BridgeSubscriptionConsumerContractIdentity,
    acknowledgement_policy_class: BridgeSubscriptionFanoutAcknowledgementPolicyClass,
    diagnostics_policy_class: BridgeSubscriptionFanoutDiagnosticsPolicyClass,
    sharing_eligibility_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionFanoutConsumerBinding {
    fn new(
        layout_identity: &BridgeSubscriptionFanoutLayoutIdentity,
        slot_index: usize,
        consumer_contract_identity: BridgeSubscriptionConsumerContractIdentity,
        sharing_eligibility_digest: Arc<str>,
    ) -> Self {
        let frontier_slot_index = slot_index;
        let acknowledgement_policy_class =
            BridgeSubscriptionFanoutAcknowledgementPolicyClass::CanonicalMemberAcknowledgement;
        let diagnostics_policy_class =
            BridgeSubscriptionFanoutDiagnosticsPolicyClass::MinimalReferenceOnly;
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-fanout-consumer-binding|layout={}|slot={}|frontier-slot={}|consumer={}|ack-policy={}|diagnostics-policy={}|sharing={}",
            layout_identity.as_str(),
            slot_index,
            frontier_slot_index,
            consumer_contract_identity.as_str(),
            acknowledgement_policy_class.as_str(),
            diagnostics_policy_class.as_str(),
            sharing_eligibility_digest.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            fanout_consumer_binding_identity: BridgeSubscriptionFanoutConsumerBindingIdentity::new(
                format!("bridge-subscription-fanout-consumer-binding-id:sha256:{digest:x}"),
            ),
            slot_index,
            frontier_slot_index,
            consumer_contract_identity,
            acknowledgement_policy_class,
            diagnostics_policy_class,
            sharing_eligibility_digest,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-fanout-consumer-binding:sha256:{digest:x}"
            )),
        }
    }

    pub fn fanout_consumer_binding_identity(
        &self,
    ) -> &BridgeSubscriptionFanoutConsumerBindingIdentity {
        &self.fanout_consumer_binding_identity
    }

    pub fn slot_index(&self) -> usize {
        self.slot_index
    }

    pub fn frontier_slot_index(&self) -> usize {
        self.frontier_slot_index
    }

    pub fn consumer_contract_identity(&self) -> &BridgeSubscriptionConsumerContractIdentity {
        &self.consumer_contract_identity
    }

    pub fn acknowledgement_policy_class(
        &self,
    ) -> BridgeSubscriptionFanoutAcknowledgementPolicyClass {
        self.acknowledgement_policy_class
    }

    pub fn diagnostics_policy_class(&self) -> BridgeSubscriptionFanoutDiagnosticsPolicyClass {
        self.diagnostics_policy_class
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionFanoutLayout {
    fanout_layout_identity: BridgeSubscriptionFanoutLayoutIdentity,
    fanout_plan_identity: BridgeSubscriptionFanoutPlanIdentity,
    active_subscription_identity: BridgeActiveSubscriptionIdentity,
    cost_profile_identity: BridgeSubscriptionDeliveryCostProfileIdentity,
    delivery_family: BridgeSubscriptionDeliveryFamily,
    sharing_eligibility_digest: Arc<str>,
    consumer_bindings: Arc<[BridgeSubscriptionFanoutConsumerBinding]>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionFanoutLayout {
    pub(crate) fn build(
        plan: BridgeSubscriptionFanoutPlan,
        delivery_family_kind: BridgeSubscriptionDeliveryFamilyKind,
    ) -> Self {
        let delivery_family = BridgeSubscriptionDeliveryFamily::select(delivery_family_kind);
        let consumer_basis = plan
            .ordered_consumer_contract_identities()
            .map(|identity| identity.as_str())
            .collect::<Vec<_>>()
            .join(",");
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-fanout-layout|plan={}|active={}|cost-profile={}|family={}|sharing={}|consumers={}",
            plan.fanout_plan_identity().as_str(),
            plan.active_subscription_identity().as_str(),
            plan.cost_profile_identity().as_str(),
            delivery_family.delivery_family_identity().as_str(),
            plan.sharing_eligibility_digest(),
            consumer_basis,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        let fanout_layout_identity = BridgeSubscriptionFanoutLayoutIdentity::new(format!(
            "bridge-subscription-fanout-layout-id:sha256:{digest:x}"
        ));
        let sharing_eligibility_digest =
            Arc::<str>::from(plan.sharing_eligibility_digest().to_owned());
        let consumer_bindings = plan
            .ordered_consumer_contract_identities()
            .enumerate()
            .map(|(slot_index, identity)| {
                BridgeSubscriptionFanoutConsumerBinding::new(
                    &fanout_layout_identity,
                    slot_index,
                    identity.clone(),
                    sharing_eligibility_digest.clone(),
                )
            })
            .collect::<Vec<_>>();
        Self {
            fanout_layout_identity,
            fanout_plan_identity: plan.fanout_plan_identity,
            active_subscription_identity: plan.active_subscription_identity,
            cost_profile_identity: plan.cost_profile_identity,
            delivery_family,
            sharing_eligibility_digest,
            counters: BridgeSubscriptionCounters::from_fanout_layout(consumer_bindings.len()),
            consumer_bindings: consumer_bindings.into(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-fanout-layout:sha256:{digest:x}"
            )),
        }
    }

    pub fn fanout_layout_identity(&self) -> &BridgeSubscriptionFanoutLayoutIdentity {
        &self.fanout_layout_identity
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

    pub fn delivery_family(&self) -> &BridgeSubscriptionDeliveryFamily {
        &self.delivery_family
    }

    pub fn consumer_bindings(&self) -> &[BridgeSubscriptionFanoutConsumerBinding] {
        &self.consumer_bindings
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionFanoutProjectionRejectionKind {
    ActiveSubscriptionMismatch,
    DeliveryFamilyMismatch,
}

impl BridgeSubscriptionFanoutProjectionRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveSubscriptionMismatch => "active_subscription_mismatch",
            Self::DeliveryFamilyMismatch => "delivery_family_mismatch",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionFanoutProjectionRejection {
    rejection_kind: BridgeSubscriptionFanoutProjectionRejectionKind,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionFanoutProjectionRejection {
    fn new(rejection_kind: BridgeSubscriptionFanoutProjectionRejectionKind) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-fanout-projection-rejection|kind={}",
            rejection_kind.as_str()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            counters: BridgeSubscriptionCounters::from_fanout_delivery_projection_rejection(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-fanout-projection-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionFanoutProjectionRejectionKind {
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
pub struct BridgeSubscriptionFanoutDeliveryProjection {
    fanout_delivery_projection_identity: BridgeSubscriptionFanoutDeliveryProjectionIdentity,
    fanout_layout_identity: BridgeSubscriptionFanoutLayoutIdentity,
    fanout_consumer_binding_identity: BridgeSubscriptionFanoutConsumerBindingIdentity,
    delivery_window_identity: BridgeSubscriptionDeliveryWindowIdentity,
    delivery_family_identity: BridgeSubscriptionDeliveryFamilyIdentity,
    canonical_member_count: usize,
    canonical_member_digest_basis: Arc<str>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionFanoutDeliveryProjection {
    fn new(
        layout: &BridgeSubscriptionFanoutLayout,
        binding: &BridgeSubscriptionFanoutConsumerBinding,
        sealed_window: &BridgeSubscriptionDeliveryWindowSealed,
        canonical_member_digest_basis: Arc<str>,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-fanout-delivery-projection|layout={}|binding={}|window={}|family={}|member-count={}|members={}",
            layout.fanout_layout_identity().as_str(),
            binding.fanout_consumer_binding_identity().as_str(),
            sealed_window.delivery_window_identity().as_str(),
            sealed_window.delivery_family().delivery_family_identity().as_str(),
            sealed_window.members().len(),
            canonical_member_digest_basis.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            fanout_delivery_projection_identity:
                BridgeSubscriptionFanoutDeliveryProjectionIdentity::new(format!(
                    "bridge-subscription-fanout-delivery-projection-id:sha256:{digest:x}"
                )),
            fanout_layout_identity: layout.fanout_layout_identity().clone(),
            fanout_consumer_binding_identity: binding.fanout_consumer_binding_identity().clone(),
            delivery_window_identity: sealed_window.delivery_window_identity().clone(),
            delivery_family_identity: sealed_window
                .delivery_family()
                .delivery_family_identity()
                .clone(),
            canonical_member_count: sealed_window.members().len(),
            canonical_member_digest_basis,
            counters: BridgeSubscriptionCounters::zero(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-fanout-delivery-projection:sha256:{digest:x}"
            )),
        }
    }

    pub(crate) fn project(
        layout: &BridgeSubscriptionFanoutLayout,
        sealed_window: &BridgeSubscriptionDeliveryWindowSealed,
    ) -> Result<
        BridgeSubscriptionFanoutDeliveryProjectionSet,
        BridgeSubscriptionFanoutProjectionRejection,
    > {
        if layout.active_subscription_identity() != sealed_window.active_subscription_identity() {
            return Err(BridgeSubscriptionFanoutProjectionRejection::new(
                BridgeSubscriptionFanoutProjectionRejectionKind::ActiveSubscriptionMismatch,
            ));
        }
        if layout.delivery_family().delivery_family_identity()
            != sealed_window.delivery_family().delivery_family_identity()
        {
            return Err(BridgeSubscriptionFanoutProjectionRejection::new(
                BridgeSubscriptionFanoutProjectionRejectionKind::DeliveryFamilyMismatch,
            ));
        }
        let canonical_member_digest_basis = Arc::<str>::from(
            sealed_window
                .members()
                .iter()
                .map(|member| member.digest())
                .collect::<Vec<_>>()
                .join(","),
        );
        let projections = layout
            .consumer_bindings()
            .iter()
            .map(|binding| {
                Self::new(
                    layout,
                    binding,
                    sealed_window,
                    canonical_member_digest_basis.clone(),
                )
            })
            .collect::<Vec<_>>()
            .into();
        Ok(BridgeSubscriptionFanoutDeliveryProjectionSet::new(
            layout,
            sealed_window,
            canonical_member_digest_basis,
            projections,
        ))
    }

    pub fn fanout_delivery_projection_identity(
        &self,
    ) -> &BridgeSubscriptionFanoutDeliveryProjectionIdentity {
        &self.fanout_delivery_projection_identity
    }

    pub fn fanout_consumer_binding_identity(
        &self,
    ) -> &BridgeSubscriptionFanoutConsumerBindingIdentity {
        &self.fanout_consumer_binding_identity
    }

    pub fn fanout_layout_identity(&self) -> &BridgeSubscriptionFanoutLayoutIdentity {
        &self.fanout_layout_identity
    }

    pub fn delivery_window_identity(&self) -> &BridgeSubscriptionDeliveryWindowIdentity {
        &self.delivery_window_identity
    }

    pub fn delivery_family_identity(&self) -> &BridgeSubscriptionDeliveryFamilyIdentity {
        &self.delivery_family_identity
    }

    pub fn canonical_member_count(&self) -> usize {
        self.canonical_member_count
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionFanoutDeliveryProjectionSet {
    fanout_delivery_projection_set_identity: BridgeSubscriptionFanoutDeliveryProjectionSetIdentity,
    fanout_layout_identity: BridgeSubscriptionFanoutLayoutIdentity,
    active_subscription_identity: BridgeActiveSubscriptionIdentity,
    cost_profile_identity: BridgeSubscriptionDeliveryCostProfileIdentity,
    sharing_eligibility_digest: Arc<str>,
    delivery_window_identity: BridgeSubscriptionDeliveryWindowIdentity,
    delivery_family_identity: BridgeSubscriptionDeliveryFamilyIdentity,
    canonical_member_digest_basis: Arc<str>,
    consumer_binding_digest_basis: Arc<str>,
    projections: Arc<[BridgeSubscriptionFanoutDeliveryProjection]>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionFanoutDeliveryProjectionSet {
    fn new(
        layout: &BridgeSubscriptionFanoutLayout,
        sealed_window: &BridgeSubscriptionDeliveryWindowSealed,
        canonical_member_digest_basis: Arc<str>,
        projections: Arc<[BridgeSubscriptionFanoutDeliveryProjection]>,
    ) -> Self {
        let projection_basis = projections
            .iter()
            .map(BridgeSubscriptionFanoutDeliveryProjection::digest)
            .collect::<Vec<_>>()
            .join(",");
        let consumer_binding_digest_basis = Arc::<str>::from(
            layout
                .consumer_bindings()
                .iter()
                .map(BridgeSubscriptionFanoutConsumerBinding::digest)
                .collect::<Vec<_>>()
                .join(","),
        );
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-fanout-delivery-projection-set|layout={}|active={}|cost-profile={}|sharing={}|window={}|family={}|member-basis={}|bindings={}|projection-count={}|projections={}",
            layout.fanout_layout_identity().as_str(),
            layout.active_subscription_identity().as_str(),
            layout.cost_profile_identity().as_str(),
            layout.sharing_eligibility_digest(),
            sealed_window.delivery_window_identity().as_str(),
            sealed_window.delivery_family().delivery_family_identity().as_str(),
            canonical_member_digest_basis.as_ref(),
            consumer_binding_digest_basis.as_ref(),
            projections.len(),
            projection_basis,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            fanout_delivery_projection_set_identity:
                BridgeSubscriptionFanoutDeliveryProjectionSetIdentity::new(format!(
                    "bridge-subscription-fanout-delivery-projection-set-id:sha256:{digest:x}"
                )),
            fanout_layout_identity: layout.fanout_layout_identity().clone(),
            active_subscription_identity: layout.active_subscription_identity().clone(),
            cost_profile_identity: layout.cost_profile_identity().clone(),
            sharing_eligibility_digest: Arc::from(layout.sharing_eligibility_digest().to_owned()),
            delivery_window_identity: sealed_window.delivery_window_identity().clone(),
            delivery_family_identity: sealed_window
                .delivery_family()
                .delivery_family_identity()
                .clone(),
            canonical_member_digest_basis,
            consumer_binding_digest_basis,
            counters: BridgeSubscriptionCounters::from_fanout_delivery_projection(
                projections.len(),
            ),
            projections,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-fanout-delivery-projection-set:sha256:{digest:x}"
            )),
        }
    }

    pub fn fanout_delivery_projection_set_identity(
        &self,
    ) -> &BridgeSubscriptionFanoutDeliveryProjectionSetIdentity {
        &self.fanout_delivery_projection_set_identity
    }

    pub fn fanout_layout_identity(&self) -> &BridgeSubscriptionFanoutLayoutIdentity {
        &self.fanout_layout_identity
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

    pub fn delivery_window_identity(&self) -> &BridgeSubscriptionDeliveryWindowIdentity {
        &self.delivery_window_identity
    }

    pub fn delivery_family_identity(&self) -> &BridgeSubscriptionDeliveryFamilyIdentity {
        &self.delivery_family_identity
    }

    pub fn projections(&self) -> &[BridgeSubscriptionFanoutDeliveryProjection] {
        &self.projections
    }

    pub fn len(&self) -> usize {
        self.projections.len()
    }

    pub fn is_empty(&self) -> bool {
        self.projections.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, BridgeSubscriptionFanoutDeliveryProjection> {
        self.projections.iter()
    }

    pub fn canonical_member_digest_basis(&self) -> &str {
        self.canonical_member_digest_basis.as_ref()
    }

    pub fn consumer_binding_digest_basis(&self) -> &str {
        self.consumer_binding_digest_basis.as_ref()
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }

    #[cfg(test)]
    pub(crate) fn with_canonical_member_digest_basis_for_test(
        &self,
        canonical_member_digest_basis: impl Into<Arc<str>>,
    ) -> Self {
        let mut tampered = self.clone();
        tampered.canonical_member_digest_basis = canonical_member_digest_basis.into();
        tampered
    }

    #[cfg(test)]
    pub(crate) fn with_consumer_binding_digest_basis_for_test(
        &self,
        consumer_binding_digest_basis: impl Into<Arc<str>>,
    ) -> Self {
        let mut tampered = self.clone();
        tampered.consumer_binding_digest_basis = consumer_binding_digest_basis.into();
        tampered
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionFanoutProjectionValidationRejectionKind {
    LayoutIdentityMismatch,
    ActiveSubscriptionMismatch,
    DeliveryFamilyMismatch,
    CostProfileMismatch,
    SharingEligibilityMismatch,
    ConsumerBindingOrderMismatch,
    ProjectionWidthMismatch,
    ProjectionDescriptorMismatch,
    CanonicalMemberDigestMismatch,
}

impl BridgeSubscriptionFanoutProjectionValidationRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LayoutIdentityMismatch => "layout_identity_mismatch",
            Self::ActiveSubscriptionMismatch => "active_subscription_mismatch",
            Self::DeliveryFamilyMismatch => "delivery_family_mismatch",
            Self::CostProfileMismatch => "cost_profile_mismatch",
            Self::SharingEligibilityMismatch => "sharing_eligibility_mismatch",
            Self::ConsumerBindingOrderMismatch => "consumer_binding_order_mismatch",
            Self::ProjectionWidthMismatch => "projection_width_mismatch",
            Self::ProjectionDescriptorMismatch => "projection_descriptor_mismatch",
            Self::CanonicalMemberDigestMismatch => "canonical_member_digest_mismatch",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionFanoutProjectionValidationRejection {
    rejection_kind: BridgeSubscriptionFanoutProjectionValidationRejectionKind,
    fanout_layout_identity: BridgeSubscriptionFanoutLayoutIdentity,
    projection_set_identity: BridgeSubscriptionFanoutDeliveryProjectionSetIdentity,
    rejected_projection_index: Option<usize>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionFanoutProjectionValidationRejection {
    fn new(
        rejection_kind: BridgeSubscriptionFanoutProjectionValidationRejectionKind,
        layout: &BridgeSubscriptionFanoutLayout,
        projection_set: &BridgeSubscriptionFanoutDeliveryProjectionSet,
    ) -> Self {
        Self::new_with_projection_index(rejection_kind, layout, projection_set, None)
    }

    fn new_with_projection_index(
        rejection_kind: BridgeSubscriptionFanoutProjectionValidationRejectionKind,
        layout: &BridgeSubscriptionFanoutLayout,
        projection_set: &BridgeSubscriptionFanoutDeliveryProjectionSet,
        rejected_projection_index: Option<usize>,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-fanout-projection-validation-rejection|kind={}|layout={}|projection-set={}|projection-index={}",
            rejection_kind.as_str(),
            layout.fanout_layout_identity().as_str(),
            projection_set.fanout_delivery_projection_set_identity().as_str(),
            rejected_projection_index
                .map(|index| index.to_string())
                .unwrap_or_else(|| "none".to_owned()),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            fanout_layout_identity: layout.fanout_layout_identity().clone(),
            projection_set_identity: projection_set
                .fanout_delivery_projection_set_identity()
                .clone(),
            rejected_projection_index,
            counters: BridgeSubscriptionCounters::from_fanout_projection_validation_rejection(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-fanout-projection-validation-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionFanoutProjectionValidationRejectionKind {
        self.rejection_kind
    }

    pub fn fanout_layout_identity(&self) -> &BridgeSubscriptionFanoutLayoutIdentity {
        &self.fanout_layout_identity
    }

    pub fn projection_set_identity(
        &self,
    ) -> &BridgeSubscriptionFanoutDeliveryProjectionSetIdentity {
        &self.projection_set_identity
    }

    pub fn rejected_projection_index(&self) -> Option<usize> {
        self.rejected_projection_index
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionFanoutProjectionValidation {
    fanout_projection_validation_identity: BridgeSubscriptionFanoutProjectionValidationIdentity,
    fanout_layout_identity: BridgeSubscriptionFanoutLayoutIdentity,
    projection_set_identity: BridgeSubscriptionFanoutDeliveryProjectionSetIdentity,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionFanoutProjectionValidation {
    pub(crate) fn validate(
        layout: &BridgeSubscriptionFanoutLayout,
        projection_set: &BridgeSubscriptionFanoutDeliveryProjectionSet,
    ) -> Result<Self, BridgeSubscriptionFanoutProjectionValidationRejection> {
        if layout.fanout_layout_identity() != projection_set.fanout_layout_identity() {
            return Err(BridgeSubscriptionFanoutProjectionValidationRejection::new(
                BridgeSubscriptionFanoutProjectionValidationRejectionKind::LayoutIdentityMismatch,
                layout,
                projection_set,
            ));
        }
        if layout.active_subscription_identity() != projection_set.active_subscription_identity() {
            return Err(BridgeSubscriptionFanoutProjectionValidationRejection::new(
                BridgeSubscriptionFanoutProjectionValidationRejectionKind::ActiveSubscriptionMismatch,
                layout,
                projection_set,
            ));
        }
        if layout.cost_profile_identity() != projection_set.cost_profile_identity() {
            return Err(BridgeSubscriptionFanoutProjectionValidationRejection::new(
                BridgeSubscriptionFanoutProjectionValidationRejectionKind::CostProfileMismatch,
                layout,
                projection_set,
            ));
        }
        if layout.sharing_eligibility_digest() != projection_set.sharing_eligibility_digest() {
            return Err(BridgeSubscriptionFanoutProjectionValidationRejection::new(
                BridgeSubscriptionFanoutProjectionValidationRejectionKind::SharingEligibilityMismatch,
                layout,
                projection_set,
            ));
        }
        if layout.delivery_family().delivery_family_identity()
            != projection_set.delivery_family_identity()
        {
            return Err(BridgeSubscriptionFanoutProjectionValidationRejection::new(
                BridgeSubscriptionFanoutProjectionValidationRejectionKind::DeliveryFamilyMismatch,
                layout,
                projection_set,
            ));
        }
        let binding_basis = layout
            .consumer_bindings()
            .iter()
            .map(BridgeSubscriptionFanoutConsumerBinding::digest)
            .collect::<Vec<_>>()
            .join(",");
        if binding_basis != projection_set.consumer_binding_digest_basis() {
            return Err(BridgeSubscriptionFanoutProjectionValidationRejection::new(
                BridgeSubscriptionFanoutProjectionValidationRejectionKind::ConsumerBindingOrderMismatch,
                layout,
                projection_set,
            ));
        }
        if layout.consumer_bindings().len() != projection_set.projections().len() {
            return Err(BridgeSubscriptionFanoutProjectionValidationRejection::new(
                BridgeSubscriptionFanoutProjectionValidationRejectionKind::ProjectionWidthMismatch,
                layout,
                projection_set,
            ));
        }
        for (projection_index, (binding, projection)) in layout
            .consumer_bindings()
            .iter()
            .zip(projection_set.projections().iter())
            .enumerate()
        {
            if projection.fanout_layout_identity() != projection_set.fanout_layout_identity()
                || projection.fanout_consumer_binding_identity()
                    != binding.fanout_consumer_binding_identity()
                || projection.delivery_window_identity()
                    != projection_set.delivery_window_identity()
                || projection.delivery_family_identity()
                    != projection_set.delivery_family_identity()
            {
                return Err(BridgeSubscriptionFanoutProjectionValidationRejection::new_with_projection_index(
                    BridgeSubscriptionFanoutProjectionValidationRejectionKind::ProjectionDescriptorMismatch,
                    layout,
                    projection_set,
                    Some(projection_index),
                ));
            }
            if projection.canonical_member_digest_basis()
                != projection_set.canonical_member_digest_basis()
            {
                return Err(BridgeSubscriptionFanoutProjectionValidationRejection::new_with_projection_index(
                    BridgeSubscriptionFanoutProjectionValidationRejectionKind::CanonicalMemberDigestMismatch,
                    layout,
                    projection_set,
                    Some(projection_index),
                ));
            }
        }
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-fanout-projection-validation|layout={}|projection-set={}|window={}|member-basis={}|bindings={}",
            layout.fanout_layout_identity().as_str(),
            projection_set.fanout_delivery_projection_set_identity().as_str(),
            projection_set.delivery_window_identity().as_str(),
            projection_set.canonical_member_digest_basis(),
            projection_set.consumer_binding_digest_basis(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            fanout_projection_validation_identity:
                BridgeSubscriptionFanoutProjectionValidationIdentity::new(format!(
                    "bridge-subscription-fanout-projection-validation-id:sha256:{digest:x}"
                )),
            fanout_layout_identity: layout.fanout_layout_identity().clone(),
            projection_set_identity: projection_set
                .fanout_delivery_projection_set_identity()
                .clone(),
            counters: BridgeSubscriptionCounters::from_fanout_projection_validation(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-fanout-projection-validation:sha256:{digest:x}"
            )),
        })
    }

    pub fn fanout_projection_validation_identity(
        &self,
    ) -> &BridgeSubscriptionFanoutProjectionValidationIdentity {
        &self.fanout_projection_validation_identity
    }

    pub fn fanout_layout_identity(&self) -> &BridgeSubscriptionFanoutLayoutIdentity {
        &self.fanout_layout_identity
    }

    pub fn projection_set_identity(
        &self,
    ) -> &BridgeSubscriptionFanoutDeliveryProjectionSetIdentity {
        &self.projection_set_identity
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

impl std::ops::Deref for BridgeSubscriptionFanoutDeliveryProjectionSet {
    type Target = [BridgeSubscriptionFanoutDeliveryProjection];

    fn deref(&self) -> &Self::Target {
        self.projections()
    }
}
