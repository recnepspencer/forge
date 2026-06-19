use crate::{
    ForgeServerCompatibilityFacade, ForgeServerCompatibilityMutationPreconditionContext,
    ForgeServerDirectContextArtifact, ForgeServerDirectRemaskPosture, ForgeServerOperationFamily,
    ForgeServerOperationInputEnvelope, ForgeServerOperationPlanner,
    ForgeServerOperationPlannerInput, ForgeServerOperationPreconditionPosture,
    ForgeServerOperationReadinessFacade, ForgeServerOperationRequestFacade,
    ForgeServerPipelineInput, ForgeServerPipelineIntent, ForgeServerQueryHandoffDenial,
    ForgeServerQueryHandoffDenialCode, ForgeServerQueryHandoffDenialFacts,
    ForgeServerQueryHandoffFailure, ForgeServerQueryHandoffOperation,
    ForgeServerQueryWorkspaceBindingRequest, ForgeServerScheduledMutationResult,
};
use forge_proof::TransitionOutcome;
use forge_query::facade::ForgeQueryRuntimeFacadeFamily;

use super::{
    envelope::ForgeServerCompatibilityMutationEnvelope,
    execution::{
        ForgeServerCompatibilityMutationExecutionInput, ForgeServerCompatibilityMutationOutcome,
    },
    idempotency::{ForgeServerIdempotencyKey, ForgeServerIdempotentReplayReceipt},
    query_execution_support::{
        canonical_mutation_request_digest, map_operation_request_denial, map_readiness_denial,
    },
    replay_cache::{record_replay, try_replay},
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
        let operation_request =
            match ForgeServerOperationRequestFacade::new(self.operation_registry.clone())
                .admit_from_compat_http(
                    &prepared_request,
                    ForgeServerOperationFamily::QueryDirectSubmission,
                    &operation_name,
                    Some(ForgeServerOperationInputEnvelope::json(
                        "compat-http.query-mutation.v1",
                        &body,
                    )),
                ) {
                Ok(value) => value,
                Err(denial) => {
                    return TransitionOutcome::Denied(map_operation_request_denial(denial));
                }
            };
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
            operation_request,
            mutation_request,
        )
    }
}
pub(crate) fn execute_compatibility_mutation_request(
    facade: &ForgeServerCompatibilityFacade,
    prepared_request: crate::ForgeServerCompatibilityPreparedRequest,
    operation_request: crate::ForgeServerOperationRequest,
    mutation_request: ForgeServerCompatibilityMutationRequest,
) -> ForgeServerCompatibilityMutationOutcome<ForgeServerCompatibilityMutation> {
    let diagnostics_profile = operation_request
        .resolved_request_context()
        .request_context()
        .diagnostics_profile();
    let operation_name = operation_request.identity().operation_name().to_string();
    if let Err(denial) = facade.admit_operation_family_for_query(
        diagnostics_profile,
        ForgeServerOperationFamily::QueryDirectSubmission,
    ) {
        return TransitionOutcome::Denied(denial);
    }
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
    let operation_admission =
        match crate::ForgeServerOperationAdmissionFacade::with_operation_registry(
            facade.operation_registry.clone(),
        )
        .admit_declared(&admission, &operation_request)
        {
            Ok(value) => value,
            Err(denial) => {
                return TransitionOutcome::Denied(
                    crate::surfaces::compat_http::map_operation_admission_denial(denial),
                );
            }
        };
    let binding_request = ForgeServerQueryWorkspaceBindingRequest::for_query_handoff(
        operation_admission
            .authorization_proof()
            .admission()
            .resolved_request_context()
            .clone(),
        ForgeServerQueryHandoffOperation::query_mutation(&operation_name),
    );
    let bound_workspace = match facade
        .query_handoff
        .config()
        .workspace_provider()
        .bind_workspace(&binding_request)
    {
        Ok(workspace) => workspace,
        Err(error) => {
            return TransitionOutcome::Denied(ForgeServerQueryHandoffDenial::new(
                ForgeServerQueryHandoffDenialCode::WorkspaceBindingFailed,
                diagnostics_profile,
                format!("{}: {}", error.stage(), error.message()),
            ));
        }
    };
    let observed_basis_digest = bound_workspace
        .snapshot_identity()
        .terminal_projection_for_reporting();
    let readiness = ForgeServerOperationReadinessFacade::with_operation_registry(
        facade.operation_registry.clone(),
    );
    let precondition = match readiness.evaluate_compatibility_mutation_preconditions(
        ForgeServerCompatibilityMutationPreconditionContext::new(
            &prepared_request,
            &operation_name,
            mutation_request.canonical_digest(),
            &observed_basis_digest,
        ),
    ) {
        Ok(value) => value,
        Err(denial) => {
            return TransitionOutcome::Denied(map_readiness_denial(&prepared_request, denial));
        }
    };
    let plan = match ForgeServerOperationPlanner::with_operation_registry(
        facade.query_handoff.config().clone(),
        facade.operation_registry.clone(),
    )
    .lower(
        ForgeServerOperationPlannerInput::new(
            operation_admission,
            ForgeServerQueryHandoffOperation::query_mutation_execution(operation.clone()),
        )
        .with_precondition_posture(
            ForgeServerOperationPreconditionPosture::CompatibilityMutation(precondition.clone()),
        )
        .with_bound_workspace(bound_workspace),
    ) {
        Ok(value) => value,
        Err(denial) => return TransitionOutcome::Denied(denial.into_query_handoff_denial()),
    };
    let plan_proof = plan.proof();
    let query_handoff = plan.query_handoff();
    if let Err(error) = query_handoff
        .workspace()
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Inspect)
    {
        return TransitionOutcome::Denied(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::UnsupportedQueryFacadeFamily,
            diagnostics_profile,
            format!("query workspace does not admit `inspect` facade family: {error}"),
        ));
    }
    let support_posture = query_handoff.support_posture().clone();
    let workspace_name = query_handoff.workspace().name().to_string();
    let handoff_digest = query_handoff.canonical_digest().to_string();
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

    let executed = match crate::ForgeServerOperationScheduler::new(facade.responses.clone())
        .schedule_batch([plan])
    {
        Ok(batch) => batch.execute(),
        Err(denial) => {
            return TransitionOutcome::Denied(ForgeServerQueryHandoffDenial::new(
                ForgeServerQueryHandoffDenialCode::CompatibilityMutationPreconditionFailed,
                diagnostics_profile,
                denial.detail(),
            ))
        }
    };
    let outcome = &executed.outcomes()[0];
    if let Some(cancellation_posture) = outcome.cancellation_posture() {
        return TransitionOutcome::Failed(ForgeServerQueryHandoffFailure::new(
            match cancellation_posture {
                crate::ForgeServerSchedulerCancellationPosture::BeforeAdmission => {
                    "compatibility_mutation_cancelled_before_admission"
                }
                crate::ForgeServerSchedulerCancellationPosture::AfterAdmissionBeforeExecution => {
                    "compatibility_mutation_cancelled_after_admission_before_execution"
                }
                crate::ForgeServerSchedulerCancellationPosture::DuringExecution => {
                    "compatibility_mutation_cancelled_during_execution"
                }
            },
        ));
    }
    if let Some(failure_posture) = outcome.failure_posture() {
        return match failure_posture {
            crate::ForgeServerSchedulerFailurePosture::StaleMutationBasis {
                expected_basis_digest,
                observed_basis_digest,
            } => TransitionOutcome::Denied(
                ForgeServerQueryHandoffDenial::new(
                    ForgeServerQueryHandoffDenialCode::CompatibilityMutationPreconditionFailed,
                    diagnostics_profile,
                    format!(
                        "compatibility mutation basis precondition `{expected_basis_digest}` did not match the scheduler-observed basis `{observed_basis_digest}`"
                    ),
                )
                .with_facts(
                    ForgeServerQueryHandoffDenialFacts::default()
                        .with_basis_mismatch(expected_basis_digest, observed_basis_digest),
                ),
            ),
            crate::ForgeServerSchedulerFailurePosture::IsolatedRuntimeFailure { .. }
            | crate::ForgeServerSchedulerFailurePosture::DependentSharedBasisFailure { .. }
            | crate::ForgeServerSchedulerFailurePosture::OrderedLaneClosed { .. } => {
                TransitionOutcome::Failed(ForgeServerQueryHandoffFailure::new(
                    "compatibility_mutation_scheduler_execution_failed",
                ))
            }
        };
    }
    let mutation_result = map_scheduled_mutation_result(
        outcome
            .mutation_result()
            .expect("scheduled compatibility mutation should carry a mutation result")
            .clone(),
    );
    let response_envelope = outcome
        .response_envelope()
        .expect("scheduled compatibility mutation should shape a response envelope")
        .clone();
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
        operation_request,
        plan_proof,
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
fn map_scheduled_mutation_result(
    mutation_result: ForgeServerScheduledMutationResult,
) -> ForgeServerCompatibilityMutationResult {
    match mutation_result {
        ForgeServerScheduledMutationResult::Single {
            receipt,
            inspection,
        } => ForgeServerCompatibilityMutationResult::Single {
            receipt,
            inspection,
        },
        ForgeServerScheduledMutationResult::Batch {
            receipt,
            inspection,
        } => ForgeServerCompatibilityMutationResult::Batch {
            receipt,
            inspection,
        },
    }
}
