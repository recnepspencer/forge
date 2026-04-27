use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryLiveSubscriptionInspectionCounters,
    ForgeQueryLiveViewInspection,
};

fn main() {
    let counters = ForgeQueryLiveSubscriptionInspectionCounters {
        declaration_counter_digest: String::new(),
        active_lane_counter_digest: String::new(),
        consumer_attachment_counter_digest: String::new(),
        family_selection_count: 0,
        declaration_count: 0,
        bridge_lowering_count: 0,
        admission_count: 0,
        activation_input_count: 0,
        active_lane_admission_count: 0,
        active_lane_creation_count: 0,
        active_lane_join_count: 0,
        active_lane_handle_issue_count: 0,
        consumer_attachment_count: 0,
        consumer_attachment_denial_count: 0,
        counter_digest: String::new(),
    };

    let _inspection = ForgeQueryLiveViewInspection {
        view_name: String::new(),
        authority_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
        query_digest: String::new(),
        view_shape_digest: String::new(),
        subscription_family: String::new(),
        subscription_family_digest: String::new(),
        subscription_declaration_digest: String::new(),
        bridge_declaration_digest: String::new(),
        admission_digest: String::new(),
        activation_digest: String::new(),
        basis_binding_digest: String::new(),
        signal_strategy_digest: String::new(),
        active_lane_digest: String::new(),
        consumer_attachment_digest: String::new(),
        consumer_digest: String::new(),
        delivery_cursor_digest: String::new(),
        subscription_budget_policy: String::new(),
        active_lifecycle_budget_policy: String::new(),
        consumer_attachment_budget_policy: String::new(),
        runtime_budget_digest: String::new(),
        support_evidence: String::new(),
        installation_digest: String::new(),
        counters,
        inspection_digest: String::new(),
    };
}
