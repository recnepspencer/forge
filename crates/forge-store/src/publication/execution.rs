use crate::{
    facade::ForgeStore,
    failure::{StoreError, StoreErrorKind},
    modes::HostedRuntimeOwnershipProof,
    wal::{DurableMutationId, DurablePublicationPhase},
};
use forge_relational::facade::{
    history::CommitId, replay::CanonicalCommitEnvelope, runtime::RelationalRuntime,
};

use super::{DurablePublicationResult, SimulatedCrashPoint};

#[derive(Debug)]
struct PublicationRequest<F> {
    runtime_session_id: String,
    operation_name: String,
    execute: F,
}

impl<F> PublicationRequest<F> {
    fn new(runtime_session_id: &str, operation_name: String, execute: F) -> Self {
        Self {
            runtime_session_id: runtime_session_id.to_string(),
            operation_name,
            execute,
        }
    }
}

#[derive(Debug)]
struct AdmittedDurableMutation {
    runtime_session_id: String,
    operation_name: String,
    durable_mutation_id: DurableMutationId,
}

impl AdmittedDurableMutation {
    fn admit<F>(
        store: &mut ForgeStore,
        request: PublicationRequest<F>,
    ) -> Result<(Self, F), StoreError> {
        let durable_mutation_id =
            store.admit_durable_mutation(&request.runtime_session_id, &request.operation_name)?;
        Ok((
            Self {
                runtime_session_id: request.runtime_session_id,
                operation_name: request.operation_name,
                durable_mutation_id,
            },
            request.execute,
        ))
    }
}

#[derive(Debug)]
struct CanonicalResultRecorded {
    runtime_session_id: String,
    durable_mutation_id: DurableMutationId,
    commit_id: CommitId,
    canonical_envelope: CanonicalCommitEnvelope,
}

impl CanonicalResultRecorded {
    fn record_from_hosted_runtime<F>(
        admitted: AdmittedDurableMutation,
        ownership: &mut HostedRuntimeOwnershipProof,
        execute: F,
        store: &mut ForgeStore,
    ) -> Result<Self, StoreError>
    where
        F: FnOnce(&mut RelationalRuntime) -> Result<CommitId, StoreError>,
    {
        let commit_id = execute(ownership.runtime_mut())?;
        let canonical_envelope = ownership
            .runtime()
            .replay()
            .canonical_commit_envelope(commit_id)
            .cloned()
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::HostedRuntimeMutationProducedNoCommit,
                    format!(
                        "durable mutation `{}` returned commit {} but the hosted runtime has no matching canonical envelope",
                        admitted.operation_name, commit_id.0
                    ),
                )
            })?;
        store.record_hosted_runtime_commit_result(
            &admitted.runtime_session_id,
            admitted.durable_mutation_id,
            canonical_envelope.clone(),
        )?;
        store.record_publication_phase(
            &admitted.runtime_session_id,
            admitted.durable_mutation_id,
            DurablePublicationPhase::CanonicalCommitProduced,
            Some(commit_id),
        )?;
        Ok(Self {
            runtime_session_id: admitted.runtime_session_id,
            durable_mutation_id: admitted.durable_mutation_id,
            commit_id,
            canonical_envelope,
        })
    }
}

#[derive(Debug)]
struct AuthoritativePublicationRecorded {
    runtime_session_id: String,
    durable_mutation_id: DurableMutationId,
    persisted: crate::PersistedAuthoritativeCommit,
}

impl AuthoritativePublicationRecorded {
    fn publish(
        canonical_result: CanonicalResultRecorded,
        store: &mut ForgeStore,
    ) -> Result<Self, StoreError> {
        let persisted = store.append_runtime_envelope(canonical_result.canonical_envelope)?;
        store.record_publication_phase(
            &canonical_result.runtime_session_id,
            canonical_result.durable_mutation_id,
            DurablePublicationPhase::AuthoritativeAppendPublished,
            Some(canonical_result.commit_id),
        )?;
        Ok(Self {
            runtime_session_id: canonical_result.runtime_session_id,
            durable_mutation_id: canonical_result.durable_mutation_id,
            persisted,
        })
    }
}

pub(crate) fn execute_durable_publication<F>(
    store: &mut ForgeStore,
    ownership: &mut HostedRuntimeOwnershipProof,
    runtime_session_id: &str,
    operation_name: String,
    execute: F,
    crash_point: Option<SimulatedCrashPoint>,
) -> Result<DurablePublicationResult, StoreError>
where
    F: FnOnce(&mut RelationalRuntime) -> Result<CommitId, StoreError>,
{
    let request = PublicationRequest::new(runtime_session_id, operation_name, execute);
    let (admitted, execute) = AdmittedDurableMutation::admit(store, request)?;
    if crash_point == Some(SimulatedCrashPoint::AfterIntentRecorded) {
        return Ok(DurablePublicationResult {
            durable_mutation_id: admitted.durable_mutation_id,
            persisted: None,
        });
    }

    let canonical_result =
        CanonicalResultRecorded::record_from_hosted_runtime(admitted, ownership, execute, store)?;
    if crash_point == Some(SimulatedCrashPoint::AfterCanonicalResultRecorded) {
        return Ok(DurablePublicationResult {
            durable_mutation_id: canonical_result.durable_mutation_id,
            persisted: None,
        });
    }

    let authoritative_publication =
        AuthoritativePublicationRecorded::publish(canonical_result, store)?;
    let prerequisite_outcome = store.classify_durable_publication(
        authoritative_publication.durable_mutation_id,
        Some(
            authoritative_publication
                .persisted
                .envelope()
                .commit
                .commit_id,
        ),
    )?;
    if !prerequisite_outcome.acknowledgment_eligible() {
        return Err(StoreError::new(
            StoreErrorKind::AcknowledgmentBoundaryViolation,
            format!(
                "durable mutation {} is not acknowledgment-eligible under publication classification {:?}",
                authoritative_publication.durable_mutation_id.0,
                prerequisite_outcome.classification()
            ),
        ));
    }
    if crash_point == Some(SimulatedCrashPoint::AfterAuthoritativeAppendPublished) {
        return Ok(DurablePublicationResult {
            durable_mutation_id: authoritative_publication.durable_mutation_id,
            persisted: None,
        });
    }

    store.record_publication_phase(
        &authoritative_publication.runtime_session_id,
        authoritative_publication.durable_mutation_id,
        DurablePublicationPhase::AcknowledgmentEligible,
        Some(
            authoritative_publication
                .persisted
                .envelope()
                .commit
                .commit_id,
        ),
    )?;
    store.record_durable_commit_acknowledged();
    let final_outcome = store.classify_durable_publication(
        authoritative_publication.durable_mutation_id,
        Some(
            authoritative_publication
                .persisted
                .envelope()
                .commit
                .commit_id,
        ),
    )?;
    if !final_outcome.sufficient_for_published_truth() {
        return Err(StoreError::new(
            StoreErrorKind::AcknowledgmentBoundaryViolation,
            format!(
                "durable mutation {} did not reach published truth after acknowledgment marker; classification {:?}",
                authoritative_publication.durable_mutation_id.0,
                final_outcome.classification()
            ),
        ));
    }
    Ok(DurablePublicationResult {
        durable_mutation_id: authoritative_publication.durable_mutation_id,
        persisted: Some(authoritative_publication.persisted),
    })
}
