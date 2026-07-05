use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

#[path = "phase_fifteen_fixture_inventory.rs"]
mod phase_fifteen_fixture_inventory;
#[path = "phase_fourteen_fixture_inventory.rs"]
mod phase_fourteen_fixture_inventory;

const GENERIC_DIGEST_WRAPPER_DENIAL_FIXTURES: &[&str] = &[
    "src/certification/public_facade_contracts/compile_fail/spatial_evidence_lookup/generic_digest_bridge_denials/lookup_digest_is_not_query_descriptor_digest.rs",
    "src/certification/public_facade_contracts/compile_fail/spatial_evidence_lookup/generic_digest_bridge_denials/query_digest_is_not_lookup_product_digest.rs",
];
const OVERLAP_REQUEST_BOUNDARY_FIXTURES: &[&str] = &[
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_overlap_region_extraction/loop_ledger_only_request_entry_not_available.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_overlap_region_extraction/request_not_hand_filled_from_copied_fields.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_overlap_region_extraction/legacy_workload_operator_facade_not_public.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_overlap_region_extraction/raw_arrangement_cells_do_not_admit_classification_input.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_overlap_region_extraction/winding_input_requires_containment_map.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_overlap_region_extraction/raw_arrangement_graph_does_not_admit_island_candidate_input.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_overlap_region_extraction/raw_arrangement_graph_does_not_admit_island_component_bundle.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_overlap_region_extraction/overlap_islands_do_not_admit_boundary_contact_classification_bundle.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_overlap_region_extraction/overlap_islands_do_not_classify_boundary_contact_components.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_overlap_region_extraction/boundary_contact_classification_input_requires_area_overlap_component_set.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_overlap_region_extraction/area_overlap_components_do_not_admit_shared_area_components.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_overlap_region_extraction/pure_boundary_only_outcomes_do_not_admit_shared_area_components.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_overlap_region_extraction/shared_area_admission_outcomes_do_not_admit_pre_region_normalization.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_overlap_region_extraction/mixed_boundary_area_outcomes_do_not_admit_pre_region_normalization.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_overlap_region_extraction/admitted_overlap_regions_do_not_normalize_post_admission_canonical_winding.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_overlap_region_extraction/boundary_only_outcomes_do_not_normalize_post_admission_canonical_winding.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_overlap_region_extraction/canonical_winding_set_does_not_mint_overlap_region_identity_lineage.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_overlap_region_extraction/identity_map_does_not_mint_overlap_region_ledger.rs",
    "src/certification/public_facade_contracts/compile_fail/planar_boolean_overlap_region_extraction/public_contract_fence_input_not_exported.rs",
];

#[test]
fn spatial_public_boundary_rejects_internal_constructor_bypass() {
    let fixtures = compile_fail_fixture_paths(compile_fail_root());

    assert!(
        !fixtures.is_empty(),
        "public-boundary compile-fail suite must discover fixtures"
    );

    let test_cases = trybuild::TestCases::new();
    for fixture in fixtures {
        test_cases.compile_fail(fixture);
    }
}

#[test]
fn public_api_cannot_forge_compiled_product_or_reuse_products() {
    let fences = phase_fifteen_fixture_inventory::phase_fifteen_spatial_compile_fail_fences();
    let unique_fixture_paths = fences
        .iter()
        .map(|fence| fence.fixture_path())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        unique_fixture_paths.len(),
        fences.len(),
        "each certified spatial fence class must map to its own executed compile-fail fixture"
    );

    let test_cases = trybuild::TestCases::new();
    for fence in fences {
        assert!(
            Path::new(fence.fixture_path()).exists(),
            "compile-fail fixture must exist: {}",
            fence.fixture_path()
        );
        run_compile_fail_fixture(&test_cases, fence.fixture_path());
    }
}

#[test]
fn generic_digest_wrappers_cannot_bridge_query_and_lookup_products() {
    run_explicit_compile_fail_fixture_inventory(GENERIC_DIGEST_WRAPPER_DENIAL_FIXTURES);
}

#[test]
fn overlap_request_boundary_rejects_loop_ledger_only_and_copied_field_entry() {
    run_explicit_compile_fail_fixture_inventory(OVERLAP_REQUEST_BOUNDARY_FIXTURES);
}

#[test]
fn phase_fourteen_spatial_reintroduction_and_raw_part_fixtures_hold() {
    let fences = phase_fourteen_fixture_inventory::phase_fourteen_spatial_compile_fail_fences();
    let unique_fixture_paths = fences
        .iter()
        .map(|fence| fence.fixture_path())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        unique_fixture_paths.len(),
        fences.len(),
        "each phase 14 spatial fence fixture should be executed exactly once"
    );

    let test_cases = trybuild::TestCases::new();
    for fence in fences {
        assert!(
            Path::new(fence.fixture_path()).exists(),
            "compile-fail fixture must exist: {}",
            fence.fixture_path()
        );
        run_compile_fail_fixture(&test_cases, fence.fixture_path());
    }
}

fn compile_fail_root() -> &'static Path {
    Path::new("src/certification/public_facade_contracts/compile_fail")
}

fn compile_fail_fixture_paths(compile_fail_root: &Path) -> Vec<String> {
    let mut fixtures = Vec::new();
    collect_compile_fail_fixtures(compile_fail_root, &mut fixtures);
    fixtures.sort();
    fixtures
}

fn run_explicit_compile_fail_fixture_inventory(fixture_inventory: &[&str]) {
    assert!(
        !fixture_inventory.is_empty(),
        "targeted compile-fail inventory must not be empty"
    );

    for fixture in fixture_inventory {
        assert!(
            Path::new(fixture).exists(),
            "compile-fail fixture must exist: {fixture}"
        );
    }

    let test_cases = trybuild::TestCases::new();
    for fixture in fixture_inventory {
        run_compile_fail_fixture(&test_cases, fixture);
    }
}

fn run_compile_fail_fixture(test_cases: &trybuild::TestCases, fixture_path: &str) {
    if let Some(manifest_path) = manifest_backed_fixture_manifest_path(fixture_path) {
        run_manifest_backed_compile_fail_fixture(fixture_path, &manifest_path);
        return;
    }

    test_cases.compile_fail(fixture_path);
}

fn manifest_backed_fixture_manifest_path(fixture_path: &str) -> Option<PathBuf> {
    let fixture_path = Path::new(fixture_path);
    let manifest_path = fixture_path.parent()?.parent()?.join("Cargo.toml");
    manifest_path.exists().then_some(manifest_path)
}

fn run_manifest_backed_compile_fail_fixture(fixture_path: &str, manifest_path: &Path) {
    let expected_diagnostic_path =
        Path::new(fixture_path.trim_end_matches(".rs")).with_extension("stderr");
    assert!(
        expected_diagnostic_path.exists(),
        "manifest-backed compile-fail diagnostic must exist: {}",
        expected_diagnostic_path.display()
    );

    let fixture_target_dir = std::env::temp_dir()
        .join("worth-spatial-phase14-fixture-target")
        .join(
            manifest_path
                .parent()
                .and_then(Path::file_name)
                .expect("manifest-backed fixture manifest should have a parent directory"),
        );
    let output = Command::new("cargo")
        .args([
            "check",
            "--manifest-path",
            &manifest_path.to_string_lossy(),
            "--target-dir",
            &fixture_target_dir.to_string_lossy(),
            "--color",
            "never",
            "--quiet",
        ])
        .output()
        .expect("manifest-backed compile-fail fixture should run cargo check");

    assert!(
        !output.status.success(),
        "manifest-backed compile-fail fixture unexpectedly compiled: {fixture_path}"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    let expected_diagnostic = std::fs::read_to_string(&expected_diagnostic_path)
        .expect("manifest-backed compile-fail diagnostic should be readable");
    for required_fragment in expected_diagnostic
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
    {
        assert!(
            stderr.contains(required_fragment),
            "manifest-backed compile-fail stderr for {fixture_path} missing required fragment `{required_fragment}`\n--- stderr ---\n{stderr}"
        );
    }
}

fn collect_compile_fail_fixtures(directory: &Path, fixtures: &mut Vec<String>) {
    for entry in std::fs::read_dir(directory).expect("compile-fail fixture directory must exist") {
        let path = entry
            .expect("compile-fail fixture entry must be readable")
            .path();
        if path.is_dir() {
            collect_compile_fail_fixtures(&path, fixtures);
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            fixtures.push(path.to_string_lossy().replace('\\', "/"));
        }
    }
}
