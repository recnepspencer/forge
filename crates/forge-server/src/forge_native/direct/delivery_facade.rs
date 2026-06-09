use forge_proof::TransitionOutcome;

use crate::{
    ForgeServerAdmittedDirectDeclaration, ForgeServerDirectContextArtifact,
    ForgeServerDirectDeliveryContract, ForgeServerDirectDeliveryRequest,
    ForgeServerDirectLeaseDeclaration, ForgeServerDirectRemaskPosture,
    ForgeServerQueryHandoffOperation, ForgeServerResponseInput,
};

use super::ForgeServerForgeNativeDirectFacade;

pub type ForgeServerDirectLeaseDeclarationOutcome = TransitionOutcome<
    ForgeServerDirectLeaseDeclaration,
    crate::ForgeServerQueryHandoffDenial,
    crate::ForgeServerQueryHandoffDeferred,
    crate::ForgeServerQueryHandoffStale,
    crate::ForgeServerQueryHandoffRebindRequired,
    crate::ForgeServerQueryHandoffFailure,
>;

pub type ForgeServerDirectDeliveryOutcome = TransitionOutcome<
    ForgeServerDirectDeliveryContract,
    crate::ForgeServerQueryHandoffDenial,
    crate::ForgeServerQueryHandoffDeferred,
    crate::ForgeServerQueryHandoffStale,
    crate::ForgeServerQueryHandoffRebindRequired,
    crate::ForgeServerQueryHandoffFailure,
>;

impl ForgeServerForgeNativeDirectFacade {
    pub fn declare_lease(
        &self,
        declaration: &ForgeServerAdmittedDirectDeclaration,
    ) -> ForgeServerDirectLeaseDeclarationOutcome {
        match ForgeServerDirectLeaseDeclaration::from_admitted_declaration(declaration) {
            Ok(lease) => TransitionOutcome::Success(lease),
            Err(error) => self.runtime_error_outcome(error),
        }
    }

    pub fn negotiate_delivery(
        &self,
        lease: &ForgeServerDirectLeaseDeclaration,
        request: &ForgeServerDirectDeliveryRequest,
    ) -> ForgeServerDirectDeliveryOutcome {
        if let Some(denial) = validate_runtime_backed_resume_request(
            self.admission.request_context().diagnostics_profile(),
            lease,
            request,
        ) {
            return TransitionOutcome::Denied(denial);
        }
        match self.prepare_handoff(ForgeServerQueryHandoffOperation::downstream_delivery(
            lease.declaration_binding_label(),
            request.freshness_mode(),
            request.delivery_class(),
            request.requested_resume().clone(),
        )) {
            TransitionOutcome::Success(handoff) => {
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
                    return TransitionOutcome::Denied(crate::ForgeServerQueryHandoffDenial::new(
                        crate::ForgeServerQueryHandoffDenialCode::LeaseDeclarationContextMismatch,
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
                    .shape_with_defaults(ForgeServerResponseInput::query_handoff_success(handoff));
                let direct_context = ForgeServerDirectContextArtifact::new(
                    self.admission.request_context(),
                    &support_posture,
                    &response_envelope,
                    Some(lease.resume_basis_digest()),
                    ForgeServerDirectRemaskPosture::visible(),
                );
                TransitionOutcome::Success(ForgeServerDirectDeliveryContract::new(
                    support_posture,
                    workspace_name,
                    handoff_digest,
                    direct_context,
                    lease.clone(),
                    request.clone(),
                    downstream_delivery_contract,
                    response_envelope,
                ))
            }
            TransitionOutcome::Denied(denial) => TransitionOutcome::Denied(denial),
            TransitionOutcome::Deferred(deferred) => TransitionOutcome::Deferred(deferred),
            TransitionOutcome::Stale(stale) => TransitionOutcome::Stale(stale),
            TransitionOutcome::RebindRequired(rebind) => TransitionOutcome::RebindRequired(rebind),
            TransitionOutcome::Failed(failure) => TransitionOutcome::Failed(failure),
        }
    }
}

fn validate_runtime_backed_resume_request(
    diagnostics_profile: forge_foundational::DiagnosticRichnessProfile,
    lease: &ForgeServerDirectLeaseDeclaration,
    request: &ForgeServerDirectDeliveryRequest,
) -> Option<crate::ForgeServerQueryHandoffDenial> {
    let crate::ForgeServerQueryRequestedResume::RuntimeBacked { basis_digest } =
        request.requested_resume()
    else {
        return None;
    };

    match basis_digest.as_deref() {
        None => Some(crate::ForgeServerQueryHandoffDenial::new(
            crate::ForgeServerQueryHandoffDenialCode::RuntimeBackedResumeMissingBasis,
            diagnostics_profile,
            format!(
                "runtime-backed resume requires basis `{}`",
                lease.resume_basis_digest()
            ),
        )),
        Some(candidate) if candidate != lease.resume_basis_digest() => {
            Some(crate::ForgeServerQueryHandoffDenial::new(
                crate::ForgeServerQueryHandoffDenialCode::RuntimeBackedResumeStaleBasis,
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
