use super::*;

#[test]
fn speculative_work_uses_live_pool_limits_and_dirty_posture() {
    let identity = store(12);
    let candidate_bytes = candidate_batch_bytes(1);
    let operation_bytes = candidate_bytes + 1;
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
    let read_allocation = allocation(&pool, READ_SCOPE);
    let write_allocation = candidate_allocation(&pool, WRITE_SCOPE, 1);
    let prefetch = pool
        .begin_speculative(&read_allocation, PhysicalSpeculativeWorkKind::Prefetch, 1)
        .unwrap();
    assert_eq!(
        pool.begin_speculative(&read_allocation, PhysicalSpeculativeWorkKind::Prefetch, 1)
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
    assert_eq!(
        pool.begin_speculative(
            &write_allocation,
            PhysicalSpeculativeWorkKind::WriteBehind,
            1,
        )
        .unwrap_err(),
        PhysicalResidencyDenial::WriteBackExceedsDirtyPosture
    );
    let dirty = pool
        .admit_dirty(
            &write_allocation,
            PhysicalFrameKey::new(identity, coordinate(1, 16)),
            vec![5; 16],
        )
        .unwrap();
    let writeback = pool
        .begin_speculative(
            &write_allocation,
            PhysicalSpeculativeWorkKind::WriteBehind,
            1,
        )
        .unwrap();
    assert_eq!(writeback.frames(), 1);
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
}
