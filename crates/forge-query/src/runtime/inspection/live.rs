use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::ordinary_outcome::ForgeQueryOrdinaryRuntimePosture;

use super::super::evidence_identities::{
    runtime_live_view_inspection_identity, RuntimeLiveViewInspectionIdentityParts,
};
use super::super::ordinary_runtime_posture::project_live_subscription_ordinary_runtime_posture;
use super::super::{
    ForgeQueryAuthorityLane, ForgeQueryRuntimeAsyncResultState,
    ForgeQueryRuntimeLiveSubscriptionState, ForgeQueryRuntimeMixedCauseDelivery,
    ForgeQueryRuntimeRemaskPosture,
};
use super::live_counters::ForgeQueryLiveSubscriptionInspectionCounters;
use crate::subscription::QuerySubscriptionDeliveryCauseKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLiveViewInspection {
    view_name: String,
    authority_lane: ForgeQueryAuthorityLane,
    query_identity: ForgeQueryEvidenceIdentity,
    view_shape_identity: ForgeQueryEvidenceIdentity,
    subscription_family: String,
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
    subscription_budget_policy: String,
    active_lifecycle_budget_policy: String,
    consumer_attachment_budget_policy: String,
    runtime_budget_identity: ForgeQueryEvidenceIdentity,
    support_identity: ForgeQueryEvidenceIdentity,
    last_delivery_cause_kind: Option<QuerySubscriptionDeliveryCauseKind>,
    last_delivery_cause_identity: Option<ForgeQueryEvidenceIdentity>,
    last_delivery_had_relational_patch: bool,
    mixed_cause_delivery: Option<ForgeQueryRuntimeMixedCauseDelivery>,
    ordinary_runtime_posture: ForgeQueryOrdinaryRuntimePosture,
    async_result_state: Option<ForgeQueryRuntimeAsyncResultState>,
    remask_posture: Option<ForgeQueryRuntimeRemaskPosture>,
    installation_identity: ForgeQueryEvidenceIdentity,
    counters: ForgeQueryLiveSubscriptionInspectionCounters,
    inspection_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryLiveViewInspection {
    pub(in crate::runtime) fn from_state(state: &ForgeQueryRuntimeLiveSubscriptionState) -> Self {
        let installation = &state.installation;
        let counters =
            ForgeQueryLiveSubscriptionInspectionCounters::from_installation(installation);
        let last_delivery_cause_kind = state
            .last_delivery
            .as_ref()
            .map(|delivery| delivery.delivery_cause_kind());
        let last_delivery_cause_identity = state
            .last_delivery
            .as_ref()
            .map(|delivery| delivery.delivery_cause_identity().clone());
        let last_delivery_had_relational_patch = state
            .last_delivery
            .as_ref()
            .map(|delivery| delivery.has_relational_patch())
            .unwrap_or(false);
        let mixed_cause_delivery = state
            .last_delivery
            .as_ref()
            .map(|delivery| delivery.mixed_cause_delivery().clone());
        let ordinary_runtime_posture = project_live_subscription_ordinary_runtime_posture(state);
        let async_result_state = state.async_result_state.clone();
        let remask_posture = state.remask_posture.clone();
        let mixed_cause_identity = mixed_cause_delivery
            .as_ref()
            .map(ForgeQueryRuntimeMixedCauseDelivery::mixed_cause_identity);
        let inspection_identity = runtime_live_view_inspection_identity(
            RuntimeLiveViewInspectionIdentityParts {
                view_name: installation.view_name(),
                authority_lane: installation.authority_lane(),
                query_identity: installation.query_identity(),
                view_shape_identity: installation.view_shape_identity(),
                subscription_family: installation.subscription_family(),
                subscription_family_identity: installation.subscription_family_identity(),
                subscription_declaration_identity: installation.subscription_declaration_identity(),
                bridge_declaration_identity: installation.bridge_declaration_identity(),
                admission_identity: installation.admission_identity(),
                activation_identity: installation.activation_identity(),
                basis_binding_identity: installation.basis_binding_identity(),
                signal_strategy_identity: installation.signal_strategy_identity(),
                active_lane_identity: installation.active_lane_identity(),
                consumer_attachment_identity: installation.consumer_attachment_identity(),
                consumer_identity: installation.consumer_identity(),
                delivery_cursor_identity: installation.delivery_cursor_identity(),
                subscription_budget_policy: installation.subscription_budget_policy(),
                active_lifecycle_budget_policy: installation.active_lifecycle_budget_policy(),
                consumer_attachment_budget_policy: installation.consumer_attachment_budget_policy(),
                runtime_budget_identity: installation.runtime_budget_identity(),
                support_identity: installation.support_identity(),
                last_delivery_cause_kind,
                last_delivery_cause_identity: last_delivery_cause_identity.as_ref(),
                last_delivery_had_relational_patch,
                mixed_cause_identity,
                ordinary_runtime_posture: Some(&ordinary_runtime_posture),
                async_result_state: async_result_state.as_ref(),
                remask_posture: remask_posture.as_ref(),
                installation_identity: installation.installation_identity(),
                counter_inspection_identity: counters.counter_inspection_identity(),
            },
        );

        Self {
            view_name: installation.view_name().to_string(),
            authority_lane: installation.authority_lane(),
            query_identity: installation.query_identity().clone(),
            view_shape_identity: installation.view_shape_identity().clone(),
            subscription_family: installation.subscription_family().to_string(),
            subscription_family_identity: installation.subscription_family_identity().clone(),
            subscription_declaration_identity: installation.subscription_declaration_identity().clone(),
            bridge_declaration_identity: installation.bridge_declaration_identity().clone(),
            admission_identity: installation.admission_identity().clone(),
            activation_identity: installation.activation_identity().clone(),
            basis_binding_identity: installation.basis_binding_identity().clone(),
            signal_strategy_identity: installation.signal_strategy_identity().clone(),
            active_lane_identity: installation.active_lane_identity().clone(),
            consumer_attachment_identity: installation.consumer_attachment_identity().clone(),
            consumer_identity: installation.consumer_identity().clone(),
            delivery_cursor_identity: installation.delivery_cursor_identity().clone(),
            subscription_budget_policy: installation.subscription_budget_policy().to_string(),
            active_lifecycle_budget_policy: installation
                .active_lifecycle_budget_policy()
                .to_string(),
            consumer_attachment_budget_policy: installation
                .consumer_attachment_budget_policy()
                .to_string(),
            runtime_budget_identity: installation.runtime_budget_identity().clone(),
            support_identity: installation.support_identity().clone(),
            last_delivery_cause_kind,
            last_delivery_cause_identity,
            last_delivery_had_relational_patch,
            mixed_cause_delivery,
            ordinary_runtime_posture,
            async_result_state,
            remask_posture,
            installation_identity: installation.installation_identity().clone(),
            counters,
            inspection_identity,
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

    pub fn view_shape_for_reporting(&self) -> &str {
        self.view_shape_identity.as_str()
    }

    pub fn subscription_family(&self) -> &str {
        &self.subscription_family
    }

    pub fn subscription_family_for_reporting(&self) -> &str {
        self.subscription_family_identity.as_str()
    }

    pub fn subscription_declaration_for_reporting(&self) -> &str {
        self.subscription_declaration_identity.as_str()
    }

    pub fn bridge_declaration_for_reporting(&self) -> &str {
        self.bridge_declaration_identity.as_str()
    }

    pub fn admission_for_reporting(&self) -> &str {
        self.admission_identity.as_str()
    }

    pub fn activation_for_reporting(&self) -> &str {
        self.activation_identity.as_str()
    }

    pub fn basis_binding_for_reporting(&self) -> &str {
        self.basis_binding_identity.as_str()
    }

    pub fn signal_strategy_for_reporting(&self) -> &str {
        self.signal_strategy_identity.as_str()
    }

    pub fn active_lane_for_reporting(&self) -> &str {
        self.active_lane_identity.as_str()
    }

    pub fn consumer_attachment_for_reporting(&self) -> &str {
        self.consumer_attachment_identity.as_str()
    }

    pub fn consumer_for_reporting(&self) -> &str {
        self.consumer_identity.as_str()
    }

    pub fn delivery_cursor_for_reporting(&self) -> &str {
        self.delivery_cursor_identity.as_str()
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

    pub fn runtime_budget_for_reporting(&self) -> &str {
        self.runtime_budget_identity.as_str()
    }

    pub fn support_for_reporting(&self) -> &str {
        self.support_identity.as_str()
    }

    pub fn last_delivery_cause_kind(&self) -> Option<QuerySubscriptionDeliveryCauseKind> {
        self.last_delivery_cause_kind
    }

    pub fn last_delivery_cause_for_reporting(&self) -> Option<&str> {
        self.last_delivery_cause_identity
            .as_ref()
            .map(ForgeQueryEvidenceIdentity::as_str)
    }

    pub fn last_delivery_had_relational_patch(&self) -> bool {
        self.last_delivery_had_relational_patch
    }

    pub fn mixed_cause_delivery(&self) -> Option<&ForgeQueryRuntimeMixedCauseDelivery> {
        self.mixed_cause_delivery.as_ref()
    }

    pub fn ordinary_runtime_posture(&self) -> &ForgeQueryOrdinaryRuntimePosture {
        &self.ordinary_runtime_posture
    }

    pub fn async_result_state(&self) -> Option<&ForgeQueryRuntimeAsyncResultState> {
        self.async_result_state.as_ref()
    }

    pub fn remask_posture(&self) -> Option<&ForgeQueryRuntimeRemaskPosture> {
        self.remask_posture.as_ref()
    }

    pub fn installation_for_reporting(&self) -> &str {
        self.installation_identity.as_str()
    }

    pub fn counters(&self) -> &ForgeQueryLiveSubscriptionInspectionCounters {
        &self.counters
    }

    pub fn inspection_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.inspection_identity
    }

    pub fn inspection_for_reporting(&self) -> &str {
        self.inspection_identity.as_str()
    }
}
