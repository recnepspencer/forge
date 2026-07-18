use crate::classification::{
    build_consolidated_suite_inventory, validate_suite_process_cohesion,
    validate_suite_semantic_authority, validate_suite_semantic_authority_for_source_edit,
    ScenarioProcessTopology,
};
use crate::evidence::read_json;

use super::scratch_workspace::ScratchCargoWorkspace;
use super::{current_inventory, workspace_root};

#[test]
fn incompatible_process_topology_cannot_hide_inside_a_consolidated_suite() {
    let root = workspace_root();
    let current = current_inventory(&root);
    let mut suites = build_consolidated_suite_inventory(&root, current.inventory()).unwrap();
    let scenario = suites
        .suites
        .iter_mut()
        .find_map(|suite| suite.scenarios.first_mut())
        .unwrap();
    scenario.process_topology = ScenarioProcessTopology::AllocatorGlobalProcess;

    assert!(validate_suite_process_cohesion(&suites.suites).is_err());
}

#[test]
fn scenario_subject_setup_and_oracle_contracts_cannot_drift() {
    let root = workspace_root();
    let current = current_inventory(&root);
    let authority = read_json::<crate::classification::ConsolidatedSuiteInventory>(
        &root.join("test-control/scenario-semantic-authority.json"),
    )
    .unwrap();
    let mut observed = build_consolidated_suite_inventory(&root, current.inventory()).unwrap();
    let scenario = &mut observed.suites[0].scenarios[0];
    scenario.proof_contract.production_subject_packages.clear();
    scenario.proof_contract.setup_authority_sources.clear();
    scenario.proof_contract.oracle_owner_packages.clear();
    let denials = validate_suite_semantic_authority(&authority, &observed).unwrap_err();
    assert!(denials
        .iter()
        .any(|denial| denial.contains("contract drifted")));
    assert!(denials
        .iter()
        .any(|denial| denial.contains("omits subject, setup, or oracle")));

    let mut fingerprint_only =
        build_consolidated_suite_inventory(&root, current.inventory()).unwrap();
    let (source, fingerprint) = fingerprint_only.suites[0]
        .suite_source_fingerprints
        .iter_mut()
        .next()
        .unwrap();
    *fingerprint = "controlled-source-edit".to_owned();
    let normalized = source.replace('\\', "/");
    let source = normalized
        .find("crates/")
        .map(|offset| normalized[offset..].to_owned())
        .unwrap_or(normalized);
    assert!(validate_suite_semantic_authority(&authority, &fingerprint_only).is_err());
    validate_suite_semantic_authority_for_source_edit(&authority, &fingerprint_only, &source)
        .unwrap();
}

#[test]
fn scenario_authority_schema_and_suite_cardinality_are_enforced() {
    let root = workspace_root();
    let current = current_inventory(&root);
    let mut authority = read_json::<crate::classification::ConsolidatedSuiteInventory>(
        &root.join("test-control/scenario-semantic-authority.json"),
    )
    .unwrap();
    let observed = build_consolidated_suite_inventory(&root, current.inventory()).unwrap();
    authority.schema_version = 99;
    authority.consolidated_suite_executables += 1;
    let mut phantom = authority.suites[0].clone();
    phantom.suite_identity = "phantom-suite".to_owned();
    authority.suites.push(phantom);
    let denials = validate_suite_semantic_authority(&authority, &observed).unwrap_err();
    assert!(denials
        .iter()
        .any(|denial| denial.contains("unsupported scenario authority schema")));
    assert!(denials
        .iter()
        .any(|denial| denial.contains("executable cardinality drifted")));
    assert!(denials
        .iter()
        .any(|denial| denial.contains("sealed suite is no longer reachable")));
}

#[test]
fn suite_entrypoint_cannot_escape_certification_test_ownership() {
    let root = workspace_root();
    let current = current_inventory(&root);
    let mut escaped = current.inventory().clone();
    let target = escaped
        .discovered
        .targets
        .iter_mut()
        .find(|target| target.identity == "worth-store-certification::test::durability_recovery")
        .unwrap();
    let source = std::fs::read_to_string(&target.source_path).unwrap();
    let scratch = ScratchCargoWorkspace::new("suite-source-escape");
    scratch.write("escaped/durability_recovery.rs", &source);
    target.source_path = scratch
        .root()
        .join("escaped/durability_recovery.rs")
        .to_string_lossy()
        .into_owned();

    let denials = build_consolidated_suite_inventory(&root, &escaped).unwrap_err();
    assert!(denials
        .iter()
        .any(|denial| denial.contains("outside certification test ownership")));
}
