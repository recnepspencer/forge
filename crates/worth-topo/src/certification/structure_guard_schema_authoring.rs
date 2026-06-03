use std::fs;
use std::path::PathBuf;

use super::structure_guard_support::{rust_files, src_relative_path};

const SCHEMA_BOUNDARY_FILES: &[&str] = &[
    "test_support/schema_topology_authoring_boundary/mod.rs",
    "test_support/schema_topology_authoring_boundary/mainline_execution.rs",
    "test_support/schema_topology_authoring_boundary/primitive_seeding.rs",
    "test_support/schema_topology_authoring_boundary/branch_execution.rs",
];

#[test]
fn schema_branch_authoring_session_entry_stays_quarantined() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let allowed_relative_files = schema_guard_allowed_files();
    let violations =
        schema_authoring_string_violations(&src, &allowed_relative_files, ".create_branch(");
    assert!(
        violations.is_empty(),
        "schema-backed branch session setup must stay quarantined behind test_support/schema_topology_authoring_boundary/*: {violations:?}"
    );
}

#[test]
fn schema_branch_session_helper_usage_stays_quarantined() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let allowed_relative_files = schema_guard_allowed_files();
    let violations = schema_authoring_string_violations(
        &src,
        &allowed_relative_files,
        "open_schema_topology_authoring_branch(",
    );
    assert!(
        violations.is_empty(),
        "raw schema-backed branch session helpers must stay quarantined behind the explicit schema authoring boundary: {violations:?}"
    );
}

#[test]
fn schema_branch_mutation_execution_entry_stays_quarantined() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let allowed_relative_files = schema_guard_allowed_files();
    let violations = schema_authoring_string_violations(
        &src,
        &allowed_relative_files,
        "commit_topology_intent_on_branch_through_schema_authority(",
    );
    assert!(
        violations.is_empty(),
        "schema-backed branch mutation execution must stay quarantined behind test_support/schema_topology_authoring_boundary/*: {violations:?}"
    );
}

#[test]
fn schema_mainline_mutation_execution_entry_stays_quarantined() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let allowed_relative_files = schema_guard_allowed_files();
    let violations = schema_authoring_string_violations(
        &src,
        &allowed_relative_files,
        "commit_topology_intent_through_schema_authority(",
    );
    assert!(
        violations.is_empty(),
        "schema-backed mainline mutation execution must stay quarantined behind test_support/schema_topology_authoring_boundary/*: {violations:?}"
    );
}

#[test]
fn schema_mutation_set_execution_entry_stays_quarantined() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut allowed_relative_files = schema_guard_allowed_files();
    allowed_relative_files
        .push("projection/runtime_boundary/query_runtime/adapters/schema_write_boundary.rs");
    let violations = schema_authoring_string_violations(
        &src,
        &allowed_relative_files,
        "commit_topology_mutation_set(",
    );
    assert!(
        violations.is_empty(),
        "schema-backed mutation-set execution must stay behind the explicit schema authoring boundaries: {violations:?}"
    );
}

#[test]
fn schema_seed_helper_names_stay_execution_shaped() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let allowed_relative_files = schema_guard_allowed_files();
    let authority_named_helper_violations = schema_authoring_string_violations(
        &src,
        &allowed_relative_files,
        "seed_milestone_one_primitive_through_schema_authority",
    );
    let minimal_authority_named_helper_violations = schema_authoring_string_violations(
        &src,
        &allowed_relative_files,
        "seed_minimal_topology_through_schema_authority",
    );
    assert!(
        authority_named_helper_violations.is_empty()
            && minimal_authority_named_helper_violations.is_empty(),
        "schema seed helpers must stay execution-shaped instead of authority-shaped: primitive={authority_named_helper_violations:?}, minimal={minimal_authority_named_helper_violations:?}"
    );
}

#[test]
fn schema_seed_types_stay_quarantined_to_boundary_subtree() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let allowed_relative_files = schema_guard_allowed_files();
    let authoring_error_violations = schema_authoring_string_violations(
        &src,
        &allowed_relative_files,
        "MilestoneOnePrimitiveAuthoringError as",
    );
    let minimal_seed_violations =
        schema_authoring_string_violations(&src, &allowed_relative_files, "MinimalTopologySeed as");
    assert!(
        authoring_error_violations.is_empty() && minimal_seed_violations.is_empty(),
        "raw schema primitive seed types must stay quarantined to the schema authoring boundary subtree: authoring_error={authoring_error_violations:?}, minimal_seed={minimal_seed_violations:?}"
    );
}

#[test]
fn closeout_scenario_mainline_seeding_stays_quarantined() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let target = src
        .join("certification")
        .join("topology_operator_closeout")
        .join("scenario_programs");
    let violations =
        schema_authoring_string_violations(&target, &[], "seed_milestone_one_primitive(");
    assert!(
        violations.is_empty(),
        "topology operator closeout scenario programs must seed through the explicit schema authoring boundary: {violations:?}"
    );
}

#[test]
fn declared_query_surface_mainline_seeding_stays_quarantined() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let target = src
        .join("projection")
        .join("runtime_boundary")
        .join("declared_query_surfaces");
    let violations =
        schema_authoring_string_violations(&target, &[], "seed_milestone_one_primitive(");
    assert!(
        violations.is_empty(),
        "declared query surface tests must seed through the explicit schema authoring boundary: {violations:?}"
    );
}

#[test]
fn relation_update_mainline_seeding_stays_quarantined() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let target = src
        .join("projection")
        .join("runtime_boundary")
        .join("query_runtime")
        .join("tests")
        .join("relation_update");
    let violations =
        schema_authoring_string_violations(&target, &[], "seed_milestone_one_primitive(");
    assert!(
        violations.is_empty(),
        "relation-update runtime tests must seed through the explicit schema authoring boundary: {violations:?}"
    );
}

#[test]
fn mutation_application_runtime_seeding_stays_quarantined() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let target = src
        .join("projection")
        .join("runtime_boundary")
        .join("query_runtime")
        .join("tests")
        .join("mutation_application");
    let primitive_violations =
        schema_authoring_string_violations(&target, &[], "seed_milestone_one_primitive(");
    let minimal_violations =
        schema_authoring_string_violations(&target, &[], "seed_minimal_topology(");
    assert!(
        primitive_violations.is_empty() && minimal_violations.is_empty(),
        "mutation-application runtime tests must seed through the explicit schema authoring boundary: primitive={primitive_violations:?}, minimal={minimal_violations:?}"
    );
}

#[test]
fn topology_read_core_proof_seeding_stays_quarantined() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let targets = [
        "certification/projection_closeout/tests/topology_reads/support/mod.rs",
        "certification/projection_closeout/tests/topology_reads/core.rs",
        "certification/projection_closeout/tests/topology_reads/handle_entry.rs",
        "certification/projection_closeout/tests/topology_reads/closeout.rs",
        "certification/projection_closeout/tests/topology_reads/lowering.rs",
        "projection/diagnostic_surfaces/mod.rs",
    ];
    let mut violations = Vec::new();
    for relative in targets {
        let file = src.join(relative);
        let text = fs::read_to_string(&file).expect("rust source is readable");
        if text.contains("seed_milestone_one_primitive(") || text.contains("seed_minimal_topology(")
        {
            violations.push(relative.to_string());
        }
    }
    assert!(
        violations.is_empty(),
        "topology-read core proof files must seed through the explicit schema authoring boundary: {violations:?}"
    );
}

#[test]
fn declaration_entry_runtime_proof_seeding_stays_quarantined() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let target = src
        .join("certification")
        .join("projection_closeout")
        .join("tests")
        .join("topology_reads")
        .join("declaration_entry");
    let primitive_violations =
        schema_authoring_string_violations(&target, &[], "seed_milestone_one_primitive(");
    let minimal_violations =
        schema_authoring_string_violations(&target, &[], "seed_minimal_topology(");
    assert!(
        primitive_violations.is_empty() && minimal_violations.is_empty(),
        "declaration-entry topology-read proof files must seed through the explicit schema authoring boundary: primitive={primitive_violations:?}, minimal={minimal_violations:?}"
    );
}

#[test]
fn runtime_foundation_proof_seeding_stays_quarantined() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let targets = [
        "projection/runtime_boundary/query_runtime/tests/core.rs",
        "projection/runtime_boundary/query_runtime/tests/bridge_verification.rs",
        "projection/runtime_boundary/query_runtime/tests/runtime_posture.rs",
    ];
    let mut violations = Vec::new();
    for relative in targets {
        let file = src.join(relative);
        let text = fs::read_to_string(&file).expect("rust source is readable");
        if text.contains("seed_milestone_one_primitive(") || text.contains("seed_minimal_topology(")
        {
            violations.push(relative.to_string());
        }
    }
    assert!(
        violations.is_empty(),
        "runtime-foundation proof files must seed through the explicit schema authoring boundary: {violations:?}"
    );
}

#[test]
fn topology_operator_closeout_proof_seeding_stays_quarantined() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let targets = [
        "certification/topology_operator_closeout/query_traversal_proof/mutation_query_traversal.rs",
        "certification/topology_operator_closeout/operator_family_proof/primitive_family_closure.rs",
        "certification/topology_operator_closeout/operator_family_proof/primitive_family_wire_closure.rs",
        "certification/topology_operator_closeout/scale_pressure_proof/scale_pressure.rs",
        "certification/topology_operator_closeout/scale_pressure_proof/scale_pressure_detach.rs",
        "certification/topology_operator_closeout/scale_pressure_proof/sweeps/radial.rs",
    ];
    let mut violations = Vec::new();
    for relative in targets {
        let file = src.join(relative);
        let text = fs::read_to_string(&file).expect("rust source is readable");
        if text.contains("seed_milestone_one_primitive(") || text.contains("seed_minimal_topology(")
        {
            violations.push(relative.to_string());
        }
    }
    assert!(
        violations.is_empty(),
        "topology-operator closeout proof files must seed through the explicit schema authoring boundary: {violations:?}"
    );
}

#[test]
fn projection_support_and_bridge_seeding_stays_quarantined() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let targets = [
        "certification/projection_closeout/tests/materialization.rs",
        "certification/projection_closeout/tests/row_lookup.rs",
        "projection/runtime_boundary/bridge/tests.rs",
        "test_support/primitive_corpus/validated_topology.rs",
        "test_support/hostile_neighborhoods/validation_neighborhoods/seeded_and_closed_shell.rs",
    ];
    let mut violations = Vec::new();
    for relative in targets {
        let file = src.join(relative);
        let text = fs::read_to_string(&file).expect("rust source is readable");
        if text.contains("seed_milestone_one_primitive(") || text.contains("seed_minimal_topology(")
        {
            violations.push(relative.to_string());
        }
    }
    assert!(
        violations.is_empty(),
        "projection-support and bridge-support files must seed through the explicit schema authoring boundary: {violations:?}"
    );
}

#[test]
fn validation_and_derived_seeding_stays_quarantined() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let targets = [
        "validation/tests.rs",
        "validation/reference_integrity/tests/bootstrap_boundary.rs",
        "derived_topology/traversal_views/tests.rs",
        "derived_topology/materialized_graph/tests.rs",
    ];
    let mut violations = Vec::new();
    for relative in targets {
        let file = src.join(relative);
        let text = fs::read_to_string(&file).expect("rust source is readable");
        if text.contains("seed_milestone_one_primitive(") || text.contains("seed_minimal_topology(")
        {
            violations.push(relative.to_string());
        }
    }
    assert!(
        violations.is_empty(),
        "validation and derived-topology files must seed through the explicit schema authoring boundary: {violations:?}"
    );
}

fn schema_authoring_string_violations(
    root: &std::path::Path,
    allowed_relative_files: &[&str],
    needle: &str,
) -> Vec<String> {
    let mut violations = Vec::new();
    for file in rust_files(root) {
        let relative = src_relative_path(&file);
        if allowed_relative_files.contains(&relative.as_str()) {
            continue;
        }
        let text = fs::read_to_string(&file).expect("rust source is readable");
        if text.contains(needle) {
            violations.push(relative);
        }
    }
    violations
}

fn schema_guard_allowed_files() -> Vec<&'static str> {
    let mut files = vec!["certification/structure_guard_schema_authoring.rs"];
    files.extend_from_slice(SCHEMA_BOUNDARY_FILES);
    files
}
