use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::subscription::{
    ActiveSubscriptionCounters, QuerySubscriptionDeclarationCounters, QuerySubscriptionFamily,
    SubscriptionConsumerAttachment,
};

use super::ForgeQueryAuthorityLane;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeLiveSubscriptionBudgetPolicyIdentity {
    policy_label: String,
    policy_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryRuntimeLiveSubscriptionBudgetPolicyIdentity {
    pub fn subscription_policy(policy_label: impl Into<String>) -> Self {
        Self::new("subscription_budget_policy", policy_label)
    }

    pub fn active_lifecycle_policy(policy_label: impl Into<String>) -> Self {
        Self::new("active_lifecycle_budget_policy", policy_label)
    }

    pub fn consumer_attachment_policy(policy_label: impl Into<String>) -> Self {
        Self::new("consumer_attachment_budget_policy", policy_label)
    }

    fn new(role: &'static str, policy_label: impl Into<String>) -> Self {
        let policy_label = policy_label.into();
        let policy_identity = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "runtime_live_subscription_budget_policy_member_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("role"), role)
        .field_shape(ForgeQueryEvidenceTag::new("policy"), &policy_label)
        .seal();

        Self {
            policy_label,
            policy_identity,
        }
    }

    pub fn policy_label(&self) -> &str {
        &self.policy_label
    }

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.policy_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeLiveSubscriptionInstallation {
    view_name: String,
    authority_lane: ForgeQueryAuthorityLane,
    query_identity: ForgeQueryEvidenceIdentity,
    view_shape_identity: ForgeQueryEvidenceIdentity,
    subscription_family: QuerySubscriptionFamily,
    subscription_family_identity: ForgeQueryEvidenceIdentity,
    subscription_declaration_identity: ForgeQueryEvidenceIdentity,
    bridge_declaration_identity: ForgeQueryEvidenceIdentity,
    admission_identity: ForgeQueryEvidenceIdentity,
    activation_identity: ForgeQueryEvidenceIdentity,
    basis_binding_identity: ForgeQueryEvidenceIdentity,
    signal_strategy_identity: ForgeQueryEvidenceIdentity,
    active_lane_identity: ForgeQueryEvidenceIdentity,
    consumer_attachment_identity: ForgeQueryEvidenceIdentity,
    consumer_identity: ForgeQueryEvidenceIdentity,
    delivery_cursor_identity: ForgeQueryEvidenceIdentity,
    subscription_budget_policy: ForgeQueryRuntimeLiveSubscriptionBudgetPolicyIdentity,
    active_lifecycle_budget_policy: ForgeQueryRuntimeLiveSubscriptionBudgetPolicyIdentity,
    consumer_attachment_budget_policy: ForgeQueryRuntimeLiveSubscriptionBudgetPolicyIdentity,
    runtime_budget_identity: ForgeQueryEvidenceIdentity,
    support_identity: ForgeQueryEvidenceIdentity,
    counters: QuerySubscriptionDeclarationCounters,
    active_lane_counters: ActiveSubscriptionCounters,
    consumer_attachment_counters: ActiveSubscriptionCounters,
    installation_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryRuntimeLiveSubscriptionInstallation {
    pub(crate) fn new(
        view_name: impl Into<String>,
        query_source_identity: ForgeQueryEvidenceIdentity,
        view_shape_source_identity: ForgeQueryEvidenceIdentity,
        subscription_family: QuerySubscriptionFamily,
        subscription_declaration_source_identity: ForgeQueryEvidenceIdentity,
        bridge_declaration_source_identity: ForgeQueryEvidenceIdentity,
        admission_source_identity: ForgeQueryEvidenceIdentity,
        activation_source_identity: ForgeQueryEvidenceIdentity,
        basis_binding_source_identity: ForgeQueryEvidenceIdentity,
        signal_strategy_source_identity: ForgeQueryEvidenceIdentity,
        active_lane_source_identity: ForgeQueryEvidenceIdentity,
        consumer_attachment: &SubscriptionConsumerAttachment,
        subscription_budget_policy: ForgeQueryRuntimeLiveSubscriptionBudgetPolicyIdentity,
        active_lifecycle_budget_policy: ForgeQueryRuntimeLiveSubscriptionBudgetPolicyIdentity,
        consumer_attachment_budget_policy: ForgeQueryRuntimeLiveSubscriptionBudgetPolicyIdentity,
        active_lane_counters: ActiveSubscriptionCounters,
        consumer_attachment_counters: ActiveSubscriptionCounters,
        support_source_identity: ForgeQueryEvidenceIdentity,
        counters: QuerySubscriptionDeclarationCounters,
    ) -> Self {
        let view_name = view_name.into();
        let query_identity = live_subscription_input_identity("query", &query_source_identity);
        let view_shape_identity =
            live_subscription_input_identity("view_shape", &view_shape_source_identity);
        let subscription_declaration_identity = live_subscription_input_identity(
            "subscription_declaration",
            &subscription_declaration_source_identity,
        );
        let bridge_declaration_identity = live_subscription_input_identity(
            "bridge_declaration",
            &bridge_declaration_source_identity,
        );
        let admission_identity =
            live_subscription_input_identity("admission", &admission_source_identity);
        let activation_identity =
            live_subscription_input_identity("activation", &activation_source_identity);
        let basis_binding_identity =
            live_subscription_input_identity("basis_binding", &basis_binding_source_identity);
        let signal_strategy_identity =
            live_subscription_input_identity("signal_strategy", &signal_strategy_source_identity);
        let active_lane_identity =
            live_subscription_input_identity("active_lane", &active_lane_source_identity);
        let consumer_attachment_identity = live_subscription_input_identity(
            "consumer_attachment",
            consumer_attachment.attachment_digest().evidence_identity(),
        );
        let consumer_identity =
            live_subscription_input_identity("consumer", consumer_attachment.consumer_identity());
        let delivery_cursor_identity = live_subscription_input_identity(
            "delivery_cursor",
            consumer_attachment.delivery_cursor_identity(),
        );
        let runtime_budget_identity = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "runtime_live_subscription_budget_policy_v1",
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("subscription_budget_policy"),
            subscription_budget_policy.evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("active_lifecycle_budget_policy"),
            active_lifecycle_budget_policy.evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("consumer_attachment_budget_policy"),
            consumer_attachment_budget_policy.evidence_identity(),
        )
        .seal();
        let support_identity =
            live_subscription_input_identity("support", &support_source_identity);
        let subscription_family_source_identity = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "runtime_live_subscription_family_source_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("role"), "subscription_family")
        .field_shape(
            ForgeQueryEvidenceTag::new("family"),
            subscription_family.as_str(),
        )
        .seal();
        let subscription_family_identity = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "runtime_live_subscription_family_v1",
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("subscription_family"),
            &subscription_family_source_identity,
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("query"), &query_identity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("view_shape"),
            &view_shape_identity,
        )
        .seal();
        let counter_identity =
            live_subscription_source_identity("counters", &counters.evidence_identity());
        let active_lane_counter_identity = live_subscription_source_identity(
            "active_lane_counters",
            &active_lane_counters.evidence_identity(),
        );
        let consumer_attachment_counter_identity = live_subscription_source_identity(
            "consumer_attachment_counters",
            &consumer_attachment_counters.evidence_identity(),
        );
        let installation_identity = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "runtime_live_subscription_installation_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("view"), view_name.as_str())
        .field_evidence_identity(ForgeQueryEvidenceTag::new("query"), &query_identity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("view_shape"),
            &view_shape_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("subscription_family"),
            &subscription_family_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("subscription_declaration"),
            &subscription_declaration_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("bridge_declaration"),
            &bridge_declaration_identity,
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("admission"), &admission_identity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("activation"),
            &activation_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis_binding"),
            &basis_binding_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("signal_strategy"),
            &signal_strategy_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("active_lane"),
            &active_lane_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("consumer_attachment"),
            &consumer_attachment_identity,
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("consumer"), &consumer_identity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("delivery_cursor"),
            &delivery_cursor_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("runtime_budget"),
            &runtime_budget_identity,
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("support"), &support_identity)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("counters"), &counter_identity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("active_lane_counters"),
            &active_lane_counter_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("consumer_attachment_counters"),
            &consumer_attachment_counter_identity,
        )
        .seal();

        Self {
            view_name,
            authority_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
            query_identity,
            view_shape_identity,
            subscription_family,
            subscription_family_identity,
            subscription_declaration_identity,
            bridge_declaration_identity,
            admission_identity,
            activation_identity,
            basis_binding_identity,
            signal_strategy_identity,
            active_lane_identity,
            consumer_attachment_identity,
            consumer_identity,
            delivery_cursor_identity,
            subscription_budget_policy,
            active_lifecycle_budget_policy,
            consumer_attachment_budget_policy,
            runtime_budget_identity,
            support_identity,
            counters,
            active_lane_counters,
            consumer_attachment_counters,
            installation_identity,
        }
    }

    pub fn view_name(&self) -> &str {
        &self.view_name
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn query_for_reporting(&self) -> &str {
        self.query_identity.as_str()
    }

    pub fn query_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.query_identity
    }

    pub fn view_shape_for_reporting(&self) -> &str {
        self.view_shape_identity.as_str()
    }

    pub fn view_shape_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.view_shape_identity
    }

    pub fn subscription_family(&self) -> &str {
        self.subscription_family.as_str()
    }

    pub fn subscription_family_kind(&self) -> &QuerySubscriptionFamily {
        &self.subscription_family
    }

    pub fn subscription_family_for_reporting(&self) -> &str {
        self.subscription_family_identity.as_str()
    }

    pub fn subscription_family_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.subscription_family_identity
    }

    pub fn subscription_declaration_for_reporting(&self) -> &str {
        self.subscription_declaration_identity.as_str()
    }

    pub fn subscription_declaration_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.subscription_declaration_identity
    }

    pub fn bridge_declaration_for_reporting(&self) -> &str {
        self.bridge_declaration_identity.as_str()
    }

    pub fn bridge_declaration_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.bridge_declaration_identity
    }

    pub fn admission_for_reporting(&self) -> &str {
        self.admission_identity.as_str()
    }

    pub fn admission_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.admission_identity
    }

    pub fn activation_for_reporting(&self) -> &str {
        self.activation_identity.as_str()
    }

    pub fn activation_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.activation_identity
    }

    pub fn basis_binding_for_reporting(&self) -> &str {
        self.basis_binding_identity.as_str()
    }

    pub fn basis_binding_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.basis_binding_identity
    }

    pub fn signal_strategy_for_reporting(&self) -> &str {
        self.signal_strategy_identity.as_str()
    }

    pub fn signal_strategy_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.signal_strategy_identity
    }

    pub fn active_lane_for_reporting(&self) -> &str {
        self.active_lane_identity.as_str()
    }

    pub fn active_lane_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.active_lane_identity
    }

    pub fn consumer_attachment_for_reporting(&self) -> &str {
        self.consumer_attachment_identity.as_str()
    }

    pub fn consumer_attachment_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.consumer_attachment_identity
    }

    pub fn consumer_for_reporting(&self) -> &str {
        self.consumer_identity.as_str()
    }

    pub fn consumer_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.consumer_identity
    }

    pub fn delivery_cursor_for_reporting(&self) -> &str {
        self.delivery_cursor_identity.as_str()
    }

    pub fn delivery_cursor_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.delivery_cursor_identity
    }

    pub fn support_for_reporting(&self) -> &str {
        self.support_identity.as_str()
    }

    pub fn support_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.support_identity
    }

    pub fn subscription_budget_policy(&self) -> &str {
        self.subscription_budget_policy.policy_label()
    }

    pub fn active_lifecycle_budget_policy(&self) -> &str {
        self.active_lifecycle_budget_policy.policy_label()
    }

    pub fn consumer_attachment_budget_policy(&self) -> &str {
        self.consumer_attachment_budget_policy.policy_label()
    }

    pub fn subscription_budget_policy_identity(
        &self,
    ) -> &ForgeQueryRuntimeLiveSubscriptionBudgetPolicyIdentity {
        &self.subscription_budget_policy
    }

    pub fn active_lifecycle_budget_policy_identity(
        &self,
    ) -> &ForgeQueryRuntimeLiveSubscriptionBudgetPolicyIdentity {
        &self.active_lifecycle_budget_policy
    }

    pub fn consumer_attachment_budget_policy_identity(
        &self,
    ) -> &ForgeQueryRuntimeLiveSubscriptionBudgetPolicyIdentity {
        &self.consumer_attachment_budget_policy
    }

    pub fn runtime_budget_for_reporting(&self) -> &str {
        self.runtime_budget_identity.as_str()
    }

    pub fn runtime_budget_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.runtime_budget_identity
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

    pub fn installation_for_reporting(&self) -> &str {
        self.installation_identity.as_str()
    }

    pub fn installation_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.installation_identity
    }
}

fn live_subscription_input_identity(
    role: &str,
    source_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "runtime_live_subscription_input_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("role"), role)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("source"), source_identity)
        .seal()
}

pub(crate) fn live_subscription_view_shape_source_identity(
    family: crate::view_shape_live::LiveViewShapeFamily,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "runtime_live_subscription_view_shape_source_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("role"), "live_view")
        .field_shape(ForgeQueryEvidenceTag::new("family"), family.as_str())
        .field_shape(
            ForgeQueryEvidenceTag::new("underlying"),
            family.underlying_live_family().as_str(),
        )
        .seal()
}

pub(crate) fn live_subscription_source_identity(
    role: &str,
    source_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "runtime_live_subscription_source_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("role"), role)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("source"), source_identity)
        .seal()
}
