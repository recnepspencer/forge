use forge_proof::TransitionOutcome;
use forge_query::facade::{ForgeQueryInspection, ForgeQueryRuntimeFacadeFamily};

use crate::{
    ForgeServerDirectContextArtifact, ForgeServerDirectMutation, ForgeServerDirectMutationResult,
    ForgeServerDirectRemaskPosture, ForgeServerQueryHandoffDenial,
    ForgeServerQueryHandoffDenialCode, ForgeServerQueryOperation, ForgeServerResponseInput,
};

use super::{ForgeServerDirectMutationOutcome, ForgeServerForgeNativeDirectFacade};

impl ForgeServerForgeNativeDirectFacade {
    pub fn mutate(
        &self,
        operation: &ForgeServerQueryOperation,
    ) -> ForgeServerDirectMutationOutcome {
        match self.prepare_handoff(operation.handoff_operation()) {
            TransitionOutcome::Success(mut handoff) => {
                if let Err(error) = handoff
                    .workspace()
                    .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Inspect)
                {
                    return TransitionOutcome::Denied(ForgeServerQueryHandoffDenial::new(
                        ForgeServerQueryHandoffDenialCode::UnsupportedQueryFacadeFamily,
                        self.admission.request_context().diagnostics_profile(),
                        format!("query workspace does not admit `inspect` facade family: {error}"),
                    ));
                }

                let mutation_result = match operation {
                    ForgeServerQueryOperation::SingleMutation { command, .. } => {
                        let receipt = match handoff
                            .workspace_mut()
                            .write_intent(command.clone())
                            .review()
                        {
                            Ok(review) => match review.admit() {
                                Ok(admitted) => match admitted.execute() {
                                    Ok(receipt) => receipt,
                                    Err(error) => return self.runtime_error_outcome(error),
                                },
                                Err(error) => return self.runtime_error_outcome(error),
                            },
                            Err(error) => return self.runtime_error_outcome(error),
                        };
                        let inspection = match handoff.workspace().inspect(&receipt) {
                            Ok(ForgeQueryInspection::WriteReceipt(inspection)) => inspection,
                            Ok(other) => panic!("expected write receipt inspection, got {other:?}"),
                            Err(error) => return self.runtime_error_outcome(error),
                        };
                        ForgeServerDirectMutationResult::Single {
                            receipt,
                            inspection,
                        }
                    }
                    ForgeServerQueryOperation::BatchMutation { commands, .. } => {
                        let receipt = match handoff
                            .workspace_mut()
                            .write_batch_intent(commands.clone())
                            .review()
                        {
                            Ok(review) => match review.admit() {
                                Ok(admitted) => match admitted.execute() {
                                    Ok(receipt) => receipt,
                                    Err(error) => return self.runtime_error_outcome(error),
                                },
                                Err(error) => return self.runtime_error_outcome(error),
                            },
                            Err(error) => return self.runtime_error_outcome(error),
                        };
                        let inspection = match handoff.workspace().inspect(&receipt) {
                            Ok(ForgeQueryInspection::BatchWriteReceipt(inspection)) => inspection,
                            Ok(other) => {
                                panic!("expected batch write receipt inspection, got {other:?}")
                            }
                            Err(error) => return self.runtime_error_outcome(error),
                        };
                        ForgeServerDirectMutationResult::Batch {
                            receipt,
                            inspection,
                        }
                    }
                };

                let support_posture = handoff.support_posture().clone();
                let workspace_name = handoff.workspace().name().to_string();
                let handoff_digest = handoff.canonical_digest().to_string();
                let response_envelope = self
                    .responses
                    .shape_with_defaults(ForgeServerResponseInput::query_handoff_success(handoff));
                let direct_context = ForgeServerDirectContextArtifact::new(
                    self.admission.request_context(),
                    &support_posture,
                    &response_envelope,
                    None,
                    ForgeServerDirectRemaskPosture::visible(),
                );
                TransitionOutcome::Success(ForgeServerDirectMutation::new(
                    support_posture,
                    workspace_name,
                    handoff_digest,
                    direct_context,
                    mutation_result,
                    response_envelope,
                ))
            }
            TransitionOutcome::Denied(denial) => TransitionOutcome::Denied(denial),
            TransitionOutcome::Deferred(deferred) => TransitionOutcome::Deferred(deferred),
            TransitionOutcome::Stale(stale) => TransitionOutcome::Stale(stale),
            TransitionOutcome::RebindRequired(rebind) => TransitionOutcome::RebindRequired(rebind),
            TransitionOutcome::Failed(failure) => TransitionOutcome::Failed(failure),
        }
    }
}
