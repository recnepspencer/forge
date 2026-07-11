use crate::failure::StoreError;
use forge_relational::facade::history::CommitId;
use forge_relational::facade::replay::CanonicalCommitEnvelope;
pub use forge_store_contracts::WalRecordFamily;
use serde::{Deserialize, Serialize};

pub const CURRENT_WAL_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DurableMutationId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DurablePublicationPhase {
    IntentAdmitted,
    CanonicalCommitProduced,
    AuthoritativeAppendPublished,
    AcknowledgmentEligible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RecoveryDecisionClass {
    DiscardUnpublished,
    FinishPublicationFromCanonicalResult,
    RetainPublishedTruth,
    SuppressDuplicateReplay,
    RequiresFullRebuild,
    RequiresQuarantine,
    RequiresSalvage,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableMutationIntentRecord {
    pub durable_mutation_id: DurableMutationId,
    pub runtime_session_id: String,
    pub operation_name: String,
    pub wal_version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostedRuntimeCommitResultRecord {
    pub durable_mutation_id: DurableMutationId,
    pub runtime_session_id: String,
    pub canonical_envelope: CanonicalCommitEnvelope,
    pub wal_version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkCheckpointPublicationIntentRecord {
    pub durable_mutation_id: DurableMutationId,
    pub runtime_session_id: String,
    pub checkpoint_sequence: Option<u64>,
    pub wal_version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurablePublicationProgressRecord {
    pub durable_mutation_id: DurableMutationId,
    pub runtime_session_id: String,
    pub phase: DurablePublicationPhase,
    pub commit_id: Option<CommitId>,
    pub wal_version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryDecisionRecord {
    pub durable_mutation_id: DurableMutationId,
    pub runtime_session_id: String,
    pub decision: RecoveryDecisionClass,
    pub commit_id: Option<CommitId>,
    pub wal_version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WalRecordPayload {
    DurableMutationIntent(DurableMutationIntentRecord),
    HostedRuntimeCommitResult(HostedRuntimeCommitResultRecord),
    BulkCheckpointPublicationIntent(BulkCheckpointPublicationIntentRecord),
    DurablePublicationProgress(DurablePublicationProgressRecord),
    RecoveryDecision(RecoveryDecisionRecord),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WalRecord {
    pub wal_sequence: u64,
    pub family: WalRecordFamily,
    pub durable_mutation_id: DurableMutationId,
    pub runtime_session_id: String,
    pub wal_version: u32,
    pub record_digest: String,
    pub payload: WalRecordPayload,
}

impl WalRecord {
    pub fn canonical_commit_id(&self) -> Option<CommitId> {
        match &self.payload {
            WalRecordPayload::HostedRuntimeCommitResult(record) => {
                Some(record.canonical_envelope.commit.commit_id)
            }
            WalRecordPayload::DurablePublicationProgress(record) => record.commit_id,
            WalRecordPayload::RecoveryDecision(record) => record.commit_id,
            WalRecordPayload::DurableMutationIntent(_)
            | WalRecordPayload::BulkCheckpointPublicationIntent(_) => None,
        }
    }

    pub(crate) fn decode_from_media_bytes(bytes: Vec<u8>) -> Result<Self, StoreError> {
        let validated =
            crate::media::validate_raw_record(crate::media::RawDurableBytes::new(bytes))?;
        if validated.family() != crate::media::DurableMediaFamily::WalRecord {
            return Err(StoreError::new(
                crate::failure::StoreErrorKind::DurableRecordFramingInvalid,
                "durable frame did not contain a WAL record family",
            ));
        }
        Ok(serde_json::from_slice(validated.payload_bytes())?)
    }
}
