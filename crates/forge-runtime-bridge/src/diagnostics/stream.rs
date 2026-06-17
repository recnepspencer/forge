use crate::stream::{
    CanonicalStreamReplayRecord, ConsumerCheckpointToken, PlannedChangeStreamWindow,
    StreamProtocolCounters,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeStreamResumeSummary {
    checkpoint: ConsumerCheckpointToken,
    replay_record: CanonicalStreamReplayRecord,
    resumed_window: PlannedChangeStreamWindow,
    counters: StreamProtocolCounters,
}

impl BridgeStreamResumeSummary {
    pub(crate) fn new(
        checkpoint: ConsumerCheckpointToken,
        replay_record: CanonicalStreamReplayRecord,
        resumed_window: PlannedChangeStreamWindow,
    ) -> Self {
        let counters = resumed_window
            .counters()
            .clone()
            .with_resume_attempt(false, false);
        Self {
            checkpoint,
            replay_record,
            resumed_window,
            counters,
        }
    }

    pub fn checkpoint(&self) -> &ConsumerCheckpointToken {
        &self.checkpoint
    }

    pub fn replay_record(&self) -> &CanonicalStreamReplayRecord {
        &self.replay_record
    }

    pub fn resumed_window(&self) -> &PlannedChangeStreamWindow {
        &self.resumed_window
    }

    pub fn counters(&self) -> &StreamProtocolCounters {
        &self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeStreamCheckpointExplanation {
    checkpoint_token_identity: String,
    consumer_contract_identity: String,
    stream_protocol_identity: String,
    checkpoint_frontier_kind: crate::stream::StreamCheckpointFrontierKind,
    contiguous_acknowledged_through_position: String,
    contiguous_acknowledged_through_member_identity: String,
    acknowledged_member_set_digest: String,
    checkpoint_member_count: usize,
    source_retention_anchor: String,
}

impl BridgeStreamCheckpointExplanation {
    pub(crate) fn from_checkpoint(checkpoint: &ConsumerCheckpointToken) -> Self {
        Self {
            checkpoint_token_identity: checkpoint
                .checkpoint_token_identity_for_reporting()
                .to_owned(),
            consumer_contract_identity: checkpoint.consumer_contract_identity().as_str().to_owned(),
            stream_protocol_identity: checkpoint.stream_protocol_identity().as_str().to_owned(),
            checkpoint_frontier_kind: checkpoint.checkpoint_frontier_kind(),
            contiguous_acknowledged_through_position: checkpoint
                .contiguous_acknowledged_through_position()
                .to_owned(),
            contiguous_acknowledged_through_member_identity: checkpoint
                .contiguous_acknowledged_through_member_identity()
                .to_owned(),
            acknowledged_member_set_digest: checkpoint.acknowledged_member_set_digest().to_owned(),
            checkpoint_member_count: checkpoint.checkpoint_member_count(),
            source_retention_anchor: checkpoint.source_retention_anchor().to_owned(),
        }
    }

    pub fn checkpoint_token_identity(&self) -> &str {
        &self.checkpoint_token_identity
    }

    pub fn consumer_contract_identity(&self) -> &str {
        &self.consumer_contract_identity
    }

    pub fn stream_protocol_identity(&self) -> &str {
        &self.stream_protocol_identity
    }

    pub fn checkpoint_frontier_kind(&self) -> crate::stream::StreamCheckpointFrontierKind {
        self.checkpoint_frontier_kind
    }

    pub fn contiguous_acknowledged_through_position(&self) -> &str {
        &self.contiguous_acknowledged_through_position
    }

    pub fn acknowledged_member_set_digest(&self) -> &str {
        &self.acknowledged_member_set_digest
    }

    pub fn contiguous_acknowledged_through_member_identity(&self) -> &str {
        &self.contiguous_acknowledged_through_member_identity
    }

    pub fn checkpoint_member_count(&self) -> usize {
        self.checkpoint_member_count
    }

    pub fn source_retention_anchor(&self) -> &str {
        &self.source_retention_anchor
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeStreamReplayExplanation {
    replay_record_identity: String,
    consumer_contract_identity: String,
    stream_window_identity: String,
    checkpoint_token_identity: String,
    replay_basis_digest: String,
    protocol_semantics_version: String,
}

impl BridgeStreamReplayExplanation {
    pub(crate) fn from_replay_record(record: &CanonicalStreamReplayRecord) -> Self {
        Self {
            replay_record_identity: record.replay_record_identity().as_str().to_owned(),
            consumer_contract_identity: record.consumer_contract_identity().as_str().to_owned(),
            stream_window_identity: record.stream_window_identity().as_str().to_owned(),
            checkpoint_token_identity: record.checkpoint_token_identity_for_reporting().to_owned(),
            replay_basis_digest: record.replay_basis_digest().to_owned(),
            protocol_semantics_version: record.protocol_semantics_version().to_owned(),
        }
    }

    pub fn replay_record_identity(&self) -> &str {
        &self.replay_record_identity
    }

    pub fn consumer_contract_identity(&self) -> &str {
        &self.consumer_contract_identity
    }

    pub fn stream_window_identity(&self) -> &str {
        &self.stream_window_identity
    }

    pub fn checkpoint_token_identity(&self) -> &str {
        &self.checkpoint_token_identity
    }

    pub fn replay_basis_digest(&self) -> &str {
        &self.replay_basis_digest
    }

    pub fn protocol_semantics_version(&self) -> &str {
        &self.protocol_semantics_version
    }
}
