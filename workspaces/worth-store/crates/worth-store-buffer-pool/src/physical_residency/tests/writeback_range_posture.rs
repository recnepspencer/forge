use super::*;

fn offset_key(store: StableStoreIdentity, generation: u64) -> PhysicalFrameKey {
    let coordinate =
        RecordFrameCoordinate::new(RecordArtifactFile::RootManifest { generation }, 8, 8).unwrap();
    PhysicalFrameKey::new(store, coordinate)
}

#[test]
fn candidate_fragment_after_artifact_start_carries_tail_append_posture() {
    let identity = store(94);
    let pool =
        PhysicalResidencyPool::open(identity, limits(128, 3, 2, candidate_batch_bytes(1), 3))
            .unwrap();
    let allocation = candidate_allocation(&pool, 1);
    let key = offset_key(identity, 1);
    let dirty = pool
        .materialize_dirty_candidate(&allocation, key, |bytes| bytes.fill(0x41))
        .unwrap();
    drop(allocation);

    let claim = writeback_claim(&pool, &[key]);

    assert_eq!(
        claim.range_posture(0),
        Some(PhysicalWritebackRangePosture::CandidateArtifactTail)
    );
    drop(claim);
    drop(dirty);
}

#[test]
fn dirty_replacement_after_artifact_start_carries_existing_range_posture() {
    let identity = store(95);
    let pool = PhysicalResidencyPool::open(identity, limits(128, 3, 2, 64, 3)).unwrap();
    let allocation = pool
        .begin_foreground_write_operation(nonzero_bytes(8))
        .unwrap();
    let key = offset_key(identity, 2);
    let clean = expect_fault(&pool, &allocation, key)
        .load(|target| {
            target.fill(0x51);
            Ok::<_, ()>(())
        })
        .unwrap();
    let dirty = clean
        .begin_dirty_replacement(&allocation)
        .unwrap()
        .replace(|source, target| {
            assert_eq!(source, &[0x51; 8]);
            target.fill(0x52);
            Ok::<_, ()>(())
        })
        .unwrap();

    drop(allocation);
    let claim = writeback_claim(&pool, &[key]);

    assert_eq!(
        claim.range_posture(0),
        Some(PhysicalWritebackRangePosture::ExistingRange)
    );
    drop(claim);
    drop(dirty);
}
