use sha2::{Digest, Sha256};

use super::{
    NonCurrentStagingArtifact, NonCurrentStagingExecutionDenial, NonCurrentStagingPlanRequest,
    PhysicalRecoveryStagingOwner,
};

#[test]
fn prefix_materialization_rejects_damage_beyond_the_output_frontier() {
    let world = tempfile::tempdir().expect("world");
    let source = world.path().join("source");
    let target = world.path().join("target");
    std::fs::create_dir_all(&source).expect("source directory");
    std::fs::create_dir_all(&target).expect("target directory");
    let original = b"exact-prefix-and-declared-tail";
    let prefix = b"exact-prefix";
    std::fs::write(source.join("wal.segment"), original).expect("source artifact");
    let artifact = NonCurrentStagingArtifact::admit_prefix(
        "wal.segment",
        original.len() as u64,
        Sha256::digest(original).into(),
        prefix.len() as u64,
        Sha256::digest(prefix).into(),
    )
    .expect("valid prefix artifact");
    let plan = PhysicalRecoveryStagingOwner::lower(NonCurrentStagingPlanRequest::new(
        [7; 32],
        &source,
        &target,
        vec![artifact],
        prefix.len() as u64,
        4,
    ))
    .expect("lowered staging plan");

    std::fs::write(source.join("wal.segment"), b"exact-prefix-but-damaged-tail")
        .expect("mutated source");
    let denial = PhysicalRecoveryStagingOwner::execute_lowered(plan)
        .expect_err("damage outside copied prefix must still invalidate source custody");
    assert!(matches!(denial,
        NonCurrentStagingExecutionDenial::SourceArtifactMismatch { output_name }
            if output_name == "wal.segment"));
}

#[test]
fn inline_staging_rebuilds_owned_partial_pending_residue() {
    let world = tempfile::tempdir().expect("world");
    let source = world.path().join("source");
    let target = world.path().join("target");
    std::fs::create_dir_all(&source).expect("source directory");
    std::fs::create_dir_all(&target).expect("target directory");
    let expected = b"canonical-inline-manifest".to_vec();
    let artifact = NonCurrentStagingArtifact::admit_inline("backup.manifest", expected.clone())
        .expect("valid inline artifact");
    let plan = PhysicalRecoveryStagingOwner::lower(NonCurrentStagingPlanRequest::new(
        [8; 32],
        &source,
        &target,
        vec![artifact],
        expected.len() as u64,
        8,
    ))
    .expect("lowered staging plan");
    std::fs::create_dir_all(plan.binding().staging_root()).expect("staging root");
    std::fs::write(
        plan.binding()
            .staging_root()
            .join("backup.manifest.pending"),
        b"partial",
    )
    .expect("simulated crash residue");

    let receipt = PhysicalRecoveryStagingOwner::execute_lowered(plan)
        .expect("owner resumes its exact pending artifact");
    assert_eq!(
        std::fs::read(receipt.media().root().join("backup.manifest")).expect("published artifact"),
        expected
    );
    assert!(!receipt
        .media()
        .root()
        .join("backup.manifest.pending")
        .exists());
}
