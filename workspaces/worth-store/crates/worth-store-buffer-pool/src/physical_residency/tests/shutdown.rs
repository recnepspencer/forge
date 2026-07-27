use super::*;

#[test]
fn snapshot_is_terminal_even_when_a_lease_was_abandoned() {
    let identity = store(13);
    let pool = PhysicalResidencyPool::open(identity, limits(128, 1, 1, 64, 2)).unwrap();
    let allocation = allocation(&pool, READ_SCOPE);
    let lease = expect_fault(
        &pool,
        &allocation,
        PhysicalFrameKey::new(identity, coordinate(1, 16)),
    )
    .load(|bytes| fill(bytes, 8))
    .unwrap();
    drop(allocation);
    let shutdown = pool.close();
    assert!(shutdown.requires_inspection());
    let terminal = shutdown.counters();
    let mut copied = [0_u8; 4];
    lease.copy_range_into(0..4, &mut copied);
    assert_eq!(copied, [8; 4]);
    assert_eq!(
        pool.counters().copy_operations(),
        terminal.copy_operations() + 1
    );
    assert_eq!(pool.counters().copied_bytes(), terminal.copied_bytes() + 4);
    assert_eq!(shutdown.counters(), terminal);
    assert_eq!(
        pool.begin_operation(READ_SCOPE, nonzero_bytes(1))
            .unwrap_err(),
        PhysicalResidencyDenial::PoolClosed
    );
    drop(lease);
    assert_eq!(terminal.pin_leases(), 1);
    assert_eq!(terminal.frame_entries(), 1);
    assert_eq!(pool.counters().pin_leases(), 0);
    assert_eq!(pool.counters().frame_entries(), 0);
    assert_eq!(pool.counters().resident_bytes(), 0);
    assert_eq!(pool.drain_unpinned_clean_frames(), 0);
}

#[test]
fn live_operation_allocation_is_classified_as_shutdown_residue() {
    let identity = store(14);
    let pool = PhysicalResidencyPool::open(identity, limits(128, 1, 1, 64, 2)).unwrap();
    let allocation = allocation(&pool, READ_SCOPE);

    let shutdown = pool.close();
    assert!(shutdown.requires_inspection());
    assert!(shutdown.has_cancellable_work_residue());
    assert_eq!(shutdown.counters().active_operation_bytes(), 1);

    drop(allocation);
    assert_eq!(pool.counters().active_operation_bytes(), 0);
    assert_eq!(shutdown.counters().active_operation_bytes(), 1);
}

#[test]
fn empty_live_candidate_batch_is_not_a_clean_shutdown() {
    let identity = store(15);
    let candidate_bytes = candidate_batch_bytes(1);
    let pool =
        PhysicalResidencyPool::open(identity, limits(128, 1, 1, candidate_bytes, 2)).unwrap();
    let allocation = candidate_allocation(&pool, 1);
    let candidate =
        PhysicalCandidateFrameKey::fragment(PhysicalFrameKey::new(identity, coordinate(1, 16)));
    let batch = pool
        .reserve_candidate_frames(&allocation, &[candidate])
        .unwrap();

    let shutdown = pool.close();
    assert!(shutdown.requires_inspection());
    assert!(shutdown.has_cancellable_work_residue());
    assert_eq!(shutdown.counters().candidate_frames(), 0);
    assert_eq!(
        shutdown.counters().active_operation_bytes(),
        candidate_bytes
    );

    drop(batch);
    drop(allocation);
    assert_eq!(pool.counters().active_operation_bytes(), 0);
}

#[test]
fn speculative_grant_drop_reconciles_after_close() {
    let identity = store(16);
    let pool = PhysicalResidencyPool::open(identity, limits(128, 1, 1, 64, 2)).unwrap();
    let coordinate = coordinate(1, 16);
    let allocation = pool
        .begin_foreground_read_operation(nonzero_bytes(16))
        .unwrap();
    let speculative = pool.admit_prefetch(allocation, coordinate).unwrap();

    let shutdown = pool.close();
    assert!(shutdown.requires_inspection());
    assert!(shutdown.has_cancellable_work_residue());
    assert_eq!(
        shutdown
            .counters()
            .active_speculative_frames(PhysicalSpeculativeWorkKind::Prefetch),
        1
    );

    drop(speculative);
    assert_eq!(
        pool.counters()
            .active_speculative_frames(PhysicalSpeculativeWorkKind::Prefetch),
        0
    );
}

#[test]
fn read_ahead_grant_drop_reconciles_after_close() {
    let identity = store(112);
    let pool = PhysicalResidencyPool::open(identity, limits(128, 2, 1, 64, 2)).unwrap();
    let coordinates = [coordinate(1, 16), coordinate(2, 16)];
    let allocation = pool
        .begin_foreground_read_operation(nonzero_bytes(32))
        .unwrap();
    let speculative = pool.admit_read_ahead(allocation, &coordinates).unwrap();

    let shutdown = pool.close();
    assert!(shutdown.requires_inspection());
    assert!(shutdown.has_cancellable_work_residue());
    assert_eq!(
        shutdown
            .counters()
            .active_speculative_frames(PhysicalSpeculativeWorkKind::ReadAhead),
        2
    );

    drop(speculative);
    assert_eq!(
        pool.counters()
            .active_speculative_frames(PhysicalSpeculativeWorkKind::ReadAhead),
        0
    );
    assert_eq!(
        pool.counters()
            .speculative_completions(PhysicalSpeculativeWorkKind::ReadAhead),
        1
    );
}

#[test]
fn writeback_claim_drop_after_close_releases_claim_but_retains_dirty_truth() {
    let identity = store(17);
    let candidate_bytes = candidate_batch_bytes(1);
    let pool =
        PhysicalResidencyPool::open(identity, limits(128, 1, 1, candidate_bytes, 2)).unwrap();
    let allocation = candidate_allocation(&pool, 1);
    let key = PhysicalFrameKey::new(identity, coordinate(1, 16));
    let dirty = pool
        .materialize_dirty_candidate(&allocation, key, |bytes| bytes.fill(7))
        .unwrap();
    drop(allocation);
    let claim = writeback_claim(&pool, &[key]);

    let shutdown = pool.close();
    assert!(shutdown.requires_inspection());
    assert_eq!(shutdown.counters().active_writeback_claims(), 1);
    assert_eq!(
        shutdown
            .counters()
            .active_speculative_frames(PhysicalSpeculativeWorkKind::WriteBehind),
        1
    );

    drop(claim);
    assert_eq!(pool.counters().active_writeback_claims(), 0);
    assert_eq!(
        pool.counters()
            .active_speculative_frames(PhysicalSpeculativeWorkKind::WriteBehind),
        0
    );
    assert_eq!(pool.counters().dirty_frames(), 1);

    drop(dirty);
    assert_eq!(pool.counters().pin_leases(), 0);
    assert_eq!(pool.counters().dirty_frames(), 1);
    assert_eq!(pool.counters().active_operation_bytes(), 0);
}

#[test]
fn exact_waiter_can_be_the_final_post_close_pin_without_leaking_the_frame() {
    let identity = store(18);
    let pool = PhysicalResidencyPool::open(identity, limits(128, 2, 1, 64, 2)).unwrap();
    let allocation = allocation(&pool, READ_SCOPE);
    let key = PhysicalFrameKey::new(identity, coordinate(1, 16));
    let owner = expect_fault(&pool, &allocation, key);
    let waiter = match pool.access_frame(&allocation, key).unwrap() {
        PhysicalFrameAccess::Coalesced(waiter) => waiter,
        _ => panic!("second exact access must join the active fault"),
    };
    let lease = owner
        .load(|target| {
            target.fill(3);
            Ok::<_, ()>(())
        })
        .unwrap();
    drop(allocation);

    let shutdown = pool.close();
    assert!(shutdown.requires_inspection());
    assert_eq!(shutdown.counters().pin_leases(), 2);
    drop(lease);
    assert_eq!(pool.counters().pin_leases(), 1);
    drop(waiter);
    assert_eq!(pool.counters().pin_leases(), 0);
    assert_eq!(pool.counters().frame_entries(), 0);
    assert_eq!(pool.counters().resident_bytes(), 0);
}

#[test]
fn bounded_waiter_can_be_the_final_post_close_pin_without_leaking_the_frame() {
    let identity = store(19);
    let pool = PhysicalResidencyPool::open(identity, limits(128, 2, 1, 64, 2)).unwrap();
    let allocation = allocation(&pool, READ_SCOPE);
    let key = PhysicalBoundedFrameKey::new(
        identity,
        RecordArtifactFile::RootRoutingBlock {
            generation: 1,
            block: 9,
        },
        NonZeroU32::new(32).unwrap(),
    );
    let owner = match pool.access_bounded_frame(&allocation, key).unwrap() {
        PhysicalBoundedFrameAccess::Fault(owner) => owner,
        _ => panic!("first bounded access must own the fault"),
    };
    let waiter = match pool.access_bounded_frame(&allocation, key).unwrap() {
        PhysicalBoundedFrameAccess::Coalesced(waiter) => waiter,
        _ => panic!("second bounded access must join the active fault"),
    };
    let lease = owner
        .load(
            |_| Ok::<_, ()>(16),
            |target| {
                target.fill(4);
                Ok::<_, ()>(())
            },
        )
        .unwrap();
    drop(allocation);

    let shutdown = pool.close();
    assert!(shutdown.requires_inspection());
    assert_eq!(shutdown.counters().pin_leases(), 2);
    drop(lease);
    assert_eq!(pool.counters().pin_leases(), 1);
    drop(waiter);
    assert_eq!(pool.counters().pin_leases(), 0);
    assert_eq!(pool.counters().frame_entries(), 0);
    assert_eq!(pool.counters().resident_bytes(), 0);
}
