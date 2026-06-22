use forge_proof::TransitionOutcome;

use crate::{
    declaration_intake::ForgeServerNamedLiveProjectionExecutionError,
    ForgeServerAdmittedDirectDeclaration, ForgeServerDirectProjectionRequest,
    ForgeServerOperationFamily, ForgeServerQueryHandoffOperation, ForgeServerResponseInput,
};

use super::{
    ForgeServerDirectContextArtifact, ForgeServerDirectProjection, ForgeServerDirectRemaskPosture,
    ForgeServerForgeNativeDirectFacade,
};

impl ForgeServerForgeNativeDirectFacade {
    pub fn project(
        &self,
        declaration: &ForgeServerAdmittedDirectDeclaration,
        request: &ForgeServerDirectProjectionRequest,
    ) -> super::ForgeServerDirectProjectionOutcome {
        if let Err(denial) =
            self.admit_operation_family(ForgeServerOperationFamily::QueryDirectProjection)
        {
            return self.operation_denial_outcome(denial);
        }
        let observed_basis_digest = match declaration.subscription_basis_digest() {
            Ok(value) => value,
            Err(error) => return self.runtime_error_outcome(error),
        };
        let operation_name = declaration.declaration_binding_label().to_string();
        let handoff_target = declaration.declaration_canonical_label();
        let operation_request = match self.admit_shared_read_operation_request(
            ForgeServerOperationFamily::QueryDirectProjection,
            &operation_name,
            &observed_basis_digest,
        ) {
            Ok(value) => value,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        match self.prepare_declared_plan(
            operation_request,
            ForgeServerQueryHandoffOperation::direct_projection(&handoff_target),
        ) {
            Ok(plan) => {
                let plan_proof = plan.proof();
                let handoff = plan.into_query_handoff();
                let projection_attempt = match declaration.consume_named_live_projection(request) {
                    Ok(projection_attempt) => projection_attempt,
                    Err(ForgeServerNamedLiveProjectionExecutionError::Runtime(error)) => {
                        return self.runtime_error_outcome(error);
                    }
                    Err(ForgeServerNamedLiveProjectionExecutionError::Consumption(path_error)) => {
                        return self.projection_error_outcome(path_error);
                    }
                };
                let support_posture = handoff.support_posture().clone();
                let workspace_name = handoff.workspace().name().to_string();
                let handoff_digest = handoff.canonical_digest().to_string();
                let response_envelope = self
                    .responses
                    .shape_with_defaults(ForgeServerResponseInput::query_handoff_success(handoff));
                match projection_attempt {
                    forge_query::facade::ProjectionFactConsumptionAttempt::Admitted(completed) => {
                        let direct_context = ForgeServerDirectContextArtifact::new(
                            self.admission.request_context(),
                            &support_posture,
                            &response_envelope,
                            completed.contract().basis_digest(),
                            ForgeServerDirectRemaskPosture::from_materialized_fact_posture(
                                completed.materialized_fact_posture(),
                            ),
                        );
                        TransitionOutcome::Success(ForgeServerDirectProjection::from_completed(
                            plan_proof.clone(),
                            support_posture,
                            workspace_name,
                            handoff_digest,
                            direct_context,
                            completed,
                            Vec::new(),
                            response_envelope,
                        ))
                    }
                    forge_query::facade::ProjectionFactConsumptionAttempt::AdmittedWithWarnings(
                        completed,
                        warnings,
                    ) => {
                        let direct_context = ForgeServerDirectContextArtifact::new(
                            self.admission.request_context(),
                            &support_posture,
                            &response_envelope,
                            completed.contract().basis_digest(),
                            ForgeServerDirectRemaskPosture::from_materialized_fact_posture(
                                completed.materialized_fact_posture(),
                            ),
                        );
                        TransitionOutcome::Success(ForgeServerDirectProjection::from_completed(
                            plan_proof,
                            support_posture,
                            workspace_name,
                            handoff_digest,
                            direct_context,
                            completed,
                            warnings.warning_kinds().to_vec(),
                            response_envelope,
                        ))
                    }
                    forge_query::facade::ProjectionFactConsumptionAttempt::Denied(denied) => {
                        TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                            crate::ForgeServerQueryHandoffDenialCode::ProjectionFactConsumptionDenied,
                            self.admission.request_context().diagnostics_profile(),
                            format!("{:?}", denied.reason()),
                        ))
                    }
                    forge_query::facade::ProjectionFactConsumptionAttempt::Deferred(deferred) => {
                        TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                            crate::ForgeServerQueryHandoffDenialCode::ProjectionFactConsumptionDeferred,
                            self.admission.request_context().diagnostics_profile(),
                            format!("{:?}", deferred.reason()),
                        ))
                    }
                    forge_query::facade::ProjectionFactConsumptionAttempt::SourceMismatch(
                        mismatch,
                    ) => TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                        crate::ForgeServerQueryHandoffDenialCode::ProjectionFactConsumptionSourceMismatch,
                        self.admission.request_context().diagnostics_profile(),
                        format!(
                            "{:?} unsupported for {:?}",
                            mismatch.requested_fact_kind(),
                            mismatch.source_family()
                        ),
                    )),
                }
            }
            Err(denial) => TransitionOutcome::Denied(denial),
        }
    }
}
