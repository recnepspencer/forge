use crate::snapshots::data::{SnapshotHandle, SnapshotId};

mod invariants;
mod session;
mod snapshots;
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
    InvariantFailureEffect, InvariantRule, InvariantViolation, StorageInvariantReport,
};

pub(crate) use crate::storage::logic::state::{PartitionAccess, WorkingState};
pub(crate) use state::{
    DurabilitySubsystem, HistorySubsystem, IndexingSubsystem, LineageSubsystem,
    PublicationSubsystem, ReplayRetentionState, RuntimeInstrumentation, RuntimeServices,
    RuntimeSubsystem, SnapshotHandleBinding, VisibilityResidency, VisibilitySubsystem,
};
pub use state::RelationalRuntime;
#[allow(unused_imports)]
pub use snapshots::SnapshotAccess;

#[derive(Debug, Clone)]
pub struct SnapshotGuard {
    handle: SnapshotHandle,
}

impl SnapshotGuard {
    pub fn handle(&self) -> &SnapshotHandle {
        &self.handle
    }

    pub fn snapshot_id(&self) -> SnapshotId {
        self.handle.snapshot_id
    }
}
