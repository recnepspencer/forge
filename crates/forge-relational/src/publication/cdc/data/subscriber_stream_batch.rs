use crate::diagnostics::data::RelationalDiagnosticArtifact;
use crate::history::data::CommitId;
use crate::publication::cdc::data::{
    SubscriberCheckpoint, SubscriberContinuationSummary, SubscriberRecoveryDecision,
};
use crate::publication::patch::data::RelationalPatchRecord;
use crate::schema::data::{SchemaBoundaryFingerprint, SchemaContinuationClassification};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscriberStreamBatch {
    pub patches: Vec<RelationalPatchRecord>,
    pub resumed_from: Option<SubscriberCheckpoint>,
    pub next_checkpoint: Option<SubscriberCheckpoint>,
    pub latest_available_checkpoint: Option<SubscriberCheckpoint>,
    pub recovery_decision: SubscriberRecoveryDecision,
    pub latest_commit_id: Option<CommitId>,
    pub crossed_boundaries: Vec<SchemaBoundaryFingerprint>,
    pub continuation_outcome: SchemaContinuationClassification,
    pub continuation_summary: SubscriberContinuationSummary,
    pub contract_upgrade_applied: bool,
    pub diagnostics: Vec<RelationalDiagnosticArtifact>,
}
