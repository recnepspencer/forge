use std::path::Path;

const PUBLIC_API_CANNOT_FORGE_LOOKUP_PRODUCTS_FIXTURES: &[&str] = &[
    "src/certification/public_facade_contracts/compile_fail/evidence_lookup_family_catalog/struct_literals/catalog_closeout_not_hand_filled.rs",
    "src/certification/public_facade_contracts/compile_fail/evidence_lookup_family_catalog/struct_literals/family_declaration_not_hand_filled.rs",
    "src/certification/public_facade_contracts/compile_fail/evidence_lookup_input_admission/struct_literals/admitted_input_not_hand_filled.rs",
    "src/certification/public_facade_contracts/compile_fail/evidence_lookup_plan_selection/struct_literals/selected_plan_not_hand_filled.rs",
    "src/certification/public_facade_contracts/compile_fail/evidence_lookup_execution/struct_literals/receipt_not_hand_filled.rs",
    "src/certification/public_facade_contracts/compile_fail/evidence_lookup_diagnostics/struct_literals/row_not_hand_filled.rs",
    "src/certification/public_facade_contracts/compile_fail/evidence_lookup_source_firewall/struct_literals/report_not_hand_filled.rs",
    "src/certification/public_facade_contracts/compile_fail/evidence_lookup_source_firewall/struct_literals/row_not_hand_filled.rs",
    "src/certification/public_facade_contracts/compile_fail/evidence_lookup_query_consumer_kit/closeout_not_hand_filled.rs",
    "src/certification/public_facade_contracts/compile_fail/evidence_lookup_public_closeout/closeout_not_hand_filled.rs",
    "src/certification/public_facade_contracts/compile_fail/evidence_lookup_public_closeout/seed_not_hand_filled.rs",
    "src/certification/public_facade_contracts/compile_fail/evidence_lookup_inventory/inventory_closeout_not_hand_filled.rs",
    "src/certification/public_facade_contracts/compile_fail/evidence_lookup_index_product/struct_literals/product_not_hand_filled.rs",
];

const GENERIC_DIGEST_WRAPPER_DENIAL_FIXTURES: &[&str] = &[
    "src/certification/public_facade_contracts/compile_fail/spatial_evidence_lookup/generic_digest_bridge_denials/lookup_digest_is_not_query_descriptor_digest.rs",
    "src/certification/public_facade_contracts/compile_fail/spatial_evidence_lookup/generic_digest_bridge_denials/query_digest_is_not_lookup_product_digest.rs",
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
fn public_api_cannot_forge_lookup_products() {
    run_explicit_compile_fail_fixture_inventory(PUBLIC_API_CANNOT_FORGE_LOOKUP_PRODUCTS_FIXTURES);
}

#[test]
fn generic_digest_wrappers_cannot_bridge_query_and_lookup_products() {
    run_explicit_compile_fail_fixture_inventory(GENERIC_DIGEST_WRAPPER_DENIAL_FIXTURES);
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
        test_cases.compile_fail(*fixture);
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
