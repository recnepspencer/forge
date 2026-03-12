pub mod diff;
mod publication_error;

#[allow(unused_imports)]
pub use self::diff::*;
pub use publication_error::*;

use serde::{Deserialize, Serialize};

use self::diff::RelationalPatchRecord;
use crate::diagnostics::data::RelationalDiagnosticArtifact;
use crate::history::data::CommitReference;
use crate::snapshots::data::SnapshotHandle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicationStage {
    Apply,
    InvariantCheck,
    BundleAssembly,
    Visibility,
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
