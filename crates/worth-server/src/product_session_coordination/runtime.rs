use crate::{
    WorthServerAdmission, WorthServerCompatibilityPreparedRequest,
    WorthServerOperationAdmissionFacade, WorthServerOperationFamily,
    WorthServerOperationReadinessFacade, WorthServerOperationRegistry,
    WorthServerOperationRequestFacade, WorthServerOperationRequestInput, WorthServerProductSession,
    WorthServerProductSessionCreationRequest, WorthServerProductSessionDenial,
    WorthServerProductSessionDenialCode, WorthServerProductSessionIdentity,
    WorthServerProductSessionRegistry,
};

use super::{
    WorthServerCompletedProductSessionCoordination,
    WorthServerLoweredProductSessionCoordinationPlan, WorthServerProductSessionCoordinationCommand,
    WorthServerProductSessionSchedulerAdmission,
};

#[derive(Clone, Debug)]
pub struct WorthServerProductSessionCoordinationRuntime {
    operation_registry: WorthServerOperationRegistry,
    product_session_registry: WorthServerProductSessionRegistry,
}

impl WorthServerProductSessionCoordinationRuntime {
    pub(crate) fn new(
        operation_registry: WorthServerOperationRegistry,
        product_session_registry: WorthServerProductSessionRegistry,
    ) -> Self {
        Self {
            operation_registry,
            product_session_registry,
        }
    }

    pub fn open_preview_from_worth_native(
        &self,
        admission: &WorthServerAdmission,
        request: WorthServerProductSessionCreationRequest,
    ) -> Result<WorthServerCompletedProductSessionCoordination, WorthServerProductSessionDenial>
    {
        self.execute_from_worth_native(
            admission,
            build_creation_input("product_session.open_preview", &request),
            WorthServerProductSessionCoordinationCommand::OpenPreview(request),
        )
    }

    pub fn open_mutation_from_worth_native(
        &self,
        admission: &WorthServerAdmission,
        request: WorthServerProductSessionCreationRequest,
    ) -> Result<WorthServerCompletedProductSessionCoordination, WorthServerProductSessionDenial>
    {
        self.execute_from_worth_native(
            admission,
            build_creation_input("product_session.open_mutation", &request),
            WorthServerProductSessionCoordinationCommand::OpenMutation(request),
        )
    }

    pub fn close_from_worth_native(
        &self,
        admission: &WorthServerAdmission,
        identity: &WorthServerProductSessionIdentity,
    ) -> Result<WorthServerCompletedProductSessionCoordination, WorthServerProductSessionDenial>
    {
        self.execute_from_worth_native(
            admission,
            WorthServerOperationRequestInput::builder()
                .with_operation_family(WorthServerOperationFamily::ProductSessionCoordination)
                .with_operation_name("product_session.close")
                .with_product_session_identity(identity.as_str())
                .build(),
            WorthServerProductSessionCoordinationCommand::CloseExisting(identity.clone()),
        )
    }

    pub fn open_preview_from_compat_http(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
        request: WorthServerProductSessionCreationRequest,
    ) -> Result<WorthServerCompletedProductSessionCoordination, WorthServerProductSessionDenial>
    {
        self.execute_from_compat_http(
            prepared_request,
            "product_session.open_preview",
            request.basis_digest().map(str::to_string),
            None,
            WorthServerProductSessionCoordinationCommand::OpenPreview(request),
        )
    }

    pub fn open_mutation_from_compat_http(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
        request: WorthServerProductSessionCreationRequest,
    ) -> Result<WorthServerCompletedProductSessionCoordination, WorthServerProductSessionDenial>
    {
        self.execute_from_compat_http(
            prepared_request,
            "product_session.open_mutation",
            request.basis_digest().map(str::to_string),
            None,
            WorthServerProductSessionCoordinationCommand::OpenMutation(request),
        )
    }

    pub fn close_from_compat_http(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
        identity: &WorthServerProductSessionIdentity,
    ) -> Result<WorthServerCompletedProductSessionCoordination, WorthServerProductSessionDenial>
    {
        self.execute_from_compat_http(
            prepared_request,
            "product_session.close",
            None,
            Some(identity.as_str()),
            WorthServerProductSessionCoordinationCommand::CloseExisting(identity.clone()),
        )
    }

    fn execute_from_worth_native(
        &self,
        admission: &WorthServerAdmission,
        input: WorthServerOperationRequestInput,
        command: WorthServerProductSessionCoordinationCommand,
    ) -> Result<WorthServerCompletedProductSessionCoordination, WorthServerProductSessionDenial>
    {
        let request = WorthServerOperationRequestFacade::new(self.operation_registry.clone())
            .admit_from_worth_native_admission(admission, input)
            .map_err(map_request_denial)?;
        self.execute(admission, request, command)
    }

    fn execute_from_compat_http(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
        operation_name: &str,
        basis_digest: Option<String>,
        product_session_identity: Option<&str>,
        command: WorthServerProductSessionCoordinationCommand,
    ) -> Result<WorthServerCompletedProductSessionCoordination, WorthServerProductSessionDenial>
    {
        let request = WorthServerOperationRequestFacade::new(self.operation_registry.clone())
            .admit_from_compat_http_with_basis_digest(
                prepared_request,
                WorthServerOperationFamily::ProductSessionCoordination,
                operation_name,
                basis_digest.as_deref(),
                None,
            )
            .map_err(map_request_denial)?;
        let request = if let Some(product_session_identity) = product_session_identity {
            WorthServerOperationRequestFacade::new(self.operation_registry.clone())
                .admit_from_compat_http_with_product_session_identity(
                    prepared_request,
                    WorthServerOperationFamily::ProductSessionCoordination,
                    operation_name,
                    Some(product_session_identity),
                    None,
                )
                .map_err(map_request_denial)?
        } else {
            request
        };
        self.execute(prepared_request.admission(), request, command)
    }

    fn execute(
        &self,
        admission: &WorthServerAdmission,
        request: crate::WorthServerOperationRequest,
        command: WorthServerProductSessionCoordinationCommand,
    ) -> Result<WorthServerCompletedProductSessionCoordination, WorthServerProductSessionDenial>
    {
        let operation_admission = WorthServerOperationAdmissionFacade::with_operation_registry(
            self.operation_registry.clone(),
        )
        .admit_declared(admission, &request)
        .map_err(map_admission_denial)?;
        let readiness = WorthServerOperationReadinessFacade::with_operation_registry(
            self.operation_registry.clone(),
        )
        .close_readiness(&operation_admission, None, None)
        .map_err(map_readiness_denial)?;
        let plan = WorthServerLoweredProductSessionCoordinationPlan::new(
            operation_admission,
            command,
            readiness.support_posture().clone(),
            readiness.precondition_posture().clone(),
            readiness.concurrency_class(),
        );
        let scheduler_admission = WorthServerProductSessionSchedulerAdmission::from_plan(&plan);
        let session = execute_registry_command(&self.product_session_registry, &plan)?;
        Ok(WorthServerCompletedProductSessionCoordination::new(
            session,
            plan,
            scheduler_admission,
        ))
    }
}

fn build_creation_input(
    operation_name: &str,
    request: &WorthServerProductSessionCreationRequest,
) -> WorthServerOperationRequestInput {
    let mut builder = WorthServerOperationRequestInput::builder()
        .with_operation_family(WorthServerOperationFamily::ProductSessionCoordination)
        .with_operation_name(operation_name);
    if let Some(basis_digest) = request.basis_digest() {
        builder = builder.with_basis_digest(basis_digest);
    }
    builder.build()
}

fn execute_registry_command(
    registry: &WorthServerProductSessionRegistry,
    plan: &WorthServerLoweredProductSessionCoordinationPlan,
) -> Result<WorthServerProductSession, WorthServerProductSessionDenial> {
    match plan.command() {
        WorthServerProductSessionCoordinationCommand::OpenPreview(request) => Ok(registry
            .open_preview(
                plan.operation_admission()
                    .operation_request()
                    .resolved_request_context(),
                request.clone(),
            )),
        WorthServerProductSessionCoordinationCommand::OpenMutation(request) => Ok(registry
            .open_mutation(
                plan.operation_admission()
                    .operation_request()
                    .resolved_request_context(),
                request.clone(),
            )),
        WorthServerProductSessionCoordinationCommand::CloseExisting(identity) => registry.close(
            identity.as_str(),
            plan.operation_admission()
                .operation_request()
                .resolved_request_context(),
        ),
    }
}

fn map_request_denial(
    denial: crate::WorthServerOperationRequestDenial,
) -> WorthServerProductSessionDenial {
    WorthServerProductSessionDenial::new(
        WorthServerProductSessionDenialCode::CoordinationRequestDenied,
        denial.detail(),
    )
}

fn map_admission_denial(
    denial: crate::WorthServerOperationAdmissionDenial,
) -> WorthServerProductSessionDenial {
    WorthServerProductSessionDenial::new(
        WorthServerProductSessionDenialCode::CoordinationAdmissionDenied,
        denial.detail(),
    )
}

fn map_readiness_denial(
    denial: crate::WorthServerOperationReadinessDenial,
) -> WorthServerProductSessionDenial {
    WorthServerProductSessionDenial::new(
        WorthServerProductSessionDenialCode::CoordinationReadinessDenied,
        denial.detail(),
    )
}
