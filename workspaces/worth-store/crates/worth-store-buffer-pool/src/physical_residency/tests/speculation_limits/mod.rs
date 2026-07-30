use super::*;

mod global_operation_envelope;
mod kind_ceiling;

fn speculation_limits(
    operation_bytes: u64,
    dirty_frames: u32,
    speculative_frames: [u32; 3],
) -> PhysicalResidencyLimits {
    use PhysicalOperationAllocationScope as Scope;
    use PhysicalSpeculativeWorkKind as Kind;

    PhysicalResidencyLimits::builder()
        .total_bytes(nonzero_bytes(4352 + operation_bytes))
        .resident_bytes(nonzero_bytes(128))
        .metadata_bytes(nonzero_bytes(4096))
        .frame_entries(nonzero_count(4))
        .pinned_frames(nonzero_count(3))
        .pin_leases(nonzero_count(3))
        .dirty_frames(nonzero_count(dirty_frames))
        .dirty_replacement_bytes(nonzero_bytes(128))
        .operation_bytes(nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::ForegroundRead, nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::ForegroundWrite, nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::Recovery, nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::Scrub, nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::Maintenance, nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::Verification, nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::Blob, nonzero_bytes(operation_bytes))
        .speculative_frames(Kind::Prefetch, nonzero_count(speculative_frames[0]))
        .speculative_frames(Kind::ReadAhead, nonzero_count(speculative_frames[1]))
        .speculative_frames(Kind::WriteBehind, nonzero_count(speculative_frames[2]))
        .admit(NonZeroU64::MIN)
        .unwrap()
}
