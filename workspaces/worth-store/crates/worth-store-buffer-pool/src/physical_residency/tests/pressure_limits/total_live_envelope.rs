use super::*;

#[test]
fn resident_admission_cannot_cross_the_global_live_byte_envelope() {
    let identity = store(22);
    let total = 4096;
    let pool = PhysicalResidencyPool::open(
        identity,
        pressure_limits(PressureLimitDeclaration {
            total,
            resident: 512,
            metadata: 4096,
            operation: 4096,
            read_scope: 4096,
            write_scope: 4096,
        }),
    )
    .unwrap();
    let metadata = pool.counters().metadata_bytes();
    assert!(metadata < total);
    let operation = pool
        .begin_operation(READ_SCOPE, nonzero_bytes(total - metadata))
        .unwrap();
    assert_eq!(pool.counters().admitted_bytes(), total);

    let key = PhysicalFrameKey::new(identity, coordinate(1, 32));
    let denial = pool.access_frame(&operation, key).unwrap_err();
    assert_pressure(
        pressure(denial),
        ExpectedPressure {
            store: identity,
            pool: pool.incarnation(),
            dimension: PhysicalResidencyDimension::TotalBytes,
            scope: READ_SCOPE,
            requested: 32,
            current: total,
            limit: total,
        },
    );
    assert_eq!(pool.counters().source_loads(), 0);
    assert_eq!(pool.counters().resident_bytes(), 0);

    drop(operation);
    let allocation = allocation(&pool, READ_SCOPE);
    let lease = expect_fault(&pool, &allocation, key)
        .load(|bytes| fill(bytes, 7))
        .unwrap();
    assert_eq!(lease.as_ref(), &[7; 32]);
    assert!(pool.counters().peak_admitted_bytes() <= total);
}
