use super::*;

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
