use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn epoch_scope_and_root_kind_misuse_does_not_compile() {
    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "epoch-scope-and-root-kind",
        cargo_dependency_manifest(
            &[
                ("worth-store-physical-isolation", root.join("crates/worth-store-physical-isolation").as_path(), &[]),
                ("worth-store-physical-format", root.join("crates/worth-store-physical-format").as_path(), &[]),
            ],
            &[],
        ),
        "production",
        "diagnostic-test",
        &root.join("crates/worth-store-certification/tests/compile_fail/physical_isolation/epoch_scope_and_root_kind"),
        FIXTURES,
    ).unwrap();
    assert_eq!(evidence.fixtures.len(), FIXTURES.len());
}

const FIXTURES: &[(&str, &[&str])] = &[
    (
        "root_epoch_cannot_be_compared_to_page_epoch.rs",
        &["RootEpoch", "PageEpoch"],
    ),
    (
        "physical_epoch_vector_cannot_be_compared_directly.rs",
        &["PhysicalEpochVector", "PartialEq"],
    ),
    (
        "checkpoint_root_cannot_admit_stable_read_plan.rs",
        &["CurrentPhysicalRoot", "CheckpointPublicationRoot"],
    ),
    (
        "raw_page_id_cannot_be_generation_counted_reference.rs",
        &["GenerationCountedPhysicalReference", "PhysicalPageId"],
    ),
    (
        "root_epoch_cannot_be_publicly_constructed.rs",
        &["cannot initialize a tuple struct which contains private fields"],
    ),
    (
        "generation_reference_cannot_mint_page_epoch.rs",
        &["page_epoch", "GenerationCountedPhysicalReference"],
    ),
    (
        "checkpoint_root_requires_checkpoint_basis.rs",
        &["CheckpointPublicationRootBasis", "RootEpoch"],
    ),
    (
        "manifest_locator_requires_locator_basis.rs",
        &["ManifestLocatorRootBasis", "RootEpoch"],
    ),
    (
        "raw_publication_ordinal_cannot_mint_page_epoch.rs",
        &["page_epoch_for_publication", "CurrentPhysicalRoot"],
    ),
    (
        "raw_generation_reference_cannot_admit_publication_epoch.rs",
        &[
            "CurrentGenerationPhysicalReference",
            "GenerationCountedPhysicalReference",
        ],
    ),
];

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
}
