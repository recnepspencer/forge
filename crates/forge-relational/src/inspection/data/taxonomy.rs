use serde::{Deserialize, Serialize};

use crate::identity::data::VersionId;
use crate::snapshots::data::SnapshotHandle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InspectionOrigin {
    CurrentTruth,
    VisibilitySnapshot,
    CanonicalCommitStorage,
    LineageGraph,
    RetentionState,
    TransactionStaging,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InspectionAccessPath {
    DirectLookup,
    SnapshotRead,
    VersionRead,
    HistoricalRetainedRead,
    HistoricalReconstructedRead,
    CommitIndexRead,
    GraphTraversal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InspectionResolutionContext {
    NoContext,
    BranchAncestry,
    LineageTraversal,
    RelationNeighborhood,
    ConnectivityTraversal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InspectionAvailability {
    Direct,
    Reconstructed,
    UnavailableByBudget,
    UnavailableByRetention,
    UnavailableByPolicy,
    UnavailableByMissingCanonicalArtifacts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InspectionDegradation {
    MissingStructuralFingerprint,
    MissingLineageIdentity,
    SummaryOnly,
    WorkBudgetExceeded,
    EntityBudgetExceeded,
    RelationBudgetExceeded,
    FrontierBudgetExceeded,
    ComponentBudgetExceeded,
    EntitySlotBudgetExceeded,
    RelationSlotBudgetExceeded,
    ReconstructionOmittedByMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InspectionRecordClass {
    Entity,
    Relation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InspectionScope {
    Current,
    Version(VersionId),
    Snapshot(SnapshotHandle),
}
