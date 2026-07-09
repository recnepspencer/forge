use crate::{
    WorthServerOperationAuthorityMetadata, WorthServerOperationPreconditionPosture,
    WorthServerOperationQuerySupportContext, WorthServerOperationReadinessFacade,
    WorthServerOperationRegistry, WorthServerOperationRequestInput,
    WorthServerPreparedQueryHandoffKind, WorthServerProductBasisPrecondition,
    WorthServerProductSessionCoordinationTarget, WorthServerProductSupportPosture,
    WorthServerQueryHandoffConfig, WorthServerQueryHandoffOperation,
    WorthServerQueryWorkspaceBindingRequest, WorthServerResponseReceipt,
};

use super::{
    WorthServerProductOperationAuthorityRequirement, WorthServerProductOperationDenial,
    WorthServerProductOperationDenialCode, WorthServerProductOperationEnvelope,
    WorthServerProductOperationEnvelopeKind, WorthServerProductOperationInput,
    WorthServerProductOperationOutcome, WorthServerProductOperationSurfaceDenial,
    WorthServerProductOperationSurfaceDenialCode, WorthServerProductOperationSurfaceDenialFacts,
    WorthServerScheduledProductOperation,
};

pub(super) fn build_request_input(
    declaration: &super::WorthServerProductOperationDeclaration,
    input: &WorthServerProductOperationInput,
) -> WorthServerOperationRequestInput {
    let mut builder = WorthServerOperationRequestInput::builder()
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
    declaration: &super::WorthServerProductOperationDeclaration,
    payload: &super::WorthServerProductOperationPayload,
) -> Result<(), WorthServerProductOperationDenial> {
    if payload.envelope().declared_schema_identity() != Some(declaration.payload_schema_identity())
    {
        return Err(WorthServerProductOperationDenial::new(
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
        .with_code(WorthServerProductOperationDenialCode::PayloadSchemaMismatch));
    }
    if let Some(validator) = declaration.payload_validator() {
        validator.validate(payload).map_err(|denial| {
            denial.with_code(WorthServerProductOperationDenialCode::DeclaredPayloadValidator)
        })?;
    }
    Ok(())
}

pub(super) fn declaration_metadata(
    declaration: &super::WorthServerProductOperationDeclaration,
    request: &crate::WorthServerOperationRequest,
) -> Result<WorthServerOperationAuthorityMetadata, WorthServerProductOperationSurfaceDenial> {
    match declaration.authority_requirement() {
        WorthServerProductOperationAuthorityRequirement::SharedRead => {
            let basis_digest = request.identity().basis_digest().ok_or_else(|| {
                WorthServerProductOperationSurfaceDenial::new(
                    WorthServerProductOperationSurfaceDenialCode::AdmissionDenied,
                    "product shared-read operations require an admitted basis digest".into(),
                )
            })?;
            Ok(
                WorthServerOperationAuthorityMetadata::shared_read_with_support_posture(
                    declaration.basis_kind().as_shared_read_basis_kind(),
                    basis_digest,
                    declaration.operation_name(),
                    product_support_posture_label(declaration.support_snapshot().posture()),
                ),
            )
        }
        WorthServerProductOperationAuthorityRequirement::DraftMutation { draft_scope } => {
            let product_session_identity = request
                .identity()
                .product_session_identity()
                .ok_or_else(|| {
                    WorthServerProductOperationSurfaceDenial::new(
                        WorthServerProductOperationSurfaceDenialCode::AdmissionDenied,
                        "product mutation operations require a product session identity".into(),
                    )
                })?;
            Ok(
                WorthServerOperationAuthorityMetadata::product_draft_mutation(
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
        WorthServerProductOperationAuthorityRequirement::SessionCoordination {
            coordination_lane,
        } => {
            let product_session_identity = request
                .identity()
                .product_session_identity()
                .ok_or_else(|| {
                    WorthServerProductOperationSurfaceDenial::new(
                        WorthServerProductOperationSurfaceDenialCode::AdmissionDenied,
                        "product session operations require a product session identity".into(),
                    )
                })?;
            Ok(
                WorthServerOperationAuthorityMetadata::product_session_coordination(
                    WorthServerProductSessionCoordinationTarget::ExistingSession {
                        product_session_identity: product_session_identity.to_string(),
                    },
                    coordination_lane,
                ),
            )
        }
    }
}

pub(super) fn close_product_operation_readiness(
    operation_registry: &WorthServerOperationRegistry,
    query_handoff_config: &WorthServerQueryHandoffConfig,
    admission: &crate::WorthServerOperationAdmissionPosture,
    declaration: &super::WorthServerProductOperationDeclaration,
    resolved_request_context: &crate::WorthServerResolvedRequestContext,
) -> Result<crate::WorthServerOperationReadinessClosure, WorthServerProductOperationSurfaceDenial> {
    let readiness =
        WorthServerOperationReadinessFacade::with_operation_registry(operation_registry.clone());
    match declaration.basis_kind() {
        super::WorthServerProductOperationBasisKind::QueryDerived => {
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
            .map_err(WorthServerProductOperationSurfaceDenial::from_readiness_denial),
    }
}

fn close_query_derived_product_readiness(
    readiness: &WorthServerOperationReadinessFacade,
    query_handoff_config: &WorthServerQueryHandoffConfig,
    admission: &crate::WorthServerOperationAdmissionPosture,
    declaration: &super::WorthServerProductOperationDeclaration,
    resolved_request_context: &crate::WorthServerResolvedRequestContext,
) -> Result<crate::WorthServerOperationReadinessClosure, WorthServerProductOperationSurfaceDenial> {
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
    let binding_request = WorthServerQueryWorkspaceBindingRequest::for_query_handoff(
        resolved_request_context.clone(),
        operation.clone(),
    );
    let workspace = query_handoff_config
        .workspace_provider()
        .bind_workspace(&binding_request)
        .map_err(|error| {
            WorthServerProductOperationSurfaceDenial::new(
                WorthServerProductOperationSurfaceDenialCode::ReadinessDenied,
                format!("{}: {}", error.stage(), error.message()),
            )
        })?;
    let downstream_delivery_contract = workspace.public_downstream_delivery_contract();
    let query_context = WorthServerOperationQuerySupportContext::new(
        prepared_kind,
        &operation,
        &workspace,
        &downstream_delivery_contract,
    );
    let precondition_posture = WorthServerProductBasisPrecondition::evaluate(
        declaration.operation_name(),
        admission.operation_request().identity().basis_digest(),
        &workspace
            .snapshot_identity()
            .terminal_projection_for_reporting(),
    )
    .map(WorthServerOperationPreconditionPosture::ProductBasis)
    .map_err(WorthServerProductOperationSurfaceDenial::from_readiness_denial)?;
    readiness
        .close_readiness(admission, Some(query_context), Some(precondition_posture))
        .map_err(WorthServerProductOperationSurfaceDenial::from_readiness_denial)
}

pub(super) fn build_envelope(
    scheduled: &WorthServerScheduledProductOperation,
    outcome: &WorthServerProductOperationOutcome,
) -> WorthServerProductOperationEnvelope {
    let (kind, outcome_label, receipt): (
        WorthServerProductOperationEnvelopeKind,
        &str,
        WorthServerResponseReceipt,
    ) = match outcome {
        WorthServerProductOperationOutcome::Success(success) => (
            WorthServerProductOperationEnvelopeKind::Success,
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
        WorthServerProductOperationOutcome::Denied(denial) => (
            WorthServerProductOperationEnvelopeKind::Denial,
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
        WorthServerProductOperationOutcome::Failed(failure) => (
            WorthServerProductOperationEnvelopeKind::Failure,
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
        "worth-server-product-operation-envelope-v1|operation={}|scheduled={}|outcome={outcome_label}",
        scheduled.plan().declaration().operation_name(),
        scheduled.canonical_digest(),
    );
    let provenance =
        crate::response::build_provenance("product-operation-envelope", &canonical_digest);
    WorthServerProductOperationEnvelope::new(
        kind,
        scheduled.plan().declaration().operation_name(),
        canonical_digest,
        provenance,
        receipt,
    )
}

pub(super) fn build_early_envelope(
    operation_name: &str,
    request: &crate::WorthServerOperationRequest,
    outcome: &WorthServerProductOperationOutcome,
) -> WorthServerProductOperationEnvelope {
    let canonical_digest = format!(
        "worth-server-product-operation-envelope-v1|request={}|outcome={outcome:?}",
        request.canonical_digest()
    );
    let provenance =
        crate::response::build_provenance("product-operation-envelope", &canonical_digest);
    let receipt = match outcome {
        WorthServerProductOperationOutcome::Success(_) => crate::response::build_success_receipt(
            &format!("product success {operation_name}"),
            &canonical_digest,
            provenance.clone(),
        ),
        WorthServerProductOperationOutcome::Denied(_)
        | WorthServerProductOperationOutcome::Failed(_) => crate::response::build_denial_receipt(
            &format!("product denial {operation_name}"),
            &canonical_digest,
            provenance.clone(),
        ),
    };
    WorthServerProductOperationEnvelope::new(
        match outcome {
            WorthServerProductOperationOutcome::Success(_) => {
                WorthServerProductOperationEnvelopeKind::Success
            }
            WorthServerProductOperationOutcome::Denied(_) => {
                WorthServerProductOperationEnvelopeKind::Denial
            }
            WorthServerProductOperationOutcome::Failed(_) => {
                WorthServerProductOperationEnvelopeKind::Failure
            }
        },
        operation_name,
        canonical_digest,
        provenance,
        receipt,
    )
}

fn product_support_posture_label(posture: WorthServerProductSupportPosture) -> &'static str {
    match posture {
        WorthServerProductSupportPosture::ProductionAdmitted => "production-admitted",
        WorthServerProductSupportPosture::Unsupported => "unsupported",
        WorthServerProductSupportPosture::Unknown => "unknown",
        WorthServerProductSupportPosture::IncompatibleBasis => "incompatible-basis",
    }
}

fn query_binding_operation(
    prepared_kind: WorthServerPreparedQueryHandoffKind,
    operation_family: crate::WorthServerOperationFamily,
    operation_name: &str,
) -> WorthServerQueryHandoffOperation {
    match prepared_kind {
        WorthServerPreparedQueryHandoffKind::QueryRead => {
            WorthServerQueryHandoffOperation::query_read(operation_name)
        }
        WorthServerPreparedQueryHandoffKind::QueryMutation => {
            WorthServerQueryHandoffOperation::query_mutation(operation_name)
        }
        WorthServerPreparedQueryHandoffKind::WorthNativeSession
            if operation_family
                == crate::WorthServerOperationFamily::ProductApplicationMutation =>
        {
            WorthServerQueryHandoffOperation::direct_mutation(operation_name)
        }
        WorthServerPreparedQueryHandoffKind::WorthNativeSession => {
            WorthServerQueryHandoffOperation::direct_read(operation_name)
        }
    }
}

pub(super) fn stale_basis_denial(
    detail: String,
    expected_basis_digest: &str,
    observed_basis_digest: &str,
) -> WorthServerProductOperationSurfaceDenial {
    WorthServerProductOperationSurfaceDenial::new(
        WorthServerProductOperationSurfaceDenialCode::PreconditionDenied,
        detail,
    )
    .with_facts(
        WorthServerProductOperationSurfaceDenialFacts::default()
            .with_basis_mismatch(crate::WorthServerProductStaleBasisDenial::new(
                expected_basis_digest,
                observed_basis_digest,
            ))
            .with_execution_boundary(
                super::WorthServerProductOperationExecutionBoundary::RejectedBeforeAdapterExecution,
            ),
    )
}
