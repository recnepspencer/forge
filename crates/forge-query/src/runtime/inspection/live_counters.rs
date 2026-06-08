use crate::identity::hash_parts;

use super::super::ForgeQueryRuntimeLiveSubscriptionInstallation;

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
