use forge_harness::facade::{ExecutionProfile, ExecutionRequest, MutationBatch, ScenarioPlan};
use forge_harness::runtime::HarnessAdapter;

use crate::facade::{
    BridgeBulkWorkloadRequest, BridgeBulkWorkloadSegment, BridgeDeliveryErrorKind,
    BridgeFailureClass, BridgePreparationMode, BridgeRouteRequest, RuntimeBridgeBuilder,
    TruthSnapshotIdentity,
};
use crate::routing::BridgeParallelAdmissionClass;

use super::support::{
    build_runtime, build_runtime_with_aspects, committed_patch, field_aspect_registration,
    registration, snapshot, CountingSnapshotReaderPool,
};
use crate::harness::adapter::{BridgeHarnessAdapter, BridgeHarnessMutation, BridgeHarnessTargetId};
use crate::harness::fixtures::{
    BridgeHarnessFixture, InMemoryRelationalBridgeSource, RecordingSignalBridgeSink,
    SnapshotFixture,
};

mod bulk;
mod snapshot;
mod snapshot_slice_diagnostics;
