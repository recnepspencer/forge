use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn latch_acquisition_misuse_does_not_compile() {
    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "latch-acquisition-order",
        cargo_dependency_manifest(
            &[("worth-store-physical-isolation", root.join("crates/worth-store-physical-isolation").as_path(), &[])],
            &[],
        ),
        "production",
        "diagnostic-test",
        &root.join("crates/worth-store-certification/tests/compile_fail/physical_isolation/latch_acquisition_order"),
        FIXTURES,
    ).unwrap();
    assert_eq!(evidence.fixtures.len(), FIXTURES.len());
}

const FIXTURES: &[(&str, &[&str])] = &[
    (
        "raw_root_epoch_cannot_construct_latch_key.rs",
        &["RootEpoch", "integer"],
    ),
    (
        "latch_key_cannot_be_struct_literal.rs",
        &["private", "kind"],
    ),
    (
        "raw_steps_cannot_execute_as_latch_plan.rs",
        &["LatchAcquisitionPlan", "Vec"],
    ),
    (
        "upgrade_authority_cannot_be_struct_literal.rs",
        &["private", "LatchUpgradeAuthority"],
    ),
];

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
}
