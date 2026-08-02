use worth_proof::TransitionOutcome;
use worth_store_buffer_pool::{
    PhysicalFrameKey, PhysicalOperationAllocationScope, PhysicalResidencyLimits,
    PhysicalResidencyPool, PhysicalResidencyPoolOwner, PhysicalSpeculativeWorkKind,
};
use worth_store_physical_backend::{
    ArtifactNewWriteOutcome, ArtifactNewWriteRange, ArtifactRangeWriteOutcome,
    ArtifactTreeDirectory, FilesystemAccessPosture,
};
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use crate::physical_runtime::{
    FilesystemMediaAdmission, MediaShutdownOutcome, PhysicalOperationIdentity,
    PhysicalResidencyWritebackCompletion, PhysicalRuntimeAdmission, PhysicalStore,
    PhysicalWorkGeneration, PhysicalWorkIdentity,
};

#[test]
fn writeback_settlement_rejects_wrong_bytes_and_accepts_exact_physical_receipt() {
    let root = tempfile::tempdir().unwrap();
    let runtime =
        PhysicalStore::admit(PhysicalRuntimeAdmission::new(root.path()).unwrap()).unwrap();
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let owned = match runtime.try_admit_filesystem_media(admission).into_raw() {
        TransitionOutcome::Success(media) => media,
        _ => panic!("Store-owned backend admission failed"),
    };
    let media = owned.record_serving_media();
    let artifact = RecordArtifactFile::RootManifest { generation: 1 };
    let physical = root_manifest_artifact(media, artifact);
    let coordinate = RecordFrameCoordinate::new(artifact, 0, 8).unwrap();
    match media.artifact_tree().write_new_exact(
        &physical,
        ArtifactNewWriteRange::new(8).unwrap(),
        &[0xA5; 8],
    ) {
        ArtifactNewWriteOutcome::Completed(_) => {}
        outcome => panic!("real receipt write failed: {outcome:?}"),
    };
    let wrong = match media
        .artifact_tree()
        .write_exact_at(&physical, coordinate, &[0xA5; 8])
    {
        ArtifactRangeWriteOutcome::Completed(completed) => completed,
        outcome => panic!("real range receipt write failed: {outcome:?}"),
    };

    let (pool, _, writeback_clean) =
        PhysicalResidencyPoolOwner::open(media.store_identity(), residency_limits())
            .unwrap()
            .into_parts();
    let allocation = pool
        .begin_foreground_write_operation(
            PhysicalResidencyPool::candidate_batch_operation_bytes(std::num::NonZeroUsize::MIN)
                .unwrap(),
        )
        .unwrap();
    let key = PhysicalFrameKey::new(media.store_identity(), coordinate);
    let dirty = pool
        .materialize_dirty_candidate(&allocation, key, |bytes| bytes.fill(0x5A))
        .unwrap();
    let observation = owned.observer().snapshot().unwrap();
    let identity = PhysicalWorkIdentity::from_instance_owner(
        observation.store_identity(),
        observation.runtime_identity(),
        PhysicalWorkGeneration::from_lifecycle(observation.generation()),
        PhysicalOperationIdentity::from_owner_sequence(std::num::NonZeroU64::MIN),
    );
    let wrong_completion =
        PhysicalResidencyWritebackCompletion::new(identity, writeback_claim(&pool, key), wrong);

    assert!(
        matches!(
            wrong_completion.publish_clean(&writeback_clean),
            Err(worth_store_buffer_pool::PhysicalResidencyDenial::WriteBackReceiptMismatch)
        ),
        "C5_PREDICATE:writeback-clean-without-exact-receipt"
    );
    assert_eq!(pool.counters().dirty_frames(), 1);
    let exact = match media
        .artifact_tree()
        .write_exact_at(&physical, coordinate, &[0x5A; 8])
    {
        ArtifactRangeWriteOutcome::Completed(completed) => completed,
        outcome => panic!("exact receipt write failed: {outcome:?}"),
    };
    let exact_completion =
        PhysicalResidencyWritebackCompletion::new(identity, writeback_claim(&pool, key), exact);
    assert!(
        exact_completion.publish_clean(&writeback_clean).is_ok(),
        "an exact Store completion must consume one claimed dirty frame"
    );
    assert_eq!(pool.counters().dirty_frames(), 0);
    assert_eq!(pool.counters().active_writeback_claims(), 0);

    drop(dirty);
    drop(allocation);
    assert!(!pool.close().requires_inspection());
    assert!(matches!(owned.close(), MediaShutdownOutcome::Released(_)));
}

fn root_manifest_artifact(
    media: &worth_store_physical_backend::QualifiedFilesystemMedia,
    artifact: RecordArtifactFile,
) -> worth_store_physical_backend::ArtifactTreeFile {
    let tree = media.artifact_tree();
    let records = ArtifactTreeDirectory::families().child("records").unwrap();
    let roots = records.child("roots").unwrap();
    if !tree.directory_exists(&records).unwrap() {
        tree.create_directory(&records).unwrap();
    }
    if !tree.directory_exists(&roots).unwrap() {
        tree.create_directory(&roots).unwrap();
    }
    roots.file(&artifact.file_name()).unwrap()
}

fn writeback_claim(
    pool: &PhysicalResidencyPool,
    key: PhysicalFrameKey,
) -> worth_store_buffer_pool::PhysicalWritebackClaim {
    let allocation = pool
        .begin_foreground_write_operation(
            std::num::NonZeroU64::new(u64::from(key.coordinate().length())).unwrap(),
        )
        .unwrap();
    pool.claim_writeback(allocation, &[key]).unwrap()
}

fn residency_limits() -> PhysicalResidencyLimits {
    let bytes = std::num::NonZeroU64::new(4096).unwrap();
    let candidate_operation =
        PhysicalResidencyPool::candidate_batch_operation_bytes(std::num::NonZeroUsize::MIN)
            .unwrap();
    let operation = std::num::NonZeroU64::new(candidate_operation.get() + 8).unwrap();
    let frames = std::num::NonZeroU32::new(3).unwrap();
    let mut limits = PhysicalResidencyLimits::builder()
        .total_bytes(std::num::NonZeroU64::new(12_288 + operation.get()).unwrap())
        .resident_bytes(bytes)
        .metadata_bytes(bytes)
        .frame_entries(frames)
        .pinned_frames(frames)
        .pin_leases(frames)
        .dirty_frames(frames)
        .dirty_replacement_bytes(bytes)
        .operation_bytes(operation);
    for scope in [
        PhysicalOperationAllocationScope::ForegroundRead,
        PhysicalOperationAllocationScope::ForegroundWrite,
        PhysicalOperationAllocationScope::Recovery,
        PhysicalOperationAllocationScope::Scrub,
        PhysicalOperationAllocationScope::Maintenance,
        PhysicalOperationAllocationScope::Verification,
        PhysicalOperationAllocationScope::Blob,
    ] {
        limits = limits.scope_bytes(scope, operation);
    }
    for kind in [
        PhysicalSpeculativeWorkKind::Prefetch,
        PhysicalSpeculativeWorkKind::ReadAhead,
        PhysicalSpeculativeWorkKind::WriteBehind,
    ] {
        limits = limits.speculative_frames(kind, frames);
    }
    limits.admit(std::num::NonZeroU64::MIN).unwrap()
}
