use worth_proof::TransitionOutcome;

use crate::{
    WorthServerAdmittedDirectDeclaration, WorthServerDirectContextArtifact,
    WorthServerDirectDeliveryContract, WorthServerDirectDeliveryRequest,
    WorthServerDirectLeaseDeclaration, WorthServerDirectRemaskPosture, WorthServerOperationFamily,
    WorthServerOperationRequestFacade, WorthServerOperationRequestInput,
    WorthServerQueryHandoffOperation, WorthServerResponseInput,
};

use super::WorthServerWorthNativeDirectFacade;

pub type WorthServerDirectLeaseDeclarationOutcome = TransitionOutcome<
    WorthServerDirectLeaseDeclaration,
    crate::WorthServerQueryHandoffDenial,
    crate::WorthServerQueryHandoffDeferred,
    crate::WorthServerQueryHandoffStale,
    crate::WorthServerQueryHandoffRebindRequired,
    crate::WorthServerQueryHandoffFailure,
>;

pub type WorthServerDirectDeliveryOutcome = TransitionOutcome<
    WorthServerDirectDeliveryContract,
    crate::WorthServerQueryHandoffDenial,
    crate::WorthServerQueryHandoffDeferred,
    crate::WorthServerQueryHandoffStale,
    crate::WorthServerQueryHandoffRebindRequired,
    crate::WorthServerQueryHandoffFailure,
>;

impl WorthServerWorthNativeDirectFacade {
    pub fn declare_lease(
        &self,
        declaration: &WorthServerAdmittedDirectDeclaration,
    ) -> WorthServerDirectLeaseDeclarationOutcome {
        if let Err(denial) = self.admit_operation_family(WorthServerOperationFamily::SyncLease) {
            return self.operation_denial_outcome(denial);
        }
        match WorthServerDirectLeaseDeclaration::from_admitted_declaration(declaration) {
            Ok(lease) => TransitionOutcome::Success(lease),
            Err(error) => self.runtime_error_outcome(error),
        }
    }

    pub fn negotiate_delivery(
        &self,
        lease: &WorthServerDirectLeaseDeclaration,
        request: &WorthServerDirectDeliveryRequest,
    ) -> WorthServerDirectDeliveryOutcome {
        if let Err(denial) = self.admit_operation_family(WorthServerOperationFamily::SyncLease) {
            return self.operation_denial_outcome(denial);
        }
        if let Some(denial) = validate_runtime_backed_resume_request(
            self.admission.request_context().diagnostics_profile(),
            lease,
            request,
        ) {
            return TransitionOutcome::Denied(denial);
        }
        let operation_request =
            match WorthServerOperationRequestFacade::new(self.operation_registry.clone())
                .admit_from_worth_native_admission(
                    &self.admission,
                    WorthServerOperationRequestInput::builder()
                        .with_operation_family(WorthServerOperationFamily::SyncLease)
                        .with_operation_name(lease.declaration_binding_label())
                        .with_basis_digest(lease.resume_basis_digest())
                        .build(),
                ) {
                Ok(value) => value,
                Err(denial) => {
                    return TransitionOutcome::Denied(crate::WorthServerQueryHandoffDenial::new(
                        crate::WorthServerQueryHandoffDenialCode::AuthorizationDenied,
                        denial.diagnostics_profile(),
                        denial.detail(),
                    ));
                }
            };
        match self.prepare_declared_plan(
            operation_request,
            WorthServerQueryHandoffOperation::downstream_delivery(
                lease.declaration_binding_label(),
                request.freshness_mode(),
                request.delivery_class(),
                request.requested_resume().clone(),
            ),
        ) {
            Ok(plan) => {
                let plan_proof = plan.proof();
                let handoff = plan.into_query_handoff();
                let request_context = self.admission.resolved_request_context().request_context();
                let current_principal_id = request_context.authenticated_principal().principal_id();
                let current_tenant_id = request_context.workspace_target().tenant_id();
                let current_workspace_id = request_context.workspace_target().workspace_id();
                let current_branch_label = request_context.branch_target().canonical_label();
                let support_posture = handoff.support_posture().clone();
                let workspace_name = handoff.workspace().name().to_string();
                if current_principal_id != lease.principal_id()
                    || current_tenant_id != lease.tenant_id()
                    || current_workspace_id != lease.workspace_id()
                    || current_branch_label != lease.branch_label()
                    || workspace_name != lease.workspace_name()
                {
                    return TransitionOutcome::Denied(crate::WorthServerQueryHandoffDenial::new(
                        crate::WorthServerQueryHandoffDenialCode::LeaseDeclarationContextMismatch,
                        self.admission.request_context().diagnostics_profile(),
                        format!(
                            "lease context principal=`{}` tenant=`{}` workspace=`{}` branch=`{}` bound=`{}` does not match negotiated principal=`{}` tenant=`{}` workspace=`{}` branch=`{}` bound=`{workspace_name}`",
                            lease.principal_id(),
                            lease.tenant_id(),
                            lease.workspace_id(),
                            lease.branch_label(),
                            lease.workspace_name(),
                            current_principal_id,
                            current_tenant_id,
                            current_workspace_id,
                            current_branch_label,
                        ),
                    ));
                }
                let handoff_digest = handoff.canonical_digest().to_string();
                let downstream_delivery_contract = handoff.downstream_delivery_contract().clone();
                let response_envelope = self
                    .responses
                    .shape_with_defaults(WorthServerResponseInput::query_handoff_success(handoff));
                let direct_context = WorthServerDirectContextArtifact::new(
                    self.admission.request_context(),
                    &support_posture,
                    &response_envelope,
                    Some(lease.resume_basis_digest()),
                    WorthServerDirectRemaskPosture::visible(),
                );
                TransitionOutcome::Success(WorthServerDirectDeliveryContract::new(
                    super::WorthServerDirectDeliveryContractParts {
                        plan_proof,
                        support_posture,
                        workspace_name,
                        handoff_digest,
                        direct_context,
                        lease_declaration: lease.clone(),
                        request: request.clone(),
                        downstream_delivery_contract,
                        response_envelope,
                    },
                ))
            }
            Err(denial) => TransitionOutcome::Denied(denial),
        }
    }
}

fn validate_runtime_backed_resume_request(
    diagnostics_profile: worth_foundational::DiagnosticRichnessProfile,
    lease: &WorthServerDirectLeaseDeclaration,
    request: &WorthServerDirectDeliveryRequest,
) -> Option<crate::WorthServerQueryHandoffDenial> {
    let crate::WorthServerQueryRequestedResume::RuntimeBacked { basis_digest } =
        request.requested_resume()
    else {
        return None;
    };

    match basis_digest.as_deref() {
        None => Some(crate::WorthServerQueryHandoffDenial::new(
            crate::WorthServerQueryHandoffDenialCode::RuntimeBackedResumeMissingBasis,
            diagnostics_profile,
            format!(
                "runtime-backed resume requires basis `{}`",
                lease.resume_basis_digest()
            ),
        )),
        Some(candidate) if candidate != lease.resume_basis_digest() => {
            Some(crate::WorthServerQueryHandoffDenial::new(
                crate::WorthServerQueryHandoffDenialCode::RuntimeBackedResumeStaleBasis,
                diagnostics_profile,
                format!(
                    "runtime-backed resume basis `{candidate}` does not match retained basis `{}`",
                    lease.resume_basis_digest()
                ),
            ))
        }
        Some(_) => None,
    }
}
