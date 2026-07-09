use worth_query::facade::{
    WorthQueryAuthorityLane, WorthQueryEffectIntentReceiptInspection, WorthQueryEffectPolicy,
    WorthQueryIntentSourceLane,
};

fn main() {
    let _ = WorthQueryEffectIntentReceiptInspection {
        effect_name: String::new(),
        trigger_commit_identity: String::new(),
        trigger_source_kind: worth_query::facade::WorthQueryEffectTriggerSourceKind::LiveView,
        pending_intent_target: String::new(),
        source_lane: WorthQueryIntentSourceLane::EffectTriggered,
        target_lane: WorthQueryAuthorityLane::AuthoritativeTruth,
        effect_policy: WorthQueryEffectPolicy::AuthoritativeAllowed,
        phase_digest: String::new(),
        intent_receipt_digest: String::new(),
        receipt_digest: String::new(),
        inspection_digest: String::new(),
    };
}
