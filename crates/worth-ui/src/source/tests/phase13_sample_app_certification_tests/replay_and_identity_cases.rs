use super::sample_app_sources::{reordered_sample_file_source_package, sample_file_source_package};
use super::sample_certification_pipeline::{
    certify_file_source_package, certify_rust_composition, semantic_equivalence,
    WorthUiSampleCertificationFailure,
};
use super::sample_rust_composition::{
    reordered_sample_rust_composition, rust_composition_with_duplicate_identity,
};
use crate::source::WorthUiIdentitySeedingDiagnosticCode;

#[test]
fn replay_and_reordering_preserve_canonical_artifact_parity() {
    let first = certify_file_source_package(sample_file_source_package())
        .expect("first sample replay should certify");
    let replay = certify_file_source_package(sample_file_source_package())
        .expect("second sample replay should certify");
    let reordered = certify_file_source_package(reordered_sample_file_source_package())
        .expect("reordered file sample should certify");
    let rust_reordered = certify_rust_composition(reordered_sample_rust_composition())
        .expect("reordered rust sample should certify");

    assert_eq!(first.semantic_digest(), replay.semantic_digest());
    assert!(semantic_equivalence(&first, &replay).is_equivalent());
    assert_eq!(first.semantic_digest(), reordered.semantic_digest());
    assert!(semantic_equivalence(&first, &reordered).is_equivalent());
    assert_eq!(first.semantic_digest(), rust_reordered.semantic_digest());
    assert!(semantic_equivalence(&first, &rust_reordered).is_equivalent());
}

#[test]
fn identity_basis_changes_are_not_silently_certified() {
    let failure = certify_rust_composition(rust_composition_with_duplicate_identity())
        .expect_err("duplicate authored identity must fail certification");

    assert_eq!(
        failure,
        WorthUiSampleCertificationFailure::IdentitySeeding(vec![
            WorthUiIdentitySeedingDiagnosticCode::DuplicateAuthoredIdentitySeed
        ])
    );
}
