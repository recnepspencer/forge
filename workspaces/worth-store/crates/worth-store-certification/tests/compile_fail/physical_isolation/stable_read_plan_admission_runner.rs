use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn stable_read_plan_admission_misuse_does_not_compile() {
    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "stable-read-plan-admission",
        cargo_dependency_manifest(
            &[("worth-store-physical-isolation", root.join("crates/worth-store-physical-isolation").as_path(), &[])],
            &[],
        ),
        "production",
        "diagnostic-test",
        &root.join("crates/worth-store-certification/tests/compile_fail/physical_isolation/stable_read_plan_admission"),
        FIXTURES,
    ).unwrap();
    assert_eq!(evidence.fixtures.len(), FIXTURES.len());
}

const FIXTURES: &[(&str, &[&str])] = &[
    (
        "handle_cannot_be_struct_literal.rs",
        &["private", "StablePhysicalReadHandle"],
    ),
    (
        "release_receipt_cannot_be_struct_literal.rs",
        &["private", "footprint_basis"],
    ),
    (
        "epoch_checked_plan_cannot_issue_handle.rs",
        &["admit_stable_read_plan", "EpochCheckedStableReadPlan"],
    ),
    (
        "raw_epoch_vector_cannot_observe_after_hazard.rs",
        &["no method named", "observe_after_publication"],
    ),
    (
        "post_hazard_root_observation_cannot_be_minted_without_hazard.rs",
        &["PostHazardRootObservation"],
    ),
    (
        "post_protection_observation_requires_published_hazard.rs",
        &["from_authority_current_root"],
    ),
];

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
}
