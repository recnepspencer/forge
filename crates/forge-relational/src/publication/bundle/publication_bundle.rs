use crate::diagnostics::data::RelationalDiagnosticArtifact;
use crate::history::data::CommitReference;
use crate::publication::bundle::publication_status::PublicationStatus;
use crate::publication::patch::data::RelationalPatchRecord;
use crate::snapshots::data::SnapshotHandle;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationBundle<ReplayRecord> {
    pub commit: CommitReference,
    pub snapshot: SnapshotHandle,
    pub diagnostics_summary: RelationalDiagnosticArtifact,
    pub patch: RelationalPatchRecord,
    pub replay: ReplayRecord,
    pub status: PublicationStatus,
}
