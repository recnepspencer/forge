use serde::{Deserialize, Serialize};

use crate::diagnostics::data::RelationalDiagnosticArtifact;
use crate::history::data::{BranchId, CommitId, CommitReference};
use crate::publication::data::diff::RelationalPatchRecord;
use crate::schema::data::{RelationalSchemaRegistry, SchemaVersionId};
use crate::transactions::data::MergedCommitPlan;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalCommitEnvelope {
    pub commit: CommitReference,
    pub branch_context: BranchId,
    pub merge_parent_branches: Vec<BranchId>,
    pub merge_base_commits: Vec<CommitId>,
    pub schema_version: SchemaVersionId,
    pub schema_registry: RelationalSchemaRegistry,
    pub merged_plan: MergedCommitPlan,
    pub patch: RelationalPatchRecord,
    pub diagnostics_summary: RelationalDiagnosticArtifact,
    pub lineage_event_ids: Vec<u64>,
    pub index_generation_ids: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplayObservableSurface {
    Snapshot,
    Patch,
    Diagnostics,
    History,
    BranchHead,
    Lineage,
    DerivedIndexes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplayExecutionMode {
    SerialDeterministic,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplayFailureClass {
    MissingCommit,
    MissingParentChain,
    BranchMismatch,
    SchemaMismatch,
    UnsupportedReplaySchema,
    ObservableMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayMismatch {
    pub surface: ReplayObservableSurface,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalReplayRequest {
    pub commit_id: CommitId,
    pub branch_id: BranchId,
    pub execution_mode: ReplayExecutionMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalReplayOutcome {
    pub requested: RelationalReplayRequest,
    pub commit: Option<CommitReference>,
    pub reconstructed_parent_chain: Vec<CommitId>,
    pub snapshot_version: Option<crate::identity::data::VersionId>,
    pub compared_surfaces: Vec<ReplayObservableSurface>,
    pub mismatches: Vec<ReplayMismatch>,
    pub failure: Option<ReplayFailureClass>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplaySchemaVersion(pub u32);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalReplayRecord {
    pub schema_version: ReplaySchemaVersion,
    pub commit_id: crate::history::data::CommitId,
    pub version_id: crate::identity::data::VersionId,
    pub snapshot_id: crate::snapshots::data::SnapshotId,
    pub patch: crate::publication::data::diff::RelationalPatchRecord,
    pub schema_registry: RelationalSchemaRegistry,
}
