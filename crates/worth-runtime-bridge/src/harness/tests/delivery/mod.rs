use crate::facade::{
    BridgeBulkWorkloadRequest, BridgeBulkWorkloadSegment, BridgeDeliveryErrorKind,
    BridgePreparationMode, BridgeRouteRequest, TruthSnapshotIdentity,
};
use crate::routing::BridgeParallelAdmissionClass;

use super::support::{
    build_runtime, build_runtime_with_aspects, committed_patch, field_aspect_registration,
    registration, snapshot, CountingSnapshotReaderPool,
};
use crate::harness::fixtures::{InMemoryRelationalBridgeSource, RecordingSignalBridgeSink};
use crate::truth_identity_fixtures::{truth_commit, truth_patch, truth_snapshot};

fn commit_a() -> crate::facade::TruthCommitIdentity {
    truth_commit(1)
}

fn commit_b() -> crate::facade::TruthCommitIdentity {
    truth_commit(2)
}

fn commit_c() -> crate::facade::TruthCommitIdentity {
    truth_commit(3)
}

fn patch_a() -> crate::facade::TruthPatchIdentity {
    truth_patch(1)
}

fn patch_b() -> crate::facade::TruthPatchIdentity {
    truth_patch(2)
}

fn patch_c() -> crate::facade::TruthPatchIdentity {
    truth_patch(3)
}

fn snapshot_a() -> TruthSnapshotIdentity {
    truth_snapshot(1, 1)
}

fn snapshot_b() -> TruthSnapshotIdentity {
    truth_snapshot(2, 1)
}

fn snapshot_c() -> TruthSnapshotIdentity {
    truth_snapshot(3, 1)
}

fn mismatched_snapshot() -> TruthSnapshotIdentity {
    truth_snapshot(99, 1)
}

mod bulk;
mod snapshot;
mod snapshot_slice_diagnostics;
