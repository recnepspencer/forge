use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

mod compile_fail_fixture_catalog;

#[path = "contracts/graph_read_access_inventory/fixture_catalog.rs"]
mod graph_read_access_inventory_contracts;
#[allow(dead_code)]
#[path = "contracts/public_api_planar_boolean_loop_reconstruction_guard_coverage.rs"]
mod loop_reconstruction_guard_coverage;

use compile_fail_fixture_catalog::{
    COMPILE_FAIL_FIXTURES, PLANAR_BOOLEAN_LOOP_RECONSTRUCTION_CORE_FIXTURES,
};
use graph_read_access_inventory_contracts::graph_read_access_inventory_expected_compile_fail_fixtures;
use loop_reconstruction_guard_coverage::loop_reconstruction_compile_fail_fixtures;

const PLANAR_BOOLEAN_ENTRY_BASIS_KERNEL_SUMMARY_FIXTURE: &str =
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_entry_basis/public_planar_boolean_entry_basis_rejects_kernel_summary_substitution.rs";

const PLANAR_BOOLEAN_COMMON_PLANE_REDUCTION_FIXTURES: &[&str] = &[
    "src/certification/public_facade_contracts/compile_fail/pb_common_plane/reduction_request_fields_private.rs",
    "src/certification/public_facade_contracts/compile_fail/pb_common_plane/scope_admitted_request_fields_private.rs",
    "src/certification/public_facade_contracts/compile_fail/pb_common_plane/plane_agreed_request_fields_private.rs",
    "src/certification/public_facade_contracts/compile_fail/pb_common_plane/local_frame_selected_request_fields_private.rs",
    "src/certification/public_facade_contracts/compile_fail/pb_common_plane/reduced_pair_request_fields_private.rs",
    "src/certification/public_facade_contracts/compile_fail/pb_common_plane/shared_plane_identified_request_fields_private.rs",
];

const PLANAR_BOOLEAN_EVENT_EXTRACTION_FIXTURES: &[&str] = &[
    "src/certification/public_facade_contracts/compile_fail/pb_events/event_extraction_request_fields_private.rs",
    "src/certification/public_facade_contracts/compile_fail/pb_events/event_extraction_request_rejects_identity_shortcut.rs",
    "src/certification/public_facade_contracts/compile_fail/pb_events/event_extraction_request_rejects_receipt_only_shortcut.rs",
    "src/certification/public_facade_contracts/compile_fail/pb_events/ledger_ctor_bypass.rs",
    "src/certification/public_facade_contracts/compile_fail/pb_events/event_row_bypass.rs",
    "src/certification/public_facade_contracts/compile_fail/pb_events/raw_pair_bypass.rs",
    "src/certification/public_facade_contracts/compile_fail/pb_events/split_no_ledger.rs",
];

const PLANAR_BOOLEAN_EDGE_SPLITTING_FIXTURES: &[&str] = &[
    "src/certification/public_facade_contracts/compile_fail/pb_edge_splitting/split_request_not_completed_split_evidence.rs",
    "src/certification/public_facade_contracts/compile_fail/pb_edge_splitting/split_request_not_boolean_evidence_row.rs",
    "src/certification/public_facade_contracts/compile_fail/pb_edge_splitting/fake_split_receipt_not_boolean_evidence_row.rs",
];

const PLANAR_BOOLEAN_EDGE_SPLITTING_EXPECTED_ERRORS: &[(&str, &str)] = &[
    (
        "src/certification/public_facade_contracts/compile_fail/pb_edge_splitting/split_request_not_completed_split_evidence.rs",
        "PlanarBooleanSplitEdgeChainLedgerReceipt",
    ),
    (
        "src/certification/public_facade_contracts/compile_fail/pb_edge_splitting/split_request_not_boolean_evidence_row.rs",
        "BooleanEvidenceRowAuthority",
    ),
    (
        "src/certification/public_facade_contracts/compile_fail/pb_edge_splitting/fake_split_receipt_not_boolean_evidence_row.rs",
        "BooleanEvidenceRowAuthority",
    ),
];

const WORTH_GRAPH_AUTHORITY_FIXTURES: &[&str] = &[
    "src/certification/public_facade_contracts/compile_fail/worth_graph_authority/raw_gate_certifier_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/worth_graph_authority/inventory_row_fields_private.rs",
    "src/certification/public_facade_contracts/compile_fail/worth_graph_authority/closeout_certifier_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/worth_graph_authority/closeout_matrix_row_fields_private.rs",
];

const QUERY_OBLIGATION_SELECTION_FIXTURES: &[&str] = &[
    "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/internal_authority/from_authority_parts_private.rs",
    "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/internal_authority/with_spatial_descriptor_private.rs",
    "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/internal_authority/raw_selection_input_fields_private.rs",
    "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/internal_authority/selected_obligations_constructor_private.rs",
    "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/internal_authority/in_memory_proof_cannot_build_selected_obligations.rs",
    "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/internal_authority/local_ceremony_audit_cannot_build_selected_obligations.rs",
    "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/internal_authority/residue_manifest_cannot_build_selected_obligations.rs",
    "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/spatial_substitution/spatial_raw_row_cannot_select_query_obligations.rs",
    "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/spatial_substitution/spatial_lookup_product_cannot_select_without_authority.rs",
    "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/spatial_substitution/spatial_query_descriptor_fields_cannot_be_copied.rs",
    "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/spatial_substitution/topology_touched_basis_cannot_select_spatial_obligations.rs",
    "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/public_facade/public_request_fields_private.rs",
    "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/public_facade/public_selected_closeout_fields_private.rs",
    "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/public_facade/public_selected_status_fields_private.rs",
    "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/public_facade/public_selected_status_from_selected_private.rs",
    "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/public_facade/public_workload_rejects_raw_string_selection.rs",
    "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/public_facade/public_workload_rejects_copied_count_selection.rs",
    "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/public_facade/public_workload_rejects_lookup_product_selection.rs",
];

const QUERY_OBLIGATION_SELECTION_SPATIAL_EXPECTED_ERRORS: &[(&str, &str)] = &[
    (
        "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/spatial_substitution/spatial_raw_row_cannot_select_query_obligations.rs",
        "expected `QueryObligationSelectionInput`",
    ),
    (
        "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/spatial_substitution/spatial_lookup_product_cannot_select_without_authority.rs",
        "expected `QueryObligationSelectionInput`",
    ),
    (
        "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/spatial_substitution/spatial_query_descriptor_fields_cannot_be_copied.rs",
        "private",
    ),
    (
        "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/spatial_substitution/topology_touched_basis_cannot_select_spatial_obligations.rs",
        "expected `&SpatialEvidenceQueryTouchDescriptor`",
    ),
];

const QUERY_OBLIGATION_SELECTION_AUTHORITY_EXPECTED_ERRORS: &[(&str, &str)] = &[
    (
        "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/internal_authority/selected_obligations_constructor_private.rs",
        "associated function `from_query_proof` is private",
    ),
    (
        "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/internal_authority/in_memory_proof_cannot_build_selected_obligations.rs",
        "associated function `from_query_proof` is private",
    ),
    (
        "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/internal_authority/local_ceremony_audit_cannot_build_selected_obligations.rs",
        "associated function `from_query_proof` is private",
    ),
    (
        "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/internal_authority/residue_manifest_cannot_build_selected_obligations.rs",
        "associated function `from_query_proof` is private",
    ),
    (
        "src/certification/public_facade_contracts/compile_fail/query_obligation_selection/public_facade/public_selected_status_from_selected_private.rs",
        "associated function `from_selected` is private",
    ),
];

const BATCH_ADMISSION_EXECUTION_RECEIPT_FIXTURES: &[&str] = &[
    "src/certification/public_facade_contracts/compile_fail/batch_admission_execution/public_execution_receipt_from_selected_plan_private.rs",
    "src/certification/public_facade_contracts/compile_fail/batch_admission_execution/public_execution_receipt_struct_literal_private.rs",
];

const ORDINARY_CONSUMER_CUTOVER_FIXTURES: &[&str] = &[
    "src/certification/public_facade_contracts/compile_fail/pb_loop_reconstruction/split_handoff_loop_wrapper_removed.rs",
    "src/certification/public_facade_contracts/compile_fail/pb_loop_reconstruction/split_handoff_chain_wrapper_removed.rs",
];

const TOUCHED_GRAPH_CONFLICT_CONSTRUCTOR_DENIAL_EXPECTED_ERRORS: &[(&str, &str)] = &[
    (
        "src/certification/public_facade_contracts/compile_fail/touched_graph_conflict/family_row_not_constructible.rs",
        "private",
    ),
    (
        "src/certification/public_facade_contracts/compile_fail/touched_graph_conflict/spatial_family_row_not_constructible.rs",
        "private",
    ),
    (
        "src/certification/public_facade_contracts/compile_fail/touched_graph_conflict/admitted_input_not_constructible.rs",
        "private",
    ),
    (
        "src/certification/public_facade_contracts/compile_fail/touched_graph_conflict/spatial_admitted_input_not_constructible.rs",
        "private",
    ),
    (
        "src/certification/public_facade_contracts/compile_fail/touched_graph_conflict/selected_plan_not_constructible.rs",
        "private",
    ),
    (
        "src/certification/public_facade_contracts/compile_fail/touched_graph_conflict/spatial_selected_plan_not_constructible.rs",
        "private",
    ),
    (
        "src/certification/public_facade_contracts/compile_fail/touched_graph_conflict/independence_proof_not_constructible.rs",
        "private",
    ),
    (
        "src/certification/public_facade_contracts/compile_fail/touched_graph_conflict/spatial_independence_proof_not_constructible.rs",
        "private",
    ),
    (
        "src/certification/public_facade_contracts/compile_fail/touched_graph_conflict/selected_batch_plan_not_constructible.rs",
        "private",
    ),
    (
        "src/certification/public_facade_contracts/compile_fail/touched_graph_conflict/closeout_product_not_constructible.rs",
        "private",
    ),
    (
        "src/certification/public_facade_contracts/compile_fail/touched_graph_conflict/public_closeout_product_not_constructible.rs",
        "private",
    ),
    (
        "src/certification/public_facade_contracts/compile_fail/touched_graph_conflict/milestone_fourteen_seed_not_constructible.rs",
        "private",
    ),
];

#[test]
fn kernel_public_boundary_rejects_internal_constructor_bypass() {
    for fixture in COMPILE_FAIL_FIXTURES {
        assert_compile_fail_fixture(fixture);
    }
}

#[test]
fn kernel_public_boundary_rejects_planar_boolean_common_plane_reduction_constructor_bypass() {
    for fixture in PLANAR_BOOLEAN_COMMON_PLANE_REDUCTION_FIXTURES {
        assert_compile_fail_fixture(fixture);
    }
}

#[test]
fn kernel_public_boundary_rejects_planar_boolean_event_extraction_constructor_bypass() {
    for fixture in PLANAR_BOOLEAN_EVENT_EXTRACTION_FIXTURES {
        assert_compile_fail_fixture(fixture);
    }
}

#[test]
fn kernel_public_boundary_rejects_incomplete_planar_boolean_edge_split_evidence() {
    for (fixture, expected_stderr) in PLANAR_BOOLEAN_EDGE_SPLITTING_EXPECTED_ERRORS {
        assert_compile_fail_fixture_with_stderr(fixture, expected_stderr);
    }
}

#[test]
fn kernel_public_boundary_rejects_worth_graph_authority_gate_forgery() {
    for fixture in WORTH_GRAPH_AUTHORITY_FIXTURES {
        assert_compile_fail_fixture(fixture);
    }
    for guard in
        worth_kernel::query_graph_authority_gate::current_worth_lower_authority_promotion_guard_plan(
        )
    {
        assert_compile_fail_fixture(guard.planned_compile_fail_path());
    }
}

#[test]
fn kernel_public_boundary_rejects_graph_read_access_inventory_forgery() {
    for (fixture, expected_stderr) in graph_read_access_inventory_expected_compile_fail_fixtures() {
        assert_compile_fail_fixture_with_stderr(fixture, expected_stderr);
    }
}

#[test]
fn kernel_public_boundary_rejects_query_obligation_selection_forgery() {
    for fixture in QUERY_OBLIGATION_SELECTION_FIXTURES {
        assert_compile_fail_fixture(fixture);
    }
    for (fixture, expected_stderr) in QUERY_OBLIGATION_SELECTION_AUTHORITY_EXPECTED_ERRORS {
        assert_compile_fail_fixture_with_stderr(fixture, expected_stderr);
    }
}

#[test]
fn kernel_public_boundary_rejects_spatial_query_obligation_substitution_for_expected_reasons() {
    for (fixture, expected_stderr) in QUERY_OBLIGATION_SELECTION_SPATIAL_EXPECTED_ERRORS {
        assert_compile_fail_fixture_with_stderr(fixture, expected_stderr);
    }
}

#[test]
fn kernel_public_boundary_rejects_planar_boolean_loop_reconstruction_constructor_bypass() {
    for fixture in PLANAR_BOOLEAN_LOOP_RECONSTRUCTION_CORE_FIXTURES {
        assert_compile_fail_fixture(fixture);
    }
    for fixture in loop_reconstruction_compile_fail_fixtures() {
        assert_compile_fail_fixture(fixture);
    }
}

#[test]
fn kernel_public_boundary_rejects_planar_boolean_summary_substitution_fixture() {
    assert_compile_fail_fixture(PLANAR_BOOLEAN_ENTRY_BASIS_KERNEL_SUMMARY_FIXTURE);
}

#[test]
fn kernel_public_boundary_rejects_batch_admission_execution_receipt_constructor_bypass() {
    for fixture in BATCH_ADMISSION_EXECUTION_RECEIPT_FIXTURES {
        assert_compile_fail_fixture(fixture);
    }
}

#[test]
fn kernel_public_boundary_rejects_deleted_ordinary_consumer_wrapper_lane() {
    for fixture in ORDINARY_CONSUMER_CUTOVER_FIXTURES {
        assert_compile_fail_fixture(fixture);
    }
}

#[test]
fn kernel_public_boundary_rejects_touched_graph_conflict_constructor_bypass() {
    for (fixture, expected_stderr) in TOUCHED_GRAPH_CONFLICT_CONSTRUCTOR_DENIAL_EXPECTED_ERRORS {
        assert_compile_fail_fixture_with_stderr(fixture, expected_stderr);
    }
}

fn assert_compile_fail_fixture(fixture: &str) {
    assert_compile_fail_fixture_failure(fixture, None);
}

fn assert_compile_fail_fixture_with_stderr(fixture: &str, expected_stderr: &str) {
    assert_compile_fail_fixture_failure(fixture, Some(expected_stderr));
}

fn assert_compile_fail_fixture_failure(fixture: &str, expected_stderr: Option<&str>) {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let temp_root = temp_fixture_dir();

    write_temp_manifest(&manifest_dir, &temp_root);
    copy_fixture_main(&manifest_dir, fixture, &temp_root);

    let output = Command::new("cargo")
        .arg("check")
        .arg("--quiet")
        .arg("--manifest-path")
        .arg(temp_root.join("Cargo.toml"))
        .env("CARGO_TARGET_DIR", compile_fail_target_dir(&manifest_dir))
        .output()
        .expect("run cargo check for compile-fail fixture");
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "expected fixture to fail: {}\nstdout:\n{}\nstderr:\n{}",
        fixture,
        String::from_utf8_lossy(&output.stdout),
        stderr
    );
    if let Some(expected_stderr) = expected_stderr {
        assert!(
            stderr.contains(expected_stderr),
            "expected fixture {} stderr to contain {:?}\nstderr:\n{}",
            fixture,
            expected_stderr,
            stderr
        );
    }

    let _ = fs::remove_dir_all(&temp_root);
}

fn write_temp_manifest(manifest_dir: &Path, temp_root: &Path) {
    let crate_root = normalize_path(manifest_dir);
    let workspace_crates = normalize_path(
        manifest_dir
            .parent()
            .expect("worth-kernel lives in crates/")
            .to_path_buf(),
    );
    let forge_query = format!("{workspace_crates}/forge-query");
    let worth_spatial = format!("{workspace_crates}/worth-spatial");

    let src_dir = temp_root.join("src");
    fs::create_dir_all(&src_dir).expect("create temp src dir");
    fs::write(
        temp_root.join("Cargo.toml"),
        format!(
            "[package]\nname = \"worth_kernel_compile_fail\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nworth-kernel = {{ path = \"{crate_root}\" }}\nforge-query = {{ path = \"{forge_query}\" }}\nworth-spatial = {{ path = \"{worth_spatial}\" }}\nworth-topo = {{ path = \"{workspace_crates}/worth-topo\" }}\n"
        ),
    )
    .expect("write temp Cargo.toml");
}

fn copy_fixture_main(manifest_dir: &Path, fixture: &str, temp_root: &Path) {
    let fixture_path = manifest_dir.join(fixture);
    fs::copy(&fixture_path, temp_root.join("src").join("main.rs")).expect("copy fixture main.rs");
}

fn normalize_path(path: impl AsRef<Path>) -> String {
    path.as_ref().to_string_lossy().replace('\\', "/")
}

fn temp_fixture_dir() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("wkcf-{stamp}"))
}

fn compile_fail_target_dir(manifest_dir: &Path) -> PathBuf {
    manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("worth-kernel lives under workspace crates/")
        .join("target")
        .join("worth-kernel-compile-fail")
}
