use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryIntentDenialInspection, ForgeQueryIntentExecutionKind,
    ForgeQueryIntentSourceLane,
};

fn main() {
    let _forged = ForgeQueryIntentDenialInspection {
        intent_name: "intent".to_string(),
        stage: "invariant-admission",
        message: "denied".to_string(),
        strategy_identity: "strategy".to_string(),
        strategy_version: "1.0".to_string(),
        returned_strategy_identity: Some("returned".to_string()),
        returned_strategy_version: Some("1.0".to_string()),
        returned_strategy_descriptor_digest: Some("descriptor".to_string()),
        canonical_input_digest: String::new(),
        source_lane: ForgeQueryIntentSourceLane::UserAuthored,
        target_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
        execution_kind: Some(ForgeQueryIntentExecutionKind::InvariantViolation),
        attempt_digest: Some(String::new()),
        invariant_evidence: Vec::new(),
        snapshot_token: Some(String::new()),
        denial_digest: String::new(),
        inspection_digest: String::new(),
    };
}
