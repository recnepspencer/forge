use crate::{
    WorthServerAdmission, WorthServerCompletedProductSessionCoordination,
    WorthServerOperationRegistry, WorthServerProductSession,
    WorthServerProductSessionCoordinationRuntime, WorthServerProductSessionCreationRequest,
    WorthServerProductSessionDenial, WorthServerProductSessionIdentity,
    WorthServerProductSessionRegistry,
};

#[derive(Clone, Debug)]
pub struct WorthServerWorthNativeProductSessionFacade {
    admission: WorthServerAdmission,
    operation_registry: WorthServerOperationRegistry,
    product_session_registry: WorthServerProductSessionRegistry,
}

impl WorthServerWorthNativeProductSessionFacade {
    pub(crate) fn new(
        admission: WorthServerAdmission,
        operation_registry: WorthServerOperationRegistry,
        product_session_registry: WorthServerProductSessionRegistry,
    ) -> Self {
        Self {
            admission,
            operation_registry,
            product_session_registry,
        }
    }

    pub fn open_preview(
        &self,
        request: WorthServerProductSessionCreationRequest,
    ) -> Result<WorthServerProductSession, WorthServerProductSessionDenial> {
        self.open_preview_with_proof(request)
            .map(WorthServerCompletedProductSessionCoordination::into_session)
    }

    pub fn open_mutation(
        &self,
        request: WorthServerProductSessionCreationRequest,
    ) -> Result<WorthServerProductSession, WorthServerProductSessionDenial> {
        self.open_mutation_with_proof(request)
            .map(WorthServerCompletedProductSessionCoordination::into_session)
    }

    pub fn close(
        &self,
        identity: &WorthServerProductSessionIdentity,
    ) -> Result<WorthServerProductSession, WorthServerProductSessionDenial> {
        self.close_with_proof(identity)
            .map(WorthServerCompletedProductSessionCoordination::into_session)
    }

    pub fn open_preview_with_proof(
        &self,
        request: WorthServerProductSessionCreationRequest,
    ) -> Result<WorthServerCompletedProductSessionCoordination, WorthServerProductSessionDenial>
    {
        self.runtime()
            .open_preview_from_worth_native(&self.admission, request)
    }

    pub fn open_mutation_with_proof(
        &self,
        request: WorthServerProductSessionCreationRequest,
    ) -> Result<WorthServerCompletedProductSessionCoordination, WorthServerProductSessionDenial>
    {
        self.runtime()
            .open_mutation_from_worth_native(&self.admission, request)
    }

    pub fn close_with_proof(
        &self,
        identity: &WorthServerProductSessionIdentity,
    ) -> Result<WorthServerCompletedProductSessionCoordination, WorthServerProductSessionDenial>
    {
        self.runtime()
            .close_from_worth_native(&self.admission, identity)
    }

    fn runtime(&self) -> WorthServerProductSessionCoordinationRuntime {
        WorthServerProductSessionCoordinationRuntime::new(
            self.operation_registry.clone(),
            self.product_session_registry.clone(),
        )
    }
}
