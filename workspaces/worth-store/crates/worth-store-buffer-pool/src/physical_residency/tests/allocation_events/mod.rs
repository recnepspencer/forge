use super::*;

mod dimension_reconciliation;
mod dirty_replacement_release;
mod fixed_cell_reconciliation;

fn assert_reconciled(
    pool: &PhysicalResidencyPool,
    events: PhysicalResidencyAllocationEventSnapshot,
) {
    let counters = pool.counters();
    for (dimension, expected) in [
        (
            PhysicalResidencyDimension::MetadataBytes,
            counters.metadata_bytes(),
        ),
        (
            PhysicalResidencyDimension::ResidentBytes,
            counters.resident_bytes(),
        ),
        (
            PhysicalResidencyDimension::DirtyReplacementBytes,
            counters.dirty_replacement_bytes(),
        ),
        (
            PhysicalResidencyDimension::FrameEntries,
            u64::from(counters.frame_entries()),
        ),
        (
            PhysicalResidencyDimension::PinnedFrames,
            u64::from(counters.pinned_frames()),
        ),
        (
            PhysicalResidencyDimension::PinLeases,
            u64::from(counters.pin_leases()),
        ),
        (
            PhysicalResidencyDimension::DirtyFrames,
            u64::from(counters.dirty_frames()),
        ),
        (
            PhysicalResidencyDimension::OperationBytes,
            counters.active_operation_bytes(),
        ),
        (
            PhysicalResidencyDimension::TotalBytes,
            counters.admitted_bytes(),
        ),
    ] {
        assert_dimension(events, dimension, expected);
    }
    for scope in operation_scopes() {
        assert_dimension(
            events,
            PhysicalResidencyDimension::OperationScope(scope),
            counters.active_operation_bytes_for(scope),
        );
    }
    for kind in speculative_kinds() {
        assert_dimension(
            events,
            PhysicalResidencyDimension::SpeculativeFrames(kind),
            u64::from(counters.active_speculative_frames(kind)),
        );
    }
}

fn assert_dimension(
    events: PhysicalResidencyAllocationEventSnapshot,
    dimension: PhysicalResidencyDimension,
    expected: u64,
) {
    assert_eq!(
        events.for_dimension(dimension).active_units(),
        expected,
        "{dimension:?} did not reconcile"
    );
}

const fn operation_scopes() -> [PhysicalOperationAllocationScope; 7] {
    use PhysicalOperationAllocationScope as Scope;
    [
        Scope::ForegroundRead,
        Scope::ForegroundWrite,
        Scope::Recovery,
        Scope::Scrub,
        Scope::Maintenance,
        Scope::Verification,
        Scope::Blob,
    ]
}

const fn speculative_kinds() -> [PhysicalSpeculativeWorkKind; 3] {
    use PhysicalSpeculativeWorkKind as Kind;
    [Kind::ReadAhead, Kind::Prefetch, Kind::WriteBehind]
}
