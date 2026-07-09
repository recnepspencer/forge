use crate::{WorthServerAdmission, WorthServerOperationRegistry, WorthServerOperationRequest};

use super::{
    admission_logic::{admit_metadata_for_family, authorize_operation, operation_reference_digest},
    WorthServerOperationAdmissionDenial, WorthServerOperationAdmissionDenialCode,
    WorthServerOperationAdmissionPosture, WorthServerOperationAuthorityFootprint,
    WorthServerOperationAuthorityMetadata, WorthServerOperationAuthorizationProof,
    WorthServerOperationFootprintReceipt,
};

#[derive(Clone, Debug, Default)]
pub struct WorthServerOperationAdmissionFacade {
    operation_registry: Option<WorthServerOperationRegistry>,
}

impl WorthServerOperationAdmissionFacade {
    pub(crate) fn with_operation_registry(
        operation_registry: WorthServerOperationRegistry,
    ) -> Self {
        Self {
            operation_registry: Some(operation_registry),
        }
    }

    pub fn admit_declared(
        &self,
        admission: &WorthServerAdmission,
        operation_request: &WorthServerOperationRequest,
    ) -> Result<WorthServerOperationAdmissionPosture, WorthServerOperationAdmissionDenial> {
        let diagnostics_profile = operation_request.receipt().diagnostics_profile();
        let operation_registry = self.operation_registry.as_ref().ok_or_else(|| {
            WorthServerOperationAdmissionDenial::new(
                WorthServerOperationAdmissionDenialCode::AuthorityDenied,
                diagnostics_profile,
                "declared operation authority admission requires an operation registry-backed facade",
            )
        })?;
        let metadata = operation_registry
            .declared_authority_for(operation_request)
            .map_err(|detail| {
                WorthServerOperationAdmissionDenial::new(
                    WorthServerOperationAdmissionDenialCode::AuthorityDenied,
                    diagnostics_profile,
                    detail,
                )
            })?;
        self.admit(admission, operation_request, metadata)
    }

    pub fn admit(
        &self,
        admission: &WorthServerAdmission,
        operation_request: &WorthServerOperationRequest,
        metadata: WorthServerOperationAuthorityMetadata,
    ) -> Result<WorthServerOperationAdmissionPosture, WorthServerOperationAdmissionDenial> {
        let diagnostics_profile = operation_request.receipt().diagnostics_profile();
        if admission.resolved_request_context() != operation_request.resolved_request_context() {
            return Err(WorthServerOperationAdmissionDenial::new(
                WorthServerOperationAdmissionDenialCode::AuthorizationDenied,
                diagnostics_profile,
                "operation authority admission requires the middleware admission and operation request to share the same resolved request context",
            ));
        }

        let (authority_kind, scope) = admit_metadata_for_family(
            operation_request.identity().operation_family(),
            admission,
            operation_request,
            &metadata,
        )
        .map_err(|detail| {
            WorthServerOperationAdmissionDenial::new(
                WorthServerOperationAdmissionDenialCode::AuthorityDenied,
                diagnostics_profile,
                detail,
            )
        })?;
        let authorization_lane = authorize_operation(
            operation_request.identity().operation_family(),
            admission.query_handoff_intent().kind(),
            self.operation_registry.as_ref().and_then(|registry| {
                registry.authorization_policy_for(operation_request.identity().operation_family())
            }),
            admission,
        )
        .map_err(|detail| {
            WorthServerOperationAdmissionDenial::new(
                WorthServerOperationAdmissionDenialCode::AuthorizationDenied,
                diagnostics_profile,
                detail,
            )
        })?;

        let footprint = WorthServerOperationAuthorityFootprint::new(
            authority_kind,
            scope,
            metadata.canonical_digest(),
        );
        let footprint_receipt = WorthServerOperationFootprintReceipt::new(
            metadata.canonical_digest(),
            footprint.canonical_digest(),
        );
        let authorization_proof = WorthServerOperationAuthorizationProof::new(
            admission.clone(),
            operation_reference_digest(operation_request),
            footprint.canonical_digest(),
            authorization_lane,
        );
        Ok(WorthServerOperationAdmissionPosture::new(
            operation_request.clone(),
            metadata,
            footprint,
            footprint_receipt,
            authorization_proof,
        ))
    }
}
