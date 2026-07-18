use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn fixture_authority_rejects_hand_filled_and_skipped_progression_at_compile_time() {
    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "production-fixture-authority",
        dependency_manifest(root),
        "production",
        "diagnostic-test",
        &root
            .join("crates/worth-store-certification/tests/compile_fail/recovery/fixture_authority"),
        FIXTURES,
    )
    .unwrap();
    assert_eq!(evidence.fixtures.len(), FIXTURES.len());
}

const FIXTURES: &[(&str, &[&str])] = &[
    (
        "fixture_receipt_cannot_be_struct_literal.rs",
        &["FixtureAuthorityReceipt", "private"],
    ),
    (
        "production_fixture_cannot_be_struct_literal.rs",
        &["ProductionBackedPhysicalFixture", "private"],
    ),
    (
        "raw_persisted_layout_cannot_satisfy_fixture_materialization.rs",
        &[
            "ProductionBackedFixtureMaterialization",
            "PersistedPhysicalLayout",
        ],
    ),
    (
        "fixture_label_cannot_satisfy_fixture_materialization.rs",
        &["ProductionBackedFixtureMaterialization", "&str"],
    ),
    (
        "synthetic_in_memory_store_cannot_satisfy_fixture_materialization.rs",
        &[
            "ProductionBackedFixtureMaterialization",
            "SyntheticInMemoryStore",
        ],
    ),
    (
        "fixture_builder_cannot_reopen_without_boundary.rs",
        &["and_reopen_through_physical_authority"],
    ),
    (
        "raw_mutation_boundary_cannot_satisfy_capability_declaration.rs",
        &["FixtureNeedsBoundary", "mutation_boundary"],
    ),
    ("fixture_receipt_cannot_be_cloned.rs", &["Clone"]),
    ("production_fixture_cannot_be_cloned.rs", &["clone"]),
];

fn dependency_manifest(root: &Path) -> String {
    cargo_dependency_manifest(
        &[
            (
                "worth-store-physical-certification",
                root.join("crates/worth-store-physical-certification")
                    .as_path(),
                &[],
            ),
            (
                "worth-store-physical-format",
                root.join("crates/worth-store-physical-format").as_path(),
                &[],
            ),
        ],
        &[],
    )
}

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("certification crate lives under the Store workspace")
}
