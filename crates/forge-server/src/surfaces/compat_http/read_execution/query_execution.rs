use forge_proof::TransitionOutcome;
use forge_query::facade::ForgeQueryInspection;

use crate::{
    ForgeServerAdmittedDirectDeclaration, ForgeServerCompatibilityFacade,
    ForgeServerCompatibilityInspection, ForgeServerCompatibilityPreparedRequest,
    ForgeServerCompatibilityRead, ForgeServerCompatibilityState, ForgeServerDirectContextArtifact,
    ForgeServerDirectRemaskPosture, ForgeServerQueryHandoffDenial, ForgeServerQueryHandoffFailure,
    ForgeServerQueryHandoffInput, ForgeServerQueryHandoffOperation, ForgeServerReadValidator,
    ForgeServerResponseInput,
};

use super::{
    basis::ForgeServerExternalBasisRequest,
    cache_policy::ForgeServerCompatibilityCachePolicy,
    conditional::ForgeServerConditionalRead,
    execution::{
        admit_declaration, named_read_declaration, ForgeServerCompatibilityExecutionInput,
        ForgeServerCompatibilityExecutionOutcome,
    },
};

impl ForgeServerCompatibilityFacade {
    pub fn read(
        &self,
        input: ForgeServerCompatibilityExecutionInput,
    ) -> ForgeServerCompatibilityExecutionOutcome<ForgeServerCompatibilityRead> {
        let (prepared_request, operation_name) = input.into_parts();
        let basis_request = match compatibility_basis_request(&prepared_request) {
            Ok(value) => value,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        let conditional_read =
            match ForgeServerConditionalRead::from_prepared_request(&prepared_request) {
                Ok(value) => value,
                Err(denial) => return TransitionOutcome::Denied(denial),
            };
        let declaration =
            match admitted_named_read_declaration(self, &prepared_request, &operation_name) {
                Ok(value) => value,
                Err(denial) => return TransitionOutcome::Denied(denial),
            };
        let observed_basis_digest = match declaration.subscription_basis_digest() {
            Ok(value) => value,
            Err(error) => return runtime_error_outcome(&prepared_request, error),
        };
        if let Err(denial) = basis_request.validate_observed_basis(
            prepared_request
                .admission()
                .request_context()
                .diagnostics_profile(),
            Some(&observed_basis_digest),
        ) {
            return TransitionOutcome::Denied(denial);
        }

        let handoff = match compatibility_handoff(
            self,
            &prepared_request,
            ForgeServerQueryHandoffOperation::direct_read(&operation_name),
        ) {
            TransitionOutcome::Success(value) => value,
            TransitionOutcome::Denied(denial) => return TransitionOutcome::Denied(denial),
            TransitionOutcome::Deferred(value) => return TransitionOutcome::Deferred(value),
            TransitionOutcome::Stale(value) => return TransitionOutcome::Stale(value),
            TransitionOutcome::RebindRequired(value) => {
                return TransitionOutcome::RebindRequired(value);
            }
            TransitionOutcome::Failed(value) => return TransitionOutcome::Failed(value),
        };
        let read_result = match declaration.execute_named_live_read() {
            Ok(value) => value,
            Err(error) => return runtime_error_outcome(&prepared_request, error),
        };
        let support_posture = handoff.support_posture().clone();
        let workspace_name = handoff.workspace().name().to_string();
        let handoff_digest = handoff.canonical_digest().to_string();
        let response_envelope = self
            .responses
            .shape_with_defaults(ForgeServerResponseInput::query_handoff_success(handoff));
        let direct_context = ForgeServerDirectContextArtifact::new(
            prepared_request.admission().request_context(),
            &support_posture,
            &response_envelope,
            Some(&observed_basis_digest),
            ForgeServerDirectRemaskPosture::visible(),
        );
        let validator = ForgeServerReadValidator::new(
            read_result.receipt().result_digest(),
            direct_context.basis_digest(),
        );
        if let Err(denial) =
            validate_conditional_read(&prepared_request, &conditional_read, &validator)
        {
            return TransitionOutcome::Denied(denial);
        }
        let cache_policy = ForgeServerCompatibilityCachePolicy::for_scoped_read(
            &prepared_request,
            direct_context.remask_posture(),
        );
        TransitionOutcome::Success(ForgeServerCompatibilityRead::new(
            support_posture,
            workspace_name,
            declaration.declaration_digest().to_string(),
            handoff_digest,
            direct_context,
            basis_request,
            conditional_read,
            read_result,
            response_envelope,
            validator,
            cache_policy,
        ))
    }

    pub fn state(
        &self,
        input: ForgeServerCompatibilityExecutionInput,
    ) -> ForgeServerCompatibilityExecutionOutcome<ForgeServerCompatibilityState> {
        let (prepared_request, operation_name) = input.into_parts();
        let basis_request = match compatibility_basis_request(&prepared_request) {
            Ok(value) => value,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        let declaration =
            match admitted_named_read_declaration(self, &prepared_request, &operation_name) {
                Ok(value) => value,
                Err(denial) => return TransitionOutcome::Denied(denial),
            };
        let observed_basis_digest = match declaration.subscription_basis_digest() {
            Ok(value) => value,
            Err(error) => return runtime_error_outcome(&prepared_request, error),
        };
        if let Err(denial) = basis_request.validate_observed_basis(
            prepared_request
                .admission()
                .request_context()
                .diagnostics_profile(),
            Some(&observed_basis_digest),
        ) {
            return TransitionOutcome::Denied(denial);
        }

        let handoff = match compatibility_handoff(
            self,
            &prepared_request,
            ForgeServerQueryHandoffOperation::direct_state(
                declaration.declaration_canonical_label(),
            ),
        ) {
            TransitionOutcome::Success(value) => value,
            TransitionOutcome::Denied(denial) => return TransitionOutcome::Denied(denial),
            TransitionOutcome::Deferred(value) => return TransitionOutcome::Deferred(value),
            TransitionOutcome::Stale(value) => return TransitionOutcome::Stale(value),
            TransitionOutcome::RebindRequired(value) => {
                return TransitionOutcome::RebindRequired(value);
            }
            TransitionOutcome::Failed(value) => return TransitionOutcome::Failed(value),
        };
        let runtime_state = match declaration.snapshot_named_live_state() {
            Ok(value) => value,
            Err(error) => return runtime_error_outcome(&prepared_request, error),
        };
        let support_posture = handoff.support_posture().clone();
        let workspace_name = handoff.workspace().name().to_string();
        let handoff_digest = handoff.canonical_digest().to_string();
        let response_envelope = self
            .responses
            .shape_with_defaults(ForgeServerResponseInput::query_handoff_success(handoff));
        let direct_context = ForgeServerDirectContextArtifact::new(
            prepared_request.admission().request_context(),
            &support_posture,
            &response_envelope,
            Some(runtime_state.basis_digest()),
            ForgeServerDirectRemaskPosture::from_state_snapshot(&runtime_state),
        );
        let validator = ForgeServerReadValidator::new(
            runtime_state.state_digest(),
            direct_context.basis_digest(),
        );
        let cache_policy = ForgeServerCompatibilityCachePolicy::for_scoped_read(
            &prepared_request,
            direct_context.remask_posture(),
        );
        TransitionOutcome::Success(ForgeServerCompatibilityState::new(
            support_posture,
            workspace_name,
            declaration.declaration_digest().to_string(),
            handoff_digest,
            direct_context,
            basis_request,
            runtime_state,
            response_envelope,
            validator,
            cache_policy,
        ))
    }

    pub fn inspect(
        &self,
        input: ForgeServerCompatibilityExecutionInput,
    ) -> ForgeServerCompatibilityExecutionOutcome<ForgeServerCompatibilityInspection> {
        let (prepared_request, operation_name) = input.into_parts();
        let basis_request = match compatibility_basis_request(&prepared_request) {
            Ok(value) => value,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        let declaration =
            match admitted_named_read_declaration(self, &prepared_request, &operation_name) {
                Ok(value) => value,
                Err(denial) => return TransitionOutcome::Denied(denial),
            };
        let observed_basis_digest = match declaration.subscription_basis_digest() {
            Ok(value) => value,
            Err(error) => return runtime_error_outcome(&prepared_request, error),
        };
        if let Err(denial) = basis_request.validate_observed_basis(
            prepared_request
                .admission()
                .request_context()
                .diagnostics_profile(),
            Some(&observed_basis_digest),
        ) {
            return TransitionOutcome::Denied(denial);
        }

        let handoff = match compatibility_handoff(
            self,
            &prepared_request,
            ForgeServerQueryHandoffOperation::direct_inspection(
                declaration.declaration_canonical_label(),
            ),
        ) {
            TransitionOutcome::Success(value) => value,
            TransitionOutcome::Denied(denial) => return TransitionOutcome::Denied(denial),
            TransitionOutcome::Deferred(value) => return TransitionOutcome::Deferred(value),
            TransitionOutcome::Stale(value) => return TransitionOutcome::Stale(value),
            TransitionOutcome::RebindRequired(value) => {
                return TransitionOutcome::RebindRequired(value);
            }
            TransitionOutcome::Failed(value) => return TransitionOutcome::Failed(value),
        };
        let inspection_result = match declaration.inspect_named_live_view() {
            Ok(value) => value,
            Err(error) => return runtime_error_outcome(&prepared_request, error),
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
            prepared_request.admission().request_context(),
            &support_posture,
            &response_envelope,
            basis_digest,
            remask_posture,
        );
        let validator = ForgeServerReadValidator::new(
            inspection_result.receipt().result_digest(),
            direct_context.basis_digest(),
        );
        let cache_policy = ForgeServerCompatibilityCachePolicy::for_scoped_read(
            &prepared_request,
            direct_context.remask_posture(),
        );
        TransitionOutcome::Success(ForgeServerCompatibilityInspection::new(
            support_posture,
            workspace_name,
            declaration.declaration_digest().to_string(),
            handoff_digest,
            direct_context,
            basis_request,
            inspection_result,
            response_envelope,
            validator,
            cache_policy,
        ))
    }
}

fn compatibility_basis_request(
    prepared_request: &ForgeServerCompatibilityPreparedRequest,
) -> Result<ForgeServerExternalBasisRequest, ForgeServerQueryHandoffDenial> {
    ForgeServerExternalBasisRequest::from_prepared_request(prepared_request)
}

fn admitted_named_read_declaration(
    facade: &ForgeServerCompatibilityFacade,
    prepared_request: &ForgeServerCompatibilityPreparedRequest,
    operation_name: &str,
) -> Result<ForgeServerAdmittedDirectDeclaration, ForgeServerQueryHandoffDenial> {
    admit_declaration(
        &facade.declaration_intake,
        prepared_request.admission().clone(),
        named_read_declaration(operation_name),
    )
}

fn compatibility_handoff(
    facade: &ForgeServerCompatibilityFacade,
    prepared_request: &ForgeServerCompatibilityPreparedRequest,
    operation: ForgeServerQueryHandoffOperation,
) -> crate::ForgeServerQueryHandoffOutcome {
    facade
        .query_handoff
        .prepare(ForgeServerQueryHandoffInput::new(
            prepared_request.admission().clone(),
            operation,
        ))
}

fn validate_conditional_read(
    prepared_request: &ForgeServerCompatibilityPreparedRequest,
    conditional_read: &ForgeServerConditionalRead,
    validator: &ForgeServerReadValidator,
) -> Result<(), ForgeServerQueryHandoffDenial> {
    if let Some(expected) = conditional_read.if_match() {
        if expected != validator.entity_tag() {
            return Err(crate::ForgeServerQueryHandoffDenial::new(
                crate::ForgeServerQueryHandoffDenialCode::CompatibilityConditionalReadPreconditionFailed,
                prepared_request.admission().request_context().diagnostics_profile(),
                "compatibility if-match validator does not match the canonical read validator",
            ));
        }
    }
    if let Some(expected) = conditional_read.if_none_match() {
        if expected == validator.entity_tag() {
            return Err(crate::ForgeServerQueryHandoffDenial::new(
                crate::ForgeServerQueryHandoffDenialCode::CompatibilityConditionalReadNotModified,
                prepared_request.admission().request_context().diagnostics_profile(),
                "compatibility if-none-match validator already matches the canonical read validator",
            ));
        }
    }
    Ok(())
}

fn runtime_error_outcome<T>(
    prepared_request: &ForgeServerCompatibilityPreparedRequest,
    error: forge_query::facade::ForgeQueryRuntimeError,
) -> ForgeServerCompatibilityExecutionOutcome<T> {
    match error {
        forge_query::facade::ForgeQueryRuntimeError::MissingLiveView(_)
        | forge_query::facade::ForgeQueryRuntimeError::MissingLiveSubscription(_) => {
            TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                crate::ForgeServerQueryHandoffDenialCode::RetainedQueryArtifactUnavailable,
                prepared_request
                    .admission()
                    .request_context()
                    .diagnostics_profile(),
                error.to_string(),
            ))
        }
        _ => TransitionOutcome::Failed(ForgeServerQueryHandoffFailure::new(
            "compatibility_query_execution_failed",
        )),
    }
}
