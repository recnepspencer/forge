use super::*;

#[test]
fn exact_writeback_claims_cannot_alias_live_posture() {
    let identity = store(15);
    let pool =
        PhysicalResidencyPool::open(identity, limits(256, 2, 2, candidate_batch_bytes(1), 4))
            .unwrap();
    let allocation = candidate_allocation(&pool, 1);
    let first_key = PhysicalFrameKey::new(identity, coordinate(11, 32));
    let second_key = PhysicalFrameKey::new(identity, coordinate(12, 32));
    let first_dirty = pool
        .materialize_dirty_candidate(&allocation, first_key, |bytes| bytes.fill(4))
        .unwrap();
    let second_dirty = pool
        .materialize_dirty_candidate(&allocation, second_key, |bytes| bytes.fill(5))
        .unwrap();
    drop(allocation);
    let first_claim = writeback_claim(&pool, &[first_key]);

    assert_eq!(
        {
            let allocation = pool
                .begin_foreground_write_operation(nonzero_bytes(32))
                .unwrap();
            pool.claim_writeback(allocation, &[first_key]).unwrap_err()
        },
        PhysicalResidencyDenial::WriteBackFrameAlreadyClaimed
    );
    let second_claim = writeback_claim(&pool, &[second_key]);
    assert_eq!(first_claim.frames(), [first_key]);
    assert_eq!(second_claim.frames(), [second_key]);
    let counters = pool.counters();
    assert_eq!(
        counters.speculative_attempts(PhysicalSpeculativeWorkKind::WriteBehind),
        3
    );
    assert_eq!(
        counters.speculative_admissions(PhysicalSpeculativeWorkKind::WriteBehind),
        2
    );
    assert_eq!(
        counters.speculative_denials(PhysicalSpeculativeWorkKind::WriteBehind),
        1
    );
    assert_eq!(
        counters.active_speculative_frames(PhysicalSpeculativeWorkKind::WriteBehind),
        2
    );
    assert_eq!(
        counters.peak_speculative_frames(PhysicalSpeculativeWorkKind::WriteBehind),
        2
    );
    assert_eq!(counters.active_operation_bytes(), 64);

    drop((first_claim, second_claim));
    let counters = pool.counters();
    assert_eq!(
        counters.speculative_completions(PhysicalSpeculativeWorkKind::WriteBehind),
        2
    );
    assert_eq!(
        counters.active_speculative_frames(PhysicalSpeculativeWorkKind::WriteBehind),
        0
    );
    assert_eq!(
        counters.peak_speculative_frames(PhysicalSpeculativeWorkKind::WriteBehind),
        2
    );
    assert_eq!(counters.active_operation_bytes(), 0);
    first_dirty.discard_candidate().unwrap();
    second_dirty.discard_candidate().unwrap();
    assert!(!pool.close().requires_inspection());
}
