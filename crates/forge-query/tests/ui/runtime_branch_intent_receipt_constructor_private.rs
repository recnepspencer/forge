use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryBranchIntentReceipt, ForgeQueryEffectPolicy,
    ForgeQueryIntentSourceLane,
};

fn main() {
    let _forged = ForgeQueryBranchIntentReceipt {
        intent_name: "intent".to_string(),
        strategy_identity: "strategy".to_string(),
        strategy_version: "1.0".to_string(),
        canonical_input_digest: "input".to_string(),
        source_lane: ForgeQueryIntentSourceLane::BranchLocal,
        target_lane: ForgeQueryAuthorityLane::BranchLocalTruth,
        effect_policy: ForgeQueryEffectPolicy::SandboxedWriteIntent,
        basis_evidence: vec!["basis".to_string()],
        basis_snapshot_token: "snapshot".to_string(),
        admission_digest: "admission".to_string(),
        receipt_digest: "receipt".to_string(),
    };
}
