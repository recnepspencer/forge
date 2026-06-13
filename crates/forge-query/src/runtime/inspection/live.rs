use crate::identity::hash_parts;
use crate::ordinary_outcome::ForgeQueryOrdinaryRuntimePosture;

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
    last_delivery_cause_kind: Option<QuerySubscriptionDeliveryCauseKind>,
    last_delivery_cause_digest: Option<String>,
    last_delivery_had_relational_patch: bool,
    mixed_cause_delivery: Option<ForgeQueryRuntimeMixedCauseDelivery>,
    ordinary_runtime_posture: ForgeQueryOrdinaryRuntimePosture,
    async_result_state: Option<ForgeQueryRuntimeAsyncResultState>,
    remask_posture: Option<ForgeQueryRuntimeRemaskPosture>,
    installation_digest: String,
    counters: ForgeQueryLiveSubscriptionInspectionCounters,
    inspection_digest: String,
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
        let last_delivery_cause_digest = state
            .last_delivery
            .as_ref()
            .map(|delivery| delivery.delivery_cause_digest().to_string());
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
        let inspection_digest = hash_parts(&[
            "forge_query_live_view_inspection_v1".to_string(),
            format!("view:{}", installation.view_name()),
            format!("authority-lane:{}", installation.authority_lane()),
            format!("query:{}", installation.query_for_reporting()),
            format!("view-shape:{}", installation.view_shape_for_reporting()),
            format!("family:{}", installation.subscription_family()),
            format!(
                "family-digest:{}",
                installation.subscription_family_for_reporting()
            ),
            format!(
                "subscription-declaration:{}",
                installation.subscription_declaration_for_reporting()
            ),
            format!("bridge:{}", installation.bridge_declaration_for_reporting()),
            format!("admission:{}", installation.admission_for_reporting()),
            format!("activation:{}", installation.activation_for_reporting()),
            format!("basis:{}", installation.basis_binding_for_reporting()),
            format!("signal:{}", installation.signal_strategy_for_reporting()),
            format!("active-lane:{}", installation.active_lane_for_reporting()),
            format!(
                "consumer-attachment:{}",
                installation.consumer_attachment_for_reporting()
            ),
            format!("consumer:{}", installation.consumer_for_reporting()),
            format!("delivery-cursor:{}", installation.delivery_cursor_for_reporting()),
            format!("runtime-budget:{}", installation.runtime_budget_for_reporting()),
            format!("support:{}", installation.support_evidence()),
            format!(
                "last-delivery-cause:{}",
                last_delivery_cause_kind
                    .map(QuerySubscriptionDeliveryCauseKind::as_str)
                    .unwrap_or("none")
            ),
            format!(
                "last-delivery-digest:{}",
                last_delivery_cause_digest.as_deref().unwrap_or("none")
            ),
            format!("last-delivery-relational:{last_delivery_had_relational_patch}"),
            format!(
                "mixed-cause:{}",
                mixed_cause_delivery
                    .as_ref()
                    .map(|delivery| delivery.mixed_cause_digest())
                    .unwrap_or("none")
            ),
            format!(
                "ordinary-runtime-posture:{}",
                ordinary_runtime_posture.posture_digest()
            ),
            format!(
                "async-result-state:{}",
                async_result_state
                    .as_ref()
                    .map(|state| state.result_state_digest())
                    .unwrap_or("none")
            ),
            format!(
                "remask:{}",
                remask_posture
                    .as_ref()
                    .map(|posture| posture.remask_digest())
                    .unwrap_or("none")
            ),
            format!("installation:{}", installation.installation_for_reporting()),
            format!("counters:{}", counters.counter_digest()),
        ]);

        Self {
            view_name: installation.view_name().to_string(),
            authority_lane: installation.authority_lane(),
            query_digest: installation.query_for_reporting().to_string(),
            view_shape_digest: installation.view_shape_for_reporting().to_string(),
            subscription_family: installation.subscription_family().to_string(),
            subscription_family_digest: installation.subscription_family_for_reporting().to_string(),
            subscription_declaration_digest: installation
                .subscription_declaration_for_reporting()
                .to_string(),
            bridge_declaration_digest: installation.bridge_declaration_for_reporting().to_string(),
            admission_digest: installation.admission_for_reporting().to_string(),
            activation_digest: installation.activation_for_reporting().to_string(),
            basis_binding_digest: installation.basis_binding_for_reporting().to_string(),
            signal_strategy_digest: installation.signal_strategy_for_reporting().to_string(),
            active_lane_digest: installation.active_lane_for_reporting().to_string(),
            consumer_attachment_digest: installation.consumer_attachment_for_reporting().to_string(),
            consumer_digest: installation.consumer_for_reporting().to_string(),
            delivery_cursor_digest: installation.delivery_cursor_for_reporting().to_string(),
            subscription_budget_policy: installation.subscription_budget_policy().to_string(),
            active_lifecycle_budget_policy: installation
                .active_lifecycle_budget_policy()
                .to_string(),
            consumer_attachment_budget_policy: installation
                .consumer_attachment_budget_policy()
                .to_string(),
            runtime_budget_digest: installation.runtime_budget_for_reporting().to_string(),
            support_evidence: installation.support_evidence().to_string(),
            last_delivery_cause_kind,
            last_delivery_cause_digest,
            last_delivery_had_relational_patch,
            mixed_cause_delivery,
            ordinary_runtime_posture,
            async_result_state,
            remask_posture,
            installation_digest: installation.installation_for_reporting().to_string(),
            counters,
            inspection_digest,
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

    pub fn support_evidence(&self) -> &str {
        &self.support_evidence
    }

    pub fn last_delivery_cause_kind(&self) -> Option<QuerySubscriptionDeliveryCauseKind> {
        self.last_delivery_cause_kind
    }

    pub fn last_delivery_cause_digest(&self) -> Option<&str> {
        self.last_delivery_cause_digest.as_deref()
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

    pub fn installation_digest(&self) -> &str {
        &self.installation_digest
    }

    pub fn counters(&self) -> &ForgeQueryLiveSubscriptionInspectionCounters {
        &self.counters
    }

    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }
}
