use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryEffectPolicy, ForgeQueryIntentSourceLane,
    ForgeQueryPreviewIntentReceipt,
};

fn main() {
    let _forged = ForgeQueryPreviewIntentReceipt {
        intent_name: "intent".to_string(),
        strategy_identity: "strategy".to_string(),
        strategy_version: "1.0".to_string(),
        canonical_input_digest: "input".to_string(),
        source_lane: ForgeQueryIntentSourceLane::PreviewLocal,
        target_lane: ForgeQueryAuthorityLane::PreviewTruth,
        effect_policy: ForgeQueryEffectPolicy::SandboxedWriteIntent,
        basis_evidence: vec!["basis".to_string()],
        admission_digest: "admission".to_string(),
        receipt_digest: "receipt".to_string(),
    };
}
