use worth_query::facade::{
    WorthQueryAuthorityLane, WorthQueryIntentDenialEvidence, WorthQueryIntentExecutionKind,
    WorthQueryIntentSourceLane,
};

fn main() {
    let _worthd = WorthQueryIntentDenialEvidence {
        intent_name: "intent".to_string(),
        stage: "invariant-admission",
        message: "denied".to_string(),
        strategy_identity: "strategy".to_string(),
        strategy_version: "1.0".to_string(),
        returned_strategy_identity: Some("returned".to_string()),
        returned_strategy_version: Some("1.0".to_string()),
        returned_strategy_descriptor_digest: Some("descriptor".to_string()),
        canonical_input_digest: String::new(),
        source_lane: WorthQueryIntentSourceLane::UserAuthored,
        target_lane: WorthQueryAuthorityLane::AuthoritativeTruth,
        execution_kind: Some(WorthQueryIntentExecutionKind::InvariantViolation),
        attempt_digest: Some(String::new()),
        invariant_evidence: Vec::new(),
        snapshot_token: Some(String::new()),
        denial_digest: String::new(),
    };
}
