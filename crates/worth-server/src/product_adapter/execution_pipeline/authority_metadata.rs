use crate::{
    WorthServerOperationAuthorityMetadata, WorthServerProductOperationAuthorityRequirement,
    WorthServerProductOperationDeclaration, WorthServerProductOperationSurfaceDenial,
    WorthServerProductOperationSurfaceDenialCode, WorthServerProductSessionCoordinationTarget,
    WorthServerProductSupportPosture,
};

pub(in crate::product_adapter) fn declaration_metadata(
    declaration: &WorthServerProductOperationDeclaration,
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
                    declaration.basis_kind().as_str(),
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
        WorthServerProductOperationAuthorityRequirement::DurableMutation { contract } => {
            let expected_basis_digest = request.identity().basis_digest().ok_or_else(|| {
                WorthServerProductOperationSurfaceDenial::new(
                    WorthServerProductOperationSurfaceDenialCode::PreconditionDenied,
                    "durable product mutation requires an expected basis precondition".into(),
                )
            })?;
            let idempotency_key = request.identity().idempotency_key().ok_or_else(|| {
                WorthServerProductOperationSurfaceDenial::new(
                    WorthServerProductOperationSurfaceDenialCode::PreconditionDenied,
                    "durable product mutation requires an idempotency key".into(),
                )
            })?;
            Ok(
                WorthServerOperationAuthorityMetadata::durable_product_mutation(
                    contract.authority_scope().value(),
                    expected_basis_digest,
                    idempotency_key,
                    contract.canonical_digest(),
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

fn product_support_posture_label(posture: WorthServerProductSupportPosture) -> &'static str {
    match posture {
        WorthServerProductSupportPosture::ProductionAdmitted => "production-admitted",
        WorthServerProductSupportPosture::Unsupported => "unsupported",
        WorthServerProductSupportPosture::Unknown => "unknown",
        WorthServerProductSupportPosture::IncompatibleBasis => "incompatible-basis",
    }
}
