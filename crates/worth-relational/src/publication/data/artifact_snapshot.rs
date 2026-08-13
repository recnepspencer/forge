use serde::{Deserialize, Serialize};

use crate::diagnostics::data::RelationalDiagnosticArtifact;
use crate::publication::patch::data::PublishedAuthoritativePatchEnvelope;
use crate::runtime::RelationalReplayRecord;

use super::PublicationObservationSnapshot;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationArtifactSnapshot {
    pub observation: PublicationObservationSnapshot,
    pub latest_patch: Option<PublishedAuthoritativePatchEnvelope>,
    pub latest_replay: Option<RelationalReplayRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationDiagnosticsSnapshot {
    pub observation: PublicationObservationSnapshot,
    pub diagnostics: Vec<RelationalDiagnosticArtifact>,
}
