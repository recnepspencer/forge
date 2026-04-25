use forge_query::facade::{
    ActiveSubscriptionCounters, ForgeQueryAuthorityLane,
    ForgeQueryRuntimeLiveSubscriptionInstallation, QuerySubscriptionDeclarationCounters,
};

fn main() {
    let _ = ForgeQueryRuntimeLiveSubscriptionInstallation {
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
        counters: QuerySubscriptionDeclarationCounters::default(),
        active_lane_counters: ActiveSubscriptionCounters::default(),
        consumer_attachment_counters: ActiveSubscriptionCounters::default(),
        installation_digest: String::new(),
    };
}
