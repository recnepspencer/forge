use serde::{Deserialize, Serialize};

use crate::diagnostics::data::RelationalDiagnosticArtifact;
use crate::logic::runtime::RelationalReplayRecord;
use crate::publication::patch::data::RelationalPatchRecord;

use super::PublicationObservationSnapshot;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationArtifactSnapshot {
    pub observation: PublicationObservationSnapshot,
    pub latest_patch: Option<RelationalPatchRecord>,
    pub latest_replay: Option<RelationalReplayRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationDiagnosticsSnapshot {
    pub observation: PublicationObservationSnapshot,
    pub diagnostics: Vec<RelationalDiagnosticArtifact>,
}
