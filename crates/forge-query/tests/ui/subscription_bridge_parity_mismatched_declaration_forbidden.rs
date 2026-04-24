use forge_query::subscription::{
    QuerySubscriptionBridgeParityComparison, QuerySubscriptionBridgeParityCounters,
    QuerySubscriptionBridgeParityExplanation, QuerySubscriptionManualBridgeWitness,
};

fn main() {
    let _ = QuerySubscriptionBridgeParityExplanation {
        comparison: unsafe { std::mem::zeroed::<QuerySubscriptionBridgeParityComparison>() },
        witness: unsafe { std::mem::zeroed::<QuerySubscriptionManualBridgeWitness>() },
        query_family_label: String::from("detail_exact"),
        declaration_family_label: String::from("detail_exact"),
        bridge_family_label: String::from("detail_exact"),
        bridge_slice_labels: vec![String::from("primary")],
        basis_posture_label: String::from("current"),
        signal_strategy_class_label: String::from("forged"),
        counter_snapshot: String::from("forged"),
        explanation_digest: String::from("forged"),
        counters: QuerySubscriptionBridgeParityCounters::default(),
    };
}
