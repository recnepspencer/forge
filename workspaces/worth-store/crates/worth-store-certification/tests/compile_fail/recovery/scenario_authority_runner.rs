use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn scenario_authority_rejects_lower_authority_callers_at_compile_time() {
    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "physical-scenario-authority",
        dependency_manifest(root),
        "production",
        "diagnostic-test",
        &root.join(
            "crates/worth-store-certification/tests/compile_fail/recovery/scenario_authority",
        ),
        FIXTURES,
    )
    .unwrap();
    assert_eq!(evidence.fixtures.len(), FIXTURES.len());
}

const FIXTURES: &[(&str, &[&str])] = &[
    ("json_value_cannot_define_scenario.rs", &["String", "Value"]),
    (
        "terminal_projection_cannot_be_fixture.rs",
        &["StoreAspectBoundaryFact", "StoreTerminalProjectionText"],
    ),
    (
        "scenario_identity_cannot_be_minted.rs",
        &["PhysicalScenarioCanonicalIdentity", "private"],
    ),
    (
        "authority_witness_cannot_be_minted.rs",
        &["PhysicalScenarioAuthorityWitness", "private"],
    ),
    (
        "certified_scenario_struct_literal_cannot_be_minted.rs",
        &["CertifiedPhysicalScenario", "private"],
    ),
    (
        "raw_string_cannot_certify_scenario.rs",
        &["CertifiedPhysicalScenario", "&str"],
    ),
    (
        "fixture_label_cannot_be_fixture.rs",
        &["StoreAspectBoundaryFact", "&str"],
    ),
    (
        "copied_digest_cannot_be_identity.rs",
        &["PhysicalScenarioCanonicalIdentity", "&str"],
    ),
];

fn dependency_manifest(root: &Path) -> String {
    cargo_dependency_manifest(
        &[
            (
                "worth-store-aspect-native",
                root.join("crates/worth-store-aspect-native").as_path(),
                &[],
            ),
            (
                "worth-store-physical-certification",
                root.join("crates/worth-store-physical-certification")
                    .as_path(),
                &[],
            ),
        ],
        &[("serde_json", "1")],
    )
}

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("certification crate lives under the Store workspace")
}
