use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const PLANAR_BOOLEAN_ENTRY_BASIS_KERNEL_SUMMARY_FIXTURE: &str =
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_entry_basis/public_planar_boolean_entry_basis_rejects_kernel_summary_substitution.rs";

const COMPILE_FAIL_FIXTURES: &[&str] = &[
    "src/certification/public_facade_contracts/compile_fail/authority/public_authoring_session_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/authority/public_authoring_session_prepare_helpers_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/authority/public_construction_query_native_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/authority/public_construction_policy_and_arbitration_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/authority/public_construction_tolerance_precision_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/authority/public_authoring_entry_prepare_outcome_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/authority/public_authoring_entry_prepare_result_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/authority/public_kernel_certification_plan_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_raw_request_cannot_skip_admission.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_raw_spatial_fixture_cannot_be_operator_workload.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_intent_admission_phase_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_authoring_input_traits_are_sealed.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_motion_intent_direct_admission_helpers_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_motion_intent_catalog_admission_helpers_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_motion_intent_finish_helpers_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_motion_intent_catalog_finish_helpers_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_construction_motion_witness_resolution_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_construction_branch_preview_basis_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_admitted_intent_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_admitted_handoff_helper_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_scaffold_handoff_helper_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_raw_handoff_helper_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_execution_phase_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_binding_declaration_entry_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_entry/public_planar_boolean_declaration_receipt_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_entry/public_planar_boolean_blocker_evidence_receipt_fields_not_public.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_entry/public_planar_boolean_operand_pair_construction_receipt_fields_not_public.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_entry/public_planar_boolean_outcome_receipt_fields_not_public.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_entry/public_planar_boolean_support_receipt_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_entry_basis/public_planar_boolean_entry_basis_fields_not_public.rs",
    PLANAR_BOOLEAN_ENTRY_BASIS_KERNEL_SUMMARY_FIXTURE,
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_entry_basis/public_planar_boolean_entry_basis_rejects_generic_ledger_substitution.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_entry_basis/public_planar_boolean_entry_basis_rejects_hand_built_planar_facts.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_entry_basis/public_planar_boolean_entry_basis_rejects_topology_seed_substitution.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_entry_basis/public_planar_boolean_entry_basis_rejects_worth_workload_substitution.rs",
    "src/certification/public_facade_contracts/compile_fail/workload_catalog/public_built_boolean_operand_pair_recipe_fields_not_public.rs",
    "src/certification/public_facade_contracts/compile_fail/workload_catalog/public_workload_catalog_static_fixture_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/workload_catalog/public_workload_catalog_rejects_raw_topology_rows_for_nmt.rs",
    "src/certification/public_facade_contracts/compile_fail/authority/public_binding_and_anchoring_authoring_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/results/public_canonical_artifact_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/results/public_prepared_result_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/results/public_root_happy_path_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/results/public_create_placement_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/results/public_prepared_result_realization_report_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/results/public_outcome_prepared_happy_path_helpers_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/results/public_prelude_happy_path_helpers_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_query_basis_preview_parity_report_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_realization_exhaustion_report_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_realization_diagnostics_bucket_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_intent_arbitration_report_bundle_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_intent_arbitration_hostility_suite_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_intent_arbitration_helper_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_authoring_bucket_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_authoring_runtime_basis_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_top_level_motion_intent_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_motion_intent_error_not_exported_from_intents.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_prelude_motion_intent_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_diagnostics_bucket_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_authoring_policy_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_construction_policy_assessment_bags_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_arbitration_diagnostics_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_preview_diagnostics_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_continuity_diagnostics_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_policy_diagnostics_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_motion_diagnostics_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_misclassified_surface_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_preview_report_bundle_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_preview_helper_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_continuity_report_bundle_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_policy_profile_report_bundle_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_preview_continuity_hostility_suite_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_certification_bucket_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_verified_intent_arbitration_report_bundle_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_motion_certification_reports_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_preview_certification_reports_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_continuity_certification_reports_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_policy_certification_reports_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_arbitration_certification_reports_not_exported.rs",
];

#[test]
fn kernel_public_boundary_rejects_internal_constructor_bypass() {
    for fixture in COMPILE_FAIL_FIXTURES {
        assert_compile_fail_fixture(fixture);
    }
}

#[test]
fn kernel_public_boundary_rejects_planar_boolean_summary_substitution_fixture() {
    assert_compile_fail_fixture(PLANAR_BOOLEAN_ENTRY_BASIS_KERNEL_SUMMARY_FIXTURE);
}

fn assert_compile_fail_fixture(fixture: &str) {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let temp_root = temp_fixture_dir();

    write_temp_manifest(&manifest_dir, &temp_root);
    copy_fixture_main(&manifest_dir, fixture, &temp_root);

    let output = Command::new("cargo")
        .arg("check")
        .arg("--quiet")
        .arg("--manifest-path")
        .arg(temp_root.join("Cargo.toml"))
        .output()
        .expect("run cargo check for compile-fail fixture");

    assert!(
        !output.status.success(),
        "expected fixture to fail: {}\nstdout:\n{}\nstderr:\n{}",
        fixture,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

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
    std::env::temp_dir().join(format!("worth-kernel-compile-fail-{stamp}"))
}
