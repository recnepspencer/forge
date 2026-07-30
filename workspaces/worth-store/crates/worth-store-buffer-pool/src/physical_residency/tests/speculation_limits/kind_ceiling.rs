use super::*;

const SPECULATIVE_KIND_BUDGET_BYPASS: &str = "speculative-kind-budget-bypass";

#[derive(Clone, Copy)]
struct ExpectedPrefetchState {
    attempts: u64,
    admissions: u64,
    denials: u64,
    completions: u64,
    active: u32,
    peak: u32,
    operation_bytes: u64,
}

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
        speculative_pressure(
            &pool,
            identity,
            PhysicalSpeculativeWorkKind::WriteBehind,
            WRITE_SCOPE
        )
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
fn prefetch_kind_ceiling_cannot_be_bypassed_by_valid_scope_authority() {
    let identity = store(112);
    let pool = PhysicalResidencyPool::open(identity, speculation_limits(64, 1, [1, 1, 1])).unwrap();
    let first = pool
        .admit_prefetch(
            pool.begin_foreground_read_operation(nonzero_bytes(32))
                .unwrap(),
            coordinate(1, 32),
        )
        .unwrap();
    let second = pool.admit_prefetch(
        pool.begin_foreground_read_operation(nonzero_bytes(32))
            .unwrap(),
        coordinate(2, 32),
    );

    match second {
        Err(denial) => {
            assert_eq!(
                denial,
                speculative_pressure(
                    &pool,
                    identity,
                    PhysicalSpeculativeWorkKind::Prefetch,
                    READ_SCOPE,
                )
            );
            assert_prefetch_state(
                &pool,
                ExpectedPrefetchState {
                    attempts: 2,
                    admissions: 1,
                    denials: 1,
                    completions: 0,
                    active: 1,
                    peak: 1,
                    operation_bytes: 32,
                },
            );
        }
        Ok(illicit) => {
            assert_prefetch_state(
                &pool,
                ExpectedPrefetchState {
                    attempts: 2,
                    admissions: 2,
                    denials: 0,
                    completions: 0,
                    active: 2,
                    peak: 2,
                    operation_bytes: 64,
                },
            );
            drop((first, illicit));
            assert_prefetch_state(
                &pool,
                ExpectedPrefetchState {
                    attempts: 2,
                    admissions: 2,
                    denials: 0,
                    completions: 2,
                    active: 0,
                    peak: 2,
                    operation_bytes: 0,
                },
            );
            panic!("MUTANT_PREDICATE:{SPECULATIVE_KIND_BUDGET_BYPASS}");
        }
    }

    drop(first);
    assert_prefetch_state(
        &pool,
        ExpectedPrefetchState {
            attempts: 2,
            admissions: 1,
            denials: 1,
            completions: 1,
            active: 0,
            peak: 1,
            operation_bytes: 0,
        },
    );
    assert!(!pool.close().requires_inspection());
}

fn assert_prefetch_state(pool: &PhysicalResidencyPool, expected: ExpectedPrefetchState) {
    let actual = pool.counters();
    let kind = PhysicalSpeculativeWorkKind::Prefetch;
    assert_eq!(actual.speculative_attempts(kind), expected.attempts);
    assert_eq!(actual.speculative_admissions(kind), expected.admissions);
    assert_eq!(actual.speculative_denials(kind), expected.denials);
    assert_eq!(actual.speculative_completions(kind), expected.completions);
    assert_eq!(actual.active_speculative_frames(kind), expected.active);
    assert_eq!(actual.peak_speculative_frames(kind), expected.peak);
    assert_eq!(actual.active_operation_bytes(), expected.operation_bytes);
}

fn speculative_pressure(
    pool: &PhysicalResidencyPool,
    identity: StableStoreIdentity,
    kind: PhysicalSpeculativeWorkKind,
    scope: PhysicalOperationAllocationScope,
) -> PhysicalResidencyDenial {
    PhysicalResidencyDenial::Pressure(PhysicalResidencyPressureDenial::new(
        identity,
        pool.incarnation(),
        PhysicalResidencyPressureDemand {
            dimension: PhysicalResidencyDimension::SpeculativeFrames(kind),
            scope,
            requested: 1,
            current: 1,
            limit: 1,
        },
    ))
}
