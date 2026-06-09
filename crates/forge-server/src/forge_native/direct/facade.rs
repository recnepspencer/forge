use forge_proof::TransitionOutcome;
use forge_query::facade::{ForgeQueryInspection, ForgeQueryRuntimeError};

use crate::{
    declaration_intake::{
        ForgeServerDirectDeclarationIntakeFacade, ForgeServerNamedLiveProjectionExecutionError,
    },
    ForgeServerAdmission, ForgeServerAdmittedDirectDeclaration, ForgeServerDirectProjectionRequest,
    ForgeServerQueryHandoffFacade, ForgeServerQueryHandoffInput, ForgeServerQueryHandoffOperation,
    ForgeServerQueryHandoffOutcome, ForgeServerResponseFacade, ForgeServerResponseInput,
};

use super::{
    ForgeServerDirectContextArtifact, ForgeServerDirectInspection, ForgeServerDirectProjection,
    ForgeServerDirectRead, ForgeServerDirectRemaskPosture, ForgeServerDirectState,
};

pub type ForgeServerDirectReadOutcome = TransitionOutcome<
    ForgeServerDirectRead,
    crate::ForgeServerQueryHandoffDenial,
    crate::ForgeServerQueryHandoffDeferred,
    crate::ForgeServerQueryHandoffStale,
    crate::ForgeServerQueryHandoffRebindRequired,
    crate::ForgeServerQueryHandoffFailure,
>;

pub type ForgeServerDirectStateOutcome = TransitionOutcome<
    ForgeServerDirectState,
    crate::ForgeServerQueryHandoffDenial,
    crate::ForgeServerQueryHandoffDeferred,
    crate::ForgeServerQueryHandoffStale,
    crate::ForgeServerQueryHandoffRebindRequired,
    crate::ForgeServerQueryHandoffFailure,
>;

pub type ForgeServerDirectInspectionOutcome = TransitionOutcome<
    ForgeServerDirectInspection,
    crate::ForgeServerQueryHandoffDenial,
    crate::ForgeServerQueryHandoffDeferred,
    crate::ForgeServerQueryHandoffStale,
    crate::ForgeServerQueryHandoffRebindRequired,
    crate::ForgeServerQueryHandoffFailure,
>;

pub type ForgeServerDirectProjectionOutcome = TransitionOutcome<
    ForgeServerDirectProjection,
    crate::ForgeServerQueryHandoffDenial,
    crate::ForgeServerQueryHandoffDeferred,
    crate::ForgeServerQueryHandoffStale,
    crate::ForgeServerQueryHandoffRebindRequired,
    crate::ForgeServerQueryHandoffFailure,
>;

pub type ForgeServerDirectMutationOutcome = TransitionOutcome<
    crate::ForgeServerDirectMutation,
    crate::ForgeServerQueryHandoffDenial,
    crate::ForgeServerQueryHandoffDeferred,
    crate::ForgeServerQueryHandoffStale,
    crate::ForgeServerQueryHandoffRebindRequired,
    crate::ForgeServerQueryHandoffFailure,
>;

#[derive(Clone, Debug)]
pub struct ForgeServerForgeNativeDirectFacade {
    pub(super) admission: ForgeServerAdmission,
    pub(super) declaration_intake: ForgeServerDirectDeclarationIntakeFacade,
    pub(super) query_handoff: ForgeServerQueryHandoffFacade,
    pub(super) responses: ForgeServerResponseFacade,
}

impl ForgeServerForgeNativeDirectFacade {
    pub(crate) fn new(
        admission: ForgeServerAdmission,
        declaration_intake: ForgeServerDirectDeclarationIntakeFacade,
        query_handoff: ForgeServerQueryHandoffFacade,
        responses: ForgeServerResponseFacade,
    ) -> Self {
        Self {
            admission,
            declaration_intake,
            query_handoff,
            responses,
        }
    }

    pub fn read(
        &self,
        declaration: &ForgeServerAdmittedDirectDeclaration,
    ) -> ForgeServerDirectReadOutcome {
        match self.prepare_handoff(ForgeServerQueryHandoffOperation::direct_read(
            declaration.declaration_binding_label(),
        )) {
            TransitionOutcome::Success(handoff) => {
                let read_result = match declaration.execute_named_live_read() {
                    Ok(read_result) => read_result,
                    Err(error) => return self.runtime_error_outcome(error),
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
                    declaration.subscription_basis_digest().ok().as_deref(),
                    ForgeServerDirectRemaskPosture::visible(),
                );
                TransitionOutcome::Success(ForgeServerDirectRead::new(
                    support_posture,
                    workspace_name,
                    handoff_digest,
                    direct_context,
                    read_result,
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

    pub fn state(
        &self,
        declaration: &ForgeServerAdmittedDirectDeclaration,
    ) -> ForgeServerDirectStateOutcome {
        match self.prepare_handoff(ForgeServerQueryHandoffOperation::direct_state(
            declaration.declaration_canonical_label(),
        )) {
            TransitionOutcome::Success(handoff) => {
                let runtime_state = match declaration.snapshot_named_live_state() {
                    Ok(runtime_state) => runtime_state,
                    Err(error) => return self.runtime_error_outcome(error),
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
                    Some(runtime_state.basis_digest()),
                    ForgeServerDirectRemaskPosture::from_state_snapshot(&runtime_state),
                );
                TransitionOutcome::Success(ForgeServerDirectState::new(
                    support_posture,
                    workspace_name,
                    handoff_digest,
                    direct_context,
                    runtime_state,
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

    pub fn inspect(
        &self,
        declaration: &ForgeServerAdmittedDirectDeclaration,
    ) -> ForgeServerDirectInspectionOutcome {
        match self.prepare_handoff(ForgeServerQueryHandoffOperation::direct_inspection(
            declaration.declaration_canonical_label(),
        )) {
            TransitionOutcome::Success(handoff) => {
                let inspection_result = match declaration.inspect_named_live_view() {
                    Ok(inspection_result) => inspection_result,
                    Err(error) => return self.runtime_error_outcome(error),
                };
                let support_posture = handoff.support_posture().clone();
                let workspace_name = handoff.workspace().name().to_string();
                let handoff_digest = handoff.canonical_digest().to_string();
                let response_envelope = self
                    .responses
                    .shape_with_defaults(ForgeServerResponseInput::query_handoff_success(handoff));
                let (basis_digest, remask_posture) = match inspection_result.inspection() {
                    ForgeQueryInspection::LiveView(live) => (
                        Some(live.basis_binding_digest()),
                        ForgeServerDirectRemaskPosture::from_live_inspection(live),
                    ),
                    _ => (None, ForgeServerDirectRemaskPosture::visible()),
                };
                let direct_context = ForgeServerDirectContextArtifact::new(
                    self.admission.request_context(),
                    &support_posture,
                    &response_envelope,
                    basis_digest,
                    remask_posture,
                );
                TransitionOutcome::Success(ForgeServerDirectInspection::new(
                    support_posture,
                    workspace_name,
                    handoff_digest,
                    direct_context,
                    inspection_result,
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

    pub fn project(
        &self,
        declaration: &ForgeServerAdmittedDirectDeclaration,
        request: &ForgeServerDirectProjectionRequest,
    ) -> ForgeServerDirectProjectionOutcome {
        match self.prepare_handoff(ForgeServerQueryHandoffOperation::direct_projection(
            declaration.declaration_canonical_label(),
        )) {
            TransitionOutcome::Success(handoff) => {
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
            TransitionOutcome::Denied(denial) => TransitionOutcome::Denied(denial),
            TransitionOutcome::Deferred(deferred) => TransitionOutcome::Deferred(deferred),
            TransitionOutcome::Stale(stale) => TransitionOutcome::Stale(stale),
            TransitionOutcome::RebindRequired(rebind) => TransitionOutcome::RebindRequired(rebind),
            TransitionOutcome::Failed(failure) => TransitionOutcome::Failed(failure),
        }
    }

    pub(super) fn prepare_handoff(
        &self,
        operation: ForgeServerQueryHandoffOperation,
    ) -> ForgeServerQueryHandoffOutcome {
        self.query_handoff
            .prepare(ForgeServerQueryHandoffInput::new(
                self.admission.clone(),
                operation,
            ))
    }

    pub(super) fn runtime_error_outcome<T>(
        &self,
        error: ForgeQueryRuntimeError,
    ) -> TransitionOutcome<
        T,
        crate::ForgeServerQueryHandoffDenial,
        crate::ForgeServerQueryHandoffDeferred,
        crate::ForgeServerQueryHandoffStale,
        crate::ForgeServerQueryHandoffRebindRequired,
        crate::ForgeServerQueryHandoffFailure,
    > {
        match error {
            ForgeQueryRuntimeError::MissingLiveView(_)
            | ForgeQueryRuntimeError::MissingLiveSubscription(_) => {
                TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                    crate::ForgeServerQueryHandoffDenialCode::RetainedQueryArtifactUnavailable,
                    self.admission.request_context().diagnostics_profile(),
                    error.to_string(),
                ))
            }
            ForgeQueryRuntimeError::MutationBindingDenied(_) => {
                TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                    crate::ForgeServerQueryHandoffDenialCode::DirectMutationBindingDenied,
                    self.admission.request_context().diagnostics_profile(),
                    error.to_string(),
                ))
            }
            ForgeQueryRuntimeError::ExistingTruthAssertionDenied(_) => {
                TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                    crate::ForgeServerQueryHandoffDenialCode::DirectMutationAssertionDenied,
                    self.admission.request_context().diagnostics_profile(),
                    error.to_string(),
                ))
            }
            ForgeQueryRuntimeError::MutationContinuityDenied(_) => {
                TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                    crate::ForgeServerQueryHandoffDenialCode::DirectMutationContinuityDenied,
                    self.admission.request_context().diagnostics_profile(),
                    error.to_string(),
                ))
            }
            ForgeQueryRuntimeError::MutationNamingDenied(_) => {
                TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                    crate::ForgeServerQueryHandoffDenialCode::DirectMutationNamingDenied,
                    self.admission.request_context().diagnostics_profile(),
                    error.to_string(),
                ))
            }
            ForgeQueryRuntimeError::MutationTargetReferenceDenied(_) => {
                TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                    crate::ForgeServerQueryHandoffDenialCode::DirectMutationTargetReferenceDenied,
                    self.admission.request_context().diagnostics_profile(),
                    error.to_string(),
                ))
            }
            _ => TransitionOutcome::failed(crate::ForgeServerQueryHandoffFailure::new(
                "direct_query_execution_failed",
            )),
        }
    }

    pub(super) fn projection_error_outcome(
        &self,
        error: forge_query::facade::ProjectionFactConsumptionPathError,
    ) -> ForgeServerDirectProjectionOutcome {
        match error {
            forge_query::facade::ProjectionFactConsumptionPathError::Declaration(detail) => {
                TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                    crate::ForgeServerQueryHandoffDenialCode::DirectProjectionBindingInvalid,
                    self.admission.request_context().diagnostics_profile(),
                    format!("{detail:?}"),
                ))
            }
            forge_query::facade::ProjectionFactConsumptionPathError::Extraction(_) => {
                TransitionOutcome::failed(crate::ForgeServerQueryHandoffFailure::new(
                    "direct_projection_extraction_failed",
                ))
            }
        }
    }
}
