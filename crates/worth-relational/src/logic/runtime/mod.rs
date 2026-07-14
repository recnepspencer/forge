use crate::snapshots::data::{SnapshotHandle, SnapshotId};

mod configuration;
mod construction;
mod guided;
mod state;
mod transactions;

pub use crate::config::data::RelationalRuntimeConfig;
pub use crate::durability::data::RecoveryOutcome;
pub use crate::performance::data::{
    ComplexityContract, ComplexityStatus, RuntimeComplexityCounters,
};
pub use crate::replay::data::{RelationalReplayRecord, ReplaySchemaVersion};
pub use crate::simulation::data::{
    CompiledArtifactAuthorityStatus, CompiledArtifactError, CompiledExecutionArtifact,
    TopologyFreezeMode,
};
pub use crate::simulation::logic::{SimulationAccess, SimulationAuthority};
#[cfg(test)]
pub use crate::validation::engine::HarnessAuditMode;
pub use crate::validation::logic::InvariantAccess;
pub use crate::visibility::materialization::read_records::{
    EntityProjectionRecord, EntityRecordProjection, RelationProjectionRecord,
    RelationRecordProjection, VisibilityProjectionView, VisibilityReadContext,
};
pub use crate::visibility::retention::VisibilityRetentionAuthority;

pub(crate) use crate::storage::logic::state::{PartitionAccess, WorkingState};
pub(crate) use construction::RuntimeExtensions;
pub use state::RelationalRuntime;
pub(crate) use state::{
    CommitStrategiesSubsystem, DurabilitySubsystem, HistorySubsystem, IndexingSubsystem,
    LineageSubsystem, PublicationSubsystem, ReplayRetentionState, RuntimeInstrumentation,
    RuntimeServices, RuntimeSubsystem, SchemaContractRuntimeSubsystem, SnapshotHandleBinding,
    VisibilityResidency, VisibilitySubsystem,
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
