use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn diagnostic_model_surfaces_cannot_open_production_authority_doors() {
    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "protocol-model-authority-inversion",
        cargo_dependency_manifest(
            &[
                ("worth-store-formal-models", root.join("crates/worth-store-formal-models").as_path(), &[]),
                ("worth-store-operations", root.join("crates/worth-store-operations").as_path(), &[]),
                ("worth-store-physical-isolation", root.join("crates/worth-store-physical-isolation").as_path(), &[]),
            ],
            &[],
        ),
        "production",
        "diagnostic-test",
        &root.join("crates/worth-store-certification/tests/compile_fail/protocol_models/cases/authority_inversion/src/bin"),
        FIXTURES,
    ).unwrap();
    assert_eq!(evidence.fixtures.len(), FIXTURES.len());
}

const FIXTURES: &[(&str, &[&str])] = &[(
    "model_action_as_publication_authority.rs",
    &["ImportPublicationAction", "ImportPublicationReadiness"],
)];

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
}
