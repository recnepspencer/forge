use super::*;

#[test]
fn writebehind_reaches_its_exact_limit_and_denies_one_past_without_losing_dirty_truth() {
    let identity = store(110);
    let candidate_bytes = candidate_batch_bytes(2);
    let pool =
        PhysicalResidencyPool::open(identity, speculation_limits(candidate_bytes, 2, [2, 2, 1]))
            .unwrap();
    let allocation = candidate_allocation(&pool, 2);
    let first_key = PhysicalFrameKey::new(identity, coordinate(1, 32));
    let second_key = PhysicalFrameKey::new(identity, coordinate(2, 32));
    let first_dirty = pool
        .materialize_dirty_candidate(&allocation, first_key, |bytes| bytes.fill(1))
        .unwrap();
    let second_dirty = pool
        .materialize_dirty_candidate(&allocation, second_key, |bytes| bytes.fill(2))
        .unwrap();
    drop(allocation);

    let admitted = writeback_claim(&pool, &[first_key]);
    let denied_allocation = pool
        .begin_foreground_write_operation(nonzero_bytes(32))
        .unwrap();
    assert_eq!(
        pool.claim_writeback(denied_allocation, &[second_key])
            .unwrap_err(),
        PhysicalResidencyDenial::Pressure(PhysicalResidencyPressureDenial::new(
            identity,
            pool.incarnation(),
            PhysicalResidencyPressureDemand {
                dimension: PhysicalResidencyDimension::SpeculativeFrames(
                    PhysicalSpeculativeWorkKind::WriteBehind,
                ),
                scope: WRITE_SCOPE,
                requested: 1,
                current: 1,
                limit: 1,
            },
        ))
    );

    let live = pool.counters();
    assert_eq!(live.dirty_frames(), 2);
    assert_eq!(
        live.speculative_attempts(PhysicalSpeculativeWorkKind::WriteBehind),
        2
    );
    assert_eq!(
        live.speculative_admissions(PhysicalSpeculativeWorkKind::WriteBehind),
        1
    );
    assert_eq!(
        live.speculative_denials(PhysicalSpeculativeWorkKind::WriteBehind),
        1
    );
    assert_eq!(
        live.active_speculative_frames(PhysicalSpeculativeWorkKind::WriteBehind),
        1
    );
    assert_eq!(
        live.peak_speculative_frames(PhysicalSpeculativeWorkKind::WriteBehind),
        1
    );

    drop(admitted);
    let released = pool.counters();
    assert_eq!(released.dirty_frames(), 2);
    assert_eq!(
        released.speculative_completions(PhysicalSpeculativeWorkKind::WriteBehind),
        1
    );
    assert_eq!(
        released.active_speculative_frames(PhysicalSpeculativeWorkKind::WriteBehind),
        0
    );
    first_dirty.discard_candidate().unwrap();
    second_dirty.discard_candidate().unwrap();
    assert!(!pool.close().requires_inspection());
}

#[test]
fn all_three_speculative_kinds_share_the_global_operation_envelope() {
    let identity = store(111);
    let pool = PhysicalResidencyPool::open(identity, speculation_limits(24, 1, [1, 1, 1])).unwrap();
    let dirty_key = PhysicalFrameKey::new(identity, coordinate(1, 8));
    let dirty_allocation = pool
        .begin_foreground_write_operation(nonzero_bytes(8))
        .unwrap();
    let clean = expect_fault(&pool, &dirty_allocation, dirty_key)
        .load(|bytes| {
            bytes.fill(1);
            Ok::<_, ()>(())
        })
        .unwrap();
    let dirty = clean
        .begin_dirty_replacement(&dirty_allocation)
        .unwrap()
        .replace(|_, bytes| {
            bytes.fill(2);
            Ok::<_, ()>(())
        })
        .unwrap();
    drop(dirty_allocation);

    let prefetch = pool
        .admit_prefetch(
            pool.begin_foreground_read_operation(nonzero_bytes(8))
                .unwrap(),
            coordinate(2, 8),
        )
        .unwrap();
    let read_ahead_coordinates = [coordinate(3, 8)];
    let read_ahead = pool
        .admit_read_ahead(
            pool.begin_foreground_read_operation(nonzero_bytes(8))
                .unwrap(),
            &read_ahead_coordinates,
        )
        .unwrap();
    let writebehind = writeback_claim(&pool, &[dirty_key]);

    assert_eq!(
        pool.begin_foreground_read_operation(NonZeroU64::MIN)
            .unwrap_err(),
        PhysicalResidencyDenial::Pressure(PhysicalResidencyPressureDenial::new(
            identity,
            pool.incarnation(),
            PhysicalResidencyPressureDemand {
                dimension: PhysicalResidencyDimension::OperationBytes,
                scope: READ_SCOPE,
                requested: 1,
                current: 24,
                limit: 24,
            },
        ))
    );
    let live = pool.counters();
    assert_eq!(live.active_operation_bytes(), 24);
    for kind in [
        PhysicalSpeculativeWorkKind::Prefetch,
        PhysicalSpeculativeWorkKind::ReadAhead,
        PhysicalSpeculativeWorkKind::WriteBehind,
    ] {
        assert_eq!(live.active_speculative_frames(kind), 1);
        assert_eq!(live.peak_speculative_frames(kind), 1);
    }

    drop((prefetch, read_ahead, writebehind));
    let released = pool.counters();
    assert_eq!(released.active_operation_bytes(), 0);
    for kind in [
        PhysicalSpeculativeWorkKind::Prefetch,
        PhysicalSpeculativeWorkKind::ReadAhead,
        PhysicalSpeculativeWorkKind::WriteBehind,
    ] {
        assert_eq!(released.speculative_completions(kind), 1);
        assert_eq!(released.active_speculative_frames(kind), 0);
    }
    dirty.discard_candidate().unwrap();
    assert!(!pool.close().requires_inspection());
}

fn speculation_limits(
    operation_bytes: u64,
    dirty_frames: u32,
    speculative_frames: [u32; 3],
) -> PhysicalResidencyLimits {
    use PhysicalOperationAllocationScope as Scope;
    use PhysicalSpeculativeWorkKind as Kind;

    PhysicalResidencyLimits::builder()
        .total_bytes(nonzero_bytes(4352 + operation_bytes))
        .resident_bytes(nonzero_bytes(128))
        .metadata_bytes(nonzero_bytes(4096))
        .frame_entries(nonzero_count(4))
        .pinned_frames(nonzero_count(3))
        .pin_leases(nonzero_count(3))
        .dirty_frames(nonzero_count(dirty_frames))
        .dirty_replacement_bytes(nonzero_bytes(128))
        .operation_bytes(nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::ForegroundRead, nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::ForegroundWrite, nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::Recovery, nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::Scrub, nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::Maintenance, nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::Verification, nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::Blob, nonzero_bytes(operation_bytes))
        .speculative_frames(Kind::Prefetch, nonzero_count(speculative_frames[0]))
        .speculative_frames(Kind::ReadAhead, nonzero_count(speculative_frames[1]))
        .speculative_frames(Kind::WriteBehind, nonzero_count(speculative_frames[2]))
        .admit(NonZeroU64::MIN)
        .unwrap()
}
