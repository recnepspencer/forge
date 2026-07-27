use std::path::Path;

use worth_store::physical_runtime::{
    AdmittedPhysicalRecordFormat, AdmittedPhysicalRecordResidencyPolicy,
    PhysicalOperationAllocationScope as Scope, PhysicalRecordInitialization, PhysicalRecordOpen,
    PhysicalRecordResidencyPolicy, PhysicalSpeculativeWorkKind as Speculation,
    PhysicalWorkCausalRecord, PhysicalWorkIdentity, ServingPhysicalRuntime,
};
use worth_store_physical_backend::MediaOperationRole;
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use super::{configuration, media, success};

const FRAME_BYTES: u32 = 8;

pub(super) fn initialize_store(root: &Path) {
    let (format, placement, access) = configuration();
    let seeded = success(
        media(root)
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    );
    assert!(!seeded.close().residency().requires_inspection());
}

pub(super) fn open_store(
    root: &Path,
    prefetch_frames: u32,
    read_ahead_frames: u32,
) -> ServingPhysicalRuntime {
    open_store_with_writebehind(root, prefetch_frames, read_ahead_frames, 2)
}

pub(super) fn open_store_with_writebehind(
    root: &Path,
    prefetch_frames: u32,
    read_ahead_frames: u32,
    writebehind_frames: u32,
) -> ServingPhysicalRuntime {
    let (format, _, access) = configuration();
    success(media(root).open_record_store(
        PhysicalRecordOpen::new(format, access).with_residency_policy(
            residency_policy_with_writebehind(
                format,
                prefetch_frames,
                read_ahead_frames,
                writebehind_frames,
            ),
        ),
    ))
}

pub(super) fn residency_policy(
    format: AdmittedPhysicalRecordFormat,
    prefetch_frames: u32,
    read_ahead_frames: u32,
) -> AdmittedPhysicalRecordResidencyPolicy {
    residency_policy_with_writebehind(format, prefetch_frames, read_ahead_frames, 2)
}

fn residency_policy_with_writebehind(
    format: AdmittedPhysicalRecordFormat,
    prefetch_frames: u32,
    read_ahead_frames: u32,
    writebehind_frames: u32,
) -> AdmittedPhysicalRecordResidencyPolicy {
    let resident_bytes = 64 * 1024;
    let metadata_bytes = 16 * 1024;
    let operation_bytes = 4 * 1024 * 1024;
    PhysicalRecordResidencyPolicy::builder()
        .total_bytes(nonzero_bytes(
            operation_bytes + metadata_bytes + (2 * resident_bytes),
        ))
        .resident_bytes(nonzero_bytes(resident_bytes))
        .metadata_bytes(nonzero_bytes(metadata_bytes))
        .frame_entries(nonzero_count(8))
        .pinned_frames(nonzero_count(8))
        .pin_leases(nonzero_count(8))
        .dirty_frames(nonzero_count(2))
        .dirty_replacement_bytes(nonzero_bytes(resident_bytes))
        .operation_bytes(nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::ForegroundRead, nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::ForegroundWrite, nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::Recovery, nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::Scrub, nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::Maintenance, nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::Verification, nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::Blob, nonzero_bytes(operation_bytes))
        .speculative_frames(Speculation::Prefetch, nonzero_count(prefetch_frames))
        .speculative_frames(Speculation::ReadAhead, nonzero_count(read_ahead_frames))
        .speculative_frames(Speculation::WriteBehind, nonzero_count(writebehind_frames))
        .admit(format)
        .into_result()
        .unwrap()
}

pub(super) fn coordinate(ordinal: u64) -> RecordFrameCoordinate {
    RecordFrameCoordinate::new(
        RecordArtifactFile::BootstrapCatalog,
        ordinal * u64::from(FRAME_BYTES),
        FRAME_BYTES,
    )
    .unwrap()
}

pub(super) fn positioned_reads(serving: &ServingPhysicalRuntime) -> u64 {
    serving
        .media_counters()
        .attempts_for(MediaOperationRole::PositionedRead)
}

pub(super) fn causal_record(
    serving: &ServingPhysicalRuntime,
    identity: PhysicalWorkIdentity,
) -> PhysicalWorkCausalRecord {
    serving
        .physical_work_observer()
        .causal()
        .records()
        .iter()
        .copied()
        .find(|record| record.identity() == identity)
        .expect("reported speculative work identity must exist in the causal ledger")
}

fn nonzero_bytes(value: u64) -> std::num::NonZeroU64 {
    std::num::NonZeroU64::new(value).unwrap()
}

fn nonzero_count(value: u32) -> std::num::NonZeroU32 {
    std::num::NonZeroU32::new(value).unwrap()
}
