use super::{audit, validate_required_cost_categories, validate_target_counts};
use crate::topology::WorkspaceSourceInventory;

fn inventory() -> WorkspaceSourceInventory {
    let workspace = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("Worth UI workspace root");
    WorkspaceSourceInventory::capture(workspace)
}

fn manifest() -> toml::Value {
    let repository = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .and_then(std::path::Path::parent)
        .and_then(std::path::Path::parent)
        .expect("repository root");
    std::fs::read_to_string(
        repository.join("_docs/worth-ui/milestone-3.10.1-phase-7-evidence.toml"),
    )
    .expect("Phase 7 evidence manifest")
    .parse()
    .expect("valid TOML")
}

#[test]
fn phase7_real_evidence_manifest_is_closed() {
    audit(&inventory(), &manifest()).expect("Phase 7 evidence authority");
}

#[test]
fn phase7_rejects_manufactured_watcher_mechanism() {
    let mut manifest = manifest();
    claim_mut(&mut manifest, "real_file_lifecycle")["mechanism"] =
        toml::Value::String("manufactured-watcher-event".to_owned());
    let error = audit(&inventory(), &manifest).expect_err("fake watcher mechanism must fail");
    assert!(error.contains("operating-system-watcher"));
}

#[test]
fn phase7_rejects_production_counter_as_sole_oracle() {
    let mut manifest = manifest();
    claim_mut(&mut manifest, "hot_frame_source_exclusion")["independent_oracle"] =
        toml::Value::Boolean(false);
    let error = audit(&inventory(), &manifest).expect_err("counter self-oracle must fail");
    assert!(error.contains("independent oracle"));
}

#[test]
fn phase7_rejects_an_omitted_cost_category() {
    let mut manifest = manifest();
    manifest["cost_categories"]
        .as_array_mut()
        .expect("cost categories")
        .pop();
    let error =
        validate_required_cost_categories(&manifest).expect_err("missing category must fail");
    assert!(error.contains("cost categories differ"));
}

#[test]
fn phase7_rejects_a_duplicate_claim_owner() {
    let mut manifest = manifest();
    let duplicate = manifest["claim"].as_array().expect("claims")[0].clone();
    manifest["claim"]
        .as_array_mut()
        .expect("claims")
        .push(duplicate);
    let error = audit(&inventory(), &manifest).expect_err("duplicate claim must fail");
    assert!(error.contains("duplicate Phase 7 evidence claim"));
}

#[test]
fn phase7_rejects_a_missing_real_witness() {
    let mut manifest = manifest();
    claim_mut(&mut manifest, "adapter_parity")["witnesses"]
        .as_array_mut()
        .expect("witnesses")[0] = toml::Value::String("fictional_adapter_proof".to_owned());
    let error = audit(&inventory(), &manifest).expect_err("missing witness must fail");
    assert!(error.contains("fictional_adapter_proof"));
}

#[test]
fn phase7_rejects_build_budget_amendment() {
    let mut manifest = manifest();
    manifest["compile_contract_cargo_sessions"] = toml::Value::Integer(3);
    let error = audit(&inventory(), &manifest).expect_err("third Cargo session must fail");
    assert!(error.contains("opening posture"));
}

#[test]
fn phase7_rejects_a_tenth_integration_target() {
    let error =
        validate_target_counts(3, 7).expect_err("a proposed product target must exceed budget");
    assert!(error.contains("total=10"));
}

fn claim_mut<'a>(manifest: &'a mut toml::Value, id: &str) -> &'a mut toml::Value {
    manifest["claim"]
        .as_array_mut()
        .expect("claims")
        .iter_mut()
        .find(|row| row["id"].as_str() == Some(id))
        .expect("named claim")
}
