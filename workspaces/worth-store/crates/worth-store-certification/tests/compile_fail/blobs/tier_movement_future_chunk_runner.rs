use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn future_chunk_placeholder_boundary_misuse_does_not_compile() {
    run(FIXTURES);
}

const FIXTURES: &[(&str, &[&str])] = &[
    (
        "future_chunk_variant_cannot_be_constructed_directly.rs",
        &["cannot create non-exhaustive variant using struct expression"],
    ),
    (
        "future_chunk_constructor_is_not_public.rs",
        &["future_chunk", "private"],
    ),
];

fn run(fixtures: &[(&str, &[&str])]) {
    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "tier-movement-future-chunk",
        cargo_dependency_manifest(
            &[
                (
                    "worth-store-physical-isolation",
                    root.join("crates/worth-store-physical-isolation").as_path(),
                    &[],
                ),
                (
                    "worth-store-physical-format",
                    root.join("crates/worth-store-physical-format").as_path(),
                    &[],
                ),
            ],
            &[],
        ),
        "production",
        "diagnostic-test",
        &root.join(
            "crates/worth-store-certification/tests/compile_fail/blobs/tier_movement_future_chunk",
        ),
        fixtures,
    )
    .unwrap();
    assert_eq!(evidence.fixtures.len(), fixtures.len());
}

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
}
