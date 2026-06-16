use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::super::{
    BridgeActiveSubscriptionIdentity, BridgeSubscriptionConsumerContractIdentity,
    BridgeSubscriptionCounters, BridgeSubscriptionDeliveryCostProfileIdentity,
    BridgeSubscriptionDeliveryFamily, BridgeSubscriptionDeliveryFamilyKind,
    BridgeSubscriptionFanoutConsumerBindingIdentity, BridgeSubscriptionFanoutLayoutIdentity,
    BridgeSubscriptionFanoutPlanIdentity,
};
use super::{
    BridgeSubscriptionFanoutAcknowledgementPolicyClass,
    BridgeSubscriptionFanoutDiagnosticsPolicyClass, BridgeSubscriptionFanoutPlan,
};

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
            fanout_consumer_binding_identity:
                BridgeSubscriptionFanoutConsumerBindingIdentity::admit_bridge_owned(format!(
                    "bridge-subscription-fanout-consumer-binding-id:sha256:{digest:x}"
                )),
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
        let fanout_layout_identity = BridgeSubscriptionFanoutLayoutIdentity::admit_bridge_owned(
            format!("bridge-subscription-fanout-layout-id:sha256:{digest:x}"),
        );
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
