use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    product_operation_contract::WorthServerStoredProductOperation,
    WorthServerCompatibilityPreparedRequest, WorthServerCompletedProductOperation,
    WorthServerOperationRegistry, WorthServerProductAdapterRegistry,
    WorthServerProductOperationInput, WorthServerProductOperationRuntime,
    WorthServerProductOperationSurfaceDenial, WorthServerProductSessionRegistry,
    WorthServerQueryHandoffConfig,
};

#[derive(Clone, Debug)]
pub struct WorthServerCompatibilityProductOperationFacade {
    runtime: WorthServerProductOperationRuntime,
}

#[derive(Clone, Debug)]
pub struct WorthServerCompatibilityAdmittedProductMutationCommand {
    input: WorthServerProductOperationInput,
}

impl WorthServerCompatibilityAdmittedProductMutationCommand {
    pub fn new(
        operation_name: impl Into<String>,
        payload: crate::WorthServerProductOperationPayload,
    ) -> Self {
        Self {
            input: WorthServerProductOperationInput::new(operation_name, payload),
        }
    }

    pub fn with_session(
        mut self,
        continuation: &super::WorthServerCompatibilityProductSessionContinuation,
    ) -> Self {
        self.input = self
            .input
            .with_product_session_identity(continuation.product_session_identity());
        self
    }

    pub fn with_product_session_identity(
        mut self,
        product_session_identity: impl Into<String>,
    ) -> Self {
        self.input = self
            .input
            .with_product_session_identity(product_session_identity);
        self
    }

    pub(crate) fn into_input(self) -> WorthServerProductOperationInput {
        self.input
    }
}

impl WorthServerCompatibilityProductOperationFacade {
    pub(crate) fn new(
        operation_registry: WorthServerOperationRegistry,
        product_adapter_registry: WorthServerProductAdapterRegistry,
        query_handoff_config: WorthServerQueryHandoffConfig,
        product_session_registry: WorthServerProductSessionRegistry,
        replay_store: Arc<Mutex<HashMap<String, WorthServerStoredProductOperation>>>,
    ) -> Self {
        Self {
            runtime: WorthServerProductOperationRuntime::new(
                operation_registry,
                product_adapter_registry,
                query_handoff_config,
                product_session_registry,
                replay_store,
            ),
        }
    }

    pub fn execute(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
        input: WorthServerProductOperationInput,
    ) -> Result<WorthServerCompletedProductOperation, WorthServerProductOperationSurfaceDenial>
    {
        self.runtime
            .execute_from_compat_http(prepared_request, input)
    }

    pub fn execute_admitted_mutation(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
        command: WorthServerCompatibilityAdmittedProductMutationCommand,
    ) -> Result<WorthServerCompletedProductOperation, WorthServerProductOperationSurfaceDenial>
    {
        self.execute(prepared_request, command.into_input())
    }
}
