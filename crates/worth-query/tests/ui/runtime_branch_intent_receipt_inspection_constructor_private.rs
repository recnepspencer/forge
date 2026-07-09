use worth_query::facade::{
    WorthQueryAuthorityLane, WorthQueryBranchIntentReceiptInspection, WorthQueryEffectPolicy,
    WorthQueryIntentSourceLane,
};

fn main() {
    let _worthd = WorthQueryBranchIntentReceiptInspection {
        intent_name: String::new(),
        strategy_identity: String::new(),
        strategy_version: String::new(),
        canonical_input_digest: String::new(),
        source_lane: WorthQueryIntentSourceLane::BranchLocal,
        target_lane: WorthQueryAuthorityLane::BranchLocalTruth,
        effect_policy: WorthQueryEffectPolicy::SandboxedWriteIntent,
        basis_evidence: Vec::new(),
        basis_digest: String::new(),
        basis_snapshot_token: String::new(),
        admission_digest: String::new(),
        receipt_digest: String::new(),
        inspection_digest: String::new(),
    };
}
