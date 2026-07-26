use std::num::{NonZeroU32, NonZeroU64};

use super::super::{
    PhysicalOperationAllocationScope as Scope, PhysicalResidencyDimension as Dimension,
    PhysicalResidencyLimits, PhysicalResidencyLimitsAdmissionDenial as Denial,
    PhysicalSpeculativeWorkKind as Speculation,
};

#[test]
fn every_required_dimension_is_explicit() {
    for dimension in required_dimensions() {
        assert_eq!(
            builder_missing(dimension).admit(bytes(4096)).unwrap_err(),
            Denial::Missing(dimension),
        );
    }
}

#[test]
fn category_and_scope_relationships_are_typed() {
    for (dimension, denial) in [
        (
            Dimension::ResidentBytes,
            complete_builder()
                .resident_bytes(bytes(32_769))
                .admit(bytes(4096)),
        ),
        (
            Dimension::MetadataBytes,
            complete_builder()
                .metadata_bytes(bytes(32_769))
                .admit(bytes(4096)),
        ),
        (
            Dimension::DirtyReplacementBytes,
            complete_builder()
                .dirty_replacement_bytes(bytes(32_769))
                .admit(bytes(4096)),
        ),
        (
            Dimension::OperationBytes,
            complete_builder()
                .operation_bytes(bytes(32_769))
                .admit(bytes(4096)),
        ),
    ] {
        assert_eq!(
            denial.unwrap_err(),
            Denial::CategoryExceedsTotal {
                dimension,
                declared: 32_769,
                total: 32_768,
            },
        );
    }
    for scope in operation_scopes() {
        assert_eq!(
            complete_builder()
                .scope_bytes(scope, bytes(8193))
                .admit(bytes(4096))
                .unwrap_err(),
            Denial::ScopeExceedsOperation {
                scope,
                declared: 8193,
                operation: 8192,
            },
        );
    }
}

#[test]
fn frame_count_relationships_and_independent_pin_ceilings_are_honest() {
    for (dimension, denial) in [
        (
            Dimension::PinnedFrames,
            complete_builder()
                .pinned_frames(count(9))
                .admit(bytes(4096)),
        ),
        (
            Dimension::DirtyFrames,
            complete_builder().dirty_frames(count(9)).admit(bytes(4096)),
        ),
        (
            Dimension::SpeculativeFrames(Speculation::Prefetch),
            complete_builder()
                .speculative_frames(Speculation::Prefetch, count(9))
                .admit(bytes(4096)),
        ),
        (
            Dimension::SpeculativeFrames(Speculation::ReadAhead),
            complete_builder()
                .speculative_frames(Speculation::ReadAhead, count(9))
                .admit(bytes(4096)),
        ),
        (
            Dimension::SpeculativeFrames(Speculation::WriteBehind),
            complete_builder()
                .speculative_frames(Speculation::WriteBehind, count(9))
                .admit(bytes(4096)),
        ),
    ] {
        assert_eq!(
            denial.unwrap_err(),
            Denial::CountExceedsFrameEntries {
                dimension,
                declared: 9,
                frame_entries: 8,
            },
        );
    }
    assert!(complete_builder()
        .pinned_frames(count(8))
        .pin_leases(count(2))
        .admit(bytes(4096))
        .is_ok());
}

#[test]
fn admitted_format_page_must_fit_every_page_allocation_category() {
    assert_eq!(
        complete_builder()
            .resident_bytes(bytes(4095))
            .admit(bytes(4096))
            .unwrap_err(),
        Denial::PageExceedsResidentBytes {
            page: 4096,
            resident: 4095,
        },
    );
    assert_eq!(
        with_operation_ceiling(complete_builder(), 4095)
            .admit(bytes(4096))
            .unwrap_err(),
        Denial::PageExceedsOperationBytes {
            page: 4096,
            operation: 4095,
        },
    );
    assert_eq!(
        complete_builder()
            .dirty_replacement_bytes(bytes(4095))
            .admit(bytes(4096))
            .unwrap_err(),
        Denial::PageExceedsDirtyReplacementBytes {
            page: 4096,
            dirty_replacement: 4095,
        },
    );
}

fn with_operation_ceiling(
    builder: super::PhysicalResidencyLimitsBuilder,
    ceiling: u64,
) -> super::PhysicalResidencyLimitsBuilder {
    let ceiling = bytes(ceiling);
    builder
        .operation_bytes(ceiling)
        .scope_bytes(Scope::ForegroundRead, ceiling)
        .scope_bytes(Scope::ForegroundWrite, ceiling)
        .scope_bytes(Scope::Recovery, ceiling)
        .scope_bytes(Scope::Scrub, ceiling)
        .scope_bytes(Scope::Maintenance, ceiling)
        .scope_bytes(Scope::Verification, ceiling)
        .scope_bytes(Scope::Blob, ceiling)
}

fn complete_builder() -> super::PhysicalResidencyLimitsBuilder {
    PhysicalResidencyLimits::builder()
        .total_bytes(bytes(32_768))
        .resident_bytes(bytes(8192))
        .metadata_bytes(bytes(8192))
        .frame_entries(count(8))
        .pinned_frames(count(8))
        .pin_leases(count(8))
        .dirty_frames(count(4))
        .dirty_replacement_bytes(bytes(8192))
        .operation_bytes(bytes(8192))
        .scope_bytes(Scope::ForegroundRead, bytes(8192))
        .scope_bytes(Scope::ForegroundWrite, bytes(8192))
        .scope_bytes(Scope::Recovery, bytes(8192))
        .scope_bytes(Scope::Scrub, bytes(8192))
        .scope_bytes(Scope::Maintenance, bytes(8192))
        .scope_bytes(Scope::Verification, bytes(8192))
        .scope_bytes(Scope::Blob, bytes(8192))
        .speculative_frames(Speculation::Prefetch, count(8))
        .speculative_frames(Speculation::ReadAhead, count(8))
        .speculative_frames(Speculation::WriteBehind, count(4))
}

fn builder_missing(missing: Dimension) -> super::PhysicalResidencyLimitsBuilder {
    let mut builder = PhysicalResidencyLimits::builder();
    for (dimension, declare) in [
        (Dimension::TotalBytes, declare_total as fn(_) -> _),
        (Dimension::ResidentBytes, declare_resident),
        (Dimension::MetadataBytes, declare_metadata),
        (Dimension::FrameEntries, declare_entries),
        (Dimension::PinnedFrames, declare_pinned),
        (Dimension::PinLeases, declare_leases),
        (Dimension::DirtyFrames, declare_dirty),
        (Dimension::DirtyReplacementBytes, declare_replacement),
        (Dimension::OperationBytes, declare_operation),
    ] {
        if dimension != missing {
            builder = declare(builder);
        }
    }
    for scope in operation_scopes() {
        if missing != Dimension::OperationScope(scope) {
            builder = builder.scope_bytes(scope, bytes(8192));
        }
    }
    for (kind, frames) in [
        (Speculation::Prefetch, 8),
        (Speculation::ReadAhead, 8),
        (Speculation::WriteBehind, 4),
    ] {
        if missing != Dimension::SpeculativeFrames(kind) {
            builder = builder.speculative_frames(kind, count(frames));
        }
    }
    builder
}

fn declare_total(
    builder: super::PhysicalResidencyLimitsBuilder,
) -> super::PhysicalResidencyLimitsBuilder {
    builder.total_bytes(bytes(32_768))
}

fn declare_resident(
    builder: super::PhysicalResidencyLimitsBuilder,
) -> super::PhysicalResidencyLimitsBuilder {
    builder.resident_bytes(bytes(8192))
}

fn declare_metadata(
    builder: super::PhysicalResidencyLimitsBuilder,
) -> super::PhysicalResidencyLimitsBuilder {
    builder.metadata_bytes(bytes(8192))
}

fn declare_entries(
    builder: super::PhysicalResidencyLimitsBuilder,
) -> super::PhysicalResidencyLimitsBuilder {
    builder.frame_entries(count(8))
}

fn declare_pinned(
    builder: super::PhysicalResidencyLimitsBuilder,
) -> super::PhysicalResidencyLimitsBuilder {
    builder.pinned_frames(count(8))
}

fn declare_leases(
    builder: super::PhysicalResidencyLimitsBuilder,
) -> super::PhysicalResidencyLimitsBuilder {
    builder.pin_leases(count(8))
}

fn declare_dirty(
    builder: super::PhysicalResidencyLimitsBuilder,
) -> super::PhysicalResidencyLimitsBuilder {
    builder.dirty_frames(count(4))
}

fn declare_replacement(
    builder: super::PhysicalResidencyLimitsBuilder,
) -> super::PhysicalResidencyLimitsBuilder {
    builder.dirty_replacement_bytes(bytes(8192))
}

fn declare_operation(
    builder: super::PhysicalResidencyLimitsBuilder,
) -> super::PhysicalResidencyLimitsBuilder {
    builder.operation_bytes(bytes(8192))
}

fn required_dimensions() -> [Dimension; 19] {
    [
        Dimension::TotalBytes,
        Dimension::ResidentBytes,
        Dimension::MetadataBytes,
        Dimension::FrameEntries,
        Dimension::PinnedFrames,
        Dimension::PinLeases,
        Dimension::DirtyFrames,
        Dimension::DirtyReplacementBytes,
        Dimension::OperationBytes,
        Dimension::OperationScope(Scope::ForegroundRead),
        Dimension::OperationScope(Scope::ForegroundWrite),
        Dimension::OperationScope(Scope::Recovery),
        Dimension::OperationScope(Scope::Scrub),
        Dimension::OperationScope(Scope::Maintenance),
        Dimension::OperationScope(Scope::Verification),
        Dimension::OperationScope(Scope::Blob),
        Dimension::SpeculativeFrames(Speculation::Prefetch),
        Dimension::SpeculativeFrames(Speculation::ReadAhead),
        Dimension::SpeculativeFrames(Speculation::WriteBehind),
    ]
}

const fn operation_scopes() -> [Scope; 7] {
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

fn bytes(value: u64) -> NonZeroU64 {
    NonZeroU64::new(value).unwrap()
}

fn count(value: u32) -> NonZeroU32 {
    NonZeroU32::new(value).unwrap()
}
