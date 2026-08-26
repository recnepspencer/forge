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
    services: WorthServerWorthNativeSessionServices,
}

#[derive(Clone, Debug)]
pub(crate) struct WorthServerWorthNativeSessionServices {
    operation_registry: WorthServerOperationRegistry,
    product_adapter_registry: WorthServerProductAdapterRegistry,
    product_session_registry: WorthServerProductSessionRegistry,
    product_operation_retry_store: Arc<Mutex<HashMap<String, WorthServerStoredProductOperation>>>,
    counters: Arc<crate::diagnostics::WorthServerCounters>,
    query_handoff_config: WorthServerQueryHandoffConfig,
    declaration_intake: WorthServerDirectDeclarationIntakeFacade,
    query_handoff: WorthServerQueryHandoffFacade,
    responses: WorthServerResponseFacade,
}

impl WorthServerWorthNativeSessionServices {
    pub(crate) fn new(parts: WorthServerWorthNativeSessionServiceParts) -> Self {
        let WorthServerWorthNativeSessionServiceParts {
            operation_registry,
            product_adapter_registry,
            product_session_registry,
            product_operation_retry_store,
            counters,
            query_handoff_config,
            declaration_intake,
            query_handoff,
            responses,
        } = parts;
        Self {
            operation_registry,
            product_adapter_registry,
            product_session_registry,
            product_operation_retry_store,
            counters,
            query_handoff_config,
            declaration_intake,
            query_handoff,
            responses,
        }
    }
}

pub(crate) struct WorthServerWorthNativeSessionServiceParts {
    pub(crate) operation_registry: WorthServerOperationRegistry,
    pub(crate) product_adapter_registry: WorthServerProductAdapterRegistry,
    pub(crate) product_session_registry: WorthServerProductSessionRegistry,
    pub(crate) product_operation_retry_store:
        Arc<Mutex<HashMap<String, WorthServerStoredProductOperation>>>,
    pub(crate) counters: Arc<crate::diagnostics::WorthServerCounters>,
    pub(crate) query_handoff_config: WorthServerQueryHandoffConfig,
    pub(crate) declaration_intake: WorthServerDirectDeclarationIntakeFacade,
    pub(crate) query_handoff: WorthServerQueryHandoffFacade,
    pub(crate) responses: WorthServerResponseFacade,
}

impl WorthServerWorthNativePreparedSession {
    pub(crate) fn new(
        admission: WorthServerAdmission,
        services: WorthServerWorthNativeSessionServices,
    ) -> Self {
        Self {
            admission,
            services,
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
            self.services.declaration_intake.clone(),
        )
    }

    pub fn direct(&self) -> WorthServerWorthNativeDirectFacade {
        WorthServerWorthNativeDirectFacade::new(
            self.admission.clone(),
            self.services.operation_registry.clone(),
            self.services.declaration_intake.clone(),
            self.services.query_handoff.clone(),
            self.services.responses.clone(),
        )
    }

    pub fn product_operations(&self) -> WorthServerWorthNativeProductOperationFacade {
        WorthServerWorthNativeProductOperationFacade::new(
            self.admission.clone(),
            self.services.operation_registry.clone(),
            self.services.product_adapter_registry.clone(),
            self.services.query_handoff_config.clone(),
            self.services.product_session_registry.clone(),
            self.services.product_operation_retry_store.clone(),
            self.services.counters.clone(),
        )
    }

    pub fn product_sessions(&self) -> WorthServerWorthNativeProductSessionFacade {
        WorthServerWorthNativeProductSessionFacade::new(
            self.admission.clone(),
            self.services.operation_registry.clone(),
            self.services.product_adapter_registry.clone(),
            self.services.product_session_registry.clone(),
        )
    }

    pub fn into_session(self) -> WorthServerWorthNativeSession {
        WorthServerWorthNativeSession::new(self.admission, self.services)
    }
}

#[derive(Clone, Debug)]
pub struct WorthServerWorthNativeSession {
    admission: WorthServerAdmission,
    services: WorthServerWorthNativeSessionServices,
}

impl WorthServerWorthNativeSession {
    pub(crate) fn new(
        admission: WorthServerAdmission,
        services: WorthServerWorthNativeSessionServices,
    ) -> Self {
        Self {
            admission,
            services,
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
            self.services.declaration_intake.clone(),
        )
    }

    pub fn direct(&self) -> WorthServerWorthNativeDirectFacade {
        WorthServerWorthNativeDirectFacade::new(
            self.admission.clone(),
            self.services.operation_registry.clone(),
            self.services.declaration_intake.clone(),
            self.services.query_handoff.clone(),
            self.services.responses.clone(),
        )
    }

    pub fn product_operations(&self) -> WorthServerWorthNativeProductOperationFacade {
        WorthServerWorthNativeProductOperationFacade::new(
            self.admission.clone(),
            self.services.operation_registry.clone(),
            self.services.product_adapter_registry.clone(),
            self.services.query_handoff_config.clone(),
            self.services.product_session_registry.clone(),
            self.services.product_operation_retry_store.clone(),
            self.services.counters.clone(),
        )
    }

    pub fn product_sessions(&self) -> WorthServerWorthNativeProductSessionFacade {
        WorthServerWorthNativeProductSessionFacade::new(
            self.admission.clone(),
            self.services.operation_registry.clone(),
            self.services.product_adapter_registry.clone(),
            self.services.product_session_registry.clone(),
        )
    }
}
