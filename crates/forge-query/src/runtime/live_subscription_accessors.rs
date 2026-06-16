use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::identity::CanonicalResultShapeDigest;
use crate::identity_authority::{
    project_query_subscription_evidence, QueryProjectionIdentity, QuerySubscriptionIdentityKind,
};
use crate::subscription::{
    ActiveSubscriptionCounters, QuerySubscriptionDeclarationCounters, QuerySubscriptionFamily,
};

use super::{
    ForgeQueryAuthorityLane, ForgeQueryRuntimeLiveSubscriptionBudgetPolicyIdentity,
    ForgeQueryRuntimeLiveSubscriptionInstallation,
};

impl ForgeQueryRuntimeLiveSubscriptionInstallation {
    pub fn view_name(&self) -> &str {
        &self.view_name
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn query_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.query_identity)
    }

    pub fn query_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.query_identity
    }

    pub fn view_shape_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.view_shape_identity)
    }

    pub fn view_shape_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.view_shape_identity
    }

    pub fn canonical_result_shape_digest(&self) -> &CanonicalResultShapeDigest {
        &self.canonical_result_shape_digest
    }

    pub fn canonical_result_shape_identity(&self) -> &ForgeQueryEvidenceIdentity {
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

    pub fn subscription_family_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.subscription_family_identity
    }

    pub fn subscription_declaration_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.subscription_declaration_identity)
    }

    pub fn subscription_declaration_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.subscription_declaration_identity
    }

    pub fn bridge_declaration_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.bridge_declaration_identity)
    }

    pub fn bridge_declaration_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.bridge_declaration_identity
    }

    pub fn admission_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.admission_identity)
    }

    pub fn admission_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.admission_identity
    }

    pub fn activation_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.activation_identity)
    }

    pub fn activation_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.activation_identity
    }

    pub fn basis_binding_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.basis_binding_identity)
    }

    pub fn basis_binding_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.basis_binding_identity
    }

    pub fn signal_strategy_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.signal_strategy_identity)
    }

    pub fn signal_strategy_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.signal_strategy_identity
    }

    pub fn active_lane_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.active_lane_identity)
    }

    pub fn active_lane_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.active_lane_identity
    }

    pub fn consumer_attachment_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.consumer_attachment_identity)
    }

    pub fn consumer_attachment_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.consumer_attachment_identity
    }

    pub fn consumer_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.consumer_identity)
    }

    pub fn consumer_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.consumer_identity
    }

    pub fn delivery_cursor_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.delivery_cursor_identity)
    }

    pub fn delivery_cursor_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.delivery_cursor_identity
    }

    pub fn support_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.support_identity)
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

    pub fn runtime_budget_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.runtime_budget_identity)
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

    pub fn installation_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.installation_identity)
    }

    pub fn installation_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.installation_identity
    }
}
