use forge_foundational::DiagnosticRichnessProfile;
use forge_proof::TransitionOutcome;

use crate::{config::ForgeServerQueryHandoffConfig, ForgeServerPreparedQueryHandoffKind};

use super::{
    ForgeServerQueryHandoff, ForgeServerQueryHandoffDenial, ForgeServerQueryHandoffDenialCode,
    ForgeServerQueryHandoffInput, ForgeServerQueryHandoffOperation,
    ForgeServerQueryWorkspaceBindingRequest,
};
use crate::{
    ForgeServerOperationQuerySupportContext, ForgeServerOperationReadinessDenialCode,
    ForgeServerOperationReadinessFacade,
};

pub(crate) fn prepare_query_handoff(
    config: &ForgeServerQueryHandoffConfig,
    input: ForgeServerQueryHandoffInput,
) -> super::ForgeServerQueryHandoffOutcome {
    let (operation_admission, operation) = input.into_parts();
    let admission = operation_admission.authorization_proof().admission();
    let diagnostics_profile = admission.request_context().diagnostics_profile();

    if let Some(denial) = validate_prepared_intent(&admission, &operation) {
        return TransitionOutcome::Denied(denial);
    }

    let binding_request = ForgeServerQueryWorkspaceBindingRequest::for_query_handoff(
        admission.resolved_request_context().clone(),
        operation.clone(),
    );
    let workspace = match config.workspace_provider().bind_workspace(&binding_request) {
        Ok(workspace) => workspace,
        Err(error) => {
            return TransitionOutcome::Denied(ForgeServerQueryHandoffDenial::new(
                ForgeServerQueryHandoffDenialCode::WorkspaceBindingFailed,
                diagnostics_profile,
                format!("{}: {}", error.stage(), error.message()),
            ));
        }
    };

    let downstream_delivery_contract = workspace.public_downstream_delivery_contract();
    let readiness = ForgeServerOperationReadinessFacade::default();
    let operation_support_posture = match readiness.compose_support(
        &operation_admission,
        Some(ForgeServerOperationQuerySupportContext::new(
            admission.query_handoff_intent().kind(),
            &operation,
            &workspace,
            &downstream_delivery_contract,
        )),
    ) {
        Ok(value) => value,
        Err(denial) => {
            return TransitionOutcome::Denied(map_readiness_denial(denial, diagnostics_profile));
        }
    };
    let support_posture = match operation_support_posture.query_support_posture() {
        Some(value) => value.clone(),
        None => {
            return TransitionOutcome::Denied(ForgeServerQueryHandoffDenial::new(
                ForgeServerQueryHandoffDenialCode::UnsupportedQueryFacadeFamily,
                diagnostics_profile,
                "query handoff requires resolved query support posture before planning",
            ));
        }
    };
    let precondition_posture = readiness.default_precondition_posture(&operation_admission);
    let concurrency_class = match readiness.classify_concurrency(
        &operation_admission,
        &operation_support_posture,
        &precondition_posture,
    ) {
        Ok(value) => value,
        Err(denial) => {
            return TransitionOutcome::Denied(map_readiness_denial(denial, diagnostics_profile));
        }
    };

    let canonical_digest = canonical_digest(
        admission
            .resolved_request_context()
            .request_context()
            .workspace_target()
            .tenant_id(),
        admission
            .resolved_request_context()
            .request_context()
            .workspace_target()
            .workspace_id(),
        workspace.name(),
        &operation,
        operation_admission.canonical_digest(),
        operation_support_posture.canonical_digest(),
        precondition_posture.canonical_digest(),
        concurrency_class_label(&concurrency_class),
        downstream_delivery_contract.contract_for_reporting(),
    );

    TransitionOutcome::Success(ForgeServerQueryHandoff::new(
        operation_admission,
        operation,
        workspace,
        downstream_delivery_contract,
        operation_support_posture.clone(),
        operation_support_posture.composition_receipt().clone(),
        precondition_posture,
        concurrency_class,
        support_posture,
        canonical_digest,
    ))
}

fn validate_prepared_intent(
    admission: &crate::ForgeServerAdmission,
    operation: &ForgeServerQueryHandoffOperation,
) -> Option<ForgeServerQueryHandoffDenial> {
    let prepared = admission.query_handoff_intent();
    match operation {
        ForgeServerQueryHandoffOperation::QueryRead { operation_name }
            if prepared.kind() == ForgeServerPreparedQueryHandoffKind::QueryRead
                && prepared.operation_name() == operation_name =>
        {
            None
        }
        ForgeServerQueryHandoffOperation::QueryMutation { operation_name, .. }
            if prepared.kind() == ForgeServerPreparedQueryHandoffKind::QueryMutation
                && prepared.operation_name() == operation_name =>
        {
            None
        }
        ForgeServerQueryHandoffOperation::DirectRead { .. }
        | ForgeServerQueryHandoffOperation::DirectState { .. }
        | ForgeServerQueryHandoffOperation::DirectInspection { .. }
        | ForgeServerQueryHandoffOperation::DirectProjection { .. }
        | ForgeServerQueryHandoffOperation::DirectMutation { .. }
        | ForgeServerQueryHandoffOperation::DownstreamDelivery { .. }
            if prepared.kind() == ForgeServerPreparedQueryHandoffKind::ForgeNativeSession =>
        {
            None
        }
        ForgeServerQueryHandoffOperation::DirectRead { .. }
        | ForgeServerQueryHandoffOperation::DirectState { .. }
        | ForgeServerQueryHandoffOperation::DirectInspection { .. }
        | ForgeServerQueryHandoffOperation::DirectProjection { .. }
            if prepared.kind() == ForgeServerPreparedQueryHandoffKind::QueryRead =>
        {
            None
        }
        ForgeServerQueryHandoffOperation::DownstreamDelivery { .. } => None,
        _ => Some(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::PreparedIntentMismatch,
            admission.request_context().diagnostics_profile(),
            "query handoff operation does not match the middleware-admitted prepared intent",
        )),
    }
}

fn canonical_digest(
    tenant_id: &str,
    workspace_id: &str,
    workspace_name: &str,
    operation: &ForgeServerQueryHandoffOperation,
    operation_admission_digest: &str,
    support_digest: &str,
    precondition_digest: &str,
    concurrency_label: &str,
    contract_digest: &str,
) -> String {
    format!(
        "forge-server-query-handoff-v3|tenant:{tenant_id}|workspace:{workspace_id}|bound:{workspace_name}|operation:{}|operation_admission:{operation_admission_digest}|support:{support_digest}|precondition:{precondition_digest}|concurrency:{concurrency_label}|contract:{contract_digest}",
        operation.canonical_label(),
    )
}

fn map_readiness_denial(
    denial: crate::ForgeServerOperationReadinessDenial,
    diagnostics_profile: DiagnosticRichnessProfile,
) -> ForgeServerQueryHandoffDenial {
    let code = match denial.code() {
        ForgeServerOperationReadinessDenialCode::MissingQuerySupport
        | ForgeServerOperationReadinessDenialCode::UnsupportedQuerySupport
        | ForgeServerOperationReadinessDenialCode::UnsupportedProductSupport
        | ForgeServerOperationReadinessDenialCode::UnknownProductSupport
        | ForgeServerOperationReadinessDenialCode::FixtureOnlyProductSupport
        | ForgeServerOperationReadinessDenialCode::IncompatibleSupportBasis => {
            ForgeServerQueryHandoffDenialCode::UnsupportedQueryFacadeFamily
        }
        ForgeServerOperationReadinessDenialCode::DownstreamDeliveryRequiresReadIntent => {
            ForgeServerQueryHandoffDenialCode::DownstreamDeliveryRequiresReadIntent
        }
        ForgeServerOperationReadinessDenialCode::RuntimeBackedResumeUnsupported => {
            ForgeServerQueryHandoffDenialCode::RuntimeBackedResumeUnsupported
        }
        ForgeServerOperationReadinessDenialCode::DurableResumeDeferred => {
            ForgeServerQueryHandoffDenialCode::DurableResumeDeferred
        }
        ForgeServerOperationReadinessDenialCode::InvalidPreconditionInput
        | ForgeServerOperationReadinessDenialCode::PreconditionFailed => {
            ForgeServerQueryHandoffDenialCode::CompatibilityMutationPreconditionFailed
        }
    };
    ForgeServerQueryHandoffDenial::new(code, diagnostics_profile, denial.detail())
}

fn concurrency_class_label(
    concurrency_class: &crate::ForgeServerOperationConcurrencyClass,
) -> &'static str {
    match concurrency_class {
        crate::ForgeServerOperationConcurrencyClass::ConcurrentSharedRead => {
            "concurrent-shared-read"
        }
        crate::ForgeServerOperationConcurrencyClass::SerializeDeterministically => {
            "serialize-deterministically"
        }
    }
}
