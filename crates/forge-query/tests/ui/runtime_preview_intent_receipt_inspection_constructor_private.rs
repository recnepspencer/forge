use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryEffectPolicy, ForgeQueryIntentSourceLane,
    ForgeQueryPreviewIntentReceiptInspection,
};

fn main() {
    let _forged = ForgeQueryPreviewIntentReceiptInspection {
        intent_name: String::new(),
        strategy_identity: String::new(),
        strategy_version: String::new(),
        canonical_input_digest: String::new(),
        source_lane: ForgeQueryIntentSourceLane::PreviewLocal,
        target_lane: ForgeQueryAuthorityLane::PreviewTruth,
        effect_policy: ForgeQueryEffectPolicy::SandboxedWriteIntent,
        basis_evidence: Vec::new(),
        basis_digest: String::new(),
        admission_digest: String::new(),
        receipt_digest: String::new(),
        inspection_digest: String::new(),
    };
}
