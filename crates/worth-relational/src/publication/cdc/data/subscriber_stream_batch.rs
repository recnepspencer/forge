use crate::diagnostics::data::RelationalDiagnosticArtifact;
use crate::history::data::CommitId;
use crate::publication::cdc::data::{
    SubscriberCheckpoint, SubscriberContinuationAssessment, SubscriberRecoveryDecision,
};
use crate::publication::patch::data::PublishedAuthoritativePatchEnvelope;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscriberStreamBatch {
    pub patches: Vec<PublishedAuthoritativePatchEnvelope>,
    pub resumed_from: Option<SubscriberCheckpoint>,
    pub next_checkpoint: Option<SubscriberCheckpoint>,
    pub latest_available_checkpoint: Option<SubscriberCheckpoint>,
    pub recovery_decision: SubscriberRecoveryDecision,
    pub latest_commit_id: Option<CommitId>,
    pub continuation: SubscriberContinuationAssessment,
    pub diagnostics: Vec<RelationalDiagnosticArtifact>,
}
