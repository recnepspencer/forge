use crate::source::{
    WorthUiArtifactDigestor, WorthUiArtifactEquivalenceBasis, WorthUiArtifactEquivalenceComparator,
};

use super::digest_fixture_support::{
    artifact_from_file_sources, artifact_from_rust_modules,
    equivalent_file_authored_inspector_module_source, equivalent_file_authored_main_module_source,
    equivalent_rust_authored_modules,
};

#[test]
fn diagnostic_richness_does_not_change_artifact_digest() {
    let rust_artifact = artifact_from_rust_modules(equivalent_rust_authored_modules());
    let file_artifact = artifact_from_file_sources(
        equivalent_file_authored_main_module_source(),
        equivalent_file_authored_inspector_module_source(),
    );

    let rust_digest = WorthUiArtifactDigestor::digest(
        &rust_artifact,
        WorthUiArtifactEquivalenceBasis::semantic(),
    );
    let file_digest = WorthUiArtifactDigestor::digest(
        &file_artifact,
        WorthUiArtifactEquivalenceBasis::semantic(),
    );
    let equivalence = WorthUiArtifactEquivalenceComparator::compare(
        &rust_artifact,
        &file_artifact,
        WorthUiArtifactEquivalenceBasis::semantic(),
    );

    assert_eq!(rust_digest, file_digest);
    assert!(equivalence.is_equivalent());
    assert_eq!(equivalence.metrics().broad_scans(), 0);
}
