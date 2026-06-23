use crate::{
    ForgeServerCompatibilityPreparedRequest, ForgeServerCompletedProductSessionCoordination,
    ForgeServerOperationRegistry, ForgeServerProductSession,
    ForgeServerProductSessionCoordinationRuntime, ForgeServerProductSessionCreationRequest,
    ForgeServerProductSessionDenial, ForgeServerProductSessionIdentity,
    ForgeServerProductSessionRegistry,
};

#[derive(Clone, Debug)]
pub struct ForgeServerCompatibilityProductSessionFacade {
    runtime: ForgeServerProductSessionCoordinationRuntime,
}

impl ForgeServerCompatibilityProductSessionFacade {
    pub(crate) fn new(
        operation_registry: ForgeServerOperationRegistry,
        product_session_registry: ForgeServerProductSessionRegistry,
    ) -> Self {
        Self {
            runtime: ForgeServerProductSessionCoordinationRuntime::new(
                operation_registry,
                product_session_registry,
            ),
        }
    }

    pub fn open_preview(
        &self,
        prepared_request: &ForgeServerCompatibilityPreparedRequest,
        request: ForgeServerProductSessionCreationRequest,
    ) -> Result<ForgeServerProductSession, ForgeServerProductSessionDenial> {
        self.open_preview_with_proof(prepared_request, request)
            .map(ForgeServerCompletedProductSessionCoordination::into_session)
    }

    pub fn open_mutation(
        &self,
        prepared_request: &ForgeServerCompatibilityPreparedRequest,
        request: ForgeServerProductSessionCreationRequest,
    ) -> Result<ForgeServerProductSession, ForgeServerProductSessionDenial> {
        self.open_mutation_with_proof(prepared_request, request)
            .map(ForgeServerCompletedProductSessionCoordination::into_session)
    }

    pub fn close(
        &self,
        prepared_request: &ForgeServerCompatibilityPreparedRequest,
        identity: &ForgeServerProductSessionIdentity,
    ) -> Result<ForgeServerProductSession, ForgeServerProductSessionDenial> {
        self.close_with_proof(prepared_request, identity)
            .map(ForgeServerCompletedProductSessionCoordination::into_session)
    }

    pub fn open_preview_with_proof(
        &self,
        prepared_request: &ForgeServerCompatibilityPreparedRequest,
        request: ForgeServerProductSessionCreationRequest,
    ) -> Result<ForgeServerCompletedProductSessionCoordination, ForgeServerProductSessionDenial>
    {
        self.runtime
            .open_preview_from_compat_http(prepared_request, request)
    }

    pub fn open_preview_for_product_operation(
        &self,
        prepared_request: &ForgeServerCompatibilityPreparedRequest,
        request: ForgeServerProductSessionCreationRequest,
    ) -> Result<super::ForgeServerCompatibilityOpenedProductSession, ForgeServerProductSessionDenial>
    {
        self.open_preview_with_proof(prepared_request, request)
            .map(|completed| {
                super::ForgeServerCompatibilityOpenedProductSession::new(completed.into_session())
            })
    }

    pub fn open_mutation_with_proof(
        &self,
        prepared_request: &ForgeServerCompatibilityPreparedRequest,
        request: ForgeServerProductSessionCreationRequest,
    ) -> Result<ForgeServerCompletedProductSessionCoordination, ForgeServerProductSessionDenial>
    {
        self.runtime
            .open_mutation_from_compat_http(prepared_request, request)
    }

    pub fn open_mutation_for_product_operation(
        &self,
        prepared_request: &ForgeServerCompatibilityPreparedRequest,
        request: ForgeServerProductSessionCreationRequest,
    ) -> Result<super::ForgeServerCompatibilityOpenedProductSession, ForgeServerProductSessionDenial>
    {
        self.open_mutation_with_proof(prepared_request, request)
            .map(|completed| {
                super::ForgeServerCompatibilityOpenedProductSession::new(completed.into_session())
            })
    }

    pub fn close_with_proof(
        &self,
        prepared_request: &ForgeServerCompatibilityPreparedRequest,
        identity: &ForgeServerProductSessionIdentity,
    ) -> Result<ForgeServerCompletedProductSessionCoordination, ForgeServerProductSessionDenial>
    {
        self.runtime
            .close_from_compat_http(prepared_request, identity)
    }
}
