use std::sync::Arc;

use crate::error::BridgeStreamError;
use crate::identity::{BridgeIdentity, StreamReplayRecordIdentityTag};
use crate::routing::canonicalization::digest_string;

use super::checkpoints::{validate_checkpoint_for_window, ConsumerCheckpointToken};
use super::counters::StreamProtocolCounters;
use super::protocol::AdmittedConsumerContract;
use super::window::PlannedChangeStreamWindow;
use super::StreamReplayMode;

pub type StreamReplayRecordIdentity = BridgeIdentity<StreamReplayRecordIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalStreamReplayRecord {
    replay_record_identity: StreamReplayRecordIdentity,
    consumer_contract_identity: super::protocol::ConsumerContractIdentity,
    stream_window_identity: super::window::StreamWindowIdentity,
    checkpoint_token_identity: Arc<str>,
    replay_basis_digest: Arc<str>,
    protocol_semantics_version: Arc<str>,
    counters: StreamProtocolCounters,
    digest: Arc<str>,
}

impl CanonicalStreamReplayRecord {
    pub(crate) fn from_window_and_checkpoint(
        contract: &AdmittedConsumerContract,
        window: &PlannedChangeStreamWindow,
        checkpoint: &ConsumerCheckpointToken,
    ) -> Result<Self, BridgeStreamError> {
        if contract.admitted_replay_mode() != StreamReplayMode::Enabled {
            return Err(BridgeStreamError::new(
                crate::error::BridgeStreamErrorKind::StreamDeliveryRejected,
                "Replay record construction requires replay mode to be enabled on the admitted consumer contract.",
            ));
        }
        validate_checkpoint_for_window(contract, window, checkpoint)?;
        let replay_basis_digest: Arc<str> = Arc::from(digest_string(
            "stream-replay-basis",
            &format!(
                "{}|{}|{}",
                window.member_set_digest(),
                checkpoint.acknowledged_member_set_digest(),
                checkpoint.contiguous_acknowledged_through_position(),
            ),
        ));
        let basis = format!(
            "canonical-stream-replay-record|contract={}|window={}|checkpoint={}|replay-basis-digest={}|protocol-semantics-version={}",
            contract.consumer_contract_identity().as_str(),
            window.stream_window_identity().as_str(),
            checkpoint.checkpoint_token_identity(),
            replay_basis_digest.as_ref(),
            checkpoint.protocol_semantics_version(),
        );
        let digest = digest_string("canonical-stream-replay-record", &basis);
        Ok(Self {
            replay_record_identity: StreamReplayRecordIdentity::new(digest.clone()),
            consumer_contract_identity: contract.consumer_contract_identity().clone(),
            stream_window_identity: window.stream_window_identity().clone(),
            checkpoint_token_identity: Arc::from(checkpoint.checkpoint_token_identity()),
            replay_basis_digest,
            protocol_semantics_version: Arc::from(checkpoint.protocol_semantics_version()),
            counters: checkpoint.counters().clone().with_replay(false),
            digest,
        })
    }

    pub fn replay_record_identity(&self) -> &StreamReplayRecordIdentity {
        &self.replay_record_identity
    }

    pub fn consumer_contract_identity(&self) -> &super::protocol::ConsumerContractIdentity {
        &self.consumer_contract_identity
    }

    pub fn stream_window_identity(&self) -> &super::window::StreamWindowIdentity {
        &self.stream_window_identity
    }

    pub fn checkpoint_token_identity(&self) -> &str {
        self.checkpoint_token_identity.as_ref()
    }

    pub fn replay_basis_digest(&self) -> &str {
        self.replay_basis_digest.as_ref()
    }

    pub fn protocol_semantics_version(&self) -> &str {
        self.protocol_semantics_version.as_ref()
    }

    pub fn counters(&self) -> &StreamProtocolCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

pub(crate) fn canonicalize_stream_replay_record(
    contract: &AdmittedConsumerContract,
    window: &PlannedChangeStreamWindow,
    checkpoint: &ConsumerCheckpointToken,
) -> Result<CanonicalStreamReplayRecord, BridgeStreamError> {
    CanonicalStreamReplayRecord::from_window_and_checkpoint(contract, window, checkpoint)
}

pub(crate) fn validate_stream_replay_record(
    contract: &AdmittedConsumerContract,
    window: &PlannedChangeStreamWindow,
    checkpoint: &ConsumerCheckpointToken,
    record: &CanonicalStreamReplayRecord,
) -> Result<(), BridgeStreamError> {
    let expected =
        CanonicalStreamReplayRecord::from_window_and_checkpoint(contract, window, checkpoint)?;
    if record.consumer_contract_identity() != contract.consumer_contract_identity()
        || record.stream_window_identity() != window.stream_window_identity()
        || record.checkpoint_token_identity() != checkpoint.checkpoint_token_identity()
        || record.replay_basis_digest() != expected.replay_basis_digest()
        || record.protocol_semantics_version() != expected.protocol_semantics_version()
        || record.digest() != expected.digest()
    {
        return Err(BridgeStreamError::new(
            crate::error::BridgeStreamErrorKind::StreamReplayMismatch,
            "The canonical stream replay record did not match the admitted contract, planned window, and checkpoint basis.",
        ));
    }

    Ok(())
}
