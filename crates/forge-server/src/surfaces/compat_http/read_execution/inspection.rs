use forge_proof::TransitionOutcome;
use forge_query::facade::ForgeQueryUnifiedInspectionResult;

use crate::{
    ForgeServerCompatibilityCachePolicy, ForgeServerCompatibilityCertificationBundle,
    ForgeServerCompatibilityFacade, ForgeServerCompatibilityFileEnvelope,
    ForgeServerDirectContextArtifact, ForgeServerDirectRemaskPosture,
    ForgeServerExternalBasisRequest, ForgeServerOperationFamily, ForgeServerOperationRequestFacade,
    ForgeServerQueryHandoffOperation, ForgeServerQuerySupportPosture, ForgeServerReadValidator,
    ForgeServerResponseEnvelope, ForgeServerResponseInput,
};

use super::super::project_metadata_inspection_envelope;
use super::{
    execution::{ForgeServerCompatibilityExecutionInput, ForgeServerCompatibilityExecutionOutcome},
    query_support::{
        admitted_named_read_declaration, compatibility_basis_request, compatibility_plan,
        runtime_error_outcome,
    },
};

#[derive(Debug)]
pub struct ForgeServerCompatibilityInspection {
    operation_name: String,
    plan_proof: crate::ForgeServerOperationPlanProof,
    support_posture: ForgeServerQuerySupportPosture,
    workspace_name: String,
    declaration_digest: String,
    handoff_digest: String,
    direct_context: ForgeServerDirectContextArtifact,
    basis_request: ForgeServerExternalBasisRequest,
    inspection_result: ForgeQueryUnifiedInspectionResult,
    response_envelope: ForgeServerResponseEnvelope,
    validator: ForgeServerReadValidator,
    cache_policy: ForgeServerCompatibilityCachePolicy,
    file_envelope: ForgeServerCompatibilityFileEnvelope,
    certification_bundle: ForgeServerCompatibilityCertificationBundle,
    canonical_digest: String,
}

impl ForgeServerCompatibilityInspection {
    pub(crate) fn new(
        operation_name: impl Into<String>,
        plan_proof: crate::ForgeServerOperationPlanProof,
        support_posture: ForgeServerQuerySupportPosture,
        workspace_name: String,
        declaration_digest: String,
        handoff_digest: String,
        direct_context: ForgeServerDirectContextArtifact,
        basis_request: ForgeServerExternalBasisRequest,
        inspection_result: ForgeQueryUnifiedInspectionResult,
        response_envelope: ForgeServerResponseEnvelope,
        validator: ForgeServerReadValidator,
        cache_policy: ForgeServerCompatibilityCachePolicy,
        certification_bundle: ForgeServerCompatibilityCertificationBundle,
    ) -> Self {
        let operation_name = operation_name.into().trim().to_string();
        let file_envelope = project_metadata_inspection_envelope(
            &direct_context,
            &operation_name,
            inspection_result.receipt().result_digest(),
            &response_envelope,
            &support_posture,
            &cache_policy,
        );
        let canonical_digest = format!(
            "forge-server-compat-inspection-v3:{}:{}:{}:{}:{}:{}:{}:{}",
            operation_name,
            handoff_digest,
            basis_request.canonical_digest(),
            validator.canonical_digest(),
            cache_policy.canonical_digest(),
            inspection_result.receipt().result_digest(),
            file_envelope.canonical_digest(),
            certification_bundle.canonical_digest(),
        );
        Self {
            operation_name,
            plan_proof,
            support_posture,
            workspace_name,
            declaration_digest,
            handoff_digest,
            direct_context,
            basis_request,
            inspection_result,
            response_envelope,
            validator,
            cache_policy,
            file_envelope,
            certification_bundle,
            canonical_digest,
        }
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub fn plan_proof(&self) -> &crate::ForgeServerOperationPlanProof {
        &self.plan_proof
    }

    pub fn support_posture(&self) -> &ForgeServerQuerySupportPosture {
        &self.support_posture
    }

    pub fn workspace_name(&self) -> &str {
        &self.workspace_name
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn handoff_digest(&self) -> &str {
        &self.handoff_digest
    }

    pub fn direct_context(&self) -> &ForgeServerDirectContextArtifact {
        &self.direct_context
    }

    pub fn basis_request(&self) -> &ForgeServerExternalBasisRequest {
        &self.basis_request
    }

    pub fn inspection_result(&self) -> &ForgeQueryUnifiedInspectionResult {
        &self.inspection_result
    }

    pub fn response_envelope(&self) -> &ForgeServerResponseEnvelope {
        &self.response_envelope
    }

    pub fn validator(&self) -> &ForgeServerReadValidator {
        &self.validator
    }

    pub fn cache_policy(&self) -> &ForgeServerCompatibilityCachePolicy {
        &self.cache_policy
    }

    pub fn file_envelope(&self) -> &ForgeServerCompatibilityFileEnvelope {
        &self.file_envelope
    }

    pub fn certification_bundle(&self) -> &ForgeServerCompatibilityCertificationBundle {
        &self.certification_bundle
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

impl ForgeServerCompatibilityFacade {
    pub fn inspect(
        &self,
        input: ForgeServerCompatibilityExecutionInput,
    ) -> ForgeServerCompatibilityExecutionOutcome<ForgeServerCompatibilityInspection> {
        let (prepared_request, operation_name) = input.into_parts();
        if let Err(denial) = self.admit_operation_family_for_query(
            prepared_request
                .admission()
                .request_context()
                .diagnostics_profile(),
            ForgeServerOperationFamily::QueryDirectRead,
        ) {
            return TransitionOutcome::Denied(denial);
        }
        if let Err(denial) = crate::surfaces::compat_http::validate_canonical_filename(
            &operation_name,
            prepared_request
                .admission()
                .request_context()
                .diagnostics_profile(),
            crate::ForgeServerQueryHandoffDenialCode::DirectDeclarationBindingInvalid,
        ) {
            return TransitionOutcome::Denied(denial);
        }
        if let Err(denial) = crate::surfaces::compat_http::validate_operation_name_binding(
            prepared_request.request_contract(),
            &operation_name,
            crate::ForgeServerQueryHandoffDenialCode::DirectDeclarationBindingInvalid,
            prepared_request
                .admission()
                .request_context()
                .diagnostics_profile(),
        ) {
            return TransitionOutcome::Denied(denial);
        }
        let basis_request = match compatibility_basis_request(&prepared_request) {
            Ok(value) => value,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        let declaration =
            match admitted_named_read_declaration(self, &prepared_request, &operation_name) {
                Ok(value) => value,
                Err(denial) => return TransitionOutcome::Denied(denial),
            };
        let observed_basis_digest = match declaration.subscription_basis_digest() {
            Ok(value) => value,
            Err(error) => return runtime_error_outcome(&prepared_request, error),
        };
        if let Err(denial) = basis_request.validate_observed_basis(
            prepared_request
                .admission()
                .request_context()
                .diagnostics_profile(),
            Some(&observed_basis_digest),
        ) {
            return TransitionOutcome::Denied(denial);
        }
        let operation_request =
            match ForgeServerOperationRequestFacade::new(self.operation_registry.clone())
                .admit_from_compat_http_with_basis_digest(
                    &prepared_request,
                    ForgeServerOperationFamily::QueryDirectRead,
                    &operation_name,
                    Some(&observed_basis_digest),
                    None,
                ) {
                Ok(value) => value,
                Err(denial) => {
                    return TransitionOutcome::Denied(
                        super::query_execution::map_operation_request_denial(denial),
                    );
                }
            };

        let operation_admission =
            match crate::ForgeServerOperationAdmissionFacade::with_operation_registry(
                self.operation_registry.clone(),
            )
            .admit_declared(prepared_request.admission(), &operation_request)
            {
                Ok(value) => value,
                Err(denial) => {
                    return TransitionOutcome::Denied(
                        crate::surfaces::compat_http::map_operation_admission_denial(denial),
                    );
                }
            };
        let plan = match compatibility_plan(
            self,
            operation_admission,
            ForgeServerQueryHandoffOperation::direct_inspection(
                declaration.declaration_canonical_label(),
            ),
        ) {
            Ok(value) => value,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        let plan_proof = plan.proof();
        let handoff = plan.into_query_handoff();
        let inspection_result = match declaration.inspect_named_live_view() {
            Ok(value) => value,
            Err(error) => return runtime_error_outcome(&prepared_request, error),
        };
        let support_posture = handoff.support_posture().clone();
        let workspace_name = handoff.workspace().name().to_string();
        let handoff_digest = handoff.canonical_digest().to_string();
        let response_envelope = self
            .responses
            .shape_with_defaults(ForgeServerResponseInput::query_handoff_success(handoff));
        let (basis_digest, remask_posture) = match inspection_result.inspection() {
            forge_query::facade::ForgeQueryInspection::LiveView(live) => {
                let basis = live
                    .basis_binding_identity()
                    .terminal_projection_for_reporting();
                (
                    Some(basis),
                    ForgeServerDirectRemaskPosture::from_live_inspection(live),
                )
            }
            _ => (None, ForgeServerDirectRemaskPosture::visible()),
        };
        let direct_context = ForgeServerDirectContextArtifact::new(
            prepared_request.admission().request_context(),
            &support_posture,
            &response_envelope,
            basis_digest,
            remask_posture,
        );
        let validator = ForgeServerReadValidator::new(
            inspection_result.receipt().result_digest(),
            direct_context.basis_digest(),
            direct_context.branch_digest(),
        );
        let cache_policy = ForgeServerCompatibilityCachePolicy::for_scoped_read(
            &prepared_request,
            direct_context.remask_posture(),
        );
        let file_envelope = crate::surfaces::compat_http::project_metadata_inspection_envelope(
            &direct_context,
            &operation_name,
            inspection_result.receipt().result_digest(),
            &response_envelope,
            &support_posture,
            &cache_policy,
        );
        let certification_bundle =
            crate::surfaces::compat_http::build_inspection_certification_bundle(
                &self.operator_evidence,
                &support_posture,
                &file_envelope,
                &response_envelope,
            );
        TransitionOutcome::Success(ForgeServerCompatibilityInspection::new(
            operation_name,
            plan_proof,
            support_posture,
            workspace_name,
            declaration.declaration_digest().to_string(),
            handoff_digest,
            direct_context,
            basis_request,
            inspection_result,
            response_envelope,
            validator,
            cache_policy,
            certification_bundle,
        ))
    }
}
