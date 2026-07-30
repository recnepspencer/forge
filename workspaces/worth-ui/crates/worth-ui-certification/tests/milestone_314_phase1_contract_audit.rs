use super::{milestone_314_ledger, repository_document};

fn phase_1_inputs() -> (toml::Value, String) {
    let contract_text = repository_document("_docs/worth-ui/milestone-3.14-phase-1-contract.toml");
    let contract = toml::from_str(&contract_text).expect("Phase 1 contract should parse");
    let ledger = repository_document("_docs/worth-ui/milestone-3.14-proof-ledger.csv");
    (contract, ledger)
}

fn assert_rejected(contract: &toml::Value, ledger: &str, phase: i64, label: &str) {
    assert!(
        milestone_314_ledger::validate_at_phase(contract, ledger, phase).is_err(),
        "{label} mutation should be rejected"
    );
}

#[test]
fn milestone_314_contract_and_phase_1_ledger_are_exact() {
    let (contract, ledger) = phase_1_inputs();
    milestone_314_ledger::validate_phase_1(&contract, &ledger)
        .expect("the frozen contract and all-open Phase 1 ledger should agree");
}

#[test]
fn milestone_314_phase_1_rejects_any_closed_row() {
    let (contract, ledger) = phase_1_inputs();
    let mut rows = milestone_314_ledger::parse_ledger(&ledger).expect("ledger should parse");
    rows[1][8] = "PROVED".to_owned();
    rows[1][9] =
        "A deliberately substantial but premature Phase 2 proof may not close during Phase 1."
            .to_owned();
    assert_rejected(
        &contract,
        &milestone_314_ledger::render_ledger(&rows),
        1,
        "premature Phase 2 proof",
    );
}

#[test]
fn milestone_314_ledger_rejects_hostile_structure_mutations() {
    let (contract, ledger) = phase_1_inputs();
    let rows = milestone_314_ledger::parse_ledger(&ledger).expect("ledger should parse");

    let mut missing = rows.clone();
    missing.remove(4);
    assert_rejected(
        &contract,
        &milestone_314_ledger::render_ledger(&missing),
        1,
        "missing row",
    );

    let mut duplicate = rows.clone();
    duplicate[4] = duplicate[3].clone();
    assert_rejected(
        &contract,
        &milestone_314_ledger::render_ledger(&duplicate),
        1,
        "duplicate row",
    );

    let mut reordered = rows.clone();
    reordered.swap(4, 5);
    assert_rejected(
        &contract,
        &milestone_314_ledger::render_ledger(&reordered),
        1,
        "reordered row",
    );

    let mut fabricated = rows;
    fabricated[4][0] = "IA-99".to_owned();
    assert_rejected(
        &contract,
        &milestone_314_ledger::render_ledger(&fabricated),
        1,
        "fabricated row",
    );
}

#[test]
fn milestone_314_ledger_rejects_command_and_evidence_drift() {
    let (contract, ledger) = phase_1_inputs();
    let rows = milestone_314_ledger::parse_ledger(&ledger).expect("ledger should parse");

    let mut command_drift = rows.clone();
    command_drift[0][7].push_str(" --ignored");
    assert_rejected(
        &contract,
        &milestone_314_ledger::render_ledger(&command_drift),
        1,
        "command drift",
    );

    let mut fabricated_evidence = rows;
    fabricated_evidence[8][9] = "source=fiction; result=passed".to_owned();
    assert_rejected(
        &contract,
        &milestone_314_ledger::render_ledger(&fabricated_evidence),
        1,
        "evidence on an open row",
    );
}

#[test]
fn milestone_314_contract_freezes_native_and_query_boundaries() {
    let (contract, _) = phase_1_inputs();
    assert_eq!(
        contract["native_reachability"]["application_seam"].as_str(),
        Some("eframe::App::raw_input_hook")
    );
    assert_eq!(
        contract["native_reachability"]["eframe_version"].as_str(),
        Some("0.31.1")
    );
    assert_eq!(
        contract["ownership"]["raw_query_owner"].as_str(),
        Some("worth-ui-query-binding")
    );
    assert_eq!(
        contract["execution"]["ui_admission_is_query_admission"].as_bool(),
        Some(false)
    );
    assert_eq!(
        contract["public_contract"]["string_intent_authority"].as_bool(),
        Some(false)
    );
}
