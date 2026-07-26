use super::*;

#[test]
fn exact_clean_lease_becomes_one_dirty_candidate_atomically() {
    let identity = store(91);
    let pool = PhysicalResidencyPool::open(identity, limits(128, 3, 2, 64, 3)).unwrap();
    let allocation = allocation(&pool, READ_SCOPE);
    let key = PhysicalFrameKey::new(identity, coordinate(1, 8));
    let clean = expect_fault(&pool, &allocation, key)
        .load(|target| {
            target.copy_from_slice(&[1; 8]);
            Ok::<_, ()>(())
        })
        .unwrap();

    let dirty = clean
        .begin_dirty_replacement(&allocation)
        .unwrap()
        .replace(|source, target| {
            assert_eq!(source, &[1; 8]);
            target.copy_from_slice(&[2; 8]);
            Ok::<_, ()>(())
        })
        .unwrap();
    assert_eq!(dirty.bytes(), &[2; 8]);
    assert_eq!(pool.counters().dirty_frames(), 1);
    assert_eq!(pool.counters().candidate_frames(), 1);

    let claim = pool.claim_writeback(vec![key]).unwrap();
    assert_eq!(claim.frame_bytes(0), Some([2; 8].as_slice()));
    drop(claim);
    drop(dirty);
    assert_eq!(pool.counters().dirty_frames(), 1);
}

#[test]
fn competing_pin_prevents_clean_to_dirty_transition() {
    let identity = store(92);
    let pool = PhysicalResidencyPool::open(identity, limits(128, 3, 2, 64, 3)).unwrap();
    let allocation = allocation(&pool, READ_SCOPE);
    let key = PhysicalFrameKey::new(identity, coordinate(1, 8));
    let first = expect_fault(&pool, &allocation, key)
        .load(|target| {
            target.copy_from_slice(&[1; 8]);
            Ok::<_, ()>(())
        })
        .unwrap();
    let second = expect_hit(&pool, &allocation, key);

    assert_eq!(
        first.begin_dirty_replacement(&allocation).unwrap_err(),
        PhysicalResidencyDenial::FramePinned
    );
    assert_eq!(second.as_ref(), &[1; 8]);
    assert_eq!(pool.counters().dirty_frames(), 0);
}
