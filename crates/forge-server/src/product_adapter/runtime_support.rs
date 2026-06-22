use crate::{
    ForgeServerOperationAuthorityMetadata, ForgeServerOperationPreconditionPosture,
    ForgeServerOperationQuerySupportContext, ForgeServerOperationReadinessFacade,
    ForgeServerOperationRegistry, ForgeServerOperationRequestInput,
    ForgeServerPreparedQueryHandoffKind, ForgeServerProductBasisPrecondition,
    ForgeServerProductSessionCoordinationTarget, ForgeServerProductSupportPosture,
    ForgeServerQueryHandoffConfig, ForgeServerQueryHandoffOperation,
    ForgeServerQueryWorkspaceBindingRequest, ForgeServerResponseReceipt,
};

use super::{
    ForgeServerProductOperationAuthorityRequirement, ForgeServerProductOperationDenial,
    ForgeServerProductOperationDenialCode, ForgeServerProductOperationEnvelope,
    ForgeServerProductOperationEnvelopeKind, ForgeServerProductOperationInput,
    ForgeServerProductOperationOutcome, ForgeServerProductOperationSurfaceDenial,
    ForgeServerProductOperationSurfaceDenialCode, ForgeServerProductOperationSurfaceDenialFacts,
    ForgeServerScheduledProductOperation,
};

pub(super) fn build_request_input(
    declaration: &super::ForgeServerProductOperationDeclaration,
    input: &ForgeServerProductOperationInput,
) -> ForgeServerOperationRequestInput {
    let mut builder = ForgeServerOperationRequestInput::builder()
        .with_operation_family(declaration.operation_family())
        .with_operation_name(declaration.operation_name())
        .with_payload_envelope(input.payload().envelope().clone());
    if let Some(basis_digest) = input.basis_digest() {
        builder = builder.with_basis_digest(basis_digest);
    }
    if let Some(idempotency_key) = input.idempotency_key() {
        builder = builder.with_idempotency_key(idempotency_key.value());
    }
    if let Some(product_session_identity) = input.product_session_identity() {
        builder = builder.with_product_session_identity(product_session_identity);
    }
    builder.build()
}

pub(super) fn validate_payload_schema(
    declaration: &super::ForgeServerProductOperationDeclaration,
    payload: &super::ForgeServerProductOperationPayload,
) -> Result<(), ForgeServerProductOperationDenial> {
    if payload.envelope().declared_schema_identity() != Some(declaration.payload_schema_identity())
    {
        return Err(ForgeServerProductOperationDenial::new(
            "invalid_payload_schema",
            format!(
                "payload schema `{}` did not match declared schema `{}`",
                payload
                    .envelope()
                    .declared_schema_identity()
                    .unwrap_or("none"),
                declaration.payload_schema_identity()
            ),
        )
        .with_code(ForgeServerProductOperationDenialCode::PayloadSchemaMismatch));
    }
    if let Some(validator) = declaration.payload_validator() {
        validator.validate(payload).map_err(|denial| {
            denial.with_code(ForgeServerProductOperationDenialCode::DeclaredPayloadValidator)
        })?;
    }
    Ok(())
}

pub(super) fn declaration_metadata(
    declaration: &super::ForgeServerProductOperationDeclaration,
    request: &crate::ForgeServerOperationRequest,
) -> Result<ForgeServerOperationAuthorityMetadata, ForgeServerProductOperationSurfaceDenial> {
    match declaration.authority_requirement() {
        ForgeServerProductOperationAuthorityRequirement::SharedRead => {
            let basis_digest = request.identity().basis_digest().ok_or_else(|| {
                ForgeServerProductOperationSurfaceDenial::new(
                    ForgeServerProductOperationSurfaceDenialCode::AdmissionDenied,
                    "product shared-read operations require an admitted basis digest".into(),
                )
            })?;
            Ok(
                ForgeServerOperationAuthorityMetadata::shared_read_with_support_posture(
                    declaration.basis_kind().as_shared_read_basis_kind(),
                    basis_digest,
                    declaration.operation_name(),
                    product_support_posture_label(declaration.support_snapshot().posture()),
                ),
            )
        }
        ForgeServerProductOperationAuthorityRequirement::DraftMutation { draft_scope } => {
            let product_session_identity = request
                .identity()
                .product_session_identity()
                .ok_or_else(|| {
                    ForgeServerProductOperationSurfaceDenial::new(
                        ForgeServerProductOperationSurfaceDenialCode::AdmissionDenied,
                        "product mutation operations require a product session identity".into(),
                    )
                })?;
            Ok(
                ForgeServerOperationAuthorityMetadata::product_draft_mutation(
                    product_session_identity,
                    draft_scope,
                    if request.identity().basis_digest().is_some() {
                        "caller-basis-bound"
                    } else {
                        "caller-basis-unbound"
                    },
                    if request.identity().idempotency_key().is_some() {
                        "idempotent"
                    } else {
                        "best-effort"
                    },
                ),
            )
        }
        ForgeServerProductOperationAuthorityRequirement::SessionCoordination {
            coordination_lane,
        } => {
            let product_session_identity = request
                .identity()
                .product_session_identity()
                .ok_or_else(|| {
                    ForgeServerProductOperationSurfaceDenial::new(
                        ForgeServerProductOperationSurfaceDenialCode::AdmissionDenied,
                        "product session operations require a product session identity".into(),
                    )
                })?;
            Ok(
                ForgeServerOperationAuthorityMetadata::product_session_coordination(
                    ForgeServerProductSessionCoordinationTarget::ExistingSession {
                        product_session_identity: product_session_identity.to_string(),
                    },
                    coordination_lane,
                ),
            )
        }
    }
}

pub(super) fn close_product_operation_readiness(
    operation_registry: &ForgeServerOperationRegistry,
    query_handoff_config: &ForgeServerQueryHandoffConfig,
    admission: &crate::ForgeServerOperationAdmissionPosture,
    declaration: &super::ForgeServerProductOperationDeclaration,
    resolved_request_context: &crate::ForgeServerResolvedRequestContext,
) -> Result<crate::ForgeServerOperationReadinessClosure, ForgeServerProductOperationSurfaceDenial> {
    let readiness =
        ForgeServerOperationReadinessFacade::with_operation_registry(operation_registry.clone());
    match declaration.basis_kind() {
        super::ForgeServerProductOperationBasisKind::QueryDerived => {
            close_query_derived_product_readiness(
                &readiness,
                query_handoff_config,
                admission,
                declaration,
                resolved_request_context,
            )
        }
        _ => readiness
            .close_readiness(admission, None, None)
            .map_err(ForgeServerProductOperationSurfaceDenial::from_readiness_denial),
    }
}

fn close_query_derived_product_readiness(
    readiness: &ForgeServerOperationReadinessFacade,
    query_handoff_config: &ForgeServerQueryHandoffConfig,
    admission: &crate::ForgeServerOperationAdmissionPosture,
    declaration: &super::ForgeServerProductOperationDeclaration,
    resolved_request_context: &crate::ForgeServerResolvedRequestContext,
) -> Result<crate::ForgeServerOperationReadinessClosure, ForgeServerProductOperationSurfaceDenial> {
    let prepared_kind = admission
        .authorization_proof()
        .admission()
        .query_handoff_intent()
        .kind();
    let operation = query_binding_operation(
        prepared_kind,
        declaration.operation_family(),
        declaration.operation_name(),
    );
    let binding_request = ForgeServerQueryWorkspaceBindingRequest::for_query_handoff(
        resolved_request_context.clone(),
        operation.clone(),
    );
    let workspace = query_handoff_config
        .workspace_provider()
        .bind_workspace(&binding_request)
        .map_err(|error| {
            ForgeServerProductOperationSurfaceDenial::new(
                ForgeServerProductOperationSurfaceDenialCode::ReadinessDenied,
                format!("{}: {}", error.stage(), error.message()),
            )
        })?;
    let downstream_delivery_contract = workspace.public_downstream_delivery_contract();
    let query_context = ForgeServerOperationQuerySupportContext::new(
        prepared_kind,
        &operation,
        &workspace,
        &downstream_delivery_contract,
    );
    let precondition_posture = ForgeServerProductBasisPrecondition::evaluate(
        declaration.operation_name(),
        admission.operation_request().identity().basis_digest(),
        &workspace
            .snapshot_identity()
            .terminal_projection_for_reporting(),
    )
    .map(ForgeServerOperationPreconditionPosture::ProductBasis)
    .map_err(ForgeServerProductOperationSurfaceDenial::from_readiness_denial)?;
    readiness
        .close_readiness(admission, Some(query_context), Some(precondition_posture))
        .map_err(ForgeServerProductOperationSurfaceDenial::from_readiness_denial)
}

pub(super) fn build_envelope(
    scheduled: &ForgeServerScheduledProductOperation,
    outcome: &ForgeServerProductOperationOutcome,
) -> ForgeServerProductOperationEnvelope {
    let (kind, outcome_label, receipt): (
        ForgeServerProductOperationEnvelopeKind,
        &str,
        ForgeServerResponseReceipt,
    ) = match outcome {
        ForgeServerProductOperationOutcome::Success(success) => (
            ForgeServerProductOperationEnvelopeKind::Success,
            success.result_digest(),
            crate::response::build_success_receipt(
                &format!(
                    "product success {} {}",
                    scheduled.plan().declaration().operation_name(),
                    success.result_key()
                ),
                scheduled.canonical_digest(),
                crate::response::build_provenance("product-success", scheduled.canonical_digest()),
            ),
        ),
        ForgeServerProductOperationOutcome::Denied(denial) => (
            ForgeServerProductOperationEnvelopeKind::Denial,
            denial.reason_key(),
            crate::response::build_denial_receipt(
                &format!(
                    "product denial {} {}",
                    scheduled.plan().declaration().operation_name(),
                    denial.reason_key()
                ),
                scheduled.canonical_digest(),
                crate::response::build_provenance("product-denial", scheduled.canonical_digest()),
            ),
        ),
        ForgeServerProductOperationOutcome::Failed(failure) => (
            ForgeServerProductOperationEnvelopeKind::Failure,
            failure.reason_key(),
            crate::response::build_denial_receipt(
                &format!(
                    "product failure {} {}",
                    scheduled.plan().declaration().operation_name(),
                    failure.reason_key()
                ),
                scheduled.canonical_digest(),
                crate::response::build_provenance("product-failure", scheduled.canonical_digest()),
            ),
        ),
    };
    let canonical_digest = format!(
        "forge-server-product-operation-envelope-v1|operation={}|scheduled={}|outcome={outcome_label}",
        scheduled.plan().declaration().operation_name(),
        scheduled.canonical_digest(),
    );
    let provenance =
        crate::response::build_provenance("product-operation-envelope", &canonical_digest);
    ForgeServerProductOperationEnvelope::new(
        kind,
        scheduled.plan().declaration().operation_name(),
        canonical_digest,
        provenance,
        receipt,
    )
}

pub(super) fn build_early_envelope(
    operation_name: &str,
    request: &crate::ForgeServerOperationRequest,
    outcome: &ForgeServerProductOperationOutcome,
) -> ForgeServerProductOperationEnvelope {
    let canonical_digest = format!(
        "forge-server-product-operation-envelope-v1|request={}|outcome={outcome:?}",
        request.canonical_digest()
    );
    let provenance =
        crate::response::build_provenance("product-operation-envelope", &canonical_digest);
    let receipt = match outcome {
        ForgeServerProductOperationOutcome::Success(_) => crate::response::build_success_receipt(
            &format!("product success {operation_name}"),
            &canonical_digest,
            provenance.clone(),
        ),
        ForgeServerProductOperationOutcome::Denied(_)
        | ForgeServerProductOperationOutcome::Failed(_) => crate::response::build_denial_receipt(
            &format!("product denial {operation_name}"),
            &canonical_digest,
            provenance.clone(),
        ),
    };
    ForgeServerProductOperationEnvelope::new(
        match outcome {
            ForgeServerProductOperationOutcome::Success(_) => {
                ForgeServerProductOperationEnvelopeKind::Success
            }
            ForgeServerProductOperationOutcome::Denied(_) => {
                ForgeServerProductOperationEnvelopeKind::Denial
            }
            ForgeServerProductOperationOutcome::Failed(_) => {
                ForgeServerProductOperationEnvelopeKind::Failure
            }
        },
        operation_name,
        canonical_digest,
        provenance,
        receipt,
    )
}

fn product_support_posture_label(posture: ForgeServerProductSupportPosture) -> &'static str {
    match posture {
        ForgeServerProductSupportPosture::ProductionAdmitted => "production-admitted",
        ForgeServerProductSupportPosture::Unsupported => "unsupported",
        ForgeServerProductSupportPosture::Unknown => "unknown",
        ForgeServerProductSupportPosture::IncompatibleBasis => "incompatible-basis",
    }
}

fn query_binding_operation(
    prepared_kind: ForgeServerPreparedQueryHandoffKind,
    operation_family: crate::ForgeServerOperationFamily,
    operation_name: &str,
) -> ForgeServerQueryHandoffOperation {
    match prepared_kind {
        ForgeServerPreparedQueryHandoffKind::QueryRead => {
            ForgeServerQueryHandoffOperation::query_read(operation_name)
        }
        ForgeServerPreparedQueryHandoffKind::QueryMutation => {
            ForgeServerQueryHandoffOperation::query_mutation(operation_name)
        }
        ForgeServerPreparedQueryHandoffKind::ForgeNativeSession
            if operation_family
                == crate::ForgeServerOperationFamily::ProductApplicationMutation =>
        {
            ForgeServerQueryHandoffOperation::direct_mutation(operation_name)
        }
        ForgeServerPreparedQueryHandoffKind::ForgeNativeSession => {
            ForgeServerQueryHandoffOperation::direct_read(operation_name)
        }
    }
}

pub(super) fn stale_basis_denial(
    detail: String,
    expected_basis_digest: &str,
    observed_basis_digest: &str,
) -> ForgeServerProductOperationSurfaceDenial {
    ForgeServerProductOperationSurfaceDenial::new(
        ForgeServerProductOperationSurfaceDenialCode::PreconditionDenied,
        detail,
    )
    .with_facts(
        ForgeServerProductOperationSurfaceDenialFacts::default()
            .with_basis_mismatch(crate::ForgeServerProductStaleBasisDenial::new(
                expected_basis_digest,
                observed_basis_digest,
            ))
            .with_execution_boundary(
                super::ForgeServerProductOperationExecutionBoundary::RejectedBeforeAdapterExecution,
            ),
    )
}
