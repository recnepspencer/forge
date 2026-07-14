use crate::{
    backend::records::StoreState,
    failure::{StoreError, StoreErrorKind},
    media::DurableMediaReport,
    wal::DurableMutationId,
};
use worth_relational::facade::replay::CanonicalCommitEnvelope;

use super::{
    admit_local_wal_record, classify_durable_publication, durable_publication_facts,
    DurableRecoveryPublicationObservation,
};

pub(crate) fn observe_durable_recovery_publication<'a>(
    state: &StoreState,
    durable_mutation_id: DurableMutationId,
    wal_records: &[&'a crate::wal::WalRecord],
    backend_report: DurableMediaReport,
) -> Result<DurableRecoveryPublicationObservation, StoreError> {
    let mut canonical_envelope: Option<CanonicalCommitEnvelope> = None;
    let mut commit_id = None;
    let mut intent_present = false;

    for record in wal_records {
        let admitted = admit_local_wal_record(record)?;
        match &admitted.inner().payload {
            crate::wal::WalRecordPayload::DurableMutationIntent(_) => {
                intent_present = true;
            }
            crate::wal::WalRecordPayload::HostedRuntimeCommitResult(result) => {
                if let Some(existing) = &canonical_envelope {
                    if existing != &result.canonical_envelope {
                        return Err(StoreError::new(
                            StoreErrorKind::RecoverySourceConflict,
                            format!(
                                "durable mutation {} has conflicting hosted runtime canonical results",
                                durable_mutation_id.0
                            ),
                        ));
                    }
                } else {
                    canonical_envelope = Some(result.canonical_envelope.clone());
                }
                commit_id = Some(result.canonical_envelope.commit.commit_id);
            }
            crate::wal::WalRecordPayload::BulkCheckpointPublicationIntent(_) => {}
            crate::wal::WalRecordPayload::DurablePublicationProgress(progress) => {
                if let Some(progress_commit_id) = progress.commit_id {
                    if let Some(existing) = commit_id {
                        if existing != progress_commit_id {
                            return Err(StoreError::new(
                                StoreErrorKind::RecoverySourceConflict,
                                format!(
                                    "durable mutation {} has conflicting publication commit ids",
                                    durable_mutation_id.0
                                ),
                            ));
                        }
                    } else {
                        commit_id = Some(progress_commit_id);
                    }
                }
            }
            crate::wal::WalRecordPayload::RecoveryDecision(_) => {}
        }
    }

    let facts = durable_publication_facts(state, durable_mutation_id, commit_id)?;
    let publication = classify_durable_publication(backend_report, facts);
    Ok(DurableRecoveryPublicationObservation {
        durable_mutation_id,
        publication,
        canonical_envelope,
        commit_id,
        intent_present,
    })
}
