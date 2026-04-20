use crate::failure::StoreError;
use forge_relational::facade::history::CommitId;
use forge_relational::facade::replay::CanonicalCommitEnvelope;

use super::{
    digest::{WalRecordDigestBasis, stable_digest},
    model::{
        BulkCheckpointPublicationIntentRecord, CURRENT_WAL_VERSION, DurableMutationId,
        DurableMutationIntentRecord, DurablePublicationPhase, DurablePublicationProgressRecord,
        HostedRuntimeCommitResultRecord, RecoveryDecisionClass, RecoveryDecisionRecord, WalRecord,
        WalRecordFamily, WalRecordPayload,
    },
};

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
        let payload = WalRecordPayload::HostedRuntimeCommitResult(HostedRuntimeCommitResultRecord {
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
        let payload = WalRecordPayload::DurablePublicationProgress(DurablePublicationProgressRecord {
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

    pub(super) fn from_payload(
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
