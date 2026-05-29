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
    CompiledArtifactCompatibility, CompiledArtifactError, CompiledExecutionArtifact,
    TopologyFreezeMode,
};
pub use crate::simulation::logic::{SimulationAccess, SimulationAuthority};
#[allow(unused_imports)]
pub use crate::storage::data::{
    ChunkDiagnostics, ChunkVisibilitySummary, ChunkedStorageSummary, EntityReadRecord,
    PartitionStorageStats, RecordLifecycleState, RelationReadRecord, RelationalReadView,
    RetentionPassOutcome, RetentionPlan, StorageStats,
};
#[allow(unused_imports)]
pub use crate::validation::data::{
    BoundedStructuralTraversal, CustomInvariantDescriptor, CustomInvariantExecutionContext,
    CustomInvariantExecutionError, CustomInvariantOperationalMetadata,
    CustomInvariantPreparationError, CustomInvariantProvenance, CustomInvariantRegistration,
    CustomInvariantRegistrationError, CustomInvariantRule, CustomInvariantRuleId,
    CustomInvariantScopePlanner, CustomInvariantSemanticIdentity, CustomInvariantSemanticVersion,
    CustomInvariantTouchedSummary, CustomInvariantTraversalError, CustomInvariantTraversalSummary,
    CustomInvariantVerdict, InvariantCatalog, InvariantCheckResult, InvariantClass,
    InvariantDecisionKind, InvariantDecisionRecord, InvariantExecutionPoint,
    InvariantFailureEffect, InvariantRegistration, InvariantRule, InvariantRuleDescriptor,
    InvariantRuleId, InvariantSemanticsClass, InvariantViolation, NativeInvariantRuleId,
    PlannedEntityCreate, PlannedRelationCreate, PlannedRelationEndpointUpdate,
    StructuralAspectStateView, StructuralCountView, StructuralRelationRecord,
    StructuralRelationView, StructuralTraversalResult, SupportedExecutionPoints,
    TouchedStructuralSet, UniqueEntityAspectField,
};
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
    AspectSemanticsSubsystem, CommitStrategiesSubsystem, DurabilitySubsystem, HistorySubsystem,
    IndexingSubsystem, LineageSubsystem, PublicationSubsystem, ReplayRetentionState,
    RuntimeInstrumentation, RuntimeServices, RuntimeSubsystem, SnapshotHandleBinding,
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
