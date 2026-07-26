use super::*;
use worth_proof::TransitionOutcome;
use worth_store_buffer_pool::{
    PhysicalFrameKey, PhysicalResidencyDenial, PhysicalResidencyLimits, PhysicalResidencyPool,
};
use worth_store_physical_backend::{
    ArtifactNewWriteOutcome, ArtifactTreeDirectory, FilesystemAccessPosture,
};
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use crate::physical_runtime::{
    FilesystemMediaAdmission, MediaOwnedPhysicalRuntime, MediaShutdownOutcome,
    PhysicalRuntimeAdmission, PhysicalStore,
};

#[test]
fn foreign_real_receipt_cannot_clean_dirty_writeback() {
    let parent = tempfile::tempdir().unwrap();
    let owned = admit_store_media(&parent.path().join("store"));
    let media = owned.record_serving_media();
    let (artifact, physical) = root_manifest_artifact(media);
    let coordinate = RecordFrameCoordinate::new(artifact, 0, 8).unwrap();
    let receipt = match media
        .artifact_tree()
        .write_new_exact(&physical, coordinate, &[0xA5; 8])
    {
        ArtifactNewWriteOutcome::Completed(completed) => completed.into_write(),
        outcome => panic!("real receipt write failed: {outcome:?}"),
    };
    let pool = PhysicalResidencyPool::open(media.store_identity(), residency_limits()).unwrap();
    let allocation = publication_allocation(&pool);
    let key = PhysicalFrameKey::new(media.store_identity(), coordinate);
    let dirty = pool.admit_dirty(&allocation, key, vec![0x5A; 8]).unwrap();
    let claim = pool.claim_writeback(vec![key]).unwrap();

    let publication = claim.publish_clean(&receipt);
    if publication.is_ok() {
        panic!("C5_PREDICATE:dirty-clean-without-exact-receipt");
    }
    assert_eq!(
        publication,
        Err(PhysicalResidencyDenial::WriteBackReceiptMismatch)
    );
    assert_eq!(pool.counters().dirty_frames(), 1);
    drop(dirty);
    assert!(pool.claim_writeback(vec![key]).is_ok());
    assert!(pool.close().requires_inspection());
    assert!(matches!(owned.close(), MediaShutdownOutcome::Released(_)));
}

fn admit_store_media(root: &std::path::Path) -> MediaOwnedPhysicalRuntime {
    let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(root).unwrap()).unwrap();
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    match runtime.try_admit_filesystem_media(admission).into_raw() {
        TransitionOutcome::Success(media) => media,
        _ => panic!("Store-owned backend admission failed"),
    }
}

fn root_manifest_artifact(
    media: &worth_store_physical_backend::QualifiedFilesystemMedia,
) -> (
    RecordArtifactFile,
    worth_store_physical_backend::ArtifactTreeFile,
) {
    let tree = media.artifact_tree();
    let records = ArtifactTreeDirectory::families().child("records").unwrap();
    let roots = records.child("roots").unwrap();
    if !tree.directory_exists(&records).unwrap() {
        tree.create_directory(&records).unwrap();
    }
    if !tree.directory_exists(&roots).unwrap() {
        tree.create_directory(&roots).unwrap();
    }
    let logical = RecordArtifactFile::RootManifest { generation: 1 };
    let physical = roots.file(&logical.file_name()).unwrap();
    (logical, physical)
}

fn residency_limits() -> PhysicalResidencyLimits {
    use worth_store_buffer_pool::{
        PhysicalOperationAllocationScope as Scope, PhysicalSpeculativeWorkKind as Speculation,
    };

    let operation =
        PhysicalResidencyPool::candidate_batch_operation_bytes(std::num::NonZeroUsize::MIN)
            .unwrap();
    PhysicalResidencyLimits::builder()
        .total_bytes(nonzero_bytes(12_288 + operation.get()))
        .resident_bytes(nonzero_bytes(4096))
        .metadata_bytes(nonzero_bytes(4096))
        .frame_entries(nonzero_count(3))
        .pinned_frames(nonzero_count(3))
        .pin_leases(nonzero_count(3))
        .dirty_frames(nonzero_count(2))
        .dirty_replacement_bytes(nonzero_bytes(4096))
        .operation_bytes(operation)
        .scope_bytes(Scope::ForegroundRead, operation)
        .scope_bytes(Scope::ForegroundWrite, operation)
        .scope_bytes(Scope::Recovery, operation)
        .scope_bytes(Scope::Scrub, operation)
        .scope_bytes(Scope::Maintenance, operation)
        .scope_bytes(Scope::Verification, operation)
        .scope_bytes(Scope::Blob, operation)
        .speculative_frames(Speculation::Prefetch, nonzero_count(3))
        .speculative_frames(Speculation::ReadAhead, nonzero_count(3))
        .speculative_frames(Speculation::WriteBehind, nonzero_count(2))
        .admit(std::num::NonZeroU64::MIN)
        .unwrap()
}

fn nonzero_bytes(value: u64) -> std::num::NonZeroU64 {
    std::num::NonZeroU64::new(value).unwrap()
}

fn nonzero_count(value: u32) -> std::num::NonZeroU32 {
    std::num::NonZeroU32::new(value).unwrap()
}
