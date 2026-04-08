use std::sync::Arc;

use crate::error::{BridgeStreamError, BridgeStreamErrorKind};
use crate::identity::{BridgeIdentity, CheckpointTokenIdentityTag};
use crate::routing::canonicalization::digest_string;

use super::position::CanonicalStreamPosition;
use super::protocol::{AdmittedConsumerContract, ConsumerContractIdentity, StreamProtocolIdentity};
use super::counters::StreamProtocolCounters;
use super::window::PlannedChangeStreamWindow;

type CheckpointTokenIdentity = BridgeIdentity<CheckpointTokenIdentityTag>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StreamCheckpointFrontierKind {
    ContiguousFrontier,
    ContiguousFrontierWithObservedDuplicates,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsumerCheckpointToken {
    checkpoint_token_identity: CheckpointTokenIdentity,
    consumer_contract_identity: ConsumerContractIdentity,
    stream_protocol_identity: StreamProtocolIdentity,
    checkpoint_frontier_kind: StreamCheckpointFrontierKind,
    contiguous_acknowledged_through_position: Arc<str>,
    contiguous_acknowledged_through_member_identity: Arc<str>,
    acknowledged_member_set_digest: Arc<str>,
    checkpoint_member_count: usize,
    source_retention_anchor: Arc<str>,
    protocol_semantics_version: Arc<str>,
    counters: StreamProtocolCounters,
}

impl ConsumerCheckpointToken {
    pub(crate) fn from_window(
        contract: &AdmittedConsumerContract,
        window: &PlannedChangeStreamWindow,
        checkpoint_frontier_kind: StreamCheckpointFrontierKind,
    ) -> Self {
        let checkpoint_member_count = window.last_stream_position().ordinal_position() + 1;
        let source_retention_anchor = retention_anchor_for_position(window.last_stream_position());
        let basis = format!(
            "consumer-checkpoint-token|contract={}|protocol={}|frontier-kind={}|through-position={}|member-set-digest={}|retention-anchor={}|protocol-semantics-version=forge-runtime-bridge.stream.v1",
            contract.consumer_contract_identity().as_str(),
            contract.stream_protocol_identity().as_str(),
            checkpoint_frontier_kind_label(checkpoint_frontier_kind),
            window.last_stream_position().stream_position_identity(),
            window.member_set_digest(),
            source_retention_anchor.as_ref(),
        );
        let digest = digest_string("consumer-checkpoint-token", &basis);
        Self {
            checkpoint_token_identity: CheckpointTokenIdentity::new(digest),
            consumer_contract_identity: contract.consumer_contract_identity().clone(),
            stream_protocol_identity: contract.stream_protocol_identity().clone(),
            checkpoint_frontier_kind,
            contiguous_acknowledged_through_position: Arc::from(
                window.last_stream_position().stream_position_identity(),
            ),
            contiguous_acknowledged_through_member_identity: Arc::from(
                window
                    .last_stream_position()
                    .canonical_stream_member_identity(),
            ),
            acknowledged_member_set_digest: Arc::from(window.member_set_digest()),
            checkpoint_member_count,
            source_retention_anchor,
            protocol_semantics_version: Arc::from("forge-runtime-bridge.stream.v1"),
            counters: window.counters().clone().with_checkpoint(checkpoint_member_count),
        }
    }

    pub fn checkpoint_token_identity(&self) -> &str {
        self.checkpoint_token_identity.as_str()
    }

    pub fn consumer_contract_identity(&self) -> &ConsumerContractIdentity {
        &self.consumer_contract_identity
    }

    pub fn stream_protocol_identity(&self) -> &StreamProtocolIdentity {
        &self.stream_protocol_identity
    }

    pub fn checkpoint_frontier_kind(&self) -> StreamCheckpointFrontierKind {
        self.checkpoint_frontier_kind
    }

    pub fn contiguous_acknowledged_through_position(&self) -> &str {
        self.contiguous_acknowledged_through_position.as_ref()
    }

    pub fn acknowledged_member_set_digest(&self) -> &str {
        self.acknowledged_member_set_digest.as_ref()
    }

    pub fn contiguous_acknowledged_through_member_identity(&self) -> &str {
        self.contiguous_acknowledged_through_member_identity.as_ref()
    }

    pub fn checkpoint_member_count(&self) -> usize {
        self.checkpoint_member_count
    }

    pub fn source_retention_anchor(&self) -> &str {
        self.source_retention_anchor.as_ref()
    }

    pub fn protocol_semantics_version(&self) -> &str {
        self.protocol_semantics_version.as_ref()
    }

    pub fn counters(&self) -> &StreamProtocolCounters {
        &self.counters
    }
}

pub(crate) fn validate_checkpoint_for_window(
    contract: &AdmittedConsumerContract,
    window: &PlannedChangeStreamWindow,
    checkpoint: &ConsumerCheckpointToken,
) -> Result<(), BridgeStreamError> {
    if checkpoint.consumer_contract_identity() != contract.consumer_contract_identity() {
        return Err(BridgeStreamError::new(
            BridgeStreamErrorKind::CheckpointContractMismatch,
            "The checkpoint token was issued for a different consumer contract identity.",
        ));
    }

    if checkpoint.stream_protocol_identity() != contract.stream_protocol_identity() {
        return Err(BridgeStreamError::new(
            BridgeStreamErrorKind::ProtocolVersionMismatch,
            "The checkpoint token was issued under a different stream protocol identity.",
        ));
    }

    let expected_anchor = retention_anchor_for_position(window.last_stream_position());
    if checkpoint.contiguous_acknowledged_through_position()
        != window.last_stream_position().stream_position_identity()
        || checkpoint.acknowledged_member_set_digest() != window.member_set_digest()
        || checkpoint.source_retention_anchor() != expected_anchor.as_ref()
    {
        return Err(BridgeStreamError::new(
            BridgeStreamErrorKind::CheckpointStreamMismatch,
            "The checkpoint token did not match the canonical stream window basis required for replay or resume.",
        ));
    }

    Ok(())
}

pub(crate) fn checkpoint_frontier_kind_label(value: StreamCheckpointFrontierKind) -> &'static str {
    match value {
        StreamCheckpointFrontierKind::ContiguousFrontier => "contiguous-frontier",
        StreamCheckpointFrontierKind::ContiguousFrontierWithObservedDuplicates => {
            "contiguous-frontier-with-observed-duplicates"
        }
    }
}

pub(crate) fn retention_anchor_for_position(position: &CanonicalStreamPosition) -> Arc<str> {
    Arc::from(format!(
        "{}:{}",
        position.ordinal_position(),
        position.canonical_stream_member_identity()
    ))
}
