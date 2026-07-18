use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn authority_projection_readmission_denies_lower_authority_inputs() {
    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "authority-projection-readmission",
        cargo_dependency_manifest(
            &[
                (
                    "worth-store-aspect-native",
                    root.join("crates/worth-store-aspect-native").as_path(),
                    &[],
                ),
                (
                    "worth-store-authority",
                    root.join("crates/worth-store-authority").as_path(),
                    &[],
                ),
                (
                    "worth-store-contracts",
                    root.join("crates/worth-store-contracts").as_path(),
                    &[],
                ),
            ],
            &[],
        ),
        "production",
        "diagnostic-test",
        &root.join("crates/worth-store-certification/tests/ui/authority_projection_readmission"),
        FIXTURES,
    )
    .unwrap();
    assert_eq!(evidence.fixtures.len(), FIXTURES.len());
}

const FIXTURES: &[(&str, &[&str])] = &[
    (
        "digest_string_cannot_satisfy_current_authority.rs",
        &["StoreCurrentAuthorityWitness", "String"],
    ),
    (
        "derived_evidence_cannot_satisfy_current_authority.rs",
        &[
            "StoreCurrentAuthorityWitness",
            "StoreDerivedAuthorityEvidence",
        ],
    ),
    (
        "external_token_cannot_satisfy_current_authority.rs",
        &[
            "StoreCurrentAuthorityWitness",
            "StoreExternalAuthorityToken",
        ],
    ),
    (
        "filename_cannot_satisfy_current_authority.rs",
        &["StoreCurrentAuthorityWitness", "StoreAuthorityFilename"],
    ),
    (
        "retained_evidence_cannot_satisfy_current_physical_authority.rs",
        &[
            "StoreCurrentPhysicalAuthorityWitness",
            "StoreRetainedAuthorityEvidence",
        ],
    ),
    (
        "stable_id_cannot_construct_canonical_authority_record.rs",
        &["CanonicalAuthorityRecord", "new"],
    ),
    (
        "stable_id_cannot_satisfy_canonical_authority_witness.rs",
        &["StoreCurrentAuthorityWitness", "StableArtifactId"],
    ),
    (
        "terminal_projection_text_cannot_satisfy_current_authority.rs",
        &[
            "StoreCurrentAuthorityWitness",
            "StoreTerminalProjectionText",
        ],
    ),
];

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("certification crate lives under the Store workspace")
}
