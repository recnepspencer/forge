use super::*;

#[test]
fn candidate_batch_larger_than_cache_progresses_through_one_reserved_window() {
    let identity = store(14);
    let pool = PhysicalResidencyPool::open(identity, limits(64, 1, 1, 64, 2)).unwrap();
    let keys = (1..=16)
        .map(|block| PhysicalFrameKey::new(identity, coordinate(block, 32)))
        .collect::<Vec<_>>();
    let mut batch = pool.reserve_candidate_frames(&keys).unwrap();
    for key in keys {
        let dirty = batch.reserve_next(key).unwrap().admit(vec![7; 32]).unwrap();
        drop(dirty.publish_clean_for_pool_test().unwrap());
    }
    drop(batch);
    let counters = pool.counters();
    assert!(counters.evictions() > 0);
    assert!(counters.peak_resident_bytes() <= 64);
    assert_eq!(counters.dirty_frames(), 0);
    assert!(!pool.close().requires_inspection());
}

#[test]
fn pins_and_exact_writeback_claims_cannot_exceed_or_alias_live_posture() {
    let identity = store(15);
    let pin_pool = PhysicalResidencyPool::open(identity, limits(256, 2, 1, 64, 4)).unwrap();
    let first = pin_pool
        .load(
            PhysicalFrameKey::new(identity, coordinate(1, 32)),
            |bytes| fill(bytes, 1),
        )
        .unwrap();
    let second = pin_pool
        .load(
            PhysicalFrameKey::new(identity, coordinate(2, 32)),
            |bytes| fill(bytes, 2),
        )
        .unwrap();
    assert_eq!(
        pin_pool
            .load(
                PhysicalFrameKey::new(identity, coordinate(3, 32)),
                |bytes| { fill(bytes, 3) }
            )
            .unwrap_err(),
        PhysicalFrameLoadError::Residency(PhysicalResidencyDenial::PinnedFrameBudgetExceeded)
    );
    drop((first, second));
    assert!(!pin_pool.close().requires_inspection());

    let dirty_pool = PhysicalResidencyPool::open(identity, limits(256, 2, 2, 64, 4)).unwrap();
    let first_key = PhysicalFrameKey::new(identity, coordinate(11, 32));
    let second_key = PhysicalFrameKey::new(identity, coordinate(12, 32));
    let first_dirty = dirty_pool.admit_dirty(first_key, vec![4; 32]).unwrap();
    let second_dirty = dirty_pool.admit_dirty(second_key, vec![5; 32]).unwrap();
    let first_claim = dirty_pool.claim_writeback(vec![first_key]).unwrap();
    assert_eq!(
        dirty_pool.claim_writeback(vec![first_key]).unwrap_err(),
        PhysicalResidencyDenial::WriteBackFrameAlreadyClaimed
    );
    let second_claim = dirty_pool.claim_writeback(vec![second_key]).unwrap();
    assert_eq!(first_claim.frames(), [first_key]);
    assert_eq!(second_claim.frames(), [second_key]);
    drop((first_claim, second_claim));
    drop(first_dirty.publish_clean_for_pool_test().unwrap());
    drop(second_dirty.publish_clean_for_pool_test().unwrap());
    assert!(!dirty_pool.close().requires_inspection());
}
