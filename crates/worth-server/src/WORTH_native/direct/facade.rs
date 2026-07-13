use worth_proof::TransitionOutcome;
use worth_query::facade::runtime::WorthQueryInspection;

use crate::{
    declaration_intake::WorthServerDirectDeclarationIntakeFacade, WorthServerAdmission,
    WorthServerAdmittedDirectDeclaration, WorthServerOperationAdmissionFacade,
    WorthServerOperationDenial, WorthServerOperationFamily, WorthServerOperationPlanner,
    WorthServerOperationPlannerInput, WorthServerOperationRegistry,
    WorthServerOperationRequestFacade, WorthServerOperationRequestInput,
    WorthServerQueryHandoffFacade, WorthServerQueryHandoffOperation, WorthServerResponseFacade,
    WorthServerResponseInput, WorthServerSurfaceFamily,
};

use super::{
    WorthServerDirectContextArtifact, WorthServerDirectInspection, WorthServerDirectProjection,
    WorthServerDirectRead, WorthServerDirectRemaskPosture, WorthServerDirectState,
};

pub type WorthServerDirectReadOutcome = TransitionOutcome<
    WorthServerDirectRead,
    crate::WorthServerQueryHandoffDenial,
    crate::WorthServerQueryHandoffDeferred,
    crate::WorthServerQueryHandoffStale,
    crate::WorthServerQueryHandoffRebindRequired,
    crate::WorthServerQueryHandoffFailure,
>;

pub type WorthServerDirectStateOutcome = TransitionOutcome<
    WorthServerDirectState,
    crate::WorthServerQueryHandoffDenial,
    crate::WorthServerQueryHandoffDeferred,
    crate::WorthServerQueryHandoffStale,
    crate::WorthServerQueryHandoffRebindRequired,
    crate::WorthServerQueryHandoffFailure,
>;

pub type WorthServerDirectInspectionOutcome = TransitionOutcome<
    WorthServerDirectInspection,
    crate::WorthServerQueryHandoffDenial,
    crate::WorthServerQueryHandoffDeferred,
    crate::WorthServerQueryHandoffStale,
    crate::WorthServerQueryHandoffRebindRequired,
    crate::WorthServerQueryHandoffFailure,
>;

pub type WorthServerDirectProjectionOutcome = TransitionOutcome<
    WorthServerDirectProjection,
    crate::WorthServerQueryHandoffDenial,
    crate::WorthServerQueryHandoffDeferred,
    crate::WorthServerQueryHandoffStale,
    crate::WorthServerQueryHandoffRebindRequired,
    crate::WorthServerQueryHandoffFailure,
>;

pub type WorthServerDirectMutationOutcome = TransitionOutcome<
    crate::WorthServerDirectMutation,
    crate::WorthServerQueryHandoffDenial,
    crate::WorthServerQueryHandoffDeferred,
    crate::WorthServerQueryHandoffStale,
    crate::WorthServerQueryHandoffRebindRequired,
    crate::WorthServerQueryHandoffFailure,
>;

#[derive(Clone, Debug)]
pub struct WorthServerWorthNativeDirectFacade {
    pub(super) admission: WorthServerAdmission,
    pub(super) operation_registry: WorthServerOperationRegistry,
    pub(super) declaration_intake: WorthServerDirectDeclarationIntakeFacade,
    pub(super) query_handoff: WorthServerQueryHandoffFacade,
    pub(super) responses: WorthServerResponseFacade,
}

impl WorthServerWorthNativeDirectFacade {
    pub(crate) fn new(
        admission: WorthServerAdmission,
        operation_registry: WorthServerOperationRegistry,
        declaration_intake: WorthServerDirectDeclarationIntakeFacade,
        query_handoff: WorthServerQueryHandoffFacade,
        responses: WorthServerResponseFacade,
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
        family: WorthServerOperationFamily,
    ) -> Result<(), WorthServerOperationDenial> {
        self.operation_registry
            .admit(WorthServerSurfaceFamily::WorthNative, family)
            .map(|_| ())
    }

    pub fn read(
        &self,
        declaration: &WorthServerAdmittedDirectDeclaration,
    ) -> WorthServerDirectReadOutcome {
        if let Err(denial) =
            self.admit_operation_family(WorthServerOperationFamily::QueryDirectRead)
        {
            return self.operation_denial_outcome(denial);
        }
        let observed_basis_digest = match declaration.subscription_basis_digest() {
            Ok(value) => value,
            Err(error) => return self.runtime_error_outcome(error),
        };
        let operation_name = declaration.declaration_binding_label().to_string();
        let operation_request = match self.admit_shared_read_operation_request(
            WorthServerOperationFamily::QueryDirectRead,
            &operation_name,
            &observed_basis_digest,
        ) {
            Ok(value) => value,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        match self.prepare_declared_plan(
            operation_request,
            WorthServerQueryHandoffOperation::direct_read(&operation_name),
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
                    .shape_with_defaults(WorthServerResponseInput::query_handoff_success(handoff));
                let direct_context = WorthServerDirectContextArtifact::new(
                    self.admission.request_context(),
                    &support_posture,
                    &response_envelope,
                    Some(&observed_basis_digest),
                    WorthServerDirectRemaskPosture::visible(),
                );
                TransitionOutcome::Success(WorthServerDirectRead::new(
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
        declaration: &WorthServerAdmittedDirectDeclaration,
    ) -> WorthServerDirectStateOutcome {
        if let Err(denial) =
            self.admit_operation_family(WorthServerOperationFamily::QueryDirectRead)
        {
            return self.operation_denial_outcome(denial);
        }
        let observed_basis_digest = match declaration.subscription_basis_digest() {
            Ok(value) => value,
            Err(error) => return self.runtime_error_outcome(error),
        };
        let operation_name = declaration.declaration_binding_label().to_string();
        let handoff_target = declaration.declaration_binding_label().to_string();
        let operation_request = match self.admit_shared_read_operation_request(
            WorthServerOperationFamily::QueryDirectRead,
            &operation_name,
            &observed_basis_digest,
        ) {
            Ok(value) => value,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        match self.prepare_declared_plan(
            operation_request,
            WorthServerQueryHandoffOperation::direct_state(&handoff_target),
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
                    .shape_with_defaults(WorthServerResponseInput::query_handoff_success(handoff));
                let direct_context = WorthServerDirectContextArtifact::new(
                    self.admission.request_context(),
                    &support_posture,
                    &response_envelope,
                    Some(runtime_state.basis_for_reporting()),
                    WorthServerDirectRemaskPosture::from_state_snapshot(&runtime_state),
                );
                TransitionOutcome::Success(WorthServerDirectState::new(
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
        declaration: &WorthServerAdmittedDirectDeclaration,
    ) -> WorthServerDirectInspectionOutcome {
        if let Err(denial) =
            self.admit_operation_family(WorthServerOperationFamily::QueryDirectRead)
        {
            return self.operation_denial_outcome(denial);
        }
        let observed_basis_digest = match declaration.subscription_basis_digest() {
            Ok(value) => value,
            Err(error) => return self.runtime_error_outcome(error),
        };
        let operation_name = declaration.declaration_binding_label().to_string();
        let handoff_target = declaration.declaration_binding_label().to_string();
        let operation_request = match self.admit_shared_read_operation_request(
            WorthServerOperationFamily::QueryDirectRead,
            &operation_name,
            &observed_basis_digest,
        ) {
            Ok(value) => value,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        match self.prepare_declared_plan(
            operation_request,
            WorthServerQueryHandoffOperation::direct_inspection(&handoff_target),
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
                    .shape_with_defaults(WorthServerResponseInput::query_handoff_success(handoff));
                let (basis_digest, remask_posture) = match inspection_result.inspection() {
                    WorthQueryInspection::LiveView(live) => {
                        let basis = live
                            .basis_binding_identity()
                            .terminal_projection_for_reporting();
                        (
                            Some(basis),
                            WorthServerDirectRemaskPosture::from_live_inspection(live),
                        )
                    }
                    _ => (None, WorthServerDirectRemaskPosture::visible()),
                };
                let direct_context = WorthServerDirectContextArtifact::new(
                    self.admission.request_context(),
                    &support_posture,
                    &response_envelope,
                    basis_digest,
                    remask_posture,
                );
                TransitionOutcome::Success(WorthServerDirectInspection::new(
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
        operation_request: crate::WorthServerOperationRequest,
        operation: WorthServerQueryHandoffOperation,
    ) -> Result<crate::WorthServerLoweredOperationPlan, crate::WorthServerQueryHandoffDenial> {
        let operation_admission =
            match WorthServerOperationAdmissionFacade::with_operation_registry(
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
        WorthServerOperationPlanner::with_operation_registry(
            self.query_handoff.config().clone(),
            self.operation_registry.clone(),
        )
        .lower(WorthServerOperationPlannerInput::new(
            operation_admission,
            operation,
        ))
        .map_err(crate::WorthServerOperationPlanDenial::into_query_handoff_denial)
    }

    pub(super) fn admit_shared_read_operation_request(
        &self,
        family: WorthServerOperationFamily,
        operation_name: &str,
        basis_digest: &str,
    ) -> Result<crate::WorthServerOperationRequest, crate::WorthServerQueryHandoffDenial> {
        let input = WorthServerOperationRequestInput::builder()
            .with_operation_family(family)
            .with_operation_name(operation_name)
            .with_basis_digest(basis_digest)
            .build();
        WorthServerOperationRequestFacade::new(self.operation_registry.clone())
            .admit_from_worth_native_admission(&self.admission, input)
            .map_err(map_direct_operation_request_denial)
    }
}

fn map_direct_operation_request_denial(
    denial: crate::WorthServerOperationRequestDenial,
) -> crate::WorthServerQueryHandoffDenial {
    crate::WorthServerQueryHandoffDenial::new(
        crate::WorthServerQueryHandoffDenialCode::DirectDeclarationBindingInvalid,
        denial.diagnostics_profile(),
        denial.detail(),
    )
}
