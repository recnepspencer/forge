use crate::source::{
    WorthUiArtifactDigestor, WorthUiArtifactEquivalenceBasis, WorthUiArtifactEquivalenceComparator,
};

use super::digest_fixture_support::{
    artifact_from_rust_modules, imported_modules, reordered_modules,
};

#[test]
fn same_artifact_meaning_produces_same_digest() {
    let left = artifact_from_rust_modules(imported_modules());
    let right = artifact_from_rust_modules(reordered_modules());

    let (left_digest, left_report) = WorthUiArtifactDigestor::digest_with_report(
        &left,
        WorthUiArtifactEquivalenceBasis::semantic(),
    );
    let (right_digest, right_report) = WorthUiArtifactDigestor::digest_with_report(
        &right,
        WorthUiArtifactEquivalenceBasis::semantic(),
    );
    let equivalence = WorthUiArtifactEquivalenceComparator::compare(
        &left,
        &right,
        WorthUiArtifactEquivalenceBasis::semantic(),
    );

    assert_eq!(left_digest, right_digest);
    assert_eq!(
        left_digest.basis(),
        WorthUiArtifactEquivalenceBasis::semantic()
    );
    assert_eq!(
        left_report.basis(),
        WorthUiArtifactEquivalenceBasis::semantic()
    );
    assert_eq!(left_digest.raw(), right_digest.raw());
    assert!(equivalence.is_equivalent());
    assert_eq!(
        equivalence.basis(),
        WorthUiArtifactEquivalenceBasis::semantic()
    );
    assert_eq!(equivalence.left_digest(), left_digest);
    assert_eq!(equivalence.right_digest(), right_digest);
    assert!(equivalence.first_difference().is_none());
    assert!(equivalence.metrics().modules_compared() > 0);
    assert!(equivalence.metrics().nodes_compared() > 0);
    assert!(equivalence.metrics().semantic_payloads_compared() > 0);
    assert_eq!(left_report.metrics().broad_scans(), 0);
    assert_eq!(right_report.metrics().broad_scans(), 0);
}
