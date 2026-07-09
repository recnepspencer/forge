use worth_query::facade::{
    QuerySubscriptionBridgeParityComparison, QuerySubscriptionBridgeParityCounters,
    QuerySubscriptionBridgeParityExplanation, QuerySubscriptionManualBridgeWitness,
};

fn main() {
    let comparison: QuerySubscriptionBridgeParityComparison = todo!();
    let witness: QuerySubscriptionManualBridgeWitness = todo!();
    let _ = QuerySubscriptionBridgeParityExplanation {
        comparison,
        witness,
        query_family_label: String::new(),
        declaration_family_label: String::new(),
        bridge_family_label: String::new(),
        bridge_slice_labels: Vec::new(),
        basis_posture_label: String::new(),
        signal_strategy_class_label: String::new(),
        counter_identity: todo!(),
        explanation_identity: todo!(),
        counters: QuerySubscriptionBridgeParityCounters::default(),
    };
}
