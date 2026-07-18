use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn aspect_native_authority_denies_raw_public_callers() {
    let root = store_workspace_root();
    let forge_root = root.ancestors().nth(2).unwrap();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "aspect-native-authority",
        cargo_dependency_manifest(
            &[
                (
                    "worth-foundational",
                    forge_root.join("crates/worth-foundational").as_path(),
                    &[],
                ),
                (
                    "worth-store-aspect-native",
                    root.join("crates/worth-store-aspect-native").as_path(),
                    &[],
                ),
            ],
            &[],
        ),
        "production",
        "diagnostic-test",
        &root.join("crates/worth-store-certification/tests/ui/aspect_native_authority"),
        FIXTURES,
    )
    .unwrap();
    assert_eq!(evidence.fixtures.len(), FIXTURES.len());
}

const FIXTURES: &[(&str, &[&str])] = &[
    (
        "local_diagnostic_payload_cannot_satisfy_store_evidence.rs",
        &[
            "StoreDiagnosticSupportReportEvidence",
            "LocalDiagnosticPayload",
        ],
    ),
    (
        "local_performance_claim_cannot_satisfy_store_evidence.rs",
        &["StorePerformanceReceiptEvidence", "LocalPerformanceClaim"],
    ),
    (
        "raw_aspect_value_cannot_satisfy_boundary_fact.rs",
        &["StoreAspectBoundaryFact", "AspectValue"],
    ),
    (
        "raw_string_cannot_satisfy_store_identity.rs",
        &["StoreAspectIdentity", "String"],
    ),
    (
        "raw_struct_cannot_satisfy_authority_input.rs",
        &["StoreAspectAuthorityInput", "StructAspectValue"],
    ),
    (
        "terminal_projection_text_cannot_satisfy_locator.rs",
        &["StoreAspectBoundaryLocator", "String"],
    ),
];

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("certification crate lives under the Store workspace")
}
