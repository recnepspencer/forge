use super::*;

#[test]
fn every_scope_and_speculative_kind_has_an_independent_fixed_event_cell() {
    let identity = store(106);
    let candidate_bytes = candidate_batch_bytes(1);
    let operation_bytes = candidate_bytes + 6;
    let pool =
        PhysicalResidencyPool::open(identity, limits(128, 3, 2, operation_bytes, 3)).unwrap();
    let observer = pool.allocation_events();
    let operations = operation_scopes()
        .into_iter()
        .map(|scope| {
            let bytes = if scope == WRITE_SCOPE {
                NonZeroU64::new(candidate_bytes).unwrap()
            } else {
                NonZeroU64::MIN
            };
            (scope, pool.begin_operation(scope, bytes).unwrap())
        })
        .collect::<Vec<_>>();
    let write = &operations
        .iter()
        .find(|(scope, _)| *scope == WRITE_SCOPE)
        .unwrap()
        .1;
    let dirty = pool
        .admit_dirty(
            write,
            PhysicalFrameKey::new(identity, coordinate(9, 8)),
            vec![9; 8],
        )
        .unwrap();
    let speculation = speculative_kinds()
        .into_iter()
        .map(|kind| pool.begin_speculative(write, kind, 1).unwrap())
        .collect::<Vec<_>>();

    assert_reconciled(&pool, observer.snapshot());
    for scope in operation_scopes() {
        let expected = if scope == WRITE_SCOPE {
            candidate_bytes
        } else {
            1
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

    drop(speculation);
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
    drop(operations);
    assert_reconciled(&pool, observer.snapshot());
}
