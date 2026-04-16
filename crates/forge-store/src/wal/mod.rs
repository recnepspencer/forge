use crate::failure::{StoreError, StoreErrorKind};
use crate::media::{
    barriers::validate_barrier_satisfies_requirement,
    frame_payload,
    framing::{scan_tail, TailValidationOutcome},
    validate_raw_record, BarrierClassifiedDurableRecord, DurabilityBarrierClass,
    DurableMediaFamily, RawDurableBytes,
};
use forge_relational::facade::history::CommitId;
use forge_relational::facade::replay::CanonicalCommitEnvelope;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const CURRENT_WAL_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DurableMutationId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WalRecordFamily {
    DurableMutationIntent,
    HostedRuntimeCommitResult,
    BulkCheckpointPublicationIntent,
    DurablePublicationProgress,
    RecoveryDecision,
}

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
    pub fn durable_mutation_intent(
        wal_sequence: u64,
        durable_mutation_id: DurableMutationId,
        runtime_session_id: impl Into<String>,
        operation_name: impl Into<String>,
    ) -> Result<Self, StoreError> {
        let runtime_session_id = runtime_session_id.into();
        let payload = WalRecordPayload::DurableMutationIntent(DurableMutationIntentRecord {
            durable_mutation_id,
            runtime_session_id: runtime_session_id.clone(),
            operation_name: operation_name.into(),
            wal_version: CURRENT_WAL_VERSION,
        });
        Self::from_payload(
            wal_sequence,
            WalRecordFamily::DurableMutationIntent,
            durable_mutation_id,
            runtime_session_id,
            payload,
        )
    }

    pub fn hosted_runtime_commit_result(
        wal_sequence: u64,
        durable_mutation_id: DurableMutationId,
        runtime_session_id: impl Into<String>,
        canonical_envelope: CanonicalCommitEnvelope,
    ) -> Result<Self, StoreError> {
        let runtime_session_id = runtime_session_id.into();
        let payload =
            WalRecordPayload::HostedRuntimeCommitResult(HostedRuntimeCommitResultRecord {
                durable_mutation_id,
                runtime_session_id: runtime_session_id.clone(),
                canonical_envelope,
                wal_version: CURRENT_WAL_VERSION,
            });
        Self::from_payload(
            wal_sequence,
            WalRecordFamily::HostedRuntimeCommitResult,
            durable_mutation_id,
            runtime_session_id,
            payload,
        )
    }

    pub fn bulk_checkpoint_publication_intent(
        wal_sequence: u64,
        durable_mutation_id: DurableMutationId,
        runtime_session_id: impl Into<String>,
        checkpoint_sequence: Option<u64>,
    ) -> Result<Self, StoreError> {
        let runtime_session_id = runtime_session_id.into();
        let payload = WalRecordPayload::BulkCheckpointPublicationIntent(
            BulkCheckpointPublicationIntentRecord {
                durable_mutation_id,
                runtime_session_id: runtime_session_id.clone(),
                checkpoint_sequence,
                wal_version: CURRENT_WAL_VERSION,
            },
        );
        Self::from_payload(
            wal_sequence,
            WalRecordFamily::BulkCheckpointPublicationIntent,
            durable_mutation_id,
            runtime_session_id,
            payload,
        )
    }

    pub fn durable_publication_progress(
        wal_sequence: u64,
        durable_mutation_id: DurableMutationId,
        runtime_session_id: impl Into<String>,
        phase: DurablePublicationPhase,
        commit_id: Option<CommitId>,
    ) -> Result<Self, StoreError> {
        let runtime_session_id = runtime_session_id.into();
        let payload =
            WalRecordPayload::DurablePublicationProgress(DurablePublicationProgressRecord {
                durable_mutation_id,
                runtime_session_id: runtime_session_id.clone(),
                phase,
                commit_id,
                wal_version: CURRENT_WAL_VERSION,
            });
        Self::from_payload(
            wal_sequence,
            WalRecordFamily::DurablePublicationProgress,
            durable_mutation_id,
            runtime_session_id,
            payload,
        )
    }

    pub fn recovery_decision(
        wal_sequence: u64,
        durable_mutation_id: DurableMutationId,
        runtime_session_id: impl Into<String>,
        decision: RecoveryDecisionClass,
        commit_id: Option<CommitId>,
    ) -> Result<Self, StoreError> {
        let runtime_session_id = runtime_session_id.into();
        let payload = WalRecordPayload::RecoveryDecision(RecoveryDecisionRecord {
            durable_mutation_id,
            runtime_session_id: runtime_session_id.clone(),
            decision,
            commit_id,
            wal_version: CURRENT_WAL_VERSION,
        });
        Self::from_payload(
            wal_sequence,
            WalRecordFamily::RecoveryDecision,
            durable_mutation_id,
            runtime_session_id,
            payload,
        )
    }

    pub fn validate_integrity(&self) -> Result<(), StoreError> {
        if self.wal_version != CURRENT_WAL_VERSION {
            return Err(StoreError::new(
                StoreErrorKind::WalCanonicalizationVersionUnsupported,
                format!(
                    "wal record {} uses unsupported wal version {}",
                    self.wal_sequence, self.wal_version
                ),
            ));
        }
        let recomputed = stable_digest(&WalRecordDigestBasis {
            family: self.family,
            durable_mutation_id: self.durable_mutation_id,
            runtime_session_id: &self.runtime_session_id,
            wal_version: self.wal_version,
            payload: &self.payload,
        })?;
        if recomputed != self.record_digest {
            return Err(StoreError::new(
                StoreErrorKind::WalDigestMismatch,
                format!(
                    "wal record {} failed digest verification for durable mutation {}",
                    self.wal_sequence, self.durable_mutation_id.0
                ),
            ));
        }
        self.validate_media_frame_contract()?;
        Ok(())
    }

    pub(crate) fn classify_media_barrier(
        &self,
        barrier_class: DurabilityBarrierClass,
    ) -> Result<BarrierClassifiedDurableRecord, StoreError> {
        let framed = frame_payload(DurableMediaFamily::WalRecord, self)?;
        let validated = validate_raw_record(framed.to_raw_bytes())?;
        let decoded = Self::decode_from_media_bytes(framed.as_bytes().to_vec())?;
        if decoded != *self {
            return Err(StoreError::new(
                StoreErrorKind::DurableRecordFramingInvalid,
                format!(
                    "wal record {} failed framed media roundtrip validation",
                    self.wal_sequence
                ),
            ));
        }
        Ok(BarrierClassifiedDurableRecord::classify(
            validated,
            barrier_class,
        ))
    }

    pub(crate) fn decode_from_media_bytes(bytes: Vec<u8>) -> Result<Self, StoreError> {
        let validated = validate_raw_record(RawDurableBytes::new(bytes))?;
        if validated.family() != DurableMediaFamily::WalRecord {
            return Err(StoreError::new(
                StoreErrorKind::DurableRecordFramingInvalid,
                "durable frame did not contain a WAL record family",
            ));
        }
        Ok(serde_json::from_slice(validated.payload_bytes())?)
    }

    fn validate_media_frame_contract(&self) -> Result<(), StoreError> {
        let classified =
            self.classify_media_barrier(DurabilityBarrierClass::TransactionalCommitDurable)?;
        let report = scan_tail(classified.record().framed_record().as_bytes())?;
        if report.outcome() != TailValidationOutcome::Clean || report.valid_record_count() != 1 {
            return Err(StoreError::new(
                StoreErrorKind::DurableRecordFramingInvalid,
                format!(
                    "wal record {} did not roundtrip to one clean durable frame",
                    self.wal_sequence
                ),
            ));
        }
        validate_barrier_satisfies_requirement(
            classified.barrier_class(),
            DurabilityBarrierClass::FileContentDurable,
        )?;
        if classified.record().version() != crate::media::CURRENT_DURABLE_MEDIA_VERSION {
            return Err(StoreError::new(
                StoreErrorKind::DurableFamilyVersionUnsupported,
                format!(
                    "wal record {} uses unsupported durable media version {}",
                    self.wal_sequence,
                    classified.record().version()
                ),
            ));
        }
        if classified.record().family() != DurableMediaFamily::WalRecord {
            return Err(StoreError::new(
                StoreErrorKind::DurableRecordFramingInvalid,
                format!(
                    "wal record {} did not preserve WAL family classification",
                    self.wal_sequence
                ),
            ));
        }
        if classified.record().framed_record().payload_len() == 0 {
            return Err(StoreError::new(
                StoreErrorKind::DurableRecordFramingInvalid,
                format!(
                    "wal record {} encoded an empty durable payload",
                    self.wal_sequence
                ),
            ));
        }
        Ok(())
    }

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

    fn from_payload(
        wal_sequence: u64,
        family: WalRecordFamily,
        durable_mutation_id: DurableMutationId,
        runtime_session_id: String,
        payload: WalRecordPayload,
    ) -> Result<Self, StoreError> {
        let wal_version = CURRENT_WAL_VERSION;
        let record_digest = stable_digest(&WalRecordDigestBasis {
            family,
            durable_mutation_id,
            runtime_session_id: &runtime_session_id,
            wal_version,
            payload: &payload,
        })?;
        Ok(Self {
            wal_sequence,
            family,
            durable_mutation_id,
            runtime_session_id,
            wal_version,
            record_digest,
            payload,
        })
    }
}

#[derive(Serialize)]
struct WalRecordDigestBasis<'a> {
    family: WalRecordFamily,
    durable_mutation_id: DurableMutationId,
    runtime_session_id: &'a str,
    wal_version: u32,
    payload: &'a WalRecordPayload,
}

fn stable_digest<T: Serialize>(value: &T) -> Result<String, StoreError> {
    let bytes = serde_json::to_vec(value)?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Ok(format!("{:x}", hasher.finalize()))
}
