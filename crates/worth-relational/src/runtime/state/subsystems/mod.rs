mod commit_strategies;
mod durability;
mod history;
mod indexing;
mod lineage;
mod publication;
mod record_identity;
mod schema_contract_runtime;
mod services;
mod visibility;

pub(crate) trait RuntimeSubsystem: Sized {
    type Config;

    fn new(config: &Self::Config) -> Self;
    fn fork(&self) -> Self;
}

pub(crate) use commit_strategies::CommitStrategiesSubsystem;
pub(crate) use durability::DurabilitySubsystem;
pub use history::RelationalPatchPositionReservationCounters;
pub(crate) use history::{
    readmit_positioned_canonical_commit, BranchHeadVersionIndexAuthority,
    CanonicalCheckpointAdmissionError, CanonicalPositionAdmission, CanonicalPublicationRecordError,
    HistorySubsystem, PerformedCheckpointSelection, PreparedCanonicalPublicationRoute,
    PreparedRecoveredVersionedArtifactPublication, PreparedVersionedArtifactAccelerators,
    PreparedVersionedArtifactPublication, RelationalCanonicalPublicationRoutes,
    RelationalForkMaterializationCost,
};
pub use history::{RelationalBranchSharingCostCounters, RelationalPhase4ReferenceCostCounters};
pub(crate) use indexing::IndexingSubsystem;
pub(crate) use lineage::{LineageSubsystem, ValidatedLineageEventBatch};
pub(crate) use publication::PublicationSubsystem;
pub(crate) use record_identity::{
    PendingRecordAllocations, ReclaimedRecordSlot, RecordIdentitySubsystem,
};
pub(crate) use schema_contract_runtime::SchemaContractRuntimeSubsystem;
pub(crate) use services::{RuntimeInstrumentation, RuntimeServices};
pub(crate) use visibility::{
    PublishedSnapshotCloseout, PublishedSnapshotSlotReservation, ReplayRetentionState,
    SnapshotHandleBinding, VisibilityResidency, VisibilitySubsystem,
};
