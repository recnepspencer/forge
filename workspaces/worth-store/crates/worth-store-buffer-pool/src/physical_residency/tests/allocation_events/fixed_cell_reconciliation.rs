use super::*;

#[test]
fn every_scope_and_speculative_kind_has_an_independent_fixed_event_cell() {
    let identity = store(106);
    let candidate_bytes = candidate_batch_bytes(1);
    let operation_bytes = candidate_bytes + 30;
    let pool =
        PhysicalResidencyPool::open(identity, limits(128, 3, 2, operation_bytes, 3)).unwrap();
    let observer = pool.allocation_events();
    let operations = operation_scopes()
        .into_iter()
        .filter(|scope| *scope != WRITE_SCOPE)
        .map(|scope| (scope, pool.begin_operation(scope, NonZeroU64::MIN).unwrap()))
        .collect::<Vec<_>>();
    let write = pool
        .begin_foreground_write_operation(NonZeroU64::new(candidate_bytes).unwrap())
        .unwrap();
    let dirty = pool
        .materialize_dirty_candidate(
            &write,
            PhysicalFrameKey::new(identity, coordinate(9, 8)),
            |bytes| bytes.fill(9),
        )
        .unwrap();
    let prefetch_coordinate = coordinate(10, 8);
    let prefetch = pool
        .admit_prefetch(
            pool.begin_foreground_read_operation(nonzero_bytes(8))
                .unwrap(),
            prefetch_coordinate,
        )
        .unwrap();
    let read_ahead_coordinates = [coordinate(11, 8)];
    let read_ahead = pool
        .admit_read_ahead(
            pool.begin_foreground_read_operation(nonzero_bytes(8))
                .unwrap(),
            &read_ahead_coordinates,
        )
        .unwrap();
    let writeback = writeback_claim(&pool, &[PhysicalFrameKey::new(identity, coordinate(9, 8))]);

    assert_reconciled(&pool, observer.snapshot());
    for scope in operation_scopes() {
        let expected = match scope {
            WRITE_SCOPE => candidate_bytes + 8,
            READ_SCOPE => 17,
            _ => 1,
        };
        assert_dimension(
            observer.snapshot(),
            PhysicalResidencyDimension::OperationScope(scope),
            expected,
        );
    }
    for kind in speculative_kinds() {
        let events = observer
            .snapshot()
            .for_dimension(PhysicalResidencyDimension::SpeculativeFrames(kind));
        assert_eq!(events.attempts(), 1);
        assert_eq!(events.admissions(), 1);
        assert_eq!(events.active_units(), 1);
    }

    drop((prefetch, read_ahead, writeback));
    for kind in speculative_kinds() {
        let events = observer
            .snapshot()
            .for_dimension(PhysicalResidencyDimension::SpeculativeFrames(kind));
        assert_eq!(events.releases(), 1);
        assert_eq!(events.active_units(), 0);
        assert_eq!(pool.counters().speculative_attempts(kind), 1);
        assert_eq!(pool.counters().speculative_admissions(kind), 1);
        assert_eq!(pool.counters().speculative_completions(kind), 1);
    }
    dirty.discard_candidate().unwrap();
    drop(write);
    drop(operations);
    assert_reconciled(&pool, observer.snapshot());
}
