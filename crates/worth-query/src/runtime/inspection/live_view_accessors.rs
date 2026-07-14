use crate::evidence_identity::WorthQueryEvidenceIdentity;
use crate::identity_authority::{
    project_query_subscription_evidence, QueryProjectionIdentity, QuerySubscriptionIdentityKind,
};

use super::live::WorthQueryLiveViewInspection;

impl WorthQueryLiveViewInspection {
    pub fn query_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.query_identity
    }

    pub fn query_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.query_identity)
    }

    pub fn view_shape_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.view_shape_identity
    }

    pub fn view_shape_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.view_shape_identity)
    }

    pub fn subscription_family_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.subscription_family_identity
    }

    pub fn subscription_family_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.subscription_family_identity)
    }

    pub fn subscription_declaration_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.subscription_declaration_identity
    }

    pub fn subscription_declaration_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.subscription_declaration_identity)
    }

    pub fn bridge_declaration_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.bridge_declaration_identity
    }

    pub fn bridge_declaration_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.bridge_declaration_identity)
    }

    pub fn admission_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.admission_identity
    }

    pub fn admission_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.admission_identity)
    }

    pub fn activation_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.activation_identity
    }

    pub fn activation_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.activation_identity)
    }

    pub fn basis_binding_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.basis_binding_identity
    }

    pub fn basis_binding_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.basis_binding_identity)
    }

    pub fn signal_strategy_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.signal_strategy_identity
    }

    pub fn signal_strategy_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.signal_strategy_identity)
    }

    pub fn active_lane_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.active_lane_identity
    }

    pub fn active_lane_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.active_lane_identity)
    }

    pub fn consumer_attachment_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.consumer_attachment_identity
    }

    pub fn consumer_attachment_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.consumer_attachment_identity)
    }

    pub fn consumer_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.consumer_identity
    }

    pub fn consumer_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.consumer_identity)
    }

    pub fn delivery_cursor_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.delivery_cursor_identity
    }

    pub fn delivery_cursor_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.delivery_cursor_identity)
    }

    pub fn runtime_budget_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.runtime_budget_identity
    }

    pub fn runtime_budget_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.runtime_budget_identity)
    }

    pub fn support_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.support_identity
    }

    pub fn support_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.support_identity)
    }

    pub fn last_delivery_cause_projection(
        &self,
    ) -> Option<QueryProjectionIdentity<String, QuerySubscriptionIdentityKind>> {
        self.last_delivery_cause_identity
            .as_ref()
            .map(project_query_subscription_evidence)
    }

    pub fn installation_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.installation_identity
    }

    pub fn installation_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.installation_identity)
    }

    pub fn inspection_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        project_query_subscription_evidence(&self.inspection_identity)
    }
}
