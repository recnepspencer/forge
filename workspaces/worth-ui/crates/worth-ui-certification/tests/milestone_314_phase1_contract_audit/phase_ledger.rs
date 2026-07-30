use super::{milestone_314_ledger, repository_document};

const PHASE_LEDGER: &str = "_docs/worth-ui/milestone-3.14-phase-1-proof-ledger.csv";
const IDS: [&str; 9] = [
    "P1-01", "P1-02", "P1-03", "P1-04", "P1-05", "P1-06", "P1-07", "P1-08", "P1-09",
];
const AGENT_CONTEXT_CLAIM: &str =
    "The Phase 1 correction closes only while generated agent context agrees with the current governed sources.";
const AGENT_CONTEXT_COMMAND: &str =
    "cargo run --manifest-path tools/agent-context/Cargo.toml -- check";
const AGENT_CONTEXT_EVIDENCE: &str = "source=generated per-crate context and current governed sources; result=agent-context check passed without stale context; owner=agent-context enforcement";

fn inputs() -> (toml::Value, String) {
    let contract: toml::Value = toml::from_str(&repository_document(
        "_docs/worth-ui/milestone-3.14-phase-1-contract.toml",
    ))
    .expect("Phase 1 contract should parse");
    (contract, repository_document(PHASE_LEDGER))
}

fn validate(contract: &toml::Value, ledger: &str) -> Result<(), String> {
    if contract["phase_ledger"].as_str() != Some(PHASE_LEDGER) {
        return Err("contract phase-ledger path drifted".to_owned());
    }
    let rows = milestone_314_ledger::parse_ledger(ledger)?;
    if rows.len() != IDS.len() {
        return Err("Phase 1 proof-ledger row count drifted".to_owned());
    }
    for (row, expected_id) in rows.iter().zip(IDS) {
        if row[0] != expected_id {
            return Err(format!("expected {expected_id}, observed {}", row[0]));
        }
        if row[8] != "PROVED" {
            return Err(format!("{expected_id} is not proved"));
        }
        if row[1..8].iter().any(String::is_empty) {
            return Err(format!("{expected_id} has an empty proof obligation"));
        }
        if !row[9].contains("source=") || !row[9].contains("result=") || !row[9].contains("owner=")
        {
            return Err(format!("{expected_id} evidence is not auditable"));
        }
    }
    if rows[8][1] != AGENT_CONTEXT_CLAIM
        || rows[8][7] != AGENT_CONTEXT_COMMAND
        || rows[8][9] != AGENT_CONTEXT_EVIDENCE
    {
        return Err("P1-09 agent-context evidence ownership drifted".to_owned());
    }
    Ok(())
}

#[test]
fn phase_1_closure_ledger_is_exact_and_independent_from_the_open_ia_portfolio() {
    let (contract, ledger) = inputs();
    validate(&contract, &ledger).expect("Phase 1 closure ledger should be exact");
    let milestone = repository_document("_docs/worth-ui/milestone-3.14-proof-ledger.csv");
    assert!(milestone
        .lines()
        .skip(1)
        .all(|row| row.contains("\"OPEN\"")));
}

#[test]
fn phase_1_closure_ledger_rejects_hostile_status_and_structure_mutations() {
    let (contract, ledger) = inputs();
    let rows = milestone_314_ledger::parse_ledger(&ledger).expect("ledger should parse");

    let mut open = rows.clone();
    open[4][8] = "OPEN".to_owned();
    assert!(validate(&contract, &milestone_314_ledger::render_ledger(&open)).is_err());

    let mut missing = rows.clone();
    missing.remove(4);
    assert!(validate(&contract, &milestone_314_ledger::render_ledger(&missing)).is_err());

    let mut duplicate = rows.clone();
    duplicate[4] = duplicate[3].clone();
    assert!(validate(&contract, &milestone_314_ledger::render_ledger(&duplicate)).is_err());

    let mut unauditable = rows;
    unauditable[4][9] = "tests passed".to_owned();
    assert!(validate(
        &contract,
        &milestone_314_ledger::render_ledger(&unauditable)
    )
    .is_err());
}

#[test]
fn phase_1_ledger_assigns_each_enforcement_claim_to_its_actual_evidence() {
    let (contract, ledger) = inputs();
    let rows = milestone_314_ledger::parse_ledger(&ledger).expect("ledger should parse");
    assert_eq!(rows[8][1], AGENT_CONTEXT_CLAIM);
    assert_eq!(rows[8][7], AGENT_CONTEXT_COMMAND);
    assert_eq!(rows[8][9], AGENT_CONTEXT_EVIDENCE);
    for overclaim in ["formatting", "line-cap", "boundary"] {
        assert!(
            !rows[8][1].contains(overclaim) && !rows[8][9].contains(overclaim),
            "P1-09 must not claim `{overclaim}` from an agent-context-only command"
        );
    }

    let mut overclaimed = rows;
    overclaimed[8][1].push_str(" Formatting and line-cap gates are also green.");
    assert!(validate(
        &contract,
        &milestone_314_ledger::render_ledger(&overclaimed)
    )
    .is_err());
}
