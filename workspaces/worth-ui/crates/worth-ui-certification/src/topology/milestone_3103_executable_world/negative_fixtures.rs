use std::path::{Path, PathBuf};

use crate::topology::WorkspaceSourceInventory;

use super::{
    courtroom_contract, evidence_classification, evidence_document, opening_budget, BASELINE,
    CONTRACT,
};

#[test]
fn classification_rejects_a_missing_existing_proof() {
    let mut contract = contract();
    contract["existing_proof"]
        .as_array_mut()
        .expect("proof rows")
        .remove(0);

    let error = evidence_classification::audit(&repository_root(), &contract)
        .expect_err("missing existing evidence must fail closed");

    assert!(error.contains("existing_proof"));
}

#[test]
fn classification_rejects_in_process_proof_lane_inflation() {
    let mut contract = contract();
    let proof = contract["existing_proof"]
        .as_array_mut()
        .expect("proof rows")
        .iter_mut()
        .find(|row| row["id"].as_str() == Some("P05_REAL_WATCHER_LIFECYCLE"))
        .expect("watched proof");
    proof["lane"] = toml::Value::String("automated-executable-world".to_owned());

    let error = evidence_classification::audit(&repository_root(), &contract)
        .expect_err("in-process evidence cannot inflate into executable-world proof");

    assert!(error.contains("P05_REAL_WATCHER_LIFECYCLE"));
    assert!(error.contains("in-process-integration"));
}

#[test]
fn classification_rejects_an_incomplete_product_entry_path() {
    let mut contract = contract();
    contract["product_entry_edge"]
        .as_array_mut()
        .expect("entry edges")
        .remove(10);

    let error = evidence_classification::audit(&repository_root(), &contract)
        .expect_err("missing shutdown edge must fail closed");

    assert!(error.contains("product_entry_edge"));
}

#[test]
fn courtroom_rejects_a_missing_protocol_variant() {
    let mut contract = contract();
    contract["observation_protocol"]["variants"]
        .as_array_mut()
        .expect("protocol variants")
        .remove(3);

    let error = courtroom_contract::audit(&contract)
        .expect_err("missing preservation outcome must fail closed");

    assert!(error.contains("variants"));
}

#[test]
fn courtroom_rejects_a_rewired_typestate_transition() {
    let mut contract = contract();
    contract["typestate_transition"][5]["to"] =
        toml::Value::String("PulseExecutableWorld<Published>".to_owned());

    let error = courtroom_contract::audit(&contract)
        .expect_err("preservation cannot skip PreservedPredecessor");

    assert!(error.contains("T06_PRESERVATION"));
}

#[test]
fn courtroom_rejects_collapsed_independent_oracles() {
    let mut contract = contract();
    contract["independent_oracle"]
        .as_array_mut()
        .expect("oracles")
        .remove(0);

    let error = courtroom_contract::audit(&contract).expect_err("one oracle cannot certify itself");

    assert!(error.contains("independent_oracle"));
}

#[test]
fn courtroom_rejects_diluted_mutation_evidence() {
    let mut contract = contract();
    contract["mutation_control"][1]["must_invalidate"]
        .as_array_mut()
        .expect("event-only evidence")
        .remove(2);

    let error = courtroom_contract::audit(&contract)
        .expect_err("event-only mutation must invalidate external pixels");

    assert!(error.contains("M02_EVENT_ONLY"));
}

#[test]
fn courtroom_rejects_reordered_hostile_progression() {
    let mut contract = contract();
    contract["hostile_sequence"]["steps"]
        .as_array_mut()
        .expect("hostile sequence")
        .swap(4, 5);

    let error = courtroom_contract::audit(&contract)
        .expect_err("observation before source action must fail closed");

    assert!(error.contains("hostile executable-world sequence"));
}

#[test]
fn courtroom_rejects_a_missing_successor_home() {
    let mut contract = contract();
    contract["successor_extension"]
        .as_array_mut()
        .expect("successors")
        .remove(12);

    let error = courtroom_contract::audit(&contract)
        .expect_err("Milestone 3.23 requires a committed insertion home");

    assert!(error.contains("thirteen successor homes"));
}

#[test]
fn courtroom_rejects_a_reclassified_successor_home() {
    let mut contract = contract();
    contract["successor_extension"][10]["home"] =
        toml::Value::String("external_observation/agent_only.rs".to_owned());

    let error = courtroom_contract::audit(&contract)
        .expect_err("Milestone 3.21 cannot create an agent-only evidence home");

    assert!(error.contains("3.21"));
    assert!(error.contains("external_observation/inspection.rs"));
}

#[test]
fn opening_budget_rejects_premature_phase_progression() {
    let mut contract = contract();
    contract["phase_order"]["completed_through"] = toml::Value::Integer(2);

    let error = opening_budget::audit(&inventory(), &contract, &baseline())
        .expect_err("historical Phase 1 order cannot be rewritten");

    assert!(error.contains("historical contract"));
}

#[test]
fn opening_budget_rejects_manufactured_executable_world_measurement() {
    let contract = contract();
    let mut baseline = baseline();
    baseline["inherited_product_measurements"]["automated_executable_world_journey_seconds"] =
        serde_json::json!(1.0);

    let error = opening_budget::audit(&inventory(), &contract, &baseline)
        .expect_err("Phase 1 cannot manufacture automated journey timing");

    assert!(error.contains("absent automation"));
}

fn contract() -> toml::Value {
    evidence_document::load_toml(&repository_root().join(CONTRACT)).expect("Phase 1 contract")
}

fn baseline() -> serde_json::Value {
    evidence_document::load_json(&repository_root().join(BASELINE)).expect("Phase 1 baseline")
}

fn inventory() -> WorkspaceSourceInventory {
    WorkspaceSourceInventory::capture(workspace_root())
}

fn repository_root() -> PathBuf {
    workspace_root()
        .parent()
        .and_then(Path::parent)
        .expect("repository root")
        .to_path_buf()
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates")
        .parent()
        .expect("workspace")
        .to_path_buf()
}
