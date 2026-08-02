use super::{export_block, read_repository_document};

const DURABILITY_EXPORTS: [&str; 37] = [
    "CompletedPhysicalCheckpoint",
    "ContiguousRetainedWalTail",
    "IndeterminatePhysicalCheckpoint",
    "PhysicalCheckpointCancellationOutcome",
    "PhysicalBindingCompactionReopenFailure",
    "PhysicalCheckpointCaptureBasis",
    "PhysicalCheckpointCaptureFailureKind",
    "PhysicalCheckpointDeadline",
    "PhysicalCheckpointDisposal",
    "PhysicalCheckpointHandle",
    "PhysicalCheckpointIdempotencyKey",
    "PhysicalCheckpointOutcome",
    "PhysicalCheckpointPoll",
    "PhysicalCheckpointProgress",
    "PhysicalCheckpointProgressPhase",
    "PhysicalCheckpointProvenNoEffectCause",
    "PhysicalCheckpointRequest",
    "PhysicalCheckpointShutdown",
    "PhysicalCheckpointStartDeferred",
    "PhysicalCheckpointStartDenial",
    "PhysicalCheckpointStartFailure",
    "PhysicalCheckpointStartOutcome",
    "PhysicalCheckpointStartRebindRequired",
    "PhysicalCheckpointStartStale",
    "PhysicalCheckpointSubmission",
    "PhysicalDurabilityReopenObservation",
    "PhysicalIdempotencyReopenFailure",
    "LiveIdempotencyBindingLimit",
    "PhysicalWalFrameWriteDisposition",
    "PhysicalWalOpenFailure",
    "PhysicalWalPolicy",
    "PhysicalWalReclamationObservation",
    "PhysicalWalReclamationReport",
    "WalSegmentByteLimit",
    "WalSegmentInventoryLimit",
    "ProvenNoEffectPhysicalCheckpoint",
    "RetainedWalSegment",
];

const WORK_EXPORTS: [&str; 3] = [
    "CompletedPhysicalCheckpointAction",
    "CompletedPhysicalWalReclamationAction",
    "PhysicalCheckpointRecoveryAction",
];

const WAL_EXPORTS: [&str; 6] = [
    "WalSegmentArtifactIdentity",
    "WalSegmentInspection",
    "inspect_complete_wal_segment",
    "VerifiedWalFramePayload",
    "VerifiedWalSegment",
    "inspect_verified_wal_segment",
];

const BUFFER_POOL_EXPORTS: [&str; 7] = [
    "MaintenanceAllocationGrant",
    "PhysicalDirtyFrameBasis",
    "PhysicalDirtyGeneration",
    "PhysicalDirtyGenerationCaptureCompletion",
    "PhysicalDirtyGenerationCaptureSession",
    "PhysicalDirtyGenerationCaptureStep",
    "PhysicalDirtyGenerationSlice",
];

const PHYSICAL_FORMAT_EXPORTS: [&str; 12] = [
    "CheckpointBindingRecordFrameLength",
    "CheckpointDirtyFrameBasis",
    "CheckpointRootBasis",
    "CheckpointStreamDecodeDenial",
    "CheckpointStreamDecoder",
    "CheckpointStreamEncoder",
    "CheckpointStreamFooter",
    "CheckpointWalSourceRange",
    "PhysicalCheckpointIdentity",
    "PhysicalCheckpointSource",
    "CHECKPOINT_BINDING_RECORD_PREFIX_BYTES",
    "CHECKPOINT_DIRTY_FRAME_RECORD_BYTES",
];

pub(super) fn assert_reachability(durability_exports: &str) {
    for surface in DURABILITY_EXPORTS {
        assert!(
            durability_exports.contains(surface),
            "Phase 6 Store surface `{surface}` is not exported by physical_runtime"
        );
    }
    let runtime = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/mod.rs",
    )
    .expect("read physical runtime facade");
    assert!(runtime.contains("PhysicalDurabilityStateReopenFailure"));
    let work_exports = export_block(&runtime, "pub use work::{");
    for surface in WORK_EXPORTS {
        assert!(
            work_exports.contains(surface),
            "Phase 6 work evidence `{surface}` is not exported by physical_runtime"
        );
    }
    let wal = read_repository_document("workspaces/worth-store/crates/worth-store-wal/src/lib.rs")
        .expect("read WAL facade");
    let artifact_exports = export_block(&wal, "pub use artifact_store::{");
    for surface in WAL_EXPORTS {
        assert!(
            artifact_exports.contains(surface),
            "Phase 6 WAL surface `{surface}` is not exported by worth-store-wal"
        );
    }
    let buffer_pool = read_repository_document(
        "workspaces/worth-store/crates/worth-store-buffer-pool/src/lib.rs",
    )
    .expect("read buffer-pool facade");
    for surface in BUFFER_POOL_EXPORTS {
        assert!(
            buffer_pool.contains(surface),
            "Phase 6 buffer-pool surface `{surface}` is not exported by its crate facade"
        );
    }
    let physical_format = read_repository_document(
        "workspaces/worth-store/crates/worth-store-physical-format/src/lib.rs",
    )
    .expect("read physical-format facade");
    let checkpoint_exports = export_block(&physical_format, "pub use checkpoint::{");
    for surface in PHYSICAL_FORMAT_EXPORTS {
        assert!(
            checkpoint_exports.contains(surface),
            "Phase 6 checkpoint surface `{surface}` is not exported by worth-store-physical-format"
        );
    }
    let capture = read_repository_document(
        "workspaces/worth-store/crates/worth-store-buffer-pool/src/physical_residency/pool/dirty_generation_capture.rs",
    )
    .expect("read dirty-generation capture API");
    for method in [
        "pub fn begin_dirty_generation_capture(",
        "pub fn capture_next_dirty_generation_slice(",
    ] {
        assert!(
            capture.contains(method),
            "Phase 6 capture API lost `{method}`"
        );
    }
    let retained_tail = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/checkpoint/retained_wal_tail.rs",
    )
    .expect("read retained WAL-tail facade");
    for method in [
        "pub const fn checkpoint_identity(",
        "pub const fn checkpoint_source(",
        "pub const fn checkpoint_boundary_lsn(",
        "pub const fn durable_tail_end_lsn_exclusive(",
        "pub const fn retained_physical_bytes(",
        "pub const fn segment_count(",
        "pub fn segments(",
        "pub const fn artifact(",
        "pub const fn observed_lsn_range(",
        "pub const fn physical_bytes(",
    ] {
        assert!(
            retained_tail.contains(method),
            "Phase 6 retained-tail facade lost `{method}`"
        );
    }
    let checkpoint_outcome = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/checkpoint/outcome.rs",
    )
    .expect("read completed checkpoint facade");
    assert!(
        checkpoint_outcome.contains("pub fn retained_wal_tail("),
        "completed checkpoint no longer exposes its retained-tail authority"
    );
    assert!(
        checkpoint_outcome.contains("pub const fn wal_reclamation("),
        "completed checkpoint no longer exposes reclamation fate"
    );
    let wal_observation = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/wal/observation.rs",
    )
    .expect("read WAL observation facade");
    for method in [
        "pub const fn reclaimed_segments(",
        "pub const fn reclaimed_bytes(",
    ] {
        assert!(
            wal_observation.contains(method),
            "Phase 6 WAL observation lost reclamation counter `{method}`"
        );
    }
    let serving = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/lifecycle/serving_runtime.rs",
    )
    .expect("read serving runtime facade");
    assert!(
        serving.contains("pub fn checkpoints("),
        "Phase 6 serving facade lost checkpoint initiation"
    );
    let checkpoint_submission = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/checkpoint/runtime_owner.rs",
    )
    .expect("read checkpoint submission facade");
    assert!(
        checkpoint_submission.contains("pub fn start("),
        "Phase 6 checkpoint submission lost typed start"
    );
    let checkpoint_handle = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/checkpoint/handle.rs",
    )
    .expect("read checkpoint handle facade");
    for method in [
        "pub fn identity(",
        "pub fn source(",
        "pub fn deadline(",
        "pub fn progress(",
        "pub fn poll(",
        "pub fn request_cancellation(",
        "pub fn wait(",
        "pub fn dispose(",
    ] {
        assert!(
            checkpoint_handle.contains(method),
            "Phase 6 checkpoint handle lost `{method}`"
        );
    }
    let checkpoint_progress = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/checkpoint/progress.rs",
    )
    .expect("read checkpoint progress facade");
    for method in [
        "pub const fn current_capture_bytes(",
        "pub const fn peak_capture_bytes(",
    ] {
        assert!(
            checkpoint_progress.contains(method),
            "Phase 6 checkpoint progress lost bounded-resource observation `{method}`"
        );
    }
    let scheduler = read_repository_document(
        "workspaces/worth-store/crates/worth-store-io-scheduler/src/lib.rs",
    )
    .expect("read scheduler facade");
    let background_exports = export_block(&scheduler, "pub use background_pacing::{");
    assert!(
        background_exports.contains("BackgroundIoPressureShape"),
        "Phase 6 checkpoint pressure shape is not exported by the scheduler facade"
    );
    let checkpoint_pressure = read_repository_document(
        "workspaces/worth-store/crates/worth-store-io-scheduler/src/background_pacing/shape.rs",
    )
    .expect("read checkpoint pressure shape");
    assert!(
        checkpoint_pressure.contains("pub const fn filesystem_admitted_checkpoint_flush("),
        "Phase 6 scheduler facade lost filesystem-admitted checkpoint pressure"
    );
}
