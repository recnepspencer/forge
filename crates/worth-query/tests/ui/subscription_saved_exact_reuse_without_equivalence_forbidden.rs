use worth_query::facade::{
    declare_query_subscription, QuerySubscriptionSliceBudget, SavedQueryArtifact,
};

fn main() {
    let saved = Option::<SavedQueryArtifact>::None.unwrap();
    let budget = QuerySubscriptionSliceBudget::scratch_buffer_only(1, 1, 1, 1, 1, 1, 1, 1);
    let _declaration = declare_query_subscription(saved, budget);
}
