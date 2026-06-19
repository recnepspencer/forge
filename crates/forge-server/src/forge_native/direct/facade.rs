use forge_proof::TransitionOutcome;
use forge_query::facade::ForgeQueryInspection;

use crate::{
    declaration_intake::ForgeServerDirectDeclarationIntakeFacade, ForgeServerAdmission,
    ForgeServerAdmittedDirectDeclaration, ForgeServerOperationAdmissionFacade,
    ForgeServerOperationDenial, ForgeServerOperationFamily, ForgeServerOperationPlanner,
    ForgeServerOperationPlannerInput, ForgeServerOperationRegistry,
    ForgeServerOperationRequestFacade, ForgeServerOperationRequestInput,
    ForgeServerQueryHandoffFacade, ForgeServerQueryHandoffOperation, ForgeServerResponseFacade,
    ForgeServerResponseInput, ForgeServerSurfaceFamily,
};

use super::{
    ForgeServerDirectContextArtifact, ForgeServerDirectInspection, ForgeServerDirectProjection,
    ForgeServerDirectRead, ForgeServerDirectRemaskPosture, ForgeServerDirectState,
};

pub type ForgeServerDirectReadOutcome = TransitionOutcome<
    ForgeServerDirectRead,
    crate::ForgeServerQueryHandoffDenial,
    crate::ForgeServerQueryHandoffDeferred,
    crate::ForgeServerQueryHandoffStale,
    crate::ForgeServerQueryHandoffRebindRequired,
    crate::ForgeServerQueryHandoffFailure,
>;

pub type ForgeServerDirectStateOutcome = TransitionOutcome<
    ForgeServerDirectState,
    crate::ForgeServerQueryHandoffDenial,
    crate::ForgeServerQueryHandoffDeferred,
    crate::ForgeServerQueryHandoffStale,
    crate::ForgeServerQueryHandoffRebindRequired,
    crate::ForgeServerQueryHandoffFailure,
>;

pub type ForgeServerDirectInspectionOutcome = TransitionOutcome<
    ForgeServerDirectInspection,
    crate::ForgeServerQueryHandoffDenial,
    crate::ForgeServerQueryHandoffDeferred,
    crate::ForgeServerQueryHandoffStale,
    crate::ForgeServerQueryHandoffRebindRequired,
    crate::ForgeServerQueryHandoffFailure,
>;

pub type ForgeServerDirectProjectionOutcome = TransitionOutcome<
    ForgeServerDirectProjection,
    crate::ForgeServerQueryHandoffDenial,
    crate::ForgeServerQueryHandoffDeferred,
    crate::ForgeServerQueryHandoffStale,
    crate::ForgeServerQueryHandoffRebindRequired,
    crate::ForgeServerQueryHandoffFailure,
>;

pub type ForgeServerDirectMutationOutcome = TransitionOutcome<
    crate::ForgeServerDirectMutation,
    crate::ForgeServerQueryHandoffDenial,
    crate::ForgeServerQueryHandoffDeferred,
    crate::ForgeServerQueryHandoffStale,
    crate::ForgeServerQueryHandoffRebindRequired,
    crate::ForgeServerQueryHandoffFailure,
>;

#[derive(Clone, Debug)]
pub struct ForgeServerForgeNativeDirectFacade {
    pub(super) admission: ForgeServerAdmission,
    pub(super) operation_registry: ForgeServerOperationRegistry,
    pub(super) declaration_intake: ForgeServerDirectDeclarationIntakeFacade,
    pub(super) query_handoff: ForgeServerQueryHandoffFacade,
    pub(super) responses: ForgeServerResponseFacade,
}

impl ForgeServerForgeNativeDirectFacade {
    pub(crate) fn new(
        admission: ForgeServerAdmission,
        operation_registry: ForgeServerOperationRegistry,
        declaration_intake: ForgeServerDirectDeclarationIntakeFacade,
        query_handoff: ForgeServerQueryHandoffFacade,
        responses: ForgeServerResponseFacade,
    ) -> Self {
        Self {
            admission,
            operation_registry,
            declaration_intake,
            query_handoff,
            responses,
        }
    }

    pub(super) fn admit_operation_family(
        &self,
        family: ForgeServerOperationFamily,
    ) -> Result<(), ForgeServerOperationDenial> {
        self.operation_registry
            .admit(ForgeServerSurfaceFamily::ForgeNative, family)
            .map(|_| ())
    }

    pub fn read(
        &self,
        declaration: &ForgeServerAdmittedDirectDeclaration,
    ) -> ForgeServerDirectReadOutcome {
        if let Err(denial) =
            self.admit_operation_family(ForgeServerOperationFamily::QueryDirectRead)
        {
            return self.operation_denial_outcome(denial);
        }
        let observed_basis_digest = match declaration.subscription_basis_digest() {
            Ok(value) => value,
            Err(error) => return self.runtime_error_outcome(error),
        };
        let operation_name = declaration.declaration_binding_label().to_string();
        let operation_request = match self.admit_shared_read_operation_request(
            ForgeServerOperationFamily::QueryDirectRead,
            &operation_name,
            &observed_basis_digest,
        ) {
            Ok(value) => value,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        match self.prepare_declared_plan(
            operation_request,
            ForgeServerQueryHandoffOperation::direct_read(&operation_name),
        ) {
            Ok(plan) => {
                let plan_proof = plan.proof();
                let handoff = plan.into_query_handoff();
                let read_result = match declaration.execute_named_live_read() {
                    Ok(read_result) => read_result,
                    Err(error) => return self.runtime_error_outcome(error),
                };
                let support_posture = handoff.support_posture().clone();
                let workspace_name = handoff.workspace().name().to_string();
                let handoff_digest = handoff.canonical_digest().to_string();
                let response_envelope = self
                    .responses
                    .shape_with_defaults(ForgeServerResponseInput::query_handoff_success(handoff));
                let direct_context = ForgeServerDirectContextArtifact::new(
                    self.admission.request_context(),
                    &support_posture,
                    &response_envelope,
                    Some(&observed_basis_digest),
                    ForgeServerDirectRemaskPosture::visible(),
                );
                TransitionOutcome::Success(ForgeServerDirectRead::new(
                    plan_proof,
                    support_posture,
                    workspace_name,
                    handoff_digest,
                    direct_context,
                    read_result,
                    response_envelope,
                ))
            }
            Err(denial) => TransitionOutcome::Denied(denial),
        }
    }

    pub fn state(
        &self,
        declaration: &ForgeServerAdmittedDirectDeclaration,
    ) -> ForgeServerDirectStateOutcome {
        if let Err(denial) =
            self.admit_operation_family(ForgeServerOperationFamily::QueryDirectRead)
        {
            return self.operation_denial_outcome(denial);
        }
        let observed_basis_digest = match declaration.subscription_basis_digest() {
            Ok(value) => value,
            Err(error) => return self.runtime_error_outcome(error),
        };
        let operation_name = declaration.declaration_binding_label().to_string();
        let handoff_target = declaration.declaration_canonical_label();
        let operation_request = match self.admit_shared_read_operation_request(
            ForgeServerOperationFamily::QueryDirectRead,
            &operation_name,
            &observed_basis_digest,
        ) {
            Ok(value) => value,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        match self.prepare_declared_plan(
            operation_request,
            ForgeServerQueryHandoffOperation::direct_state(&handoff_target),
        ) {
            Ok(plan) => {
                let plan_proof = plan.proof();
                let handoff = plan.into_query_handoff();
                let runtime_state = match declaration.snapshot_named_live_state() {
                    Ok(runtime_state) => runtime_state,
                    Err(error) => return self.runtime_error_outcome(error),
                };
                let support_posture = handoff.support_posture().clone();
                let workspace_name = handoff.workspace().name().to_string();
                let handoff_digest = handoff.canonical_digest().to_string();
                let response_envelope = self
                    .responses
                    .shape_with_defaults(ForgeServerResponseInput::query_handoff_success(handoff));
                let direct_context = ForgeServerDirectContextArtifact::new(
                    self.admission.request_context(),
                    &support_posture,
                    &response_envelope,
                    Some(runtime_state.basis_for_reporting()),
                    ForgeServerDirectRemaskPosture::from_state_snapshot(&runtime_state),
                );
                TransitionOutcome::Success(ForgeServerDirectState::new(
                    plan_proof,
                    support_posture,
                    workspace_name,
                    handoff_digest,
                    direct_context,
                    runtime_state,
                    response_envelope,
                ))
            }
            Err(denial) => TransitionOutcome::Denied(denial),
        }
    }

    pub fn inspect(
        &self,
        declaration: &ForgeServerAdmittedDirectDeclaration,
    ) -> ForgeServerDirectInspectionOutcome {
        if let Err(denial) =
            self.admit_operation_family(ForgeServerOperationFamily::QueryDirectRead)
        {
            return self.operation_denial_outcome(denial);
        }
        let observed_basis_digest = match declaration.subscription_basis_digest() {
            Ok(value) => value,
            Err(error) => return self.runtime_error_outcome(error),
        };
        let operation_name = declaration.declaration_binding_label().to_string();
        let handoff_target = declaration.declaration_canonical_label();
        let operation_request = match self.admit_shared_read_operation_request(
            ForgeServerOperationFamily::QueryDirectRead,
            &operation_name,
            &observed_basis_digest,
        ) {
            Ok(value) => value,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        match self.prepare_declared_plan(
            operation_request,
            ForgeServerQueryHandoffOperation::direct_inspection(&handoff_target),
        ) {
            Ok(plan) => {
                let plan_proof = plan.proof();
                let handoff = plan.into_query_handoff();
                let inspection_result = match declaration.inspect_named_live_view() {
                    Ok(inspection_result) => inspection_result,
                    Err(error) => return self.runtime_error_outcome(error),
                };
                let support_posture = handoff.support_posture().clone();
                let workspace_name = handoff.workspace().name().to_string();
                let handoff_digest = handoff.canonical_digest().to_string();
                let response_envelope = self
                    .responses
                    .shape_with_defaults(ForgeServerResponseInput::query_handoff_success(handoff));
                let (basis_digest, remask_posture) = match inspection_result.inspection() {
                    ForgeQueryInspection::LiveView(live) => {
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
                    self.admission.request_context(),
                    &support_posture,
                    &response_envelope,
                    basis_digest,
                    remask_posture,
                );
                TransitionOutcome::Success(ForgeServerDirectInspection::new(
                    plan_proof,
                    support_posture,
                    workspace_name,
                    handoff_digest,
                    direct_context,
                    inspection_result,
                    response_envelope,
                ))
            }
            Err(denial) => TransitionOutcome::Denied(denial),
        }
    }

    pub(super) fn prepare_declared_plan(
        &self,
        operation_request: crate::ForgeServerOperationRequest,
        operation: ForgeServerQueryHandoffOperation,
    ) -> Result<crate::ForgeServerLoweredOperationPlan, crate::ForgeServerQueryHandoffDenial> {
        let operation_admission =
            match ForgeServerOperationAdmissionFacade::with_operation_registry(
                self.operation_registry.clone(),
            )
            .admit_declared(&self.admission, &operation_request)
            {
                Ok(value) => value,
                Err(denial) => {
                    return Err(
                        crate::surfaces::compat_http::map_operation_admission_denial(denial),
                    );
                }
            };
        ForgeServerOperationPlanner::with_operation_registry(
            self.query_handoff.config().clone(),
            self.operation_registry.clone(),
        )
        .lower(ForgeServerOperationPlannerInput::new(
            operation_admission,
            operation,
        ))
        .map_err(crate::ForgeServerOperationPlanDenial::into_query_handoff_denial)
    }

    pub(super) fn admit_shared_read_operation_request(
        &self,
        family: ForgeServerOperationFamily,
        operation_name: &str,
        basis_digest: &str,
    ) -> Result<crate::ForgeServerOperationRequest, crate::ForgeServerQueryHandoffDenial> {
        let input = ForgeServerOperationRequestInput::builder()
            .with_operation_family(family)
            .with_operation_name(operation_name)
            .with_basis_digest(basis_digest)
            .build();
        ForgeServerOperationRequestFacade::new(self.operation_registry.clone())
            .admit_from_forge_native_admission(&self.admission, input)
            .map_err(map_direct_operation_request_denial)
    }
}

fn map_direct_operation_request_denial(
    denial: crate::ForgeServerOperationRequestDenial,
) -> crate::ForgeServerQueryHandoffDenial {
    crate::ForgeServerQueryHandoffDenial::new(
        crate::ForgeServerQueryHandoffDenialCode::DirectDeclarationBindingInvalid,
        denial.diagnostics_profile(),
        denial.detail(),
    )
}
