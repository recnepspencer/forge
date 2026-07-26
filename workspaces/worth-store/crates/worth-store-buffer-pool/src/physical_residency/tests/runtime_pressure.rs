use super::*;

#[test]
fn one_scope_cannot_spend_past_its_own_ceiling() {
    let identity = store(21);
    let pool = operation_pressure_pool(identity);
    let read = pool.begin_operation(READ_SCOPE, nonzero_bytes(32)).unwrap();
    let scope_pressure = pressure(
        pool.begin_operation(READ_SCOPE, nonzero_bytes(1))
            .unwrap_err(),
    );
    assert_pressure(
        scope_pressure,
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
    assert_eq!(pool.counters().active_operation_bytes_for(READ_SCOPE), 32);
    drop(read);
}

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

fn pressure(denial: PhysicalResidencyDenial) -> PhysicalResidencyPressureDenial {
    let PhysicalResidencyDenial::Pressure(pressure) = denial else {
        panic!("expected typed physical pressure, got {denial:?}");
    };
    pressure
}

fn assert_pressure(pressure: PhysicalResidencyPressureDenial, expected: ExpectedPressure) {
    assert_eq!(pressure.store(), expected.store);
    assert_eq!(pressure.pool(), expected.pool);
    assert_eq!(pressure.dimension(), expected.dimension);
    assert_eq!(pressure.scope(), expected.scope);
    assert_eq!(pressure.requested(), expected.requested);
    assert_eq!(pressure.current(), expected.current);
    assert_eq!(pressure.limit(), expected.limit);
    assert!(!pressure.effect_may_have_started());
}

struct ExpectedPressure {
    store: worth_store_physical_format::store_namespace::StableStoreIdentity,
    pool: PhysicalResidencyIncarnation,
    dimension: PhysicalResidencyDimension,
    scope: PhysicalOperationAllocationScope,
    requested: u64,
    current: u64,
    limit: u64,
}

struct PressureLimitDeclaration {
    total: u64,
    resident: u64,
    metadata: u64,
    operation: u64,
    read_scope: u64,
    write_scope: u64,
}

fn operation_pressure_pool(
    identity: worth_store_physical_format::store_namespace::StableStoreIdentity,
) -> PhysicalResidencyPool {
    PhysicalResidencyPool::open(
        identity,
        pressure_limits(PressureLimitDeclaration {
            total: 16_384,
            resident: 512,
            metadata: 4096,
            operation: 64,
            read_scope: 32,
            write_scope: 48,
        }),
    )
    .unwrap()
}

fn pressure_limits(declaration: PressureLimitDeclaration) -> PhysicalResidencyLimits {
    use PhysicalOperationAllocationScope as Scope;
    use PhysicalSpeculativeWorkKind as Speculation;

    PhysicalResidencyLimits::builder()
        .total_bytes(nonzero_bytes(declaration.total))
        .resident_bytes(nonzero_bytes(declaration.resident))
        .metadata_bytes(nonzero_bytes(declaration.metadata))
        .frame_entries(nonzero_count(4))
        .pinned_frames(nonzero_count(4))
        .pin_leases(nonzero_count(4))
        .dirty_frames(nonzero_count(2))
        .dirty_replacement_bytes(nonzero_bytes(declaration.resident))
        .operation_bytes(nonzero_bytes(declaration.operation))
        .scope_bytes(Scope::ForegroundRead, nonzero_bytes(declaration.read_scope))
        .scope_bytes(
            Scope::ForegroundWrite,
            nonzero_bytes(declaration.write_scope),
        )
        .scope_bytes(Scope::Recovery, nonzero_bytes(declaration.operation))
        .scope_bytes(Scope::Scrub, nonzero_bytes(declaration.operation))
        .scope_bytes(Scope::Maintenance, nonzero_bytes(declaration.operation))
        .scope_bytes(Scope::Verification, nonzero_bytes(declaration.operation))
        .scope_bytes(Scope::Blob, nonzero_bytes(declaration.operation))
        .speculative_frames(Speculation::Prefetch, nonzero_count(2))
        .speculative_frames(Speculation::ReadAhead, nonzero_count(2))
        .speculative_frames(Speculation::WriteBehind, nonzero_count(2))
        .admit(std::num::NonZeroU64::MIN)
        .unwrap()
}
