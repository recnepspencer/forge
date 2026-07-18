use crate::evidence::sha256_serialized;
use crate::selection::{
    ProofExecutionUnit, RepositoryIdentity, StoreBuildProfileIdentity, StoreFeatureLane,
};

#[test]
fn profile_and_feature_lanes_change_cache_identity() {
    let production =
        ProofExecutionUnit::feature_compatibility("worth-store-blob-chunks".to_owned(), Vec::new());
    let certification = ProofExecutionUnit::feature_compatibility(
        "worth-store-blob-chunks".to_owned(),
        vec!["certification-test-authority".to_owned()],
    );
    assert_ne!(
        sha256_serialized(&production).unwrap(),
        sha256_serialized(&certification).unwrap()
    );
    assert_eq!(production.build_profile, StoreBuildProfileIdentity::CiTest);
    assert_eq!(
        production.feature_lane,
        StoreFeatureLane::ProductionEquivalent
    );
    let repository = RepositoryIdentity {
        source_revision: "revision".to_owned(),
        source_tree_digest: "tree-a".to_owned(),
        lockfile_digest: "lock".to_owned(),
        rustc_identity: "rustc-a".to_owned(),
        operating_system: "windows".to_owned(),
        architecture: "x86_64".to_owned(),
    };
    let mut changed_toolchain = repository.clone();
    changed_toolchain.rustc_identity = "rustc-b".to_owned();
    assert_ne!(
        sha256_serialized(&(repository.clone(), &production)).unwrap(),
        sha256_serialized(&(changed_toolchain, &production)).unwrap()
    );
    let mut changed_source = repository.clone();
    changed_source.source_tree_digest = "tree-b".to_owned();
    assert_ne!(
        sha256_serialized(&(repository, &production)).unwrap(),
        sha256_serialized(&(changed_source, &production)).unwrap()
    );
}
