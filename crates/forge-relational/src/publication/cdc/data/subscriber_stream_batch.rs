use crate::diagnostics::data::RelationalDiagnosticArtifact;
use crate::history::data::CommitId;
use crate::publication::cdc::data::{SubscriberCheckpoint, SubscriberRecoveryDecision};
use crate::publication::patch::data::RelationalPatchRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscriberStreamBatch {
    pub patches: Vec<RelationalPatchRecord>,
    pub resumed_from: Option<SubscriberCheckpoint>,
    pub next_checkpoint: Option<SubscriberCheckpoint>,
    pub latest_available_checkpoint: Option<SubscriberCheckpoint>,
    pub recovery_decision: SubscriberRecoveryDecision,
    pub latest_commit_id: Option<CommitId>,
    pub diagnostics: Vec<RelationalDiagnosticArtifact>,
}
