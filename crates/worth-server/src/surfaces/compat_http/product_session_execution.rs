use crate::{
    WorthServerCompatibilityPreparedRequest, WorthServerCompletedProductSessionCoordination,
    WorthServerOperationRegistry, WorthServerProductAdapterRegistry, WorthServerProductSession,
    WorthServerProductSessionCoordinationRuntime, WorthServerProductSessionCreationRequest,
    WorthServerProductSessionDenial, WorthServerProductSessionIdentity,
    WorthServerProductSessionRegistry,
};

#[derive(Clone, Debug)]
pub struct WorthServerCompatibilityProductSessionFacade {
    runtime: WorthServerProductSessionCoordinationRuntime,
}

impl WorthServerCompatibilityProductSessionFacade {
    pub(crate) fn new(
        operation_registry: WorthServerOperationRegistry,
        product_adapter_registry: WorthServerProductAdapterRegistry,
        product_session_registry: WorthServerProductSessionRegistry,
    ) -> Self {
        Self {
            runtime: WorthServerProductSessionCoordinationRuntime::new(
                operation_registry,
                product_adapter_registry,
                product_session_registry,
            ),
        }
    }

    pub fn open_preview(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
        request: WorthServerProductSessionCreationRequest,
    ) -> Result<WorthServerProductSession, WorthServerProductSessionDenial> {
        self.open_preview_with_proof(prepared_request, request)
            .map(WorthServerCompletedProductSessionCoordination::into_session)
    }

    pub fn open_mutation(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
        request: WorthServerProductSessionCreationRequest,
    ) -> Result<WorthServerProductSession, WorthServerProductSessionDenial> {
        self.open_mutation_with_proof(prepared_request, request)
            .map(WorthServerCompletedProductSessionCoordination::into_session)
    }

    pub fn close(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
        identity: &WorthServerProductSessionIdentity,
    ) -> Result<WorthServerProductSession, WorthServerProductSessionDenial> {
        self.close_with_proof(prepared_request, identity)
            .map(WorthServerCompletedProductSessionCoordination::into_session)
    }

    pub fn open_preview_with_proof(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
        request: WorthServerProductSessionCreationRequest,
    ) -> Result<WorthServerCompletedProductSessionCoordination, WorthServerProductSessionDenial>
    {
        self.runtime
            .open_preview_from_compat_http(prepared_request, request)
    }

    pub fn open_preview_for_product_operation(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
        request: WorthServerProductSessionCreationRequest,
    ) -> Result<super::WorthServerCompatibilityOpenedProductSession, WorthServerProductSessionDenial>
    {
        self.open_preview_with_proof(prepared_request, request)
            .map(|completed| {
                super::WorthServerCompatibilityOpenedProductSession::new(completed.into_session())
            })
    }

    pub fn open_mutation_with_proof(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
        request: WorthServerProductSessionCreationRequest,
    ) -> Result<WorthServerCompletedProductSessionCoordination, WorthServerProductSessionDenial>
    {
        self.runtime
            .open_mutation_from_compat_http(prepared_request, request)
    }

    pub fn open_mutation_for_product_operation(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
        request: WorthServerProductSessionCreationRequest,
    ) -> Result<super::WorthServerCompatibilityOpenedProductSession, WorthServerProductSessionDenial>
    {
        self.open_mutation_with_proof(prepared_request, request)
            .map(|completed| {
                super::WorthServerCompatibilityOpenedProductSession::new(completed.into_session())
            })
    }

    pub fn close_with_proof(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
        identity: &WorthServerProductSessionIdentity,
    ) -> Result<WorthServerCompletedProductSessionCoordination, WorthServerProductSessionDenial>
    {
        self.runtime
            .close_from_compat_http(prepared_request, identity)
    }
}
