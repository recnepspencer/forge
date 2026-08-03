use super::*;

mod operation_envelope;
mod scope_isolation;
mod total_live_envelope;

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
