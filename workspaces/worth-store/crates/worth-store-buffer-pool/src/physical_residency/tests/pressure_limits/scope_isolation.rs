use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScopeAccountingState {
    active_operation: u64,
    peak_operation: u64,
    active_read: u64,
    peak_read: u64,
    active_write: u64,
    peak_write: u64,
    denials: u64,
}

#[test]
fn foreground_read_cannot_spend_foreground_write_allowance() {
    let identity = store(21);
    let pool = operation_pressure_pool(identity);
    let held = pool.begin_operation(READ_SCOPE, nonzero_bytes(32)).unwrap();
    assert_eq!(
        accounting(&pool),
        ScopeAccountingState {
            active_operation: 32,
            peak_operation: 32,
            active_read: 32,
            peak_read: 32,
            active_write: 0,
            peak_write: 0,
            denials: 0,
        }
    );

    match pool.begin_operation(READ_SCOPE, nonzero_bytes(1)) {
        Err(denial) => {
            assert_pressure(
                pressure(denial),
                ExpectedPressure {
                    store: identity,
                    pool: pool.incarnation(),
                    dimension: PhysicalResidencyDimension::OperationScope(READ_SCOPE),
                    scope: READ_SCOPE,
                    requested: 1,
                    current: 32,
                    limit: 32,
                },
            );
            assert_eq!(
                accounting(&pool),
                ScopeAccountingState {
                    active_operation: 32,
                    peak_operation: 32,
                    active_read: 32,
                    peak_read: 32,
                    active_write: 0,
                    peak_write: 0,
                    denials: 1,
                }
            );
            drop(held);
            assert_terminal(&pool, 32, 1);
        }
        Ok(stolen) => {
            assert_eq!(
                accounting(&pool),
                ScopeAccountingState {
                    active_operation: 33,
                    peak_operation: 33,
                    active_read: 33,
                    peak_read: 33,
                    active_write: 0,
                    peak_write: 0,
                    denials: 0,
                }
            );
            drop((stolen, held));
            assert_terminal(&pool, 33, 0);
            panic!("MUTANT_PREDICATE:cross-scope-allowance-theft");
        }
    }
}

fn accounting(pool: &PhysicalResidencyPool) -> ScopeAccountingState {
    let counters = pool.counters();
    ScopeAccountingState {
        active_operation: counters.active_operation_bytes(),
        peak_operation: counters.peak_operation_bytes(),
        active_read: counters.active_operation_bytes_for(READ_SCOPE),
        peak_read: counters.peak_operation_bytes_for(READ_SCOPE),
        active_write: counters.active_operation_bytes_for(WRITE_SCOPE),
        peak_write: counters.peak_operation_bytes_for(WRITE_SCOPE),
        denials: counters.denials(),
    }
}

fn assert_terminal(pool: &PhysicalResidencyPool, peak: u64, denials: u64) {
    assert_eq!(
        accounting(pool),
        ScopeAccountingState {
            active_operation: 0,
            peak_operation: peak,
            active_read: 0,
            peak_read: peak,
            active_write: 0,
            peak_write: 0,
            denials,
        }
    );
}
