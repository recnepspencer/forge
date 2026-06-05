use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const COMPILE_FAIL_FIXTURES: &[&str] = &[
    "src/certification/public_facade_contracts/compile_fail/authority/public_authoring_session_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/authority/public_authoring_session_prepare_helpers_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/authority/public_kernel_certification_plan_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_raw_request_cannot_skip_admission.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_intent_admission_phase_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_spatial_intent_direct_admission_helpers_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_spatial_intent_catalog_admission_helpers_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_spatial_intent_finish_helpers_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_spatial_intent_catalog_finish_helpers_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_admitted_intent_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_admitted_handoff_helper_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_scaffold_handoff_helper_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_raw_handoff_helper_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_execution_phase_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_authoring_input_traits_are_sealed.rs",
    "src/certification/public_facade_contracts/compile_fail/phases/public_binding_declaration_entry_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/results/public_canonical_artifact_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/results/public_prepared_result_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/results/public_root_happy_path_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/results/public_create_placement_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/results/public_outcome_prepared_happy_path_helpers_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/results/public_prelude_happy_path_helpers_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_branch_local_parity_report_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_query_basis_preview_parity_report_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_query_boundary_gap_register_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_realization_exhaustion_report_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_query_diagnostics_bucket_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_realization_diagnostics_bucket_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_intent_arbitration_report_bundle_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_intent_arbitration_hostility_suite_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_intent_arbitration_helper_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_authoring_bucket_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_top_level_spatial_intent_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_spatial_intent_lowering_error_not_exported_from_intents.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_prelude_spatial_intent_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_diagnostics_bucket_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_misclassified_surface_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_preview_report_bundle_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_preview_helper_exports_demoted.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_continuity_report_bundle_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_policy_profile_report_bundle_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_preview_continuity_hostility_suite_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_certification_bucket_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_verified_intent_arbitration_report_bundle_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_motion_runtime_proof_reports_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_preview_runtime_proof_reports_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_continuity_runtime_proof_reports_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_policy_runtime_proof_reports_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_arbitration_runtime_proof_reports_not_exported.rs",
];

#[test]
fn kernel_public_boundary_rejects_internal_constructor_bypass() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let crate_root = normalize_path(&manifest_dir);
    let workspace_crates = normalize_path(
        manifest_dir
            .parent()
            .expect("worth-kernel lives in crates/")
            .to_path_buf(),
    );
    let forge_query = format!("{workspace_crates}/forge-query");
    let worth_spatial = format!("{workspace_crates}/worth-spatial");

    let temp_root = temp_fixture_dir();
    let src_dir = temp_root.join("src");
    fs::create_dir_all(&src_dir).expect("create temp src dir");
    fs::write(
        temp_root.join("Cargo.toml"),
        format!(
            "[package]\nname = \"worth_kernel_compile_fail\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nworth-kernel = {{ path = \"{crate_root}\" }}\nforge-query = {{ path = \"{forge_query}\" }}\nworth-spatial = {{ path = \"{worth_spatial}\" }}\n"
        ),
    )
    .expect("write temp Cargo.toml");

    for fixture in COMPILE_FAIL_FIXTURES {
        let fixture_path = manifest_dir.join(fixture);
        fs::copy(&fixture_path, src_dir.join("main.rs")).expect("copy fixture main.rs");

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
    }

    let _ = fs::remove_dir_all(&temp_root);
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
