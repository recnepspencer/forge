use std::num::{NonZeroU32, NonZeroU64};

use worth_store::physical_runtime::{
    AdmittedPhysicalRecordFormat, PhysicalOperationAllocationScope as Scope,
    PhysicalRecordResidencyPolicy, PhysicalRecordResidencyPolicyBuilder,
    PhysicalRecordResidencyPolicyOutcome, PhysicalSpeculativeWorkKind as Speculation,
};

use super::BoundedResidencyConfiguration;

const PRODUCER_OPERATION_BYTES: u64 = 16 * 1024 * 1024;

pub(super) fn admit_serving(
    configuration: BoundedResidencyConfiguration,
    format: AdmittedPhysicalRecordFormat,
) -> PhysicalRecordResidencyPolicyOutcome {
    fixed_dimensions(configuration)
        .total_bytes(bytes(configuration.total_bytes()))
        .operation_bytes(bytes(configuration.operation_bytes()))
        .scope_bytes(
            Scope::ForegroundRead,
            bytes(configuration.scope_bytes(Scope::ForegroundRead)),
        )
        .scope_bytes(
            Scope::ForegroundWrite,
            bytes(configuration.scope_bytes(Scope::ForegroundWrite)),
        )
        .scope_bytes(
            Scope::Recovery,
            bytes(configuration.scope_bytes(Scope::Recovery)),
        )
        .scope_bytes(Scope::Scrub, bytes(configuration.scope_bytes(Scope::Scrub)))
        .scope_bytes(
            Scope::Maintenance,
            bytes(configuration.scope_bytes(Scope::Maintenance)),
        )
        .scope_bytes(
            Scope::Verification,
            bytes(configuration.scope_bytes(Scope::Verification)),
        )
        .scope_bytes(Scope::Blob, bytes(configuration.scope_bytes(Scope::Blob)))
        .admit(format)
}

pub(super) fn admit_producer(
    configuration: BoundedResidencyConfiguration,
    format: AdmittedPhysicalRecordFormat,
) -> PhysicalRecordResidencyPolicyOutcome {
    fixed_dimensions(configuration)
        .total_bytes(bytes(producer_total_bytes(configuration)))
        .operation_bytes(bytes(PRODUCER_OPERATION_BYTES))
        .scope_bytes(Scope::ForegroundRead, bytes(PRODUCER_OPERATION_BYTES))
        .scope_bytes(Scope::ForegroundWrite, bytes(PRODUCER_OPERATION_BYTES))
        .scope_bytes(Scope::Recovery, bytes(PRODUCER_OPERATION_BYTES))
        .scope_bytes(Scope::Scrub, bytes(PRODUCER_OPERATION_BYTES))
        .scope_bytes(Scope::Maintenance, bytes(PRODUCER_OPERATION_BYTES))
        .scope_bytes(Scope::Verification, bytes(PRODUCER_OPERATION_BYTES))
        .scope_bytes(Scope::Blob, bytes(PRODUCER_OPERATION_BYTES))
        .admit(format)
}

fn fixed_dimensions(
    configuration: BoundedResidencyConfiguration,
) -> PhysicalRecordResidencyPolicyBuilder {
    PhysicalRecordResidencyPolicy::builder()
        .resident_bytes(bytes(configuration.resident_bytes()))
        .metadata_bytes(bytes(configuration.metadata_bytes()))
        .frame_entries(count(configuration.frame_entries()))
        .pinned_frames(count(configuration.pinned_frames()))
        .pin_leases(count(configuration.pin_leases()))
        .dirty_frames(count(configuration.dirty_frames()))
        .dirty_replacement_bytes(bytes(configuration.dirty_replacement_bytes()))
        .speculative_frames(
            Speculation::Prefetch,
            count(configuration.speculative_frames(Speculation::Prefetch)),
        )
        .speculative_frames(
            Speculation::ReadAhead,
            count(configuration.speculative_frames(Speculation::ReadAhead)),
        )
        .speculative_frames(
            Speculation::WriteBehind,
            count(configuration.speculative_frames(Speculation::WriteBehind)),
        )
}

fn producer_total_bytes(configuration: BoundedResidencyConfiguration) -> u64 {
    PRODUCER_OPERATION_BYTES
        .saturating_add(configuration.resident_bytes())
        .saturating_add(configuration.metadata_bytes())
        .saturating_add(configuration.dirty_replacement_bytes())
}

fn bytes(value: u64) -> NonZeroU64 {
    NonZeroU64::new(value).expect("validated bounded-residency byte dimension is nonzero")
}

fn count(value: u32) -> NonZeroU32 {
    NonZeroU32::new(value).expect("validated bounded-residency count dimension is nonzero")
}
