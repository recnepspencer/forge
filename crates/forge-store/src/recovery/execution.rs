use crate::{
    backend::records::StoreState,
    failure::{StoreError, StoreErrorKind},
    wal::{
        DurableMutationId, DurablePublicationPhase, RecoveryDecisionClass, WalRecord,
        WalRecordPayload,
    },
};
use forge_relational::facade::replay::CanonicalCommitEnvelope;

use super::DurableRecoveryDecision;

pub(crate) enum RecoveryAction {
    RetainPublishedTruth,
    FinishPublicationFromCanonicalResult {
        canonical_envelope: CanonicalCommitEnvelope,
    },
    DiscardUnpublished,
}

pub(crate) struct RecoveryEvaluation {
    pub(crate) decision: DurableRecoveryDecision,
    pub(crate) action: RecoveryAction,
}

pub(crate) fn evaluate_recovery_for_mutation(
    state: &StoreState,
    durable_mutation_id: DurableMutationId,
    wal_records: &[&WalRecord],
) -> Result<RecoveryEvaluation, StoreError> {
    let mut result_envelope = None;
    let mut commit_id = None;
    let mut authoritative_published = false;
    let mut acknowledgment_eligible = false;

    for record in wal_records {
        record.validate_integrity()?;
        match &record.payload {
            WalRecordPayload::HostedRuntimeCommitResult(result) => {
                result_envelope = Some(result.canonical_envelope.clone());
                commit_id = Some(result.canonical_envelope.commit.commit_id);
            }
            WalRecordPayload::DurablePublicationProgress(progress) => {
                if progress.phase == DurablePublicationPhase::AuthoritativeAppendPublished {
                    authoritative_published = true;
                }
                if progress.phase == DurablePublicationPhase::AcknowledgmentEligible {
                    acknowledgment_eligible = true;
                }
                if progress.commit_id.is_some() {
                    commit_id = progress.commit_id;
                }
            }
            WalRecordPayload::DurableMutationIntent(_) | WalRecordPayload::RecoveryDecision(_) => {}
        }
    }

    if authoritative_published || acknowledgment_eligible {
        let commit_id = commit_id.ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::RecoveryRequiresFullRebuild,
                format!(
                    "durable mutation {} published without a recoverable commit id",
                    durable_mutation_id.0
                ),
            )
        })?;
        if !state.has_commit(commit_id) {
            return Err(StoreError::new(
                StoreErrorKind::RecoveryRequiresFullRebuild,
                format!(
                    "durable mutation {} references missing authoritative commit {} during recovery",
                    durable_mutation_id.0, commit_id.0
                ),
            ));
        }
        let decision = if acknowledgment_eligible {
            DurableRecoveryDecision {
                durable_mutation_id,
                decision: RecoveryDecisionClass::SuppressDuplicateReplay,
                commit_id: Some(commit_id),
            }
        } else {
            DurableRecoveryDecision {
                durable_mutation_id,
                decision: RecoveryDecisionClass::RetainPublishedTruth,
                commit_id: Some(commit_id),
            }
        };
        return Ok(RecoveryEvaluation {
            decision,
            action: RecoveryAction::RetainPublishedTruth,
        });
    }

    if let Some(canonical_envelope) = result_envelope {
        return Ok(RecoveryEvaluation {
            decision: DurableRecoveryDecision {
                durable_mutation_id,
                decision: RecoveryDecisionClass::FinishPublicationFromCanonicalResult,
                commit_id: Some(canonical_envelope.commit.commit_id),
            },
            action: RecoveryAction::FinishPublicationFromCanonicalResult { canonical_envelope },
        });
    }

    Ok(RecoveryEvaluation {
        decision: DurableRecoveryDecision {
            durable_mutation_id,
            decision: RecoveryDecisionClass::DiscardUnpublished,
            commit_id: None,
        },
        action: RecoveryAction::DiscardUnpublished,
    })
}
