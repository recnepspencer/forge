use super::*;

#[test]
fn speculative_work_uses_live_pool_limits_and_dirty_posture() {
    let identity = store(12);
    let candidate_bytes = candidate_batch_bytes(1);
    let operation_bytes = candidate_bytes + 18;
    let policy = {
        use PhysicalOperationAllocationScope as Scope;
        use PhysicalSpeculativeWorkKind as Speculation;

        PhysicalResidencyLimits::builder()
            .total_bytes(nonzero_bytes(4352 + operation_bytes))
            .resident_bytes(nonzero_bytes(128))
            .metadata_bytes(nonzero_bytes(4096))
            .frame_entries(nonzero_count(3))
            .pinned_frames(nonzero_count(2))
            .pin_leases(nonzero_count(2))
            .dirty_frames(nonzero_count(1))
            .dirty_replacement_bytes(nonzero_bytes(128))
            .operation_bytes(nonzero_bytes(operation_bytes))
            .scope_bytes(Scope::ForegroundRead, nonzero_bytes(operation_bytes))
            .scope_bytes(Scope::ForegroundWrite, nonzero_bytes(operation_bytes))
            .scope_bytes(Scope::Recovery, nonzero_bytes(operation_bytes))
            .scope_bytes(Scope::Scrub, nonzero_bytes(operation_bytes))
            .scope_bytes(Scope::Maintenance, nonzero_bytes(operation_bytes))
            .scope_bytes(Scope::Verification, nonzero_bytes(operation_bytes))
            .scope_bytes(Scope::Blob, nonzero_bytes(operation_bytes))
            .speculative_frames(Speculation::Prefetch, nonzero_count(1))
            .speculative_frames(Speculation::ReadAhead, nonzero_count(1))
            .speculative_frames(Speculation::WriteBehind, nonzero_count(1))
            .admit(std::num::NonZeroU64::MIN)
            .unwrap()
    };
    let pool = PhysicalResidencyPool::open(identity, policy).unwrap();
    let write_allocation = candidate_allocation(&pool, 1);
    let prefetch_coordinate = coordinate(2, 1);
    let read_allocation = pool
        .begin_foreground_read_operation(nonzero_bytes(1))
        .unwrap();
    let prefetch = pool
        .admit_prefetch(read_allocation, prefetch_coordinate)
        .unwrap();
    let second_read = pool
        .begin_foreground_read_operation(nonzero_bytes(1))
        .unwrap();
    assert_eq!(
        pool.admit_prefetch(second_read, prefetch_coordinate)
            .unwrap_err(),
        PhysicalResidencyDenial::Pressure(PhysicalResidencyPressureDenial::new(
            identity,
            pool.incarnation(),
            PhysicalResidencyPressureDemand {
                dimension: PhysicalResidencyDimension::SpeculativeFrames(
                    PhysicalSpeculativeWorkKind::Prefetch,
                ),
                scope: READ_SCOPE,
                requested: 1,
                current: 1,
                limit: 1,
            },
        ))
    );
    drop(prefetch);
    let dirty = pool
        .materialize_dirty_candidate(
            &write_allocation,
            PhysicalFrameKey::new(identity, coordinate(1, 16)),
            |bytes| bytes.fill(5),
        )
        .unwrap();
    let writeback = writeback_claim(&pool, &[PhysicalFrameKey::new(identity, coordinate(1, 16))]);
    assert_eq!(writeback.writebehind_grant().frames().len(), 1);
    drop(writeback);
    dirty.discard_candidate().unwrap();
    let counters = pool.counters();
    assert_eq!(
        counters.speculative_attempts(PhysicalSpeculativeWorkKind::Prefetch),
        2
    );
    assert_eq!(
        counters.speculative_admissions(PhysicalSpeculativeWorkKind::Prefetch),
        1
    );
    assert_eq!(
        counters.speculative_completions(PhysicalSpeculativeWorkKind::Prefetch),
        1
    );
    assert_eq!(
        counters.speculative_denials(PhysicalSpeculativeWorkKind::Prefetch),
        1
    );
    assert_eq!(
        counters.active_speculative_frames(PhysicalSpeculativeWorkKind::Prefetch),
        0
    );
    assert_eq!(
        counters.peak_speculative_frames(PhysicalSpeculativeWorkKind::Prefetch),
        1
    );
}

#[test]
fn read_ahead_authority_binds_exact_frames_and_reconciles_every_terminal() {
    let identity = store(13);
    let pool = PhysicalResidencyPool::open(identity, limits(128, 2, 1, 64, 4)).unwrap();
    let first = coordinate(1, 8);
    let second = coordinate(2, 8);
    let third = coordinate(3, 8);

    let undersized = pool
        .begin_foreground_read_operation(nonzero_bytes(15))
        .unwrap();
    assert_eq!(
        pool.admit_read_ahead(undersized, &[first, second])
            .unwrap_err(),
        PhysicalResidencyDenial::SpeculativeAllocationMismatch {
            granted: 15,
            required: 16,
        }
    );
    let oversized_duplicates = pool
        .begin_foreground_read_operation(nonzero_bytes(24))
        .unwrap();
    assert_eq!(
        pool.admit_read_ahead(oversized_duplicates, &[first, first, first])
            .unwrap_err(),
        PhysicalResidencyDenial::Pressure(PhysicalResidencyPressureDenial::new(
            identity,
            pool.incarnation(),
            PhysicalResidencyPressureDemand {
                dimension: PhysicalResidencyDimension::SpeculativeFrames(
                    PhysicalSpeculativeWorkKind::ReadAhead,
                ),
                scope: READ_SCOPE,
                requested: 3,
                current: 0,
                limit: 2,
            },
        ))
    );
    let duplicate = pool
        .begin_foreground_read_operation(nonzero_bytes(16))
        .unwrap();
    assert_eq!(
        pool.admit_read_ahead(duplicate, &[first, first])
            .unwrap_err(),
        PhysicalResidencyDenial::DuplicateSpeculativeFrame
    );
    assert_eq!(
        pool.counters()
            .speculative_attempts(PhysicalSpeculativeWorkKind::ReadAhead),
        3
    );
    assert_eq!(
        pool.counters()
            .speculative_denials(PhysicalSpeculativeWorkKind::ReadAhead),
        3
    );

    let admitted_coordinates = [first, second];
    let admitted = pool
        .admit_read_ahead(
            pool.begin_foreground_read_operation(nonzero_bytes(16))
                .unwrap(),
            &admitted_coordinates,
        )
        .unwrap();
    assert_eq!(admitted.coordinates(), [first, second]);
    assert!(std::ptr::eq(
        admitted.coordinates(),
        admitted_coordinates.as_slice()
    ));
    let first_grant = admitted.frame(0).unwrap();
    let lease = match pool.access_read_ahead_frame(&first_grant).unwrap() {
        PhysicalFrameAccess::Fault(fault) => fault.load(|bytes| fill(bytes, 7)).unwrap(),
        _ => panic!("first exact read-ahead access must own the cold fault"),
    };
    assert_eq!(&*lease, &[7; 8]);
    drop(lease);

    let overflow = pool
        .begin_foreground_read_operation(nonzero_bytes(8))
        .unwrap();
    assert_eq!(
        pool.admit_read_ahead(overflow, &[third]).unwrap_err(),
        PhysicalResidencyDenial::Pressure(PhysicalResidencyPressureDenial::new(
            identity,
            pool.incarnation(),
            PhysicalResidencyPressureDemand {
                dimension: PhysicalResidencyDimension::SpeculativeFrames(
                    PhysicalSpeculativeWorkKind::ReadAhead,
                ),
                scope: READ_SCOPE,
                requested: 1,
                current: 2,
                limit: 2,
            },
        ))
    );
    let counters = pool.counters();
    assert_eq!(
        counters.speculative_attempts(PhysicalSpeculativeWorkKind::ReadAhead),
        5
    );
    assert_eq!(
        counters.speculative_admissions(PhysicalSpeculativeWorkKind::ReadAhead),
        1
    );
    assert_eq!(
        counters.speculative_denials(PhysicalSpeculativeWorkKind::ReadAhead),
        4
    );
    assert_eq!(
        counters.active_speculative_frames(PhysicalSpeculativeWorkKind::ReadAhead),
        2
    );
    assert_eq!(
        counters.peak_speculative_frames(PhysicalSpeculativeWorkKind::ReadAhead),
        2
    );

    drop(first_grant);
    drop(admitted);
    let counters = pool.counters();
    assert_eq!(
        counters.speculative_completions(PhysicalSpeculativeWorkKind::ReadAhead),
        1
    );
    assert_eq!(
        counters.active_speculative_frames(PhysicalSpeculativeWorkKind::ReadAhead),
        0
    );
    assert_eq!(counters.active_operation_bytes(), 0);
}

#[test]
fn speculative_read_kinds_cannot_oversubscribe_the_shared_foreground_read_scope() {
    let identity = store(14);
    let pool = PhysicalResidencyPool::open(identity, limits(128, 2, 1, 16, 4)).unwrap();
    let prefetch = pool
        .admit_prefetch(
            pool.begin_foreground_read_operation(nonzero_bytes(8))
                .unwrap(),
            coordinate(1, 8),
        )
        .unwrap();
    let read_ahead_coordinates = [coordinate(2, 8)];
    let read_ahead = pool
        .admit_read_ahead(
            pool.begin_foreground_read_operation(nonzero_bytes(8))
                .unwrap(),
            &read_ahead_coordinates,
        )
        .unwrap();

    assert_eq!(
        pool.begin_foreground_read_operation(nonzero_bytes(1))
            .unwrap_err(),
        PhysicalResidencyDenial::Pressure(PhysicalResidencyPressureDenial::new(
            identity,
            pool.incarnation(),
            PhysicalResidencyPressureDemand {
                dimension: PhysicalResidencyDimension::OperationScope(READ_SCOPE),
                scope: READ_SCOPE,
                requested: 1,
                current: 16,
                limit: 16,
            },
        ))
    );
    let live = pool.counters();
    assert_eq!(live.active_operation_bytes(), 16);
    assert_eq!(
        live.active_speculative_frames(PhysicalSpeculativeWorkKind::Prefetch),
        1
    );
    assert_eq!(
        live.active_speculative_frames(PhysicalSpeculativeWorkKind::ReadAhead),
        1
    );

    drop((prefetch, read_ahead));
    let released = pool.counters();
    assert_eq!(released.active_operation_bytes(), 0);
    assert_eq!(
        released.speculative_completions(PhysicalSpeculativeWorkKind::Prefetch),
        1
    );
    assert_eq!(
        released.speculative_completions(PhysicalSpeculativeWorkKind::ReadAhead),
        1
    );
}
