use crate::{
    ForgeServerAdmission, ForgeServerOperationAuthorizationPolicy, ForgeServerOperationFamily,
    ForgeServerOperationRequest, ForgeServerPreparedQueryHandoffKind,
};

use super::{
    ForgeServerOperationAuthorityKind, ForgeServerOperationAuthorityMetadata,
    ForgeServerOperationScope, ForgeServerProductSessionCoordinationTarget,
};

pub(super) fn operation_reference_digest(
    operation_request: &ForgeServerOperationRequest,
) -> String {
    format!(
        "forge-server-operation-reference-v2|family={}|operation={}|basis={}|idempotency={}|product_session={}|payload={}",
        operation_request.identity().operation_family().as_str(),
        operation_request.identity().operation_name(),
        operation_request
            .identity()
            .basis_digest()
            .unwrap_or("none"),
        operation_request
            .identity()
            .idempotency_key()
            .unwrap_or("none"),
        operation_request
            .identity()
            .product_session_identity()
            .unwrap_or("none"),
        operation_request
            .identity()
            .payload_identity()
            .unwrap_or("none"),
    )
}

pub(super) fn admit_metadata_for_family(
    family: ForgeServerOperationFamily,
    admission: &ForgeServerAdmission,
    operation_request: &ForgeServerOperationRequest,
    metadata: &ForgeServerOperationAuthorityMetadata,
) -> Result<(ForgeServerOperationAuthorityKind, ForgeServerOperationScope), String> {
    let request_context = admission.request_context();
    let tenant_id = request_context.workspace_target().tenant_id();
    let workspace_id = request_context.workspace_target().workspace_id();
    let branch_label = request_context.branch_target().canonical_label();
    match (family, metadata) {
        (
            ForgeServerOperationFamily::QueryDirectRead
            | ForgeServerOperationFamily::QueryDirectProjection
            | ForgeServerOperationFamily::ProductApplicationRead,
            ForgeServerOperationAuthorityMetadata::SharedReadOnly {
                basis_kind,
                basis_digest,
                ..
            },
        ) => {
            if basis_kind.trim().is_empty() || basis_digest.trim().is_empty() {
                return Err(
                    "shared-read authority requires a declared comparable basis kind and basis digest"
                        .to_string(),
                );
            }
            validate_shared_read_basis_kind(family, basis_kind)?;
            validate_shared_read_basis_digest(operation_request, basis_digest)?;
            Ok((
                ForgeServerOperationAuthorityKind::SharedReadOnly,
                ForgeServerOperationScope::workspace(tenant_id, workspace_id),
            ))
        }
        (
            ForgeServerOperationFamily::QueryDirectSubmission,
            ForgeServerOperationAuthorityMetadata::DeterministicSubmission {
                submission_lane,
                journal_posture,
                base_digest_posture,
                idempotency_posture,
            },
        ) => {
            if submission_lane.trim().is_empty() || journal_posture.trim().is_empty() {
                return Err(
                    "deterministic submission authority requires declared submission lane and journal posture"
                        .to_string(),
                );
            }
            validate_base_digest_posture(operation_request, base_digest_posture)?;
            validate_idempotency_posture(operation_request, idempotency_posture)?;
            Ok((
                ForgeServerOperationAuthorityKind::DeterministicSubmission,
                ForgeServerOperationScope::workspace_branch(tenant_id, workspace_id, branch_label),
            ))
        }
        (
            ForgeServerOperationFamily::ProductApplicationMutation,
            ForgeServerOperationAuthorityMetadata::ProductDraftMutation {
                product_session_identity,
                draft_scope,
                base_digest_posture,
                idempotency_posture,
                ..
            },
        ) => {
            if product_session_identity.trim().is_empty() || draft_scope.trim().is_empty() {
                return Err(
                    "product draft mutation authority requires product session identity and draft scope"
                        .to_string(),
                );
            }
            validate_product_session_identity(operation_request, product_session_identity)?;
            validate_base_digest_posture(operation_request, base_digest_posture)?;
            validate_idempotency_posture(operation_request, idempotency_posture)?;
            Ok((
                ForgeServerOperationAuthorityKind::ProductDraftMutation,
                ForgeServerOperationScope::product_draft(
                    tenant_id,
                    workspace_id,
                    product_session_identity,
                    draft_scope,
                ),
            ))
        }
        (
            ForgeServerOperationFamily::ProductSessionCoordination,
            ForgeServerOperationAuthorityMetadata::ProductSessionCoordination {
                target,
                coordination_lane,
            },
        ) => {
            if coordination_lane.trim().is_empty() {
                return Err(
                    "product session coordination authority requires a coordination lane"
                        .to_string(),
                );
            }
            match target {
                ForgeServerProductSessionCoordinationTarget::SessionCreation => Ok((
                    ForgeServerOperationAuthorityKind::ProductSessionCoordination,
                    ForgeServerOperationScope::workspace_branch(
                        tenant_id,
                        workspace_id,
                        branch_label,
                    ),
                )),
                ForgeServerProductSessionCoordinationTarget::ExistingSession {
                    product_session_identity,
                } => {
                    validate_product_session_identity(operation_request, product_session_identity)?;
                    Ok((
                        ForgeServerOperationAuthorityKind::ProductSessionCoordination,
                        ForgeServerOperationScope::product_draft(
                            tenant_id,
                            workspace_id,
                            product_session_identity,
                            "session-coordination",
                        ),
                    ))
                }
            }
        }
        (
            ForgeServerOperationFamily::BinaryTransfer,
            ForgeServerOperationAuthorityMetadata::BinaryStreaming {
                preflight_posture,
                size_posture,
                cancellation_posture,
                partial_failure_posture,
                ..
            },
        ) => {
            if [
                preflight_posture,
                size_posture,
                cancellation_posture,
                partial_failure_posture,
            ]
            .iter()
            .any(|value| value.trim().is_empty())
            {
                return Err(
                    "binary streaming authority requires preflight, size, cancellation, and partial-failure posture before planning"
                        .to_string(),
                );
            }
            Ok((
                ForgeServerOperationAuthorityKind::BinaryStreaming,
                ForgeServerOperationScope::workspace_branch(tenant_id, workspace_id, branch_label),
            ))
        }
        (
            ForgeServerOperationFamily::SyncLease,
            ForgeServerOperationAuthorityMetadata::LeaseCoordination {
                lease_target,
                resume_basis_digest,
                ..
            },
        ) => {
            if lease_target.trim().is_empty() {
                return Err(
                    "lease coordination authority requires a concrete lease target before planning"
                        .to_string(),
                );
            }
            validate_shared_read_basis_digest(
                operation_request,
                resume_basis_digest.as_deref().unwrap_or("none"),
            )?;
            Ok((
                ForgeServerOperationAuthorityKind::LeaseCoordination,
                ForgeServerOperationScope::sync_lease(
                    tenant_id,
                    workspace_id,
                    branch_label,
                    lease_target,
                ),
            ))
        }
        _ => Err(format!(
            "operation family `{}` does not admit the declared authority metadata for operation `{}`",
            family.as_str(),
            operation_request.identity().operation_name(),
        )),
    }
}

pub(super) fn authorize_operation(
    family: ForgeServerOperationFamily,
    prepared_kind: ForgeServerPreparedQueryHandoffKind,
    policy: Option<&ForgeServerOperationAuthorizationPolicy>,
    admission: &ForgeServerAdmission,
) -> Result<String, String> {
    let lane = admit_authorization_lane(family, prepared_kind)?;
    let policy_suffix = policy
        .unwrap_or(&ForgeServerOperationAuthorizationPolicy::AllowAuthenticated)
        .authorize(admission)?;
    Ok(format!("{lane}|policy={policy_suffix}"))
}

fn admit_authorization_lane(
    family: ForgeServerOperationFamily,
    prepared_kind: ForgeServerPreparedQueryHandoffKind,
) -> Result<String, String> {
    let admitted = match family {
        ForgeServerOperationFamily::QueryDirectRead
        | ForgeServerOperationFamily::QueryDirectProjection
        | ForgeServerOperationFamily::ProductApplicationRead => matches!(
            prepared_kind,
            ForgeServerPreparedQueryHandoffKind::QueryRead
                | ForgeServerPreparedQueryHandoffKind::ForgeNativeSession
        ),
        ForgeServerOperationFamily::QueryDirectSubmission
        | ForgeServerOperationFamily::ProductApplicationMutation => matches!(
            prepared_kind,
            ForgeServerPreparedQueryHandoffKind::QueryMutation
                | ForgeServerPreparedQueryHandoffKind::ForgeNativeSession
        ),
        ForgeServerOperationFamily::BinaryTransfer => matches!(
            prepared_kind,
            ForgeServerPreparedQueryHandoffKind::QueryRead
                | ForgeServerPreparedQueryHandoffKind::QueryMutation
                | ForgeServerPreparedQueryHandoffKind::ForgeNativeSession
        ),
        ForgeServerOperationFamily::ProductSessionCoordination => matches!(
            prepared_kind,
            ForgeServerPreparedQueryHandoffKind::ForgeNativeSession
                | ForgeServerPreparedQueryHandoffKind::QueryMutation
        ),
        ForgeServerOperationFamily::SyncLease => matches!(
            prepared_kind,
            ForgeServerPreparedQueryHandoffKind::QueryRead
                | ForgeServerPreparedQueryHandoffKind::ForgeNativeSession
        ),
    };
    if !admitted {
        return Err(format!(
            "middleware lane {:?} is not authorized for operation family `{}`",
            prepared_kind,
            family.as_str(),
        ));
    }
    Ok(format!("authorized:{}", family.as_str()))
}

fn validate_shared_read_basis_kind(
    family: ForgeServerOperationFamily,
    basis_kind: &str,
) -> Result<(), String> {
    match family {
        ForgeServerOperationFamily::QueryDirectRead
        | ForgeServerOperationFamily::QueryDirectProjection => {
            if basis_kind == "query-shared-read-basis" {
                Ok(())
            } else {
                Err(format!(
                    "query shared-read authority requires `query-shared-read-basis`, found `{basis_kind}`"
                ))
            }
        }
        ForgeServerOperationFamily::ProductApplicationRead => match basis_kind {
            "query-derived"
            | "product-session-derived"
            | "durable-product-derived"
            | "fixture-only" => Ok(()),
            _ => Err(format!(
                "product read authority requires a declared comparable basis kind, found `{basis_kind}`"
            )),
        },
        _ => Ok(()),
    }
}

fn validate_shared_read_basis_digest(
    operation_request: &ForgeServerOperationRequest,
    basis_digest: &str,
) -> Result<(), String> {
    match operation_request.identity().basis_digest() {
        Some(expected) if expected == basis_digest => Ok(()),
        Some(expected) => Err(format!(
            "authority basis digest `{basis_digest}` does not match the admitted operation basis `{expected}`"
        )),
        None => Err(
            "shared-read or resume authority requires an admitted operation basis digest"
                .to_string(),
        ),
    }
}

fn validate_base_digest_posture(
    operation_request: &ForgeServerOperationRequest,
    base_digest_posture: &str,
) -> Result<(), String> {
    let expected = if operation_request.identity().basis_digest().is_some() {
        "caller-basis-bound"
    } else {
        "caller-basis-unbound"
    };
    if base_digest_posture == expected {
        Ok(())
    } else {
        Err(format!(
            "base-digest posture `{base_digest_posture}` does not match admitted operation basis posture `{expected}`"
        ))
    }
}

fn validate_idempotency_posture(
    operation_request: &ForgeServerOperationRequest,
    idempotency_posture: &str,
) -> Result<(), String> {
    let expected = if operation_request.identity().idempotency_key().is_some() {
        "idempotent"
    } else {
        "best-effort"
    };
    if idempotency_posture == expected {
        Ok(())
    } else {
        Err(format!(
            "idempotency posture `{idempotency_posture}` does not match admitted operation idempotency posture `{expected}`"
        ))
    }
}

fn validate_product_session_identity(
    operation_request: &ForgeServerOperationRequest,
    product_session_identity: &str,
) -> Result<(), String> {
    match operation_request.identity().product_session_identity() {
        Some(expected) if expected == product_session_identity => Ok(()),
        Some(expected) => Err(format!(
            "authority product session `{product_session_identity}` does not match admitted operation session `{expected}`"
        )),
        None => Err(
            "product authority requires an admitted operation product session identity".to_string(),
        ),
    }
}
