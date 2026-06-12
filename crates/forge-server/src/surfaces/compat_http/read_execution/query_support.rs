use forge_proof::TransitionOutcome;

use crate::{
    ForgeServerAdmittedDirectDeclaration, ForgeServerCompatibilityFacade,
    ForgeServerCompatibilityPreparedRequest, ForgeServerQueryHandoffDenial,
    ForgeServerQueryHandoffFailure, ForgeServerQueryHandoffInput, ForgeServerQueryHandoffOperation,
    ForgeServerReadValidator,
};

use super::{
    basis::ForgeServerExternalBasisRequest,
    conditional::ForgeServerConditionalRead,
    execution::{
        admit_declaration, named_read_declaration, ForgeServerCompatibilityExecutionOutcome,
    },
};

pub(super) fn compatibility_basis_request(
    prepared_request: &ForgeServerCompatibilityPreparedRequest,
) -> Result<ForgeServerExternalBasisRequest, ForgeServerQueryHandoffDenial> {
    ForgeServerExternalBasisRequest::from_prepared_request(prepared_request)
}

pub(super) fn admitted_named_read_declaration(
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

pub(super) fn compatibility_handoff(
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

pub(super) fn validate_conditional_read(
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

pub(super) fn runtime_error_outcome<T>(
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
