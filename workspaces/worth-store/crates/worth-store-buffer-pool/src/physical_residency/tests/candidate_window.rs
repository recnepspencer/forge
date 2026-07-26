use super::*;

#[test]
fn candidate_batch_larger_than_cache_progresses_through_one_reserved_window() {
    let identity = store(14);
    let pool =
        PhysicalResidencyPool::open(identity, limits(64, 1, 1, candidate_batch_bytes(16), 2))
            .unwrap();
    let keys = (1..=16)
        .map(|block| {
            PhysicalCandidateFrameKey::fragment(PhysicalFrameKey::new(
                identity,
                coordinate(block, 32),
            ))
        })
        .collect::<Vec<_>>();
    let allocation = candidate_allocation(&pool, WRITE_SCOPE, 16);
    let mut batch = pool.reserve_candidate_frames(&allocation, &keys).unwrap();
    for key in keys {
        let dirty = batch.reserve_next(key).unwrap().admit(vec![7; 32]).unwrap();
        dirty.discard_candidate().unwrap();
    }
    drop(batch);
    drop(allocation);
    let counters = pool.counters();
    assert!(counters.peak_resident_bytes() <= 64);
    assert_eq!(counters.dirty_frames(), 0);
    assert!(!pool.close().requires_inspection());
}
