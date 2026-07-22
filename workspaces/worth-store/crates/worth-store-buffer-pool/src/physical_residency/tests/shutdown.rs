use super::*;

#[test]
fn snapshot_is_terminal_even_when_a_lease_was_abandoned() {
    let identity = store(13);
    let pool = PhysicalResidencyPool::open(identity, limits(128, 1, 1, 64, 2)).unwrap();
    let lease = pool
        .load(
            PhysicalFrameKey::new(identity, coordinate(1, 16)),
            |bytes| fill(bytes, 8),
        )
        .unwrap();
    let shutdown = pool.close();
    assert!(shutdown.requires_inspection());
    let terminal = shutdown.counters();
    let mut copied = [0_u8; 4];
    lease.copy_range_into(0..4, &mut copied);
    assert_eq!(copied, [8; 4]);
    assert_eq!(
        pool.begin_operation(OperationAllocationScope::ForegroundRead, 1)
            .unwrap_err(),
        PhysicalResidencyDenial::PoolClosed
    );
    drop(lease);
    assert_eq!(pool.counters(), terminal);
    assert_eq!(pool.drain_unpinned_clean_frames(), 0);
}
