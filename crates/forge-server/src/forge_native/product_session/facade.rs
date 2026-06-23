use crate::{
    ForgeServerAdmission, ForgeServerCompletedProductSessionCoordination,
    ForgeServerOperationRegistry, ForgeServerProductSession,
    ForgeServerProductSessionCoordinationRuntime, ForgeServerProductSessionCreationRequest,
    ForgeServerProductSessionDenial, ForgeServerProductSessionIdentity,
    ForgeServerProductSessionRegistry,
};

#[derive(Clone, Debug)]
pub struct ForgeServerForgeNativeProductSessionFacade {
    admission: ForgeServerAdmission,
    operation_registry: ForgeServerOperationRegistry,
    product_session_registry: ForgeServerProductSessionRegistry,
}

impl ForgeServerForgeNativeProductSessionFacade {
    pub(crate) fn new(
        admission: ForgeServerAdmission,
        operation_registry: ForgeServerOperationRegistry,
        product_session_registry: ForgeServerProductSessionRegistry,
    ) -> Self {
        Self {
            admission,
            operation_registry,
            product_session_registry,
        }
    }

    pub fn open_preview(
        &self,
        request: ForgeServerProductSessionCreationRequest,
    ) -> Result<ForgeServerProductSession, ForgeServerProductSessionDenial> {
        self.open_preview_with_proof(request)
            .map(ForgeServerCompletedProductSessionCoordination::into_session)
    }

    pub fn open_mutation(
        &self,
        request: ForgeServerProductSessionCreationRequest,
    ) -> Result<ForgeServerProductSession, ForgeServerProductSessionDenial> {
        self.open_mutation_with_proof(request)
            .map(ForgeServerCompletedProductSessionCoordination::into_session)
    }

    pub fn close(
        &self,
        identity: &ForgeServerProductSessionIdentity,
    ) -> Result<ForgeServerProductSession, ForgeServerProductSessionDenial> {
        self.close_with_proof(identity)
            .map(ForgeServerCompletedProductSessionCoordination::into_session)
    }

    pub fn open_preview_with_proof(
        &self,
        request: ForgeServerProductSessionCreationRequest,
    ) -> Result<ForgeServerCompletedProductSessionCoordination, ForgeServerProductSessionDenial>
    {
        self.runtime()
            .open_preview_from_forge_native(&self.admission, request)
    }

    pub fn open_mutation_with_proof(
        &self,
        request: ForgeServerProductSessionCreationRequest,
    ) -> Result<ForgeServerCompletedProductSessionCoordination, ForgeServerProductSessionDenial>
    {
        self.runtime()
            .open_mutation_from_forge_native(&self.admission, request)
    }

    pub fn close_with_proof(
        &self,
        identity: &ForgeServerProductSessionIdentity,
    ) -> Result<ForgeServerCompletedProductSessionCoordination, ForgeServerProductSessionDenial>
    {
        self.runtime()
            .close_from_forge_native(&self.admission, identity)
    }

    fn runtime(&self) -> ForgeServerProductSessionCoordinationRuntime {
        ForgeServerProductSessionCoordinationRuntime::new(
            self.operation_registry.clone(),
            self.product_session_registry.clone(),
        )
    }
}
