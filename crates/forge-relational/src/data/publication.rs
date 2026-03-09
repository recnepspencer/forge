use serde::{Deserialize, Serialize};

use crate::data::diagnostics::RelationalDiagnosticArtifact;
use crate::data::diff::RelationalPatchRecord;
use crate::data::history::CommitReference;
use crate::data::snapshot::SnapshotHandle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicationStage {
    Apply,
    InvariantCheck,
    BundleAssembly,
    Visibility,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationError {
    pub stage: PublicationStage,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicationStatus {
    Published,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationBundle<ReplayRecord> {
    pub commit: CommitReference,
    pub snapshot: SnapshotHandle,
    pub diagnostics_summary: RelationalDiagnosticArtifact,
    pub patch: RelationalPatchRecord,
    pub replay: ReplayRecord,
    pub status: PublicationStatus,
}
