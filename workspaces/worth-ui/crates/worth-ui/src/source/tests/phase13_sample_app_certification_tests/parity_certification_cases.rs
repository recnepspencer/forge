use super::sample_app_sources::sample_file_source_package;
use super::sample_certification_pipeline::{
    certify_file_source_package, certify_rust_composition, semantic_equivalence,
    WorthUiSampleAuthoringEvidence,
};
use super::sample_rust_composition::sample_rust_composition;

#[test]
fn file_and_rust_authoring_sample_paths_compare_equal_where_claimed() {
    let file_certificate = certify_file_source_package(sample_file_source_package())
        .expect("file sample should certify");
    let rust_certificate =
        certify_rust_composition(sample_rust_composition()).expect("rust sample should certify");

    let equivalence = semantic_equivalence(&file_certificate, &rust_certificate);
    assert!(
        equivalence.is_equivalent(),
        "first difference: {:?}",
        equivalence.first_difference()
    );
    assert_eq!(
        file_certificate.semantic_digest(),
        rust_certificate.semantic_digest()
    );
    assert_eq!(
        file_certificate.handles().len(),
        rust_certificate.handles().len()
    );
    assert_rust_authoring_evidence_is_carried(rust_certificate.authoring_evidence());
    assert_eq!(
        file_certificate.snapshot_digest(),
        rust_certificate.snapshot_digest()
    );
    assert_eq!(equivalence.metrics().broad_scans(), 0);
}

fn assert_rust_authoring_evidence_is_carried(evidence: WorthUiSampleAuthoringEvidence) {
    match evidence {
        WorthUiSampleAuthoringEvidence::RustComposition { metrics } => {
            assert_eq!(metrics.modules_declared(), 3);
            assert_eq!(metrics.declarations_declared(), 9);
        }
        WorthUiSampleAuthoringEvidence::FileSourcePackage { .. } => {
            panic!("rust certification should carry rust-composition evidence")
        }
    }
}
