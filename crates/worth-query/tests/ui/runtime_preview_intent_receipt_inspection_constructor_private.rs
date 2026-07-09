use worth_query::facade::{
    WorthQueryAuthorityLane, WorthQueryEffectPolicy, WorthQueryIntentSourceLane,
    WorthQueryPreviewIntentReceiptInspection,
};

fn main() {
    let _worthd = WorthQueryPreviewIntentReceiptInspection {
        intent_name: String::new(),
        strategy_identity: String::new(),
        strategy_version: String::new(),
        canonical_input_digest: String::new(),
        source_lane: WorthQueryIntentSourceLane::PreviewLocal,
        target_lane: WorthQueryAuthorityLane::PreviewTruth,
        effect_policy: WorthQueryEffectPolicy::SandboxedWriteIntent,
        basis_evidence: Vec::new(),
        basis_digest: String::new(),
        admission_digest: String::new(),
        receipt_digest: String::new(),
        inspection_digest: String::new(),
    };
}
