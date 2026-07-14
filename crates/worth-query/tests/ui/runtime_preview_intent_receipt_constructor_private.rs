use worth_query::facade::runtime::{WorthQueryAuthorityLane, WorthQueryEffectPolicy, WorthQueryIntentSourceLane, WorthQueryPreviewIntentReceipt};

fn main() {
    let _worthd = WorthQueryPreviewIntentReceipt {
        intent_name: "intent".to_string(),
        strategy_identity: "strategy".to_string(),
        strategy_version: "1.0".to_string(),
        canonical_input_digest: "input".to_string(),
        source_lane: WorthQueryIntentSourceLane::PreviewLocal,
        target_lane: WorthQueryAuthorityLane::PreviewTruth,
        effect_policy: WorthQueryEffectPolicy::SandboxedWriteIntent,
        basis_evidence: vec!["basis".to_string()],
        admission_digest: "admission".to_string(),
        receipt_digest: "receipt".to_string(),
    };
}
