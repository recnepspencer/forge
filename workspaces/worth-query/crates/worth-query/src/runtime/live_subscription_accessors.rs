use crate::evidence_identity::WorthQueryEvidenceIdentity;
use crate::identity::{CanonicalQueryDigest, CanonicalResultShapeDigest};
use crate::identity_authority::{
    project_query_subscription_evidence, QueryProjectionIdentity, QuerySubscriptionIdentityKind,
};
use crate::subscription::{
    ActiveSubscriptionCounters, QuerySubscriptionDeclarationCounters, QuerySubscriptionFamily,
};

use super::{
    WorthQueryAuthorityLane, WorthQueryRuntimeLiveSubscriptionBudgetPolicyIdentity,
    WorthQueryRuntimeLiveSubscriptionInstallation,
};

impl WorthQueryRuntimeLiveSubscriptionInstallation {
    pub fn view_name(&self) -> &str {
        &self.view_name
    }

    pub fn authority_lane(&self) -> WorthQueryAuthorityLane {
        self.authority_lane
    }

    pub fn query_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.query_identity)
    }

    pub fn query_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.query_identity
    }

    pub fn canonical_query_digest(&self) -> &CanonicalQueryDigest {
        &self.canonical_query_digest
    }

    pub fn view_shape_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.view_shape_identity)
    }

    pub fn view_shape_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.view_shape_identity
    }

    pub fn canonical_result_shape_digest(&self) -> &CanonicalResultShapeDigest {
        &self.canonical_result_shape_digest
    }

    pub fn canonical_result_shape_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.canonical_result_shape_identity
    }

    pub fn subscription_family(&self) -> &str {
        self.subscription_family.as_str()
    }

    pub fn subscription_family_kind(&self) -> &QuerySubscriptionFamily {
        &self.subscription_family
    }

    pub fn subscription_family_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.subscription_family_identity)
    }

    pub fn subscription_family_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.subscription_family_identity
    }

    pub fn subscription_declaration_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.subscription_declaration_identity)
    }

    pub fn subscription_declaration_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.subscription_declaration_identity
    }

    pub fn bridge_declaration_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.bridge_declaration_identity)
    }

    pub fn bridge_declaration_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.bridge_declaration_identity
    }

    pub fn admission_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.admission_identity)
    }

    pub fn admission_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.admission_identity
    }

    pub fn activation_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.activation_identity)
    }

    pub fn activation_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.activation_identity
    }

    pub fn basis_binding_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.basis_binding_identity)
    }

    pub fn basis_binding_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.basis_binding_identity
    }

    pub fn signal_strategy_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.signal_strategy_identity)
    }

    pub fn signal_strategy_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.signal_strategy_identity
    }

    pub fn active_lane_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.active_lane_identity)
    }

    pub fn active_lane_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.active_lane_identity
    }

    pub fn consumer_attachment_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.consumer_attachment_identity)
    }

    pub fn consumer_attachment_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.consumer_attachment_identity
    }

    pub fn consumer_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.consumer_identity)
    }

    pub fn consumer_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.consumer_identity
    }

    pub fn delivery_cursor_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.delivery_cursor_identity)
    }

    pub fn delivery_cursor_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.delivery_cursor_identity
    }

    pub fn support_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.support_identity)
    }

    pub fn support_identity(&self) -> &WorthQueryEvidenceIdentity {
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
    ) -> &WorthQueryRuntimeLiveSubscriptionBudgetPolicyIdentity {
        &self.subscription_budget_policy
    }

    pub fn active_lifecycle_budget_policy_identity(
        &self,
    ) -> &WorthQueryRuntimeLiveSubscriptionBudgetPolicyIdentity {
        &self.active_lifecycle_budget_policy
    }

    pub fn consumer_attachment_budget_policy_identity(
        &self,
    ) -> &WorthQueryRuntimeLiveSubscriptionBudgetPolicyIdentity {
        &self.consumer_attachment_budget_policy
    }

    pub fn runtime_budget_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.runtime_budget_identity)
    }

    pub fn runtime_budget_identity(&self) -> &WorthQueryEvidenceIdentity {
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

    pub fn installation_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.installation_identity)
    }

    pub fn installation_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.installation_identity
    }
}
