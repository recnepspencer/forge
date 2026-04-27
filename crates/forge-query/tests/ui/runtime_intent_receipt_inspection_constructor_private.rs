use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryIntentExecutionKind,
    ForgeQueryIntentInspectionDeliveryCounters, ForgeQueryIntentReceiptInspection,
    ForgeQueryIntentSourceLane,
};

fn main() {
    let counters = ForgeQueryIntentInspectionDeliveryCounters {
        affected_live_view_count: 0,
        affected_derived_view_count: 0,
        considered_computed_view_count: 0,
        considered_effect_count: 0,
        delivered_effect_count: 0,
        pending_write_intent_count: 0,
        suppressed_effect_count: 0,
        meaningful_effect_suppression_count: 0,
        effect_expression_failure_count: 0,
        refresh_fallback: false,
        counter_digest: String::new(),
    };
    let _forged = ForgeQueryIntentReceiptInspection {
        intent_name: "intent".to_string(),
        execution_kind: ForgeQueryIntentExecutionKind::Mutating,
        strategy_identity: "strategy".to_string(),
        strategy_version: "1.0".to_string(),
        strategy_descriptor_digest: String::new(),
        canonical_input_digest: String::new(),
        outcome_digest: String::new(),
        produced_mutation_digest: Some(String::new()),
        invariant_evidence: Vec::new(),
        source_lane: ForgeQueryIntentSourceLane::UserAuthored,
        target_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
        commit_identity: String::new(),
        snapshot_token: String::new(),
        receipt_digest: String::new(),
        delivery_counters: counters,
        inspection_digest: String::new(),
    };
}
