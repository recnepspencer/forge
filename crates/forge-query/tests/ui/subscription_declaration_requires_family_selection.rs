use forge_query::facade::{
    declare_query_subscription, LiveQueryAdmissionArtifact, QuerySubscriptionSliceBudget,
};

fn main() {
    let live = Option::<LiveQueryAdmissionArtifact>::None.unwrap();
    let budget = QuerySubscriptionSliceBudget::scratch_buffer_only(1, 1, 1, 1, 1, 1, 1, 1);
    let _declaration = declare_query_subscription(live, budget);
}
