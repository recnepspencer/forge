use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use forge_proof::TransitionOutcome;
use forge_query::facade::{
    ForgeQueryInspection, ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily,
};

use crate::{
    ForgeServerCompatibilityFacade, ForgeServerDirectContextArtifact,
    ForgeServerDirectRemaskPosture, ForgeServerPipelineInput, ForgeServerPipelineIntent,
    ForgeServerQueryHandoffDenial, ForgeServerQueryHandoffDenialCode,
    ForgeServerQueryHandoffFailure, ForgeServerQueryHandoffInput, ForgeServerQueryHandoffOperation,
    ForgeServerQueryOperation, ForgeServerResponseInput,
};

use super::{
    envelope::ForgeServerCompatibilityMutationEnvelope,
    execution::{
        ForgeServerCompatibilityMutationExecutionInput, ForgeServerCompatibilityMutationOutcome,
    },
    idempotency::{
        ForgeServerIdempotencyKey, ForgeServerIdempotentReplayReceipt,
        ForgeServerStoredCompatibilityMutation,
    },
    precondition::ForgeServerMutationPrecondition,
    request::ForgeServerCompatibilityMutationRequest,
    response::{ForgeServerCompatibilityMutation, ForgeServerCompatibilityMutationResult},
    schema::lower_query_operation,
};

impl ForgeServerCompatibilityFacade {
    pub fn mutate(
        &self,
        input: ForgeServerCompatibilityMutationExecutionInput,
    ) -> ForgeServerCompatibilityMutationOutcome<ForgeServerCompatibilityMutation> {
        let (prepared_request, operation_name, body) = input.into_parts();
        let diagnostics_profile = prepared_request
            .admission()
            .request_context()
            .diagnostics_profile();
        let mutation_request =
            match ForgeServerCompatibilityMutationRequest::parse(body, diagnostics_profile) {
                Ok(value) => value,
                Err(denial) => return TransitionOutcome::Denied(denial),
            };
        execute_compatibility_mutation_request(
            self,
            prepared_request,
            operation_name,
            mutation_request,
        )
    }
}

pub(crate) fn execute_compatibility_mutation_request(
    facade: &ForgeServerCompatibilityFacade,
    prepared_request: crate::ForgeServerCompatibilityPreparedRequest,
    operation_name: String,
    mutation_request: ForgeServerCompatibilityMutationRequest,
) -> ForgeServerCompatibilityMutationOutcome<ForgeServerCompatibilityMutation> {
    let diagnostics_profile = prepared_request
        .admission()
        .request_context()
        .diagnostics_profile();
    let operation =
        match lower_query_operation(&operation_name, &mutation_request, diagnostics_profile) {
            Ok(value) => value,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
    let admission = match facade.middleware.admit(ForgeServerPipelineInput::new(
        prepared_request
            .admission()
            .resolved_request_context()
            .clone(),
        ForgeServerPipelineIntent::query_mutation(&operation_name),
    )) {
        TransitionOutcome::Success(value) => value,
        TransitionOutcome::Denied(value) => {
            return TransitionOutcome::Denied(ForgeServerQueryHandoffDenial::new(
                ForgeServerQueryHandoffDenialCode::PreparedIntentMismatch,
                diagnostics_profile,
                value.detail(),
            ));
        }
        TransitionOutcome::Deferred(_)
        | TransitionOutcome::Stale(_)
        | TransitionOutcome::RebindRequired(_)
        | TransitionOutcome::Failed(_) => {
            return TransitionOutcome::Failed(ForgeServerQueryHandoffFailure::new(
                "compatibility_mutation_middleware_readmission_failed",
            ));
        }
    };
    let mut handoff = match facade
        .query_handoff
        .prepare(ForgeServerQueryHandoffInput::new(
            admission,
            ForgeServerQueryHandoffOperation::query_mutation(&operation_name),
        )) {
        TransitionOutcome::Success(value) => value,
        TransitionOutcome::Denied(value) => return TransitionOutcome::Denied(value),
        TransitionOutcome::Deferred(value) => return TransitionOutcome::Deferred(value),
        TransitionOutcome::Stale(value) => return TransitionOutcome::Stale(value),
        TransitionOutcome::RebindRequired(value) => {
            return TransitionOutcome::RebindRequired(value)
        }
        TransitionOutcome::Failed(value) => return TransitionOutcome::Failed(value),
    };
    if let Err(error) = handoff
        .workspace()
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Inspect)
    {
        return TransitionOutcome::Denied(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::UnsupportedQueryFacadeFamily,
            diagnostics_profile,
            format!("query workspace does not admit `inspect` facade family: {error}"),
        ));
    }
    let observed_basis_digest = handoff.workspace().snapshot_token();
    let precondition = match ForgeServerMutationPrecondition::from_prepared_request(
        &prepared_request,
        &operation_name,
        mutation_request.canonical_digest(),
        &observed_basis_digest,
    ) {
        Ok(value) => value,
        Err(denial) => return TransitionOutcome::Denied(denial),
    };
    if let Err(denial) = precondition.enforce(&prepared_request) {
        return TransitionOutcome::Denied(denial);
    }
    let idempotency_key = match ForgeServerIdempotencyKey::from_prepared_request(&prepared_request)
    {
        Ok(value) => value,
        Err(denial) => return TransitionOutcome::Denied(denial),
    };
    let request_digest = canonical_mutation_request_digest(
        &prepared_request,
        &operation_name,
        &mutation_request,
        &precondition,
    );
    match try_replay(
        &facade.idempotency_store,
        &prepared_request,
        &idempotency_key,
        &request_digest,
    ) {
        Ok(Some(replayed)) => return TransitionOutcome::Success(replayed),
        Ok(None) => {}
        Err(denial) => return TransitionOutcome::Denied(denial),
    }

    let mutation_result = match execute_query_mutation_operation(
        &operation,
        &prepared_request,
        handoff.workspace_mut(),
    ) {
        Ok(value) => value,
        Err(outcome) => return outcome,
    };
    let support_posture = handoff.support_posture().clone();
    let workspace_name = handoff.workspace().name().to_string();
    let handoff_digest = handoff.canonical_digest().to_string();
    let response_envelope = facade
        .responses
        .shape_with_defaults(ForgeServerResponseInput::query_handoff_success(handoff));
    let direct_context = ForgeServerDirectContextArtifact::new(
        prepared_request.admission().request_context(),
        &support_posture,
        &response_envelope,
        Some(&observed_basis_digest),
        ForgeServerDirectRemaskPosture::visible(),
    );
    let replay_receipt = idempotency_key
        .as_ref()
        .map(|key| ForgeServerIdempotentReplayReceipt::authoritative(key, &request_digest))
        .unwrap_or_else(|| ForgeServerIdempotentReplayReceipt::Authoritative {
            idempotency_key: "none".to_string(),
            request_digest: request_digest.clone(),
            canonical_digest: format!(
                "compat-http-idempotent-replay-v1|class=authoritative|key:none|request:{}",
                request_digest
            ),
        });
    let envelope = ForgeServerCompatibilityMutationEnvelope::new(
        support_posture,
        workspace_name,
        handoff_digest,
        direct_context,
        response_envelope,
        replay_receipt,
    );
    let mutation = ForgeServerCompatibilityMutation::new(
        mutation_request,
        precondition,
        mutation_result,
        envelope,
    );
    record_replay(
        &facade.idempotency_store,
        &prepared_request,
        idempotency_key,
        request_digest,
        mutation.clone(),
    );
    TransitionOutcome::Success(mutation)
}

fn try_replay(
    idempotency_store: &Arc<Mutex<HashMap<String, ForgeServerStoredCompatibilityMutation>>>,
    prepared_request: &crate::ForgeServerCompatibilityPreparedRequest,
    idempotency_key: &Option<ForgeServerIdempotencyKey>,
    request_digest: &str,
) -> Result<Option<ForgeServerCompatibilityMutation>, ForgeServerQueryHandoffDenial> {
    let Some(key) = idempotency_key.as_ref() else {
        return Ok(None);
    };
    let store = idempotency_store
        .lock()
        .expect("compatibility idempotency store mutex should not be poisoned");
    let storage_key = key.scoped_storage_key(prepared_request);
    let Some(stored) = store.get(&storage_key) else {
        return Ok(None);
    };
    if stored.request_digest() != request_digest {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityIdempotencyConflict,
            prepared_request.admission().request_context().diagnostics_profile(),
            format!(
                "compatibility mutation idempotency key `{}` was already bound to request digest `{}` and cannot be reused for `{request_digest}`",
                key.value(),
                stored.request_digest(),
            ),
        ));
    }
    Ok(Some(stored.mutation().to_replayed(
        ForgeServerIdempotentReplayReceipt::replayed(
            key,
            request_digest,
            stored.mutation().canonical_digest(),
        ),
    )))
}

fn record_replay(
    idempotency_store: &Arc<Mutex<HashMap<String, ForgeServerStoredCompatibilityMutation>>>,
    prepared_request: &crate::ForgeServerCompatibilityPreparedRequest,
    idempotency_key: Option<ForgeServerIdempotencyKey>,
    request_digest: String,
    mutation: ForgeServerCompatibilityMutation,
) {
    let Some(key) = idempotency_key else {
        return;
    };
    let storage_key = key.scoped_storage_key(prepared_request);
    idempotency_store
        .lock()
        .expect("compatibility idempotency store mutex should not be poisoned")
        .insert(
            storage_key,
            ForgeServerStoredCompatibilityMutation::new(request_digest, mutation),
        );
}

fn canonical_mutation_request_digest(
    prepared_request: &crate::ForgeServerCompatibilityPreparedRequest,
    operation_name: &str,
    mutation_request: &ForgeServerCompatibilityMutationRequest,
    precondition: &ForgeServerMutationPrecondition,
) -> String {
    format!(
        "compat-http-mutation-request-digest-v1|request:{}|operation:{}|mutation:{}|precondition:{}",
        prepared_request.request_contract().canonical_digest(),
        operation_name.trim(),
        mutation_request.canonical_digest(),
        precondition.request_identity_digest(),
    )
}

fn execute_query_mutation_operation(
    operation: &ForgeServerQueryOperation,
    prepared_request: &crate::ForgeServerCompatibilityPreparedRequest,
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
) -> Result<
    ForgeServerCompatibilityMutationResult,
    ForgeServerCompatibilityMutationOutcome<ForgeServerCompatibilityMutation>,
> {
    match operation {
        ForgeServerQueryOperation::SingleMutation { command, .. } => {
            let receipt = match workspace.write_intent(command.clone()).review() {
                Ok(review) => match review.admit() {
                    Ok(admitted) => match admitted.execute() {
                        Ok(receipt) => receipt,
                        Err(error) => return Err(runtime_error_outcome(prepared_request, error)),
                    },
                    Err(error) => return Err(runtime_error_outcome(prepared_request, error)),
                },
                Err(error) => return Err(runtime_error_outcome(prepared_request, error)),
            };
            let inspection = match workspace.inspect(&receipt) {
                Ok(ForgeQueryInspection::WriteReceipt(inspection)) => inspection,
                Ok(other) => panic!("expected write receipt inspection, got {other:?}"),
                Err(error) => return Err(runtime_error_outcome(prepared_request, error)),
            };
            Ok(ForgeServerCompatibilityMutationResult::Single {
                receipt,
                inspection,
            })
        }
        ForgeServerQueryOperation::BatchMutation { commands, .. } => {
            let receipt = match workspace.write_batch_intent(commands.clone()).review() {
                Ok(review) => match review.admit() {
                    Ok(admitted) => match admitted.execute() {
                        Ok(receipt) => receipt,
                        Err(error) => return Err(runtime_error_outcome(prepared_request, error)),
                    },
                    Err(error) => return Err(runtime_error_outcome(prepared_request, error)),
                },
                Err(error) => return Err(runtime_error_outcome(prepared_request, error)),
            };
            let inspection = match workspace.inspect(&receipt) {
                Ok(ForgeQueryInspection::BatchWriteReceipt(inspection)) => inspection,
                Ok(other) => panic!("expected batch write receipt inspection, got {other:?}"),
                Err(error) => return Err(runtime_error_outcome(prepared_request, error)),
            };
            Ok(ForgeServerCompatibilityMutationResult::Batch {
                receipt,
                inspection,
            })
        }
    }
}

fn runtime_error_outcome(
    prepared_request: &crate::ForgeServerCompatibilityPreparedRequest,
    error: ForgeQueryRuntimeError,
) -> ForgeServerCompatibilityMutationOutcome<ForgeServerCompatibilityMutation> {
    match error {
        ForgeQueryRuntimeError::MutationBindingDenied(_) => {
            TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                crate::ForgeServerQueryHandoffDenialCode::DirectMutationBindingDenied,
                prepared_request
                    .admission()
                    .request_context()
                    .diagnostics_profile(),
                error.to_string(),
            ))
        }
        ForgeQueryRuntimeError::ExistingTruthAssertionDenied(_) => {
            TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                crate::ForgeServerQueryHandoffDenialCode::DirectMutationAssertionDenied,
                prepared_request
                    .admission()
                    .request_context()
                    .diagnostics_profile(),
                error.to_string(),
            ))
        }
        ForgeQueryRuntimeError::MutationContinuityDenied(_) => {
            TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                crate::ForgeServerQueryHandoffDenialCode::DirectMutationContinuityDenied,
                prepared_request
                    .admission()
                    .request_context()
                    .diagnostics_profile(),
                error.to_string(),
            ))
        }
        ForgeQueryRuntimeError::MutationNamingDenied(_) => {
            TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                crate::ForgeServerQueryHandoffDenialCode::DirectMutationNamingDenied,
                prepared_request
                    .admission()
                    .request_context()
                    .diagnostics_profile(),
                error.to_string(),
            ))
        }
        ForgeQueryRuntimeError::MutationTargetReferenceDenied(_) => {
            TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                crate::ForgeServerQueryHandoffDenialCode::DirectMutationTargetReferenceDenied,
                prepared_request
                    .admission()
                    .request_context()
                    .diagnostics_profile(),
                error.to_string(),
            ))
        }
        _ => TransitionOutcome::Failed(ForgeServerQueryHandoffFailure::new(
            "compatibility_mutation_execution_failed",
        )),
    }
}
