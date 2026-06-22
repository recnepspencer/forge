use forge_proof::TransitionOutcome;
use forge_query::facade::ForgeQueryRuntimeFacadeFamily;

use crate::{
    ForgeServerDirectContextArtifact, ForgeServerDirectMutation, ForgeServerDirectMutationResult,
    ForgeServerDirectRemaskPosture, ForgeServerOperationFamily, ForgeServerOperationRequestDenial,
    ForgeServerOperationRequestDenialCode, ForgeServerOperationRequestFacade,
    ForgeServerOperationRequestInput, ForgeServerQueryHandoffDenial,
    ForgeServerQueryHandoffDenialCode, ForgeServerQueryHandoffDenialFacts,
    ForgeServerQueryOperation, ForgeServerScheduledMutationResult,
};

use super::{ForgeServerDirectMutationOutcome, ForgeServerForgeNativeDirectFacade};

impl ForgeServerForgeNativeDirectFacade {
    pub fn mutate(
        &self,
        operation: &ForgeServerQueryOperation,
    ) -> ForgeServerDirectMutationOutcome {
        let operation_request =
            match ForgeServerOperationRequestFacade::new(self.operation_registry.clone())
                .admit_from_forge_native_admission(
                    &self.admission,
                    ForgeServerOperationRequestInput::builder()
                        .with_operation_family(ForgeServerOperationFamily::QueryDirectSubmission)
                        .with_operation_name(operation.operation_name())
                        .build(),
                ) {
                Ok(value) => value,
                Err(denial) => {
                    return TransitionOutcome::Denied(map_operation_request_denial(denial));
                }
            };
        if let Err(denial) =
            self.admit_operation_family(ForgeServerOperationFamily::QueryDirectSubmission)
        {
            return self.operation_denial_outcome(denial);
        }
        match self.prepare_declared_plan(
            operation_request.clone(),
            crate::ForgeServerQueryHandoffOperation::direct_mutation_execution(operation.clone()),
        ) {
            Ok(plan) => {
                let plan_proof = plan.proof();
                let query_handoff = plan.query_handoff();
                if let Err(error) = query_handoff
                    .workspace()
                    .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Inspect)
                {
                    return TransitionOutcome::Denied(ForgeServerQueryHandoffDenial::new(
                        ForgeServerQueryHandoffDenialCode::UnsupportedQueryFacadeFamily,
                        self.admission.request_context().diagnostics_profile(),
                        format!("query workspace does not admit `inspect` facade family: {error}"),
                    ));
                }
                let support_posture = query_handoff.support_posture().clone();
                let workspace_name = query_handoff.workspace().name().to_string();
                let handoff_digest = query_handoff.canonical_digest().to_string();
                let executed =
                    match crate::ForgeServerOperationScheduler::new(self.responses.clone())
                        .schedule_batch([plan])
                    {
                        Ok(batch) => batch.execute(),
                        Err(denial) => {
                            return TransitionOutcome::Denied(ForgeServerQueryHandoffDenial::new(
                                ForgeServerQueryHandoffDenialCode::DirectMutationContinuityDenied,
                                self.admission.request_context().diagnostics_profile(),
                                denial.detail(),
                            ));
                        }
                    };
                let outcome = &executed.outcomes()[0];
                if let Some(cancellation_posture) = outcome.cancellation_posture() {
                    return TransitionOutcome::Failed(crate::ForgeServerQueryHandoffFailure::new(
                        match cancellation_posture {
                            crate::ForgeServerSchedulerCancellationPosture::BeforeAdmission => {
                                "direct_mutation_cancelled_before_admission"
                            }
                            crate::ForgeServerSchedulerCancellationPosture::AfterAdmissionBeforeExecution => {
                                "direct_mutation_cancelled_after_admission_before_execution"
                            }
                            crate::ForgeServerSchedulerCancellationPosture::DuringExecution => {
                                "direct_mutation_cancelled_during_execution"
                            }
                        },
                    ));
                }
                if let Some(failure_posture) = outcome.failure_posture() {
                    return map_scheduler_failure_posture(self, failure_posture);
                }
                let mutation_result = map_scheduled_mutation_result(
                    outcome
                        .mutation_result()
                        .expect("scheduled direct mutation should carry a mutation result")
                        .clone(),
                );
                let response_envelope = outcome
                    .response_envelope()
                    .expect("scheduled direct mutation should shape a response envelope")
                    .clone();
                let direct_context = ForgeServerDirectContextArtifact::new(
                    self.admission.request_context(),
                    &support_posture,
                    &response_envelope,
                    None,
                    ForgeServerDirectRemaskPosture::visible(),
                );
                TransitionOutcome::Success(ForgeServerDirectMutation::new(
                    operation_request,
                    plan_proof,
                    support_posture,
                    workspace_name,
                    handoff_digest,
                    direct_context,
                    mutation_result,
                    response_envelope,
                ))
            }
            Err(denial) => TransitionOutcome::Denied(denial),
        }
    }
}

fn map_scheduled_mutation_result(
    mutation_result: ForgeServerScheduledMutationResult,
) -> ForgeServerDirectMutationResult {
    match mutation_result {
        ForgeServerScheduledMutationResult::Single {
            receipt,
            inspection,
        } => ForgeServerDirectMutationResult::Single {
            receipt,
            inspection,
        },
        ForgeServerScheduledMutationResult::Batch {
            receipt,
            inspection,
        } => ForgeServerDirectMutationResult::Batch {
            receipt,
            inspection,
        },
    }
}

fn map_scheduler_failure_posture(
    facade: &ForgeServerForgeNativeDirectFacade,
    failure_posture: &crate::ForgeServerSchedulerFailurePosture,
) -> ForgeServerDirectMutationOutcome {
    match failure_posture {
        crate::ForgeServerSchedulerFailurePosture::IsolatedRuntimeFailure { runtime_failure } => {
            facade.direct_mutation_scheduler_runtime_outcome(runtime_failure)
        }
        crate::ForgeServerSchedulerFailurePosture::DependentSharedBasisFailure { .. } => {
            TransitionOutcome::Failed(crate::ForgeServerQueryHandoffFailure::new(
                "direct_mutation_scheduler_dependent_failure",
            ))
        }
        crate::ForgeServerSchedulerFailurePosture::StaleMutationBasis { .. } => {
            TransitionOutcome::Failed(crate::ForgeServerQueryHandoffFailure::new(
                "direct_mutation_scheduler_stale_basis",
            ))
        }
        crate::ForgeServerSchedulerFailurePosture::OrderedLaneClosed { .. } => {
            TransitionOutcome::Failed(crate::ForgeServerQueryHandoffFailure::new(
                "direct_mutation_scheduler_lane_closed",
            ))
        }
    }
}

fn map_operation_request_denial(
    denial: ForgeServerOperationRequestDenial,
) -> ForgeServerQueryHandoffDenial {
    let code = match denial.code() {
        ForgeServerOperationRequestDenialCode::InvalidOperationName
        | ForgeServerOperationRequestDenialCode::MissingOperationName
        | ForgeServerOperationRequestDenialCode::CompatibilityBindingInvalid => {
            ForgeServerQueryHandoffDenialCode::DirectMutationNamingDenied
        }
        ForgeServerOperationRequestDenialCode::UnknownOperationName => {
            ForgeServerQueryHandoffDenialCode::UnknownOperationName
        }
        _ => ForgeServerQueryHandoffDenialCode::DirectMutationBindingDenied,
    };
    let rejected_operation_name = match denial.code() {
        ForgeServerOperationRequestDenialCode::UnknownOperationName => {
            denial.detail().split('`').nth(1).map(str::to_string)
        }
        _ => None,
    };
    let denial =
        ForgeServerQueryHandoffDenial::new(code, denial.diagnostics_profile(), denial.detail());
    match rejected_operation_name {
        Some(operation_name) => denial.with_facts(
            ForgeServerQueryHandoffDenialFacts::default()
                .with_rejected_operation_name(operation_name),
        ),
        None => denial,
    }
}
