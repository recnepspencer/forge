use super::*;

#[test]
fn exact_writeback_claims_cannot_alias_live_posture() {
    let identity = store(15);
    let pool =
        PhysicalResidencyPool::open(identity, limits(256, 2, 2, candidate_batch_bytes(1), 4))
            .unwrap();
    let allocation = candidate_allocation(&pool, WRITE_SCOPE, 1);
    let first_key = PhysicalFrameKey::new(identity, coordinate(11, 32));
    let second_key = PhysicalFrameKey::new(identity, coordinate(12, 32));
    let first_dirty = pool
        .admit_dirty(&allocation, first_key, vec![4; 32])
        .unwrap();
    let second_dirty = pool
        .admit_dirty(&allocation, second_key, vec![5; 32])
        .unwrap();
    let first_claim = pool.claim_writeback(vec![first_key]).unwrap();

    assert_eq!(
        pool.claim_writeback(vec![first_key]).unwrap_err(),
        PhysicalResidencyDenial::WriteBackFrameAlreadyClaimed
    );
    let second_claim = pool.claim_writeback(vec![second_key]).unwrap();
    assert_eq!(first_claim.frames(), [first_key]);
    assert_eq!(second_claim.frames(), [second_key]);

    drop((first_claim, second_claim));
    first_dirty.discard_candidate().unwrap();
    second_dirty.discard_candidate().unwrap();
    drop(allocation);
    assert!(!pool.close().requires_inspection());
}
