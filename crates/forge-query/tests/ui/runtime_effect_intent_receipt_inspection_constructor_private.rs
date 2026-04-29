use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryEffectIntentReceiptInspection, ForgeQueryEffectPolicy,
    ForgeQueryIntentSourceLane,
};

fn main() {
    let _ = ForgeQueryEffectIntentReceiptInspection {
        effect_name: String::new(),
        trigger_commit_identity: String::new(),
        trigger_source_kind: forge_query::facade::ForgeQueryEffectTriggerSourceKind::LiveView,
        pending_intent_target: String::new(),
        source_lane: ForgeQueryIntentSourceLane::EffectTriggered,
        target_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
        effect_policy: ForgeQueryEffectPolicy::AuthoritativeAllowed,
        phase_digest: String::new(),
        intent_receipt_digest: String::new(),
        receipt_digest: String::new(),
        inspection_digest: String::new(),
    };
}
