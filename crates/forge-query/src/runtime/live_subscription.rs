use crate::identity::hash_parts;
use crate::subscription::{
    ActiveSubscriptionCounters, QuerySubscriptionDeclarationCounters,
    SubscriptionConsumerAttachment,
};

use super::ForgeQueryAuthorityLane;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeLiveSubscriptionInstallation {
    view_name: String,
    authority_lane: ForgeQueryAuthorityLane,
    query_digest: String,
    view_shape_digest: String,
    subscription_family: String,
    subscription_family_digest: String,
    subscription_declaration_digest: String,
    bridge_declaration_digest: String,
    admission_digest: String,
    activation_digest: String,
    basis_binding_digest: String,
    signal_strategy_digest: String,
    active_lane_digest: String,
    consumer_attachment_digest: String,
    consumer_digest: String,
    delivery_cursor_digest: String,
    subscription_budget_policy: String,
    active_lifecycle_budget_policy: String,
    consumer_attachment_budget_policy: String,
    runtime_budget_digest: String,
    support_evidence: String,
    counters: QuerySubscriptionDeclarationCounters,
    active_lane_counters: ActiveSubscriptionCounters,
    consumer_attachment_counters: ActiveSubscriptionCounters,
    installation_digest: String,
}

impl ForgeQueryRuntimeLiveSubscriptionInstallation {
    pub(crate) fn new(
        view_name: impl Into<String>,
        query_digest: impl Into<String>,
        view_shape_digest: impl Into<String>,
        subscription_family: impl Into<String>,
        subscription_declaration_digest: impl Into<String>,
        bridge_declaration_digest: impl Into<String>,
        admission_digest: impl Into<String>,
        activation_digest: impl Into<String>,
        basis_binding_digest: impl Into<String>,
        signal_strategy_digest: impl Into<String>,
        active_lane_digest: impl Into<String>,
        consumer_attachment: &SubscriptionConsumerAttachment,
        subscription_budget_policy: impl Into<String>,
        active_lifecycle_budget_policy: impl Into<String>,
        consumer_attachment_budget_policy: impl Into<String>,
        active_lane_counters: ActiveSubscriptionCounters,
        consumer_attachment_counters: ActiveSubscriptionCounters,
        support_evidence: impl Into<String>,
        counters: QuerySubscriptionDeclarationCounters,
    ) -> Self {
        let view_name = view_name.into();
        let query_digest = query_digest.into();
        let view_shape_digest = view_shape_digest.into();
        let subscription_family = subscription_family.into();
        let subscription_declaration_digest = subscription_declaration_digest.into();
        let bridge_declaration_digest = bridge_declaration_digest.into();
        let admission_digest = admission_digest.into();
        let activation_digest = activation_digest.into();
        let basis_binding_digest = basis_binding_digest.into();
        let signal_strategy_digest = signal_strategy_digest.into();
        let active_lane_digest = active_lane_digest.into();
        let consumer_attachment_digest =
            consumer_attachment.attachment_digest().as_str().to_string();
        let consumer_digest = consumer_attachment.consumer_digest().to_string();
        let delivery_cursor_digest = consumer_attachment.delivery_cursor_digest().to_string();
        let subscription_budget_policy = subscription_budget_policy.into();
        let active_lifecycle_budget_policy = active_lifecycle_budget_policy.into();
        let consumer_attachment_budget_policy = consumer_attachment_budget_policy.into();
        let runtime_budget_digest = hash_parts(&[
            "runtime_live_subscription_budget_policy_v1".to_string(),
            subscription_budget_policy.clone(),
            active_lifecycle_budget_policy.clone(),
            consumer_attachment_budget_policy.clone(),
        ]);
        let support_evidence = support_evidence.into();
        let subscription_family_digest = hash_parts(&[
            "runtime_live_subscription_family_v1".to_string(),
            subscription_family.clone(),
            query_digest.clone(),
            view_shape_digest.clone(),
        ]);
        let counter_digest = counters.digest();
        let installation_digest = hash_parts(&[
            "runtime_live_subscription_installation_v1".to_string(),
            view_name.clone(),
            query_digest.clone(),
            view_shape_digest.clone(),
            subscription_family_digest.clone(),
            subscription_declaration_digest.clone(),
            bridge_declaration_digest.clone(),
            admission_digest.clone(),
            activation_digest.clone(),
            basis_binding_digest.clone(),
            signal_strategy_digest.clone(),
            active_lane_digest.clone(),
            consumer_attachment_digest.clone(),
            consumer_digest.clone(),
            delivery_cursor_digest.clone(),
            runtime_budget_digest.clone(),
            support_evidence.clone(),
            counter_digest,
            active_lane_counters.digest(),
            consumer_attachment_counters.digest(),
        ]);

        Self {
            view_name,
            authority_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
            query_digest,
            view_shape_digest,
            subscription_family,
            subscription_family_digest,
            subscription_declaration_digest,
            bridge_declaration_digest,
            admission_digest,
            activation_digest,
            basis_binding_digest,
            signal_strategy_digest,
            active_lane_digest,
            consumer_attachment_digest,
            consumer_digest,
            delivery_cursor_digest,
            subscription_budget_policy,
            active_lifecycle_budget_policy,
            consumer_attachment_budget_policy,
            runtime_budget_digest,
            support_evidence,
            counters,
            active_lane_counters,
            consumer_attachment_counters,
            installation_digest,
        }
    }

    pub fn view_name(&self) -> &str {
        &self.view_name
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn view_shape_digest(&self) -> &str {
        &self.view_shape_digest
    }

    pub fn subscription_family(&self) -> &str {
        &self.subscription_family
    }

    pub fn subscription_family_digest(&self) -> &str {
        &self.subscription_family_digest
    }

    pub fn subscription_declaration_digest(&self) -> &str {
        &self.subscription_declaration_digest
    }

    pub fn bridge_declaration_digest(&self) -> &str {
        &self.bridge_declaration_digest
    }

    pub fn admission_digest(&self) -> &str {
        &self.admission_digest
    }

    pub fn activation_digest(&self) -> &str {
        &self.activation_digest
    }

    pub fn basis_binding_digest(&self) -> &str {
        &self.basis_binding_digest
    }

    pub fn signal_strategy_digest(&self) -> &str {
        &self.signal_strategy_digest
    }

    pub fn active_lane_digest(&self) -> &str {
        &self.active_lane_digest
    }

    pub fn consumer_attachment_digest(&self) -> &str {
        &self.consumer_attachment_digest
    }

    pub fn consumer_digest(&self) -> &str {
        &self.consumer_digest
    }

    pub fn delivery_cursor_digest(&self) -> &str {
        &self.delivery_cursor_digest
    }

    pub fn support_evidence(&self) -> &str {
        &self.support_evidence
    }

    pub fn subscription_budget_policy(&self) -> &str {
        &self.subscription_budget_policy
    }

    pub fn active_lifecycle_budget_policy(&self) -> &str {
        &self.active_lifecycle_budget_policy
    }

    pub fn consumer_attachment_budget_policy(&self) -> &str {
        &self.consumer_attachment_budget_policy
    }

    pub fn runtime_budget_digest(&self) -> &str {
        &self.runtime_budget_digest
    }

    pub fn counters(&self) -> &QuerySubscriptionDeclarationCounters {
        &self.counters
    }

    pub fn active_lane_counters(&self) -> &ActiveSubscriptionCounters {
        &self.active_lane_counters
    }

    pub fn consumer_attachment_counters(&self) -> &ActiveSubscriptionCounters {
        &self.consumer_attachment_counters
    }

    pub fn installation_digest(&self) -> &str {
        &self.installation_digest
    }
}
