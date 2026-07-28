use crate::milestone_312_ledger as ledger;
use crate::{repository_document, workspace_source_inventory};

#[path = "milestone_312_phase1_contract_audit/contract.rs"]
mod contract;
#[path = "milestone_312_phase1_contract_audit/protocol.rs"]
mod protocol;
#[path = "milestone_312_phase1_contract_audit/protocol_contract.rs"]
mod protocol_contract;
#[path = "milestone_312_phase1_contract_audit/topology.rs"]
mod topology;

fn phase_1_contract() -> toml::Value {
    toml::from_str(&repository_document(
        "_docs/worth-ui/milestone-3.12-phase-1-contract.toml",
    ))
    .expect("Milestone 3.12 Phase 1 contract is TOML")
}

fn phase_5_contract() -> toml::Value {
    toml::from_str(&repository_document(
        "_docs/worth-ui/milestone-3.12-phase-5-contract.toml",
    ))
    .expect("Milestone 3.12 Phase 5 contract is TOML")
}

#[test]
fn milestone_312_phase1_freezes_routes_authority_profiles_and_topology() {
    contract::validate(&phase_1_contract(), workspace_source_inventory())
        .unwrap_or_else(|failure| panic!("Phase 1 contract is invalid: {failure}"));
}

#[test]
fn milestone_312_phase1_ledger_closes_only_with_structured_final_evidence() {
    let contract = phase_1_contract();
    let text = repository_document("_docs/worth-ui/milestone-3.12-phase-1-proof-ledger.csv");
    ledger::validate_phase_1(&contract, &text)
        .unwrap_or_else(|failure| panic!("Phase 1 ledger is invalid: {failure}"));
}

#[test]
fn milestone_312_phase1_ledger_mutations_cannot_manufacture_closure() {
    let mut contract = phase_1_contract();
    contract["status"] = toml::Value::String("closed".to_owned());
    let open = repository_document("_docs/worth-ui/milestone-3.12-phase-1-proof-ledger.csv");
    let closed = ledger::prove_all(&open).expect("build a valid closed Phase 1 ledger");
    ledger::validate_phase_1(&contract, &closed).expect("closed Phase 1 fixture is valid");

    for (label, mutation) in ledger::hostile_mutations(&closed) {
        assert!(
            ledger::validate_phase_1(&contract, &mutation).is_err(),
            "{label} mutation manufactured Phase 1 closure"
        );
    }
}

#[test]
fn milestone_312_phase5_manifest_and_open_ledger_are_exact() {
    let contract = phase_5_contract();
    let text = repository_document("_docs/worth-ui/milestone-3.12-phase-5-proof-ledger.csv");
    ledger::validate_phase_5(&contract, &text, false)
        .unwrap_or_else(|failure| panic!("Phase 5 skeleton is invalid: {failure}"));
}

#[test]
fn milestone_312_phase5_ledger_mutations_cannot_manufacture_closure() {
    let mut contract = phase_5_contract();
    contract["status"] = toml::Value::String("closed".to_owned());
    let open = repository_document("_docs/worth-ui/milestone-3.12-phase-5-proof-ledger.csv");
    let closed = ledger::prove_all(&open).expect("build a valid closed in-memory ledger");
    ledger::validate_phase_5(&contract, &closed, true).expect("closed fixture is valid");

    for (label, mutation) in ledger::hostile_mutations(&closed) {
        assert!(
            ledger::validate_phase_5(&contract, &mutation, true).is_err(),
            "{label} mutation manufactured Phase 5 closure"
        );
    }
}

#[test]
fn milestone_312_phase1_keeps_raw_v2_and_failure_artifacts_distinct() {
    let fixture = repository_document(
        "workspaces/worth-ui/apps/platform-pulse/tests/fixtures/lifecycle_protocol/v2/inherited_lifecycle_envelopes.jsonl",
    );
    protocol::validate_raw_v2_fixture(&fixture)
        .unwrap_or_else(|failure| panic!("raw v2 fixture contract is invalid: {failure}"));
    protocol::validate_failure_artifact_separation(workspace_source_inventory(), &fixture)
        .unwrap_or_else(|failure| panic!("failure artifact contract is invalid: {failure}"));
}
