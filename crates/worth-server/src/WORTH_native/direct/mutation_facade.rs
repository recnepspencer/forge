use worth_proof::TransitionOutcome;
use worth_query::facade::runtime::WorthQueryRuntimeFacadeFamily;

use crate::{
    WorthServerDirectContextArtifact, WorthServerDirectMutation, WorthServerDirectMutationResult,
    WorthServerDirectRemaskPosture, WorthServerOperationFamily, WorthServerOperationRequestDenial,
    WorthServerOperationRequestDenialCode, WorthServerOperationRequestFacade,
    WorthServerOperationRequestInput, WorthServerQueryHandoffDenial,
    WorthServerQueryHandoffDenialCode, WorthServerQueryHandoffDenialFacts,
    WorthServerQueryOperation, WorthServerScheduledMutationResult,
};

use super::{WorthServerDirectMutationOutcome, WorthServerWorthNativeDirectFacade};

impl WorthServerWorthNativeDirectFacade {
    pub fn mutate(
        &self,
        operation: &WorthServerQueryOperation,
    ) -> WorthServerDirectMutationOutcome {
        let operation_request =
            match WorthServerOperationRequestFacade::new(self.operation_registry.clone())
                .admit_from_worth_native_admission(
                    &self.admission,
                    WorthServerOperationRequestInput::builder()
                        .with_operation_family(WorthServerOperationFamily::QueryDirectSubmission)
                        .with_operation_name(operation.operation_name())
                        .build(),
                ) {
                Ok(value) => value,
                Err(denial) => {
                    return TransitionOutcome::Denied(map_operation_request_denial(denial));
                }
            };
        if let Err(denial) =
            self.admit_operation_family(WorthServerOperationFamily::QueryDirectSubmission)
        {
            return self.operation_denial_outcome(denial);
        }
        match self.prepare_declared_plan(
            operation_request.clone(),
            crate::WorthServerQueryHandoffOperation::direct_mutation_execution(operation.clone()),
        ) {
            Ok(plan) => {
                let plan_proof = plan.proof();
                let query_handoff = plan.query_handoff();
                if let Err(error) = query_handoff
                    .workspace()
                    .admit_public_api_family(WorthQueryRuntimeFacadeFamily::Inspect)
                {
                    return TransitionOutcome::Denied(WorthServerQueryHandoffDenial::new(
                        WorthServerQueryHandoffDenialCode::UnsupportedQueryFacadeFamily,
                        self.admission.request_context().diagnostics_profile(),
                        format!("query workspace does not admit `inspect` facade family: {error}"),
                    ));
                }
                let support_posture = query_handoff.support_posture().clone();
                let workspace_name = query_handoff.workspace().name().to_string();
                let handoff_digest = query_handoff.canonical_digest().to_string();
                let executed =
                    match crate::WorthServerOperationScheduler::new(self.responses.clone())
                        .schedule_batch([plan])
                    {
                        Ok(batch) => batch.execute(),
                        Err(denial) => {
                            return TransitionOutcome::Denied(WorthServerQueryHandoffDenial::new(
                                WorthServerQueryHandoffDenialCode::DirectMutationContinuityDenied,
                                self.admission.request_context().diagnostics_profile(),
                                denial.detail(),
                            ));
                        }
                    };
                let outcome = &executed.outcomes()[0];
                if let Some(cancellation_posture) = outcome.cancellation_posture() {
                    return TransitionOutcome::Failed(crate::WorthServerQueryHandoffFailure::new(
                        match cancellation_posture {
                            crate::WorthServerSchedulerCancellationPosture::BeforeAdmission => {
                                "direct_mutation_cancelled_before_admission"
                            }
                            crate::WorthServerSchedulerCancellationPosture::AfterAdmissionBeforeExecution => {
                                "direct_mutation_cancelled_after_admission_before_execution"
                            }
                            crate::WorthServerSchedulerCancellationPosture::DuringExecution => {
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
                let direct_context = WorthServerDirectContextArtifact::new(
                    self.admission.request_context(),
                    &support_posture,
                    &response_envelope,
                    None,
                    WorthServerDirectRemaskPosture::visible(),
                );
                TransitionOutcome::Success(WorthServerDirectMutation::new(
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
    mutation_result: WorthServerScheduledMutationResult,
) -> WorthServerDirectMutationResult {
    match mutation_result {
        WorthServerScheduledMutationResult::Single {
            receipt,
            inspection,
        } => WorthServerDirectMutationResult::Single {
            receipt,
            inspection,
        },
        WorthServerScheduledMutationResult::Batch {
            receipt,
            inspection,
        } => WorthServerDirectMutationResult::Batch {
            receipt,
            inspection,
        },
    }
}

fn map_scheduler_failure_posture(
    facade: &WorthServerWorthNativeDirectFacade,
    failure_posture: &crate::WorthServerSchedulerFailurePosture,
) -> WorthServerDirectMutationOutcome {
    match failure_posture {
        crate::WorthServerSchedulerFailurePosture::IsolatedRuntimeFailure { runtime_failure } => {
            facade.direct_mutation_scheduler_runtime_outcome(runtime_failure)
        }
        crate::WorthServerSchedulerFailurePosture::DependentSharedBasisFailure { .. } => {
            TransitionOutcome::Failed(crate::WorthServerQueryHandoffFailure::new(
                "direct_mutation_scheduler_dependent_failure",
            ))
        }
        crate::WorthServerSchedulerFailurePosture::StaleMutationBasis { .. } => {
            TransitionOutcome::Failed(crate::WorthServerQueryHandoffFailure::new(
                "direct_mutation_scheduler_stale_basis",
            ))
        }
        crate::WorthServerSchedulerFailurePosture::OrderedLaneClosed { .. } => {
            TransitionOutcome::Failed(crate::WorthServerQueryHandoffFailure::new(
                "direct_mutation_scheduler_lane_closed",
            ))
        }
    }
}

fn map_operation_request_denial(
    denial: WorthServerOperationRequestDenial,
) -> WorthServerQueryHandoffDenial {
    let code = match denial.code() {
        WorthServerOperationRequestDenialCode::InvalidOperationName
        | WorthServerOperationRequestDenialCode::MissingOperationName
        | WorthServerOperationRequestDenialCode::CompatibilityBindingInvalid => {
            WorthServerQueryHandoffDenialCode::DirectMutationNamingDenied
        }
        WorthServerOperationRequestDenialCode::UnknownOperationName => {
            WorthServerQueryHandoffDenialCode::UnknownOperationName
        }
        _ => WorthServerQueryHandoffDenialCode::DirectMutationBindingDenied,
    };
    let rejected_operation_name = match denial.code() {
        WorthServerOperationRequestDenialCode::UnknownOperationName => {
            denial.detail().split('`').nth(1).map(str::to_string)
        }
        _ => None,
    };
    let denial =
        WorthServerQueryHandoffDenial::new(code, denial.diagnostics_profile(), denial.detail());
    match rejected_operation_name {
        Some(operation_name) => denial.with_facts(
            WorthServerQueryHandoffDenialFacts::default()
                .with_rejected_operation_name(operation_name),
        ),
        None => denial,
    }
}
