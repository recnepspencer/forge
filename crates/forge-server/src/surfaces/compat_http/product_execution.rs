use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    product_operation_contract::ForgeServerStoredProductOperation,
    ForgeServerCompatibilityPreparedRequest, ForgeServerCompletedProductOperation,
    ForgeServerOperationRegistry, ForgeServerProductAdapterRegistry,
    ForgeServerProductOperationInput, ForgeServerProductOperationRuntime,
    ForgeServerProductOperationSurfaceDenial, ForgeServerProductSessionRegistry,
    ForgeServerQueryHandoffConfig,
};

#[derive(Clone, Debug)]
pub struct ForgeServerCompatibilityProductOperationFacade {
    runtime: ForgeServerProductOperationRuntime,
}

#[derive(Clone, Debug)]
pub struct ForgeServerCompatibilityAdmittedProductMutationCommand {
    input: ForgeServerProductOperationInput,
}

impl ForgeServerCompatibilityAdmittedProductMutationCommand {
    pub fn new(
        operation_name: impl Into<String>,
        payload: crate::ForgeServerProductOperationPayload,
    ) -> Self {
        Self {
            input: ForgeServerProductOperationInput::new(operation_name, payload),
        }
    }

    pub fn with_session(
        mut self,
        continuation: &super::ForgeServerCompatibilityProductSessionContinuation,
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

    pub(crate) fn into_input(self) -> ForgeServerProductOperationInput {
        self.input
    }
}

impl ForgeServerCompatibilityProductOperationFacade {
    pub(crate) fn new(
        operation_registry: ForgeServerOperationRegistry,
        product_adapter_registry: ForgeServerProductAdapterRegistry,
        query_handoff_config: ForgeServerQueryHandoffConfig,
        product_session_registry: ForgeServerProductSessionRegistry,
        replay_store: Arc<Mutex<HashMap<String, ForgeServerStoredProductOperation>>>,
    ) -> Self {
        Self {
            runtime: ForgeServerProductOperationRuntime::new(
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
        prepared_request: &ForgeServerCompatibilityPreparedRequest,
        input: ForgeServerProductOperationInput,
    ) -> Result<ForgeServerCompletedProductOperation, ForgeServerProductOperationSurfaceDenial>
    {
        self.runtime
            .execute_from_compat_http(prepared_request, input)
    }

    pub fn execute_admitted_mutation(
        &self,
        prepared_request: &ForgeServerCompatibilityPreparedRequest,
        command: ForgeServerCompatibilityAdmittedProductMutationCommand,
    ) -> Result<ForgeServerCompletedProductOperation, ForgeServerProductOperationSurfaceDenial>
    {
        self.execute(prepared_request, command.into_input())
    }
}
