use crate::{
    ForgeServerAdmission, ForgeServerCompatibilityPreparedRequest,
    ForgeServerOperationAdmissionFacade, ForgeServerOperationFamily,
    ForgeServerOperationReadinessFacade, ForgeServerOperationRegistry,
    ForgeServerOperationRequestFacade, ForgeServerOperationRequestInput, ForgeServerProductSession,
    ForgeServerProductSessionCreationRequest, ForgeServerProductSessionDenial,
    ForgeServerProductSessionDenialCode, ForgeServerProductSessionIdentity,
    ForgeServerProductSessionRegistry,
};

use super::{
    ForgeServerCompletedProductSessionCoordination,
    ForgeServerLoweredProductSessionCoordinationPlan, ForgeServerProductSessionCoordinationCommand,
    ForgeServerProductSessionSchedulerAdmission,
};

#[derive(Clone, Debug)]
pub struct ForgeServerProductSessionCoordinationRuntime {
    operation_registry: ForgeServerOperationRegistry,
    product_session_registry: ForgeServerProductSessionRegistry,
}

impl ForgeServerProductSessionCoordinationRuntime {
    pub(crate) fn new(
        operation_registry: ForgeServerOperationRegistry,
        product_session_registry: ForgeServerProductSessionRegistry,
    ) -> Self {
        Self {
            operation_registry,
            product_session_registry,
        }
    }

    pub fn open_preview_from_forge_native(
        &self,
        admission: &ForgeServerAdmission,
        request: ForgeServerProductSessionCreationRequest,
    ) -> Result<ForgeServerCompletedProductSessionCoordination, ForgeServerProductSessionDenial>
    {
        self.execute_from_forge_native(
            admission,
            build_creation_input("product_session.open_preview", &request),
            ForgeServerProductSessionCoordinationCommand::OpenPreview(request),
        )
    }

    pub fn open_mutation_from_forge_native(
        &self,
        admission: &ForgeServerAdmission,
        request: ForgeServerProductSessionCreationRequest,
    ) -> Result<ForgeServerCompletedProductSessionCoordination, ForgeServerProductSessionDenial>
    {
        self.execute_from_forge_native(
            admission,
            build_creation_input("product_session.open_mutation", &request),
            ForgeServerProductSessionCoordinationCommand::OpenMutation(request),
        )
    }

    pub fn close_from_forge_native(
        &self,
        admission: &ForgeServerAdmission,
        identity: &ForgeServerProductSessionIdentity,
    ) -> Result<ForgeServerCompletedProductSessionCoordination, ForgeServerProductSessionDenial>
    {
        self.execute_from_forge_native(
            admission,
            ForgeServerOperationRequestInput::builder()
                .with_operation_family(ForgeServerOperationFamily::ProductSessionCoordination)
                .with_operation_name("product_session.close")
                .with_product_session_identity(identity.as_str())
                .build(),
            ForgeServerProductSessionCoordinationCommand::CloseExisting(identity.clone()),
        )
    }

    pub fn open_preview_from_compat_http(
        &self,
        prepared_request: &ForgeServerCompatibilityPreparedRequest,
        request: ForgeServerProductSessionCreationRequest,
    ) -> Result<ForgeServerCompletedProductSessionCoordination, ForgeServerProductSessionDenial>
    {
        self.execute_from_compat_http(
            prepared_request,
            "product_session.open_preview",
            request.basis_digest().map(str::to_string),
            None,
            ForgeServerProductSessionCoordinationCommand::OpenPreview(request),
        )
    }

    pub fn open_mutation_from_compat_http(
        &self,
        prepared_request: &ForgeServerCompatibilityPreparedRequest,
        request: ForgeServerProductSessionCreationRequest,
    ) -> Result<ForgeServerCompletedProductSessionCoordination, ForgeServerProductSessionDenial>
    {
        self.execute_from_compat_http(
            prepared_request,
            "product_session.open_mutation",
            request.basis_digest().map(str::to_string),
            None,
            ForgeServerProductSessionCoordinationCommand::OpenMutation(request),
        )
    }

    pub fn close_from_compat_http(
        &self,
        prepared_request: &ForgeServerCompatibilityPreparedRequest,
        identity: &ForgeServerProductSessionIdentity,
    ) -> Result<ForgeServerCompletedProductSessionCoordination, ForgeServerProductSessionDenial>
    {
        self.execute_from_compat_http(
            prepared_request,
            "product_session.close",
            None,
            Some(identity.as_str()),
            ForgeServerProductSessionCoordinationCommand::CloseExisting(identity.clone()),
        )
    }

    fn execute_from_forge_native(
        &self,
        admission: &ForgeServerAdmission,
        input: ForgeServerOperationRequestInput,
        command: ForgeServerProductSessionCoordinationCommand,
    ) -> Result<ForgeServerCompletedProductSessionCoordination, ForgeServerProductSessionDenial>
    {
        let request = ForgeServerOperationRequestFacade::new(self.operation_registry.clone())
            .admit_from_forge_native_admission(admission, input)
            .map_err(map_request_denial)?;
        self.execute(admission, request, command)
    }

    fn execute_from_compat_http(
        &self,
        prepared_request: &ForgeServerCompatibilityPreparedRequest,
        operation_name: &str,
        basis_digest: Option<String>,
        product_session_identity: Option<&str>,
        command: ForgeServerProductSessionCoordinationCommand,
    ) -> Result<ForgeServerCompletedProductSessionCoordination, ForgeServerProductSessionDenial>
    {
        let request = ForgeServerOperationRequestFacade::new(self.operation_registry.clone())
            .admit_from_compat_http_with_basis_digest(
                prepared_request,
                ForgeServerOperationFamily::ProductSessionCoordination,
                operation_name,
                basis_digest.as_deref(),
                None,
            )
            .map_err(map_request_denial)?;
        let request = if let Some(product_session_identity) = product_session_identity {
            ForgeServerOperationRequestFacade::new(self.operation_registry.clone())
                .admit_from_compat_http_with_product_session_identity(
                    prepared_request,
                    ForgeServerOperationFamily::ProductSessionCoordination,
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
        admission: &ForgeServerAdmission,
        request: crate::ForgeServerOperationRequest,
        command: ForgeServerProductSessionCoordinationCommand,
    ) -> Result<ForgeServerCompletedProductSessionCoordination, ForgeServerProductSessionDenial>
    {
        let operation_admission = ForgeServerOperationAdmissionFacade::with_operation_registry(
            self.operation_registry.clone(),
        )
        .admit_declared(admission, &request)
        .map_err(map_admission_denial)?;
        let readiness = ForgeServerOperationReadinessFacade::with_operation_registry(
            self.operation_registry.clone(),
        )
        .close_readiness(&operation_admission, None, None)
        .map_err(map_readiness_denial)?;
        let plan = ForgeServerLoweredProductSessionCoordinationPlan::new(
            operation_admission,
            command,
            readiness.support_posture().clone(),
            readiness.precondition_posture().clone(),
            readiness.concurrency_class(),
        );
        let scheduler_admission = ForgeServerProductSessionSchedulerAdmission::from_plan(&plan);
        let session = execute_registry_command(&self.product_session_registry, &plan)?;
        Ok(ForgeServerCompletedProductSessionCoordination::new(
            session,
            plan,
            scheduler_admission,
        ))
    }
}

fn build_creation_input(
    operation_name: &str,
    request: &ForgeServerProductSessionCreationRequest,
) -> ForgeServerOperationRequestInput {
    let mut builder = ForgeServerOperationRequestInput::builder()
        .with_operation_family(ForgeServerOperationFamily::ProductSessionCoordination)
        .with_operation_name(operation_name);
    if let Some(basis_digest) = request.basis_digest() {
        builder = builder.with_basis_digest(basis_digest);
    }
    builder.build()
}

fn execute_registry_command(
    registry: &ForgeServerProductSessionRegistry,
    plan: &ForgeServerLoweredProductSessionCoordinationPlan,
) -> Result<ForgeServerProductSession, ForgeServerProductSessionDenial> {
    match plan.command() {
        ForgeServerProductSessionCoordinationCommand::OpenPreview(request) => Ok(registry
            .open_preview(
                plan.operation_admission()
                    .operation_request()
                    .resolved_request_context(),
                request.clone(),
            )),
        ForgeServerProductSessionCoordinationCommand::OpenMutation(request) => Ok(registry
            .open_mutation(
                plan.operation_admission()
                    .operation_request()
                    .resolved_request_context(),
                request.clone(),
            )),
        ForgeServerProductSessionCoordinationCommand::CloseExisting(identity) => registry.close(
            identity.as_str(),
            plan.operation_admission()
                .operation_request()
                .resolved_request_context(),
        ),
    }
}

fn map_request_denial(
    denial: crate::ForgeServerOperationRequestDenial,
) -> ForgeServerProductSessionDenial {
    ForgeServerProductSessionDenial::new(
        ForgeServerProductSessionDenialCode::CoordinationRequestDenied,
        denial.detail(),
    )
}

fn map_admission_denial(
    denial: crate::ForgeServerOperationAdmissionDenial,
) -> ForgeServerProductSessionDenial {
    ForgeServerProductSessionDenial::new(
        ForgeServerProductSessionDenialCode::CoordinationAdmissionDenied,
        denial.detail(),
    )
}

fn map_readiness_denial(
    denial: crate::ForgeServerOperationReadinessDenial,
) -> ForgeServerProductSessionDenial {
    ForgeServerProductSessionDenial::new(
        ForgeServerProductSessionDenialCode::CoordinationReadinessDenied,
        denial.detail(),
    )
}
