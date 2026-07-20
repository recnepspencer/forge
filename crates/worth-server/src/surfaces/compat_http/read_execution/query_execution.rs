use worth_proof::TransitionOutcome;

use crate::{
    WorthServerCompatibilityFacade, WorthServerCompatibilityRead, WorthServerDirectContextArtifact,
    WorthServerDirectRemaskPosture, WorthServerOperationFamily, WorthServerOperationRequestDenial,
    WorthServerOperationRequestDenialCode, WorthServerOperationRequestFacade,
    WorthServerQueryHandoffDenialFacts, WorthServerQueryHandoffOperation, WorthServerReadValidator,
    WorthServerResponseInput,
};

use super::{
    cache_policy::WorthServerCompatibilityCachePolicy,
    conditional::WorthServerConditionalRead,
    execution::{WorthServerCompatibilityExecutionInput, WorthServerCompatibilityExecutionOutcome},
    query_support::{
        admitted_named_read_declaration, compatibility_basis_request, compatibility_plan,
        runtime_error_outcome, validate_conditional_read,
    },
};

impl WorthServerCompatibilityFacade {
    pub fn read(
        &self,
        input: WorthServerCompatibilityExecutionInput,
    ) -> WorthServerCompatibilityExecutionOutcome<WorthServerCompatibilityRead> {
        let (prepared_request, operation_name) = input.into_parts();
        if let Err(denial) = self.admit_operation_family_for_query(
            prepared_request
                .admission()
                .request_context()
                .diagnostics_profile(),
            WorthServerOperationFamily::QueryDirectRead,
        ) {
            return TransitionOutcome::Denied(denial);
        }
        if let Err(denial) = crate::surfaces::compat_http::validate_canonical_filename(
            &operation_name,
            prepared_request
                .admission()
                .request_context()
                .diagnostics_profile(),
            crate::WorthServerQueryHandoffDenialCode::DirectDeclarationBindingInvalid,
        ) {
            return TransitionOutcome::Denied(denial);
        }
        if let Err(denial) = crate::surfaces::compat_http::validate_operation_name_binding(
            prepared_request.request_contract(),
            &operation_name,
            crate::WorthServerQueryHandoffDenialCode::DirectDeclarationBindingInvalid,
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
        let conditional_read =
            match WorthServerConditionalRead::from_prepared_request(&prepared_request) {
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
            match WorthServerOperationRequestFacade::new(self.operation_registry.clone())
                .admit_from_compat_http_with_basis_digest(
                    &prepared_request,
                    WorthServerOperationFamily::QueryDirectRead,
                    &operation_name,
                    Some(&observed_basis_digest),
                    None,
                ) {
                Ok(value) => value,
                Err(denial) => {
                    return TransitionOutcome::Denied(map_operation_request_denial(denial));
                }
            };

        let operation_admission =
            match crate::WorthServerOperationAdmissionFacade::with_operation_registry(
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
            WorthServerQueryHandoffOperation::direct_read(&operation_name),
        ) {
            Ok(value) => value,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        let plan_proof = plan.proof();
        let handoff = plan.into_query_handoff();
        let read_result = match declaration.execute_named_live_read() {
            Ok(value) => value,
            Err(error) => return runtime_error_outcome(&prepared_request, error),
        };
        let support_posture = handoff.support_posture().clone();
        let workspace_name = handoff.workspace().name().to_string();
        let handoff_digest = handoff.canonical_digest().to_string();
        let response_envelope = self
            .responses
            .shape_with_defaults(WorthServerResponseInput::query_handoff_success(handoff));
        let direct_context = WorthServerDirectContextArtifact::new(
            prepared_request.admission().request_context(),
            &support_posture,
            &response_envelope,
            Some(&observed_basis_digest),
            WorthServerDirectRemaskPosture::visible(),
        );
        let validator = WorthServerReadValidator::new(
            read_result.receipt().result_digest(),
            direct_context.basis_digest(),
            direct_context.branch_digest(),
        );
        if let Err(denial) =
            validate_conditional_read(&prepared_request, &conditional_read, &validator)
        {
            return TransitionOutcome::Denied(denial);
        }
        let cache_policy = WorthServerCompatibilityCachePolicy::for_scoped_read(
            &prepared_request,
            direct_context.remask_posture(),
        );
        let file_envelope = crate::surfaces::compat_http::project_metadata_read_envelope(
            &direct_context,
            &operation_name,
            read_result.receipt().result_digest(),
            &response_envelope,
            &support_posture,
            &cache_policy,
        );
        let certification_bundle = crate::surfaces::compat_http::build_read_certification_bundle(
            &self.operator_evidence,
            &support_posture,
            &file_envelope,
            &response_envelope,
        );
        TransitionOutcome::Success(WorthServerCompatibilityRead::new(
            super::WorthServerCompatibilityReadParts {
                operation_request,
                plan_proof,
                operation_name,
                support_posture,
                workspace_name,
                declaration_digest: declaration.declaration_digest().to_string(),
                handoff_digest,
                direct_context,
                basis_request,
                conditional_read,
                read_result,
                response_envelope,
                validator,
                cache_policy,
                certification_bundle,
            },
        ))
    }
}

pub(crate) fn map_operation_request_denial(
    denial: WorthServerOperationRequestDenial,
) -> crate::WorthServerQueryHandoffDenial {
    let code = match denial.code() {
        WorthServerOperationRequestDenialCode::CompatibilityBindingInvalid
        | WorthServerOperationRequestDenialCode::InvalidOperationName
        | WorthServerOperationRequestDenialCode::MissingOperationName => {
            crate::WorthServerQueryHandoffDenialCode::DirectDeclarationBindingInvalid
        }
        WorthServerOperationRequestDenialCode::InvalidBasisDigest => {
            crate::WorthServerQueryHandoffDenialCode::CompatibilityBasisRequestInvalid
        }
        WorthServerOperationRequestDenialCode::UnknownOperationName => {
            crate::WorthServerQueryHandoffDenialCode::UnknownOperationName
        }
        _ => crate::WorthServerQueryHandoffDenialCode::CompatibilityConditionalRequestInvalid,
    };
    let rejected_operation_name = match denial.code() {
        WorthServerOperationRequestDenialCode::UnknownOperationName => {
            denial.detail().split('`').nth(1).map(str::to_string)
        }
        _ => None,
    };
    let denial = crate::WorthServerQueryHandoffDenial::new(
        code,
        denial.diagnostics_profile(),
        denial.detail(),
    );
    match rejected_operation_name {
        Some(operation_name) => denial.with_facts(
            WorthServerQueryHandoffDenialFacts::default()
                .with_rejected_operation_name(operation_name),
        ),
        None => denial,
    }
}
