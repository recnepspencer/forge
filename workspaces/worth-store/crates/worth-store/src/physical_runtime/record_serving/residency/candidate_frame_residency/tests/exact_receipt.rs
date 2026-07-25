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
fn canonical_residency_refuses_to_clean_without_a_physical_receipt() {
    let store = StoreNamespaceIdentityRecord::new(
        StoreNamespaceVersion::CURRENT,
        ProposedStoreIdentity::from_nonzero_bytes([19; 16]).unwrap(),
    )
    .published_identity();
    let pool = PhysicalResidencyPool::open(store, residency_limits()).unwrap();
    let publisher = BoundedCandidateFramePublisher::new(
        pool.clone(),
        Arc::new(CandidateFrameCounterCells::default()),
    );
    let mut session =
        StoreCandidateFramePublicationSession::begin(&publisher, declared_inline_frames(&[(0, 3)]))
            .unwrap();

    let failure = session
        .write_frame(
            CandidateFrame::new(
                CandidateFrameRole::InlinePage,
                segment_coordinate(0),
                vec![1, 2, 3],
            ),
            &mut |_| Ok::<_, ()>(CandidateFramePhysicalWrite::for_contract_test()),
        )
        .unwrap_err();

    assert!(matches!(
        failure,
        CandidateFrameWriteFailure::Residency(RecordAppendDenial::ResidencyUnavailable(
            PhysicalResidencyDenial::WriteBackReceiptMismatch
        ))
    ));
    assert_eq!(
        pool.counters().dirty_frames(),
        1,
        "C5_PREDICATE:dirty-clean-without-exact-receipt missing receipt cleared dirty residency"
    );
    assert!(pool.close().requires_inspection());
}

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
    let key = PhysicalFrameKey::new(media.store_identity(), coordinate);
    let dirty = pool.admit_dirty(key, vec![0x5A; 8]).unwrap();
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
    PhysicalResidencyLimits::new_with_metadata_budget(4096, 4096, 3, 2, 64, 3).unwrap()
}
