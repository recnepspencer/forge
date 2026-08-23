use serde_json::json;
use sha2::{Digest, Sha256};

pub(super) struct MutationTrace<'a> {
    pub posture: &'a str,
    pub state: &'a str,
}

pub(super) struct MutationReceiptCase<'a> {
    pub requirement: &'a str,
    pub case: &'a str,
    pub baseline: MutationTrace<'a>,
    pub mutant: MutationTrace<'a>,
    pub denial: &'a str,
    pub first_divergence: &'a str,
}

pub(super) fn emit(case: MutationReceiptCase<'_>) {
    let requirement = case.requirement;
    let mutation_case = case.case;
    let trace_digest = |posture: &str, state: &str| {
        format!(
            "{:x}",
            Sha256::digest(format!("{posture}\0{state}").as_bytes())
        )
    };
    let source_revision = std::env::var("WORTH_UI_LEDGER_SOURCE_REVISION")
        .unwrap_or_else(|_| "direct-test-unbound".to_owned());
    let source_state_digest = std::env::var("WORTH_UI_LEDGER_SOURCE_STATE_DIGEST")
        .unwrap_or_else(|_| "direct-test-unbound".to_owned());
    let receipt = json!({
        "schema": "worth-ui-native-mutation-receipt-v2",
        "source_identity": {
            "revision": source_revision,
            "state_digest": source_state_digest,
        },
        "requirement": case.requirement,
        "case": case.case,
        "schedule_id": case.case,
        "mutation_identity": format!("{}:{}", case.requirement, case.case),
        "baseline": {
            "posture": case.baseline.posture,
            "terminal_state": case.baseline.state,
            "trace": [case.baseline.posture, case.baseline.state],
            "trace_sha256": trace_digest(case.baseline.posture, case.baseline.state),
        },
        "mutant": {
            "posture": case.mutant.posture,
            "terminal_state": case.mutant.state,
            "trace": [case.mutant.posture, case.mutant.state],
            "trace_sha256": trace_digest(case.mutant.posture, case.mutant.state),
        },
        "denial": case.denial,
        "first_divergence": {
            "index": 0,
            "description": case.first_divergence,
        },
        "observed_failure": true,
    });
    println!(
        "WORTH_UI_LEDGER_MUTATION_RECEIPTS={}",
        json!({requirement: receipt})
    );
    println!(
        "WORTH_UI_LEDGER_MUTATION_CONTROLS={}",
        json!({requirement: mutation_case})
    );
    println!(
        "WORTH_UI_LEDGER_MUTATION_CASES={}",
        json!({requirement: [mutation_case]})
    );
}
