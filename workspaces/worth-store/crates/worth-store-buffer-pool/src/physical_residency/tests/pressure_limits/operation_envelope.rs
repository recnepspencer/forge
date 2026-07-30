use super::*;

#[test]
fn scopes_share_one_aggregate_operation_envelope() {
    let identity = store(23);
    let pool = operation_pressure_pool(identity);
    let read = pool.begin_operation(READ_SCOPE, nonzero_bytes(32)).unwrap();
    let write = pool
        .begin_operation(WRITE_SCOPE, nonzero_bytes(32))
        .unwrap();
    let operation_pressure = pressure(
        pool.begin_operation(
            PhysicalOperationAllocationScope::Maintenance,
            nonzero_bytes(1),
        )
        .unwrap_err(),
    );
    assert_pressure(
        operation_pressure,
        ExpectedPressure {
            store: identity,
            pool: pool.incarnation(),
            dimension: PhysicalResidencyDimension::OperationBytes,
            scope: PhysicalOperationAllocationScope::Maintenance,
            requested: 1,
            current: 64,
            limit: 64,
        },
    );

    let counters = pool.counters();
    assert_eq!(counters.active_operation_bytes(), 64);
    assert_eq!(counters.peak_operation_bytes(), 64);
    assert_eq!(counters.active_operation_bytes_for(READ_SCOPE), 32);
    assert_eq!(counters.active_operation_bytes_for(WRITE_SCOPE), 32);
    drop((read, write));
    assert_eq!(pool.counters().active_operation_bytes(), 0);
}
