use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    declaration_intake::ForgeServerDirectDeclarationIntakeFacade,
    forge_native::{
        declaration::ForgeServerForgeNativeDeclarationFacade,
        product::ForgeServerForgeNativeProductOperationFacade,
        product_session::ForgeServerForgeNativeProductSessionFacade,
        ForgeServerForgeNativeDirectFacade,
    },
    product_operation_contract::ForgeServerStoredProductOperation,
    ForgeServerAdmission, ForgeServerOperationRegistry, ForgeServerProductAdapterRegistry,
    ForgeServerProductSessionRegistry, ForgeServerQueryHandoffConfig,
    ForgeServerQueryHandoffFacade, ForgeServerResolvedRequestContext, ForgeServerResponseFacade,
};

#[derive(Clone, Debug)]
pub struct ForgeServerForgeNativePreparedSession {
    admission: ForgeServerAdmission,
    operation_registry: ForgeServerOperationRegistry,
    product_adapter_registry: ForgeServerProductAdapterRegistry,
    product_session_registry: ForgeServerProductSessionRegistry,
    product_operation_replay_store: Arc<Mutex<HashMap<String, ForgeServerStoredProductOperation>>>,
    query_handoff_config: ForgeServerQueryHandoffConfig,
    declaration_intake: ForgeServerDirectDeclarationIntakeFacade,
    query_handoff: ForgeServerQueryHandoffFacade,
    responses: ForgeServerResponseFacade,
}

impl ForgeServerForgeNativePreparedSession {
    pub(crate) fn new(
        admission: ForgeServerAdmission,
        operation_registry: ForgeServerOperationRegistry,
        product_adapter_registry: ForgeServerProductAdapterRegistry,
        product_session_registry: ForgeServerProductSessionRegistry,
        product_operation_replay_store: Arc<
            Mutex<HashMap<String, ForgeServerStoredProductOperation>>,
        >,
        query_handoff_config: ForgeServerQueryHandoffConfig,
        declaration_intake: ForgeServerDirectDeclarationIntakeFacade,
        query_handoff: ForgeServerQueryHandoffFacade,
        responses: ForgeServerResponseFacade,
    ) -> Self {
        Self {
            admission,
            operation_registry,
            product_adapter_registry,
            product_session_registry,
            product_operation_replay_store,
            query_handoff_config,
            declaration_intake,
            query_handoff,
            responses,
        }
    }

    pub fn resolved_request_context(&self) -> &ForgeServerResolvedRequestContext {
        self.admission.resolved_request_context()
    }

    pub fn admission(&self) -> &ForgeServerAdmission {
        &self.admission
    }

    pub fn declarations(&self) -> ForgeServerForgeNativeDeclarationFacade {
        ForgeServerForgeNativeDeclarationFacade::new(
            self.admission.clone(),
            self.declaration_intake.clone(),
        )
    }

    pub fn direct(&self) -> ForgeServerForgeNativeDirectFacade {
        ForgeServerForgeNativeDirectFacade::new(
            self.admission.clone(),
            self.operation_registry.clone(),
            self.declaration_intake.clone(),
            self.query_handoff.clone(),
            self.responses.clone(),
        )
    }

    pub fn product_operations(&self) -> ForgeServerForgeNativeProductOperationFacade {
        ForgeServerForgeNativeProductOperationFacade::new(
            self.admission.clone(),
            self.operation_registry.clone(),
            self.product_adapter_registry.clone(),
            self.query_handoff_config.clone(),
            self.product_session_registry.clone(),
            self.product_operation_replay_store.clone(),
        )
    }

    pub fn product_sessions(&self) -> ForgeServerForgeNativeProductSessionFacade {
        ForgeServerForgeNativeProductSessionFacade::new(
            self.admission.clone(),
            self.operation_registry.clone(),
            self.product_session_registry.clone(),
        )
    }

    pub fn into_session(self) -> ForgeServerForgeNativeSession {
        ForgeServerForgeNativeSession::new(
            self.admission,
            self.operation_registry,
            self.product_adapter_registry,
            self.product_session_registry,
            self.product_operation_replay_store,
            self.query_handoff_config,
            self.declaration_intake,
            self.query_handoff,
            self.responses,
        )
    }
}

#[derive(Clone, Debug)]
pub struct ForgeServerForgeNativeSession {
    admission: ForgeServerAdmission,
    operation_registry: ForgeServerOperationRegistry,
    product_adapter_registry: ForgeServerProductAdapterRegistry,
    product_session_registry: ForgeServerProductSessionRegistry,
    product_operation_replay_store: Arc<Mutex<HashMap<String, ForgeServerStoredProductOperation>>>,
    query_handoff_config: ForgeServerQueryHandoffConfig,
    declaration_intake: ForgeServerDirectDeclarationIntakeFacade,
    query_handoff: ForgeServerQueryHandoffFacade,
    responses: ForgeServerResponseFacade,
}

impl ForgeServerForgeNativeSession {
    pub(crate) fn new(
        admission: ForgeServerAdmission,
        operation_registry: ForgeServerOperationRegistry,
        product_adapter_registry: ForgeServerProductAdapterRegistry,
        product_session_registry: ForgeServerProductSessionRegistry,
        product_operation_replay_store: Arc<
            Mutex<HashMap<String, ForgeServerStoredProductOperation>>,
        >,
        query_handoff_config: ForgeServerQueryHandoffConfig,
        declaration_intake: ForgeServerDirectDeclarationIntakeFacade,
        query_handoff: ForgeServerQueryHandoffFacade,
        responses: ForgeServerResponseFacade,
    ) -> Self {
        Self {
            admission,
            operation_registry,
            product_adapter_registry,
            product_session_registry,
            product_operation_replay_store,
            query_handoff_config,
            declaration_intake,
            query_handoff,
            responses,
        }
    }

    pub fn resolved_request_context(&self) -> &ForgeServerResolvedRequestContext {
        self.admission.resolved_request_context()
    }

    pub fn admission(&self) -> &ForgeServerAdmission {
        &self.admission
    }

    pub fn declarations(&self) -> ForgeServerForgeNativeDeclarationFacade {
        ForgeServerForgeNativeDeclarationFacade::new(
            self.admission.clone(),
            self.declaration_intake.clone(),
        )
    }

    pub fn direct(&self) -> ForgeServerForgeNativeDirectFacade {
        ForgeServerForgeNativeDirectFacade::new(
            self.admission.clone(),
            self.operation_registry.clone(),
            self.declaration_intake.clone(),
            self.query_handoff.clone(),
            self.responses.clone(),
        )
    }

    pub fn product_operations(&self) -> ForgeServerForgeNativeProductOperationFacade {
        ForgeServerForgeNativeProductOperationFacade::new(
            self.admission.clone(),
            self.operation_registry.clone(),
            self.product_adapter_registry.clone(),
            self.query_handoff_config.clone(),
            self.product_session_registry.clone(),
            self.product_operation_replay_store.clone(),
        )
    }

    pub fn product_sessions(&self) -> ForgeServerForgeNativeProductSessionFacade {
        ForgeServerForgeNativeProductSessionFacade::new(
            self.admission.clone(),
            self.operation_registry.clone(),
            self.product_session_registry.clone(),
        )
    }
}
