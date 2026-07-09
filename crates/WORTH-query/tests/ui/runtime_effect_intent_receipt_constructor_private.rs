use worth_query::facade::{
    WorthQueryAuthorityLane, WorthQueryEffectIntentReceipt, WorthQueryIntentSourceLane,
};

fn main() {
    let _worthd = WorthQueryEffectIntentReceipt {
        effect_name: "effect".to_string(),
        trigger_commit_identity: "commit".to_string(),
        pending_intent_target: "strategy".to_string(),
        source_lane: WorthQueryIntentSourceLane::EffectTriggered,
        target_lane: WorthQueryAuthorityLane::AuthoritativeTruth,
        phase_evidence: todo!(),
        intent_receipt: todo!(),
        receipt_digest: String::new(),
    };
}
