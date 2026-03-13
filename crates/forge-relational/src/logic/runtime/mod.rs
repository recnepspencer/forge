use crate::snapshots::data::{SnapshotHandle, SnapshotId};

mod session;
mod state;

pub use crate::config::data::RelationalRuntimeConfig;
pub use crate::durability::data::RecoveryOutcome;
#[allow(unused_imports)]
pub use crate::performance::data::{
    ComplexityContract, ComplexityStatus, RuntimeComplexityCounters, COMPLEXITY_CONTRACTS,
};
pub use crate::replay::data::{RelationalReplayRecord, ReplaySchemaVersion};
pub use crate::simulation::data::{
    CompiledArtifactCompatibility, CompiledArtifactError, CompiledExecutionArtifact,
    TopologyFreezeMode,
};
#[allow(unused_imports)]
pub use crate::storage::data::{
    ChunkDiagnostics, ChunkVisibilitySummary, ChunkedStorageSummary, EntityReadRecord,
    IndexedReadOutcome, PacketResult, PartitionStorageStats, RecordLifecycleState,
    RelationReadRecord, RelationalReadView, RetentionPassOutcome, RetentionPlan, StorageStats,
};
#[allow(unused_imports)]
pub use crate::validation::data::{
    InvariantCatalog, InvariantCheckResult, InvariantClass, InvariantExecutionPoint,
    InvariantFailureEffect, InvariantRegistration, InvariantRule, InvariantViolation,
};
pub use crate::validation::engine::HarnessAuditMode;
pub use crate::visibility::materialization::read_records::{
    EntityRecordProjection, ProjectionAspect, RelationRecordProjection, VisibilityProjectionView,
};

pub(crate) use crate::storage::logic::state::{PartitionAccess, WorkingState};
pub use state::RelationalRuntime;
pub(crate) use state::{
    DurabilitySubsystem, HistorySubsystem, IndexingSubsystem, LineageSubsystem,
    PublicationSubsystem, ReplayRetentionState, RuntimeInstrumentation, RuntimeServices,
    RuntimeSubsystem, SnapshotHandleBinding, VisibilityResidency, VisibilitySubsystem,
};

#[derive(Debug, Clone)]
pub struct SnapshotGuard {
    handle: SnapshotHandle,
}

impl SnapshotGuard {
    pub(crate) fn new(handle: SnapshotHandle) -> Self {
        Self { handle }
    }

    pub fn handle(&self) -> &SnapshotHandle {
        &self.handle
    }

    pub fn snapshot_id(&self) -> SnapshotId {
        self.handle.snapshot_id
    }
}
