use crate::diagnostics::history::{
    BridgeHistoricalEvaluationDecisionLogIdentity, BridgeHistoricalEvaluationRecord,
    BridgeHistoricalEvaluationRecordIdentity,
};
use crate::snapshot::TruthSnapshotIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeHistoricalEvaluationReplaySummary {
    record_identity: BridgeHistoricalEvaluationRecordIdentity,
    decision_log_identity: BridgeHistoricalEvaluationDecisionLogIdentity,
    snapshot_identity: TruthSnapshotIdentity,
}

impl BridgeHistoricalEvaluationReplaySummary {
    pub(crate) fn from_record(record: &BridgeHistoricalEvaluationRecord) -> Self {
        Self {
            record_identity: record.record_identity().clone(),
            decision_log_identity: record.decision_log().decision_log_identity().clone(),
            snapshot_identity: record.decision_log().snapshot_identity().clone(),
        }
    }

    pub fn record_identity(&self) -> &BridgeHistoricalEvaluationRecordIdentity {
        &self.record_identity
    }

    pub fn decision_log_identity(&self) -> &BridgeHistoricalEvaluationDecisionLogIdentity {
        &self.decision_log_identity
    }

    pub fn snapshot_identity(&self) -> &TruthSnapshotIdentity {
        &self.snapshot_identity
    }
}
