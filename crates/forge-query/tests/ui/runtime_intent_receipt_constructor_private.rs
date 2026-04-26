use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryIntentReceipt, ForgeQueryIntentSourceLane,
};

fn main() {
    let _forged = ForgeQueryIntentReceipt {
        intent_name: "intent".to_string(),
        strategy_identity: "strategy".to_string(),
        strategy_version: "1.0".to_string(),
        strategy_descriptor_digest: String::new(),
        canonical_input_digest: String::new(),
        produced_mutation_digest: String::new(),
        invariant_evidence: Vec::new(),
        source_lane: ForgeQueryIntentSourceLane::UserAuthored,
        target_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
        commit_identity: String::new(),
        snapshot_token: String::new(),
        affected_live_view_ids: Vec::new(),
        affected_derived_view_ids: Vec::new(),
        considered_computed_view_count: 0,
        considered_effect_count: 0,
        delivered_effect_count: 0,
        pending_write_intent_count: 0,
        receipt_digest: String::new(),
    };
}
