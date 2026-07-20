use crate::{
    WorthServerAdmission, WorthServerOperationAuthorizationPolicy, WorthServerOperationFamily,
    WorthServerOperationRequest, WorthServerPreparedQueryHandoffKind,
};

use super::{
    WorthServerOperationAuthorityKind, WorthServerOperationAuthorityMetadata,
    WorthServerOperationScope, WorthServerProductSessionCoordinationTarget,
};

pub(super) fn operation_reference_digest(
    operation_request: &WorthServerOperationRequest,
) -> String {
    format!(
        "worth-server-operation-reference-v2|family={}|operation={}|basis={}|idempotency={}|product_session={}|payload={}",
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
    family: WorthServerOperationFamily,
    admission: &WorthServerAdmission,
    operation_request: &WorthServerOperationRequest,
    metadata: &WorthServerOperationAuthorityMetadata,
) -> Result<(WorthServerOperationAuthorityKind, WorthServerOperationScope), String> {
    let request_context = admission.request_context();
    let tenant_id = request_context.workspace_target().tenant_id();
    let workspace_id = request_context.workspace_target().workspace_id();
    let branch_label = request_context.branch_target().canonical_label();
    match (family, metadata) {
        (
            WorthServerOperationFamily::QueryDirectRead
            | WorthServerOperationFamily::QueryDirectProjection
            | WorthServerOperationFamily::ProductApplicationRead,
            WorthServerOperationAuthorityMetadata::SharedReadOnly {
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
                WorthServerOperationAuthorityKind::SharedReadOnly,
                WorthServerOperationScope::workspace(tenant_id, workspace_id),
            ))
        }
        (
            WorthServerOperationFamily::QueryDirectSubmission,
            WorthServerOperationAuthorityMetadata::DeterministicSubmission {
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
                WorthServerOperationAuthorityKind::DeterministicSubmission,
                WorthServerOperationScope::workspace_branch(tenant_id, workspace_id, branch_label),
            ))
        }
        (
            WorthServerOperationFamily::ProductApplicationMutation,
            WorthServerOperationAuthorityMetadata::ProductDraftMutation {
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
                WorthServerOperationAuthorityKind::ProductDraftMutation,
                WorthServerOperationScope::product_draft(
                    tenant_id,
                    workspace_id,
                    product_session_identity,
                    draft_scope,
                ),
            ))
        }
        (
            WorthServerOperationFamily::ProductApplicationMutation,
            WorthServerOperationAuthorityMetadata::DurableProductMutation {
                authority_scope,
                expected_basis_digest,
                idempotency_key,
                durability_contract_digest,
            },
        ) => super::durable_product_mutation_admission::admit_durable_product_mutation(
            operation_request,
            tenant_id,
            workspace_id,
            authority_scope,
            expected_basis_digest,
            idempotency_key,
            durability_contract_digest,
        ),
        (
            WorthServerOperationFamily::ProductSessionCoordination,
            WorthServerOperationAuthorityMetadata::ProductSessionCoordination {
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
                WorthServerProductSessionCoordinationTarget::SessionCreation => Ok((
                    WorthServerOperationAuthorityKind::ProductSessionCoordination,
                    WorthServerOperationScope::workspace_branch(
                        tenant_id,
                        workspace_id,
                        branch_label,
                    ),
                )),
                WorthServerProductSessionCoordinationTarget::ExistingSession {
                    product_session_identity,
                } => {
                    validate_product_session_identity(operation_request, product_session_identity)?;
                    Ok((
                        WorthServerOperationAuthorityKind::ProductSessionCoordination,
                        WorthServerOperationScope::product_draft(
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
            WorthServerOperationFamily::BinaryTransfer,
            WorthServerOperationAuthorityMetadata::BinaryStreaming {
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
                WorthServerOperationAuthorityKind::BinaryStreaming,
                WorthServerOperationScope::workspace_branch(tenant_id, workspace_id, branch_label),
            ))
        }
        (
            WorthServerOperationFamily::SyncLease,
            WorthServerOperationAuthorityMetadata::LeaseCoordination {
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
                WorthServerOperationAuthorityKind::LeaseCoordination,
                WorthServerOperationScope::sync_lease(
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
    family: WorthServerOperationFamily,
    prepared_kind: WorthServerPreparedQueryHandoffKind,
    policy: Option<&WorthServerOperationAuthorizationPolicy>,
    admission: &WorthServerAdmission,
) -> Result<String, String> {
    let lane = admit_authorization_lane(family, prepared_kind)?;
    let policy_suffix = policy
        .unwrap_or(&WorthServerOperationAuthorizationPolicy::AllowAuthenticated)
        .authorize(admission)?;
    Ok(format!("{lane}|policy={policy_suffix}"))
}

fn admit_authorization_lane(
    family: WorthServerOperationFamily,
    prepared_kind: WorthServerPreparedQueryHandoffKind,
) -> Result<String, String> {
    let admitted = match family {
        WorthServerOperationFamily::QueryDirectRead
        | WorthServerOperationFamily::QueryDirectProjection
        | WorthServerOperationFamily::ProductApplicationRead => matches!(
            prepared_kind,
            WorthServerPreparedQueryHandoffKind::QueryRead
                | WorthServerPreparedQueryHandoffKind::WorthNativeSession
        ),
        WorthServerOperationFamily::QueryDirectSubmission
        | WorthServerOperationFamily::ProductApplicationMutation => matches!(
            prepared_kind,
            WorthServerPreparedQueryHandoffKind::QueryMutation
                | WorthServerPreparedQueryHandoffKind::WorthNativeSession
        ),
        WorthServerOperationFamily::BinaryTransfer => matches!(
            prepared_kind,
            WorthServerPreparedQueryHandoffKind::QueryRead
                | WorthServerPreparedQueryHandoffKind::QueryMutation
                | WorthServerPreparedQueryHandoffKind::WorthNativeSession
        ),
        WorthServerOperationFamily::ProductSessionCoordination => matches!(
            prepared_kind,
            WorthServerPreparedQueryHandoffKind::WorthNativeSession
                | WorthServerPreparedQueryHandoffKind::QueryMutation
        ),
        WorthServerOperationFamily::SyncLease => matches!(
            prepared_kind,
            WorthServerPreparedQueryHandoffKind::QueryRead
                | WorthServerPreparedQueryHandoffKind::WorthNativeSession
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
    family: WorthServerOperationFamily,
    basis_kind: &str,
) -> Result<(), String> {
    match family {
        WorthServerOperationFamily::QueryDirectRead
        | WorthServerOperationFamily::QueryDirectProjection => {
            if basis_kind == "query-shared-read-basis" {
                Ok(())
            } else {
                Err(format!(
                    "query shared-read authority requires `query-shared-read-basis`, found `{basis_kind}`"
                ))
            }
        }
        WorthServerOperationFamily::ProductApplicationRead => match basis_kind {
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

pub(super) fn validate_shared_read_basis_digest(
    operation_request: &WorthServerOperationRequest,
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
    operation_request: &WorthServerOperationRequest,
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
    operation_request: &WorthServerOperationRequest,
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
    operation_request: &WorthServerOperationRequest,
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
