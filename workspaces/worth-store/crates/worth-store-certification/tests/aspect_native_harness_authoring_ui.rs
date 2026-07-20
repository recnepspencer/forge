use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn aspect_native_harness_public_facade_rejects_json_shortcuts() {
    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "aspect-native-harness-authoring",
        cargo_dependency_manifest(
            &[(
                "worth-store-test-support",
                root.join("crates/worth-store-test-support").as_path(),
                &["boundary-fixtures"],
            )],
            &[],
        ),
        "boundary-fixtures",
        "diagnostic-test",
        &root.join("crates/worth-store-certification/tests/ui/aspect_native_harness_authoring"),
        FIXTURES,
    )
    .unwrap();
    assert_eq!(evidence.fixtures.len(), FIXTURES.len());
}

const FIXTURES: &[(&str, &[&str])] = &[
    (
        "terminal_json_fixture_cannot_satisfy_native_fixture.rs",
        &[
            "NativeStoreAspectFixture",
            "StoreTerminalProjectionJsonFixture",
        ],
    ),
    (
        "hostile_json_fixture_cannot_satisfy_native_fixture.rs",
        &[
            "NativeStoreAspectFixture",
            "StoreHostileReadmissionJsonFixture",
        ],
    ),
    (
        "terminal_json_projection_requires_terminal_suite_witness.rs",
        &["StoreTerminalProjectionJsonFixtureBoundaryWitness"],
    ),
    (
        "hostile_json_payload_requires_hostile_suite_witness.rs",
        &["StoreHostileReadmissionJsonFixtureBoundaryWitness"],
    ),
    (
        "json_suite_boundary_cannot_be_self_declared.rs",
        &["StoreJsonFixtureSuiteBoundary"],
    ),
    (
        "ordinary_prelude_does_not_export_json_macro.rs",
        &["no `json` in the root"],
    ),
    (
        "ordinary_prelude_does_not_export_value.rs",
        &["no `Value` in the root"],
    ),
    (
        "terminal_json_boundary_witness_cannot_be_constructed.rs",
        &[
            "StoreTerminalProjectionJsonFixtureBoundaryWitness",
            "private",
        ],
    ),
    (
        "hostile_readmission_boundary_witness_cannot_be_constructed.rs",
        &[
            "StoreHostileReadmissionJsonFixtureBoundaryWitness",
            "private",
        ],
    ),
];

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("certification crate lives under the Store workspace")
}
