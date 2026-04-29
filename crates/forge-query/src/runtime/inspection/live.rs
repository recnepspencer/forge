use crate::identity::hash_parts;

use super::super::{ForgeQueryAuthorityLane, ForgeQueryRuntimeLiveSubscriptionInstallation};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLiveSubscriptionInspectionCounters {
    declaration_counter_digest: String,
    active_lane_counter_digest: String,
    consumer_attachment_counter_digest: String,
    family_selection_count: u64,
    declaration_count: u64,
    bridge_lowering_count: u64,
    admission_count: u64,
    activation_input_count: u64,
    active_lane_admission_count: u64,
    active_lane_creation_count: u64,
    active_lane_join_count: u64,
    active_lane_handle_issue_count: u64,
    consumer_attachment_count: u64,
    consumer_attachment_denial_count: u64,
    counter_digest: String,
}

impl ForgeQueryLiveSubscriptionInspectionCounters {
    pub(in crate::runtime) fn from_installation(
        installation: &ForgeQueryRuntimeLiveSubscriptionInstallation,
    ) -> Self {
        let counters = installation.counters();
        let active_lane_counters = installation.active_lane_counters();
        let consumer_attachment_counters = installation.consumer_attachment_counters();
        let declaration_counter_digest = counters.digest();
        let active_lane_counter_digest = active_lane_counters.digest();
        let consumer_attachment_counter_digest = consumer_attachment_counters.digest();
        let family_selection_count = counters.family_selection_count();
        let declaration_count = counters.declaration_count();
        let bridge_lowering_count = counters.bridge_lowering_count();
        let admission_count = counters.admission_count();
        let activation_input_count = counters.activation_input_count();
        let active_lane_admission_count = active_lane_counters.active_lane_admission_count();
        let active_lane_creation_count = active_lane_counters.active_lane_creation_count();
        let active_lane_join_count = active_lane_counters.active_lane_join_count();
        let active_lane_handle_issue_count = active_lane_counters.active_lane_handle_issue_count();
        let consumer_attachment_count = consumer_attachment_counters.consumer_attachment_count();
        let consumer_attachment_denial_count =
            consumer_attachment_counters.consumer_attachment_denial_count();
        let counter_digest = hash_parts(&[
            "forge_query_live_subscription_inspection_counters_v1".to_string(),
            format!("declaration-digest:{declaration_counter_digest}"),
            format!("active-lane-digest:{active_lane_counter_digest}"),
            format!("consumer-attachment-digest:{consumer_attachment_counter_digest}"),
            format!("family-selection:{family_selection_count}"),
            format!("declaration:{declaration_count}"),
            format!("bridge-lowering:{bridge_lowering_count}"),
            format!("admission:{admission_count}"),
            format!("activation-input:{activation_input_count}"),
            format!("active-lane-admission:{active_lane_admission_count}"),
            format!("active-lane-creation:{active_lane_creation_count}"),
            format!("active-lane-join:{active_lane_join_count}"),
            format!("active-lane-handle-issue:{active_lane_handle_issue_count}"),
            format!("consumer-attachment:{consumer_attachment_count}"),
            format!("consumer-attachment-denial:{consumer_attachment_denial_count}"),
        ]);

        Self {
            declaration_counter_digest,
            active_lane_counter_digest,
            consumer_attachment_counter_digest,
            family_selection_count,
            declaration_count,
            bridge_lowering_count,
            admission_count,
            activation_input_count,
            active_lane_admission_count,
            active_lane_creation_count,
            active_lane_join_count,
            active_lane_handle_issue_count,
            consumer_attachment_count,
            consumer_attachment_denial_count,
            counter_digest,
        }
    }

    pub fn declaration_counter_digest(&self) -> &str {
        &self.declaration_counter_digest
    }

    pub fn active_lane_counter_digest(&self) -> &str {
        &self.active_lane_counter_digest
    }

    pub fn consumer_attachment_counter_digest(&self) -> &str {
        &self.consumer_attachment_counter_digest
    }

    pub fn family_selection_count(&self) -> u64 {
        self.family_selection_count
    }

    pub fn declaration_count(&self) -> u64 {
        self.declaration_count
    }

    pub fn bridge_lowering_count(&self) -> u64 {
        self.bridge_lowering_count
    }

    pub fn admission_count(&self) -> u64 {
        self.admission_count
    }

    pub fn activation_input_count(&self) -> u64 {
        self.activation_input_count
    }

    pub fn active_lane_admission_count(&self) -> u64 {
        self.active_lane_admission_count
    }

    pub fn active_lane_creation_count(&self) -> u64 {
        self.active_lane_creation_count
    }

    pub fn active_lane_join_count(&self) -> u64 {
        self.active_lane_join_count
    }

    pub fn active_lane_handle_issue_count(&self) -> u64 {
        self.active_lane_handle_issue_count
    }

    pub fn consumer_attachment_count(&self) -> u64 {
        self.consumer_attachment_count
    }

    pub fn consumer_attachment_denial_count(&self) -> u64 {
        self.consumer_attachment_denial_count
    }

    pub fn counter_digest(&self) -> &str {
        &self.counter_digest
    }
}

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
    installation_digest: String,
    counters: ForgeQueryLiveSubscriptionInspectionCounters,
    inspection_digest: String,
}

impl ForgeQueryLiveViewInspection {
    pub(in crate::runtime) fn from_installation(
        installation: &ForgeQueryRuntimeLiveSubscriptionInstallation,
    ) -> Self {
        let counters =
            ForgeQueryLiveSubscriptionInspectionCounters::from_installation(installation);
        let inspection_digest = hash_parts(&[
            "forge_query_live_view_inspection_v1".to_string(),
            format!("view:{}", installation.view_name()),
            format!("authority-lane:{}", installation.authority_lane()),
            format!("query:{}", installation.query_digest()),
            format!("view-shape:{}", installation.view_shape_digest()),
            format!("family:{}", installation.subscription_family()),
            format!(
                "family-digest:{}",
                installation.subscription_family_digest()
            ),
            format!(
                "subscription-declaration:{}",
                installation.subscription_declaration_digest()
            ),
            format!("bridge:{}", installation.bridge_declaration_digest()),
            format!("admission:{}", installation.admission_digest()),
            format!("activation:{}", installation.activation_digest()),
            format!("basis:{}", installation.basis_binding_digest()),
            format!("signal:{}", installation.signal_strategy_digest()),
            format!("active-lane:{}", installation.active_lane_digest()),
            format!(
                "consumer-attachment:{}",
                installation.consumer_attachment_digest()
            ),
            format!("consumer:{}", installation.consumer_digest()),
            format!("delivery-cursor:{}", installation.delivery_cursor_digest()),
            format!("runtime-budget:{}", installation.runtime_budget_digest()),
            format!("support:{}", installation.support_evidence()),
            format!("installation:{}", installation.installation_digest()),
            format!("counters:{}", counters.counter_digest()),
        ]);

        Self {
            view_name: installation.view_name().to_string(),
            authority_lane: installation.authority_lane(),
            query_digest: installation.query_digest().to_string(),
            view_shape_digest: installation.view_shape_digest().to_string(),
            subscription_family: installation.subscription_family().to_string(),
            subscription_family_digest: installation.subscription_family_digest().to_string(),
            subscription_declaration_digest: installation
                .subscription_declaration_digest()
                .to_string(),
            bridge_declaration_digest: installation.bridge_declaration_digest().to_string(),
            admission_digest: installation.admission_digest().to_string(),
            activation_digest: installation.activation_digest().to_string(),
            basis_binding_digest: installation.basis_binding_digest().to_string(),
            signal_strategy_digest: installation.signal_strategy_digest().to_string(),
            active_lane_digest: installation.active_lane_digest().to_string(),
            consumer_attachment_digest: installation.consumer_attachment_digest().to_string(),
            consumer_digest: installation.consumer_digest().to_string(),
            delivery_cursor_digest: installation.delivery_cursor_digest().to_string(),
            subscription_budget_policy: installation.subscription_budget_policy().to_string(),
            active_lifecycle_budget_policy: installation
                .active_lifecycle_budget_policy()
                .to_string(),
            consumer_attachment_budget_policy: installation
                .consumer_attachment_budget_policy()
                .to_string(),
            runtime_budget_digest: installation.runtime_budget_digest().to_string(),
            support_evidence: installation.support_evidence().to_string(),
            installation_digest: installation.installation_digest().to_string(),
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
