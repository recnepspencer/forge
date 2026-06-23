use crate::{ForgeServerAdmission, ForgeServerOperationRegistry, ForgeServerOperationRequest};

use super::{
    admission_logic::{admit_metadata_for_family, authorize_operation, operation_reference_digest},
    ForgeServerOperationAdmissionDenial, ForgeServerOperationAdmissionDenialCode,
    ForgeServerOperationAdmissionPosture, ForgeServerOperationAuthorityFootprint,
    ForgeServerOperationAuthorityMetadata, ForgeServerOperationAuthorizationProof,
    ForgeServerOperationFootprintReceipt,
};

#[derive(Clone, Debug, Default)]
pub struct ForgeServerOperationAdmissionFacade {
    operation_registry: Option<ForgeServerOperationRegistry>,
}

impl ForgeServerOperationAdmissionFacade {
    pub(crate) fn with_operation_registry(
        operation_registry: ForgeServerOperationRegistry,
    ) -> Self {
        Self {
            operation_registry: Some(operation_registry),
        }
    }

    pub fn admit_declared(
        &self,
        admission: &ForgeServerAdmission,
        operation_request: &ForgeServerOperationRequest,
    ) -> Result<ForgeServerOperationAdmissionPosture, ForgeServerOperationAdmissionDenial> {
        let diagnostics_profile = operation_request.receipt().diagnostics_profile();
        let operation_registry = self.operation_registry.as_ref().ok_or_else(|| {
            ForgeServerOperationAdmissionDenial::new(
                ForgeServerOperationAdmissionDenialCode::AuthorityDenied,
                diagnostics_profile,
                "declared operation authority admission requires an operation registry-backed facade",
            )
        })?;
        let metadata = operation_registry
            .declared_authority_for(operation_request)
            .map_err(|detail| {
                ForgeServerOperationAdmissionDenial::new(
                    ForgeServerOperationAdmissionDenialCode::AuthorityDenied,
                    diagnostics_profile,
                    detail,
                )
            })?;
        self.admit(admission, operation_request, metadata)
    }

    pub fn admit(
        &self,
        admission: &ForgeServerAdmission,
        operation_request: &ForgeServerOperationRequest,
        metadata: ForgeServerOperationAuthorityMetadata,
    ) -> Result<ForgeServerOperationAdmissionPosture, ForgeServerOperationAdmissionDenial> {
        let diagnostics_profile = operation_request.receipt().diagnostics_profile();
        if admission.resolved_request_context() != operation_request.resolved_request_context() {
            return Err(ForgeServerOperationAdmissionDenial::new(
                ForgeServerOperationAdmissionDenialCode::AuthorizationDenied,
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
            ForgeServerOperationAdmissionDenial::new(
                ForgeServerOperationAdmissionDenialCode::AuthorityDenied,
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
            ForgeServerOperationAdmissionDenial::new(
                ForgeServerOperationAdmissionDenialCode::AuthorizationDenied,
                diagnostics_profile,
                detail,
            )
        })?;

        let footprint = ForgeServerOperationAuthorityFootprint::new(
            authority_kind,
            scope,
            metadata.canonical_digest(),
        );
        let footprint_receipt = ForgeServerOperationFootprintReceipt::new(
            metadata.canonical_digest(),
            footprint.canonical_digest(),
        );
        let authorization_proof = ForgeServerOperationAuthorizationProof::new(
            admission.clone(),
            operation_reference_digest(operation_request),
            footprint.canonical_digest(),
            authorization_lane,
        );
        Ok(ForgeServerOperationAdmissionPosture::new(
            operation_request.clone(),
            metadata,
            footprint,
            footprint_receipt,
            authorization_proof,
        ))
    }
}
