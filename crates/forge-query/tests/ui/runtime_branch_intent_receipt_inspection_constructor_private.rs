use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryBranchIntentReceiptInspection, ForgeQueryEffectPolicy,
    ForgeQueryIntentSourceLane,
};

fn main() {
    let _forged = ForgeQueryBranchIntentReceiptInspection {
        intent_name: String::new(),
        strategy_identity: String::new(),
        strategy_version: String::new(),
        canonical_input_digest: String::new(),
        source_lane: ForgeQueryIntentSourceLane::BranchLocal,
        target_lane: ForgeQueryAuthorityLane::BranchLocalTruth,
        effect_policy: ForgeQueryEffectPolicy::SandboxedWriteIntent,
        basis_evidence: Vec::new(),
        basis_digest: String::new(),
        basis_snapshot_token: String::new(),
        admission_digest: String::new(),
        receipt_digest: String::new(),
        inspection_digest: String::new(),
    };
}
