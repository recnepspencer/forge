use super::checkpoints::{ConsumerCheckpointToken, StreamCheckpointFrontierKind};
use super::counters::StreamProtocolCounters;
use super::declaration::diagnostics_policy_class_label;
use super::protocol::AdmittedConsumerContract;
use super::replay::{canonicalize_stream_replay_record, CanonicalStreamReplayRecord};
use super::window::{PlannedChangeStreamWindow, StreamWindowIdentity};
use super::{StreamConsumerShape, StreamDeliveryIntent, StreamReplayMode};
use crate::error::{BridgeStreamError, BridgeStreamErrorKind};
use crate::routing::canonicalization::digest_string;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamReplayAuditSummary {
    stream_window_identity: StreamWindowIdentity,
    consumer_contract_identity: super::protocol::ConsumerContractIdentity,
    stream_digest: std::sync::Arc<str>,
    window_digest: std::sync::Arc<str>,
    consumer_contract_digest: std::sync::Arc<str>,
    diagnostics_digest: std::sync::Arc<str>,
    audited_member_count: usize,
    counters: StreamProtocolCounters,
}

impl StreamReplayAuditSummary {
    pub(crate) fn new(
        window: &PlannedChangeStreamWindow,
        checkpoint: &ConsumerCheckpointToken,
        replay_record: &CanonicalStreamReplayRecord,
    ) -> Self {
        Self {
            stream_window_identity: window.stream_window_identity().clone(),
            consumer_contract_identity: window.consumer_contract_identity().clone(),
            stream_digest: std::sync::Arc::from(window.member_set_digest()),
            window_digest: std::sync::Arc::from(window.digest()),
            consumer_contract_digest: std::sync::Arc::from(window.consumer_contract_identity().as_str()),
            diagnostics_digest: std::sync::Arc::from(digest_string(
                "stream-diagnostics-policy",
                diagnostics_policy_class_label(window.diagnostics_policy_class()).as_ref(),
            )),
            audited_member_count: window.members().len(),
            counters: replay_record
                .counters()
                .clone()
                .with_checkpoint(checkpoint.checkpoint_member_count()),
        }
    }

    pub fn stream_window_identity(&self) -> &StreamWindowIdentity {
        &self.stream_window_identity
    }

    pub fn consumer_contract_identity(&self) -> &super::protocol::ConsumerContractIdentity {
        &self.consumer_contract_identity
    }

    pub fn audited_member_count(&self) -> usize {
        self.audited_member_count
    }

    pub fn stream_digest(&self) -> &str {
        self.stream_digest.as_ref()
    }

    pub fn window_digest(&self) -> &str {
        self.window_digest.as_ref()
    }

    pub fn consumer_contract_digest(&self) -> &str {
        self.consumer_contract_digest.as_ref()
    }

    pub fn diagnostics_digest(&self) -> &str {
        self.diagnostics_digest.as_ref()
    }

    pub fn counters(&self) -> &StreamProtocolCounters {
        &self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamReplayAuditResult {
    summary: StreamReplayAuditSummary,
    checkpoint: ConsumerCheckpointToken,
    replay_record: CanonicalStreamReplayRecord,
}

impl StreamReplayAuditResult {
    pub(crate) fn new(
        summary: StreamReplayAuditSummary,
        checkpoint: ConsumerCheckpointToken,
        replay_record: CanonicalStreamReplayRecord,
    ) -> Self {
        Self {
            summary,
            checkpoint,
            replay_record,
        }
    }

    pub fn summary(&self) -> &StreamReplayAuditSummary {
        &self.summary
    }

    pub fn checkpoint(&self) -> &ConsumerCheckpointToken {
        &self.checkpoint
    }

    pub fn replay_record(&self) -> &CanonicalStreamReplayRecord {
        &self.replay_record
    }
}

pub(crate) fn audit_change_stream_window(
    contract: &AdmittedConsumerContract,
    window: &PlannedChangeStreamWindow,
) -> Result<StreamReplayAuditResult, BridgeStreamError> {
    if contract.consumer_shape() != StreamConsumerShape::ReplayAuditConsumer {
        return Err(BridgeStreamError::new(
            BridgeStreamErrorKind::UnsupportedConsumerShape,
            "Only replay-audit consumer contracts may execute the replay-audit stream path.",
        ));
    }
    if contract.admitted_delivery_intent() != StreamDeliveryIntent::ReplayAudit {
        return Err(BridgeStreamError::new(
            BridgeStreamErrorKind::StreamDeliveryRejected,
            "The admitted consumer contract does not allow replay-audit delivery for this stream path.",
        ));
    }
    if contract.admitted_replay_mode() != StreamReplayMode::Enabled {
        return Err(BridgeStreamError::new(
            BridgeStreamErrorKind::StreamDeliveryRejected,
            "Replay-audit delivery requires replay mode to be enabled on the admitted consumer contract.",
        ));
    }

    if contract.consumer_contract_identity() != window.consumer_contract_identity() {
        return Err(BridgeStreamError::new(
            BridgeStreamErrorKind::StreamDeliveryRejected,
            "The replay-audit consumer contract did not match the planned stream window contract identity.",
        ));
    }
    if window.lowered_change_set().is_none() {
        return Err(BridgeStreamError::new(
            BridgeStreamErrorKind::StreamDeliveryRejected,
            "The planned stream window was not lowered into an admitted replay-audit batch before execution.",
        ));
    }

    let checkpoint = ConsumerCheckpointToken::from_window(
        contract,
        window,
        StreamCheckpointFrontierKind::ContiguousFrontier,
    );
    let replay_record = canonicalize_stream_replay_record(contract, window, &checkpoint)?;
    let summary = StreamReplayAuditSummary::new(window, &checkpoint, &replay_record);

    Ok(StreamReplayAuditResult::new(summary, checkpoint, replay_record))
}
