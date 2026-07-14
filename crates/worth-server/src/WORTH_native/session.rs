use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    declaration_intake::WorthServerDirectDeclarationIntakeFacade,
    product_operation_contract::WorthServerStoredProductOperation,
    worth_native::{
        declaration::WorthServerWorthNativeDeclarationFacade,
        product::WorthServerWorthNativeProductOperationFacade,
        product_session::WorthServerWorthNativeProductSessionFacade,
        WorthServerWorthNativeDirectFacade,
    },
    WorthServerAdmission, WorthServerOperationRegistry, WorthServerProductAdapterRegistry,
    WorthServerProductSessionRegistry, WorthServerQueryHandoffConfig,
    WorthServerQueryHandoffFacade, WorthServerResolvedRequestContext, WorthServerResponseFacade,
};

#[derive(Clone, Debug)]
pub struct WorthServerWorthNativePreparedSession {
    admission: WorthServerAdmission,
    operation_registry: WorthServerOperationRegistry,
    product_adapter_registry: WorthServerProductAdapterRegistry,
    product_session_registry: WorthServerProductSessionRegistry,
    product_operation_replay_store: Arc<Mutex<HashMap<String, WorthServerStoredProductOperation>>>,
    query_handoff_config: WorthServerQueryHandoffConfig,
    declaration_intake: WorthServerDirectDeclarationIntakeFacade,
    query_handoff: WorthServerQueryHandoffFacade,
    responses: WorthServerResponseFacade,
}

impl WorthServerWorthNativePreparedSession {
    pub(crate) fn new(
        admission: WorthServerAdmission,
        operation_registry: WorthServerOperationRegistry,
        product_adapter_registry: WorthServerProductAdapterRegistry,
        product_session_registry: WorthServerProductSessionRegistry,
        product_operation_replay_store: Arc<
            Mutex<HashMap<String, WorthServerStoredProductOperation>>,
        >,
        query_handoff_config: WorthServerQueryHandoffConfig,
        declaration_intake: WorthServerDirectDeclarationIntakeFacade,
        query_handoff: WorthServerQueryHandoffFacade,
        responses: WorthServerResponseFacade,
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

    pub fn resolved_request_context(&self) -> &WorthServerResolvedRequestContext {
        self.admission.resolved_request_context()
    }

    pub fn admission(&self) -> &WorthServerAdmission {
        &self.admission
    }

    pub fn declarations(&self) -> WorthServerWorthNativeDeclarationFacade {
        WorthServerWorthNativeDeclarationFacade::new(
            self.admission.clone(),
            self.declaration_intake.clone(),
        )
    }

    pub fn direct(&self) -> WorthServerWorthNativeDirectFacade {
        WorthServerWorthNativeDirectFacade::new(
            self.admission.clone(),
            self.operation_registry.clone(),
            self.declaration_intake.clone(),
            self.query_handoff.clone(),
            self.responses.clone(),
        )
    }

    pub fn product_operations(&self) -> WorthServerWorthNativeProductOperationFacade {
        WorthServerWorthNativeProductOperationFacade::new(
            self.admission.clone(),
            self.operation_registry.clone(),
            self.product_adapter_registry.clone(),
            self.query_handoff_config.clone(),
            self.product_session_registry.clone(),
            self.product_operation_replay_store.clone(),
        )
    }

    pub fn product_sessions(&self) -> WorthServerWorthNativeProductSessionFacade {
        WorthServerWorthNativeProductSessionFacade::new(
            self.admission.clone(),
            self.operation_registry.clone(),
            self.product_session_registry.clone(),
        )
    }

    pub fn into_session(self) -> WorthServerWorthNativeSession {
        WorthServerWorthNativeSession::new(
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
pub struct WorthServerWorthNativeSession {
    admission: WorthServerAdmission,
    operation_registry: WorthServerOperationRegistry,
    product_adapter_registry: WorthServerProductAdapterRegistry,
    product_session_registry: WorthServerProductSessionRegistry,
    product_operation_replay_store: Arc<Mutex<HashMap<String, WorthServerStoredProductOperation>>>,
    query_handoff_config: WorthServerQueryHandoffConfig,
    declaration_intake: WorthServerDirectDeclarationIntakeFacade,
    query_handoff: WorthServerQueryHandoffFacade,
    responses: WorthServerResponseFacade,
}

impl WorthServerWorthNativeSession {
    pub(crate) fn new(
        admission: WorthServerAdmission,
        operation_registry: WorthServerOperationRegistry,
        product_adapter_registry: WorthServerProductAdapterRegistry,
        product_session_registry: WorthServerProductSessionRegistry,
        product_operation_replay_store: Arc<
            Mutex<HashMap<String, WorthServerStoredProductOperation>>,
        >,
        query_handoff_config: WorthServerQueryHandoffConfig,
        declaration_intake: WorthServerDirectDeclarationIntakeFacade,
        query_handoff: WorthServerQueryHandoffFacade,
        responses: WorthServerResponseFacade,
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

    pub fn resolved_request_context(&self) -> &WorthServerResolvedRequestContext {
        self.admission.resolved_request_context()
    }

    pub fn admission(&self) -> &WorthServerAdmission {
        &self.admission
    }

    pub fn declarations(&self) -> WorthServerWorthNativeDeclarationFacade {
        WorthServerWorthNativeDeclarationFacade::new(
            self.admission.clone(),
            self.declaration_intake.clone(),
        )
    }

    pub fn direct(&self) -> WorthServerWorthNativeDirectFacade {
        WorthServerWorthNativeDirectFacade::new(
            self.admission.clone(),
            self.operation_registry.clone(),
            self.declaration_intake.clone(),
            self.query_handoff.clone(),
            self.responses.clone(),
        )
    }

    pub fn product_operations(&self) -> WorthServerWorthNativeProductOperationFacade {
        WorthServerWorthNativeProductOperationFacade::new(
            self.admission.clone(),
            self.operation_registry.clone(),
            self.product_adapter_registry.clone(),
            self.query_handoff_config.clone(),
            self.product_session_registry.clone(),
            self.product_operation_replay_store.clone(),
        )
    }

    pub fn product_sessions(&self) -> WorthServerWorthNativeProductSessionFacade {
        WorthServerWorthNativeProductSessionFacade::new(
            self.admission.clone(),
            self.operation_registry.clone(),
            self.product_session_registry.clone(),
        )
    }
}
