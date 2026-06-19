use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    product_operation_contract::ForgeServerStoredProductOperation, ForgeServerAdmission,
    ForgeServerCompletedProductOperation, ForgeServerExecutedProductReadBatch,
    ForgeServerOperationRegistry, ForgeServerProductAdapterRegistry,
    ForgeServerProductOperationInput, ForgeServerProductOperationRuntime,
    ForgeServerProductOperationSurfaceDenial, ForgeServerProductSessionRegistry,
    ForgeServerQueryHandoffConfig,
};

#[derive(Clone, Debug)]
pub struct ForgeServerForgeNativeProductOperationFacade {
    admission: ForgeServerAdmission,
    runtime: ForgeServerProductOperationRuntime,
}

impl ForgeServerForgeNativeProductOperationFacade {
    pub(crate) fn new(
        admission: ForgeServerAdmission,
        operation_registry: ForgeServerOperationRegistry,
        product_adapter_registry: ForgeServerProductAdapterRegistry,
        query_handoff_config: ForgeServerQueryHandoffConfig,
        product_session_registry: ForgeServerProductSessionRegistry,
        replay_store: Arc<Mutex<HashMap<String, ForgeServerStoredProductOperation>>>,
    ) -> Self {
        Self {
            admission,
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
        input: ForgeServerProductOperationInput,
    ) -> Result<ForgeServerCompletedProductOperation, ForgeServerProductOperationSurfaceDenial>
    {
        self.runtime
            .execute_from_forge_native(&self.admission, input)
    }

    pub fn execute_mutation(
        &self,
        command: super::ForgeServerForgeNativeProductMutationCommand,
    ) -> Result<ForgeServerCompletedProductOperation, ForgeServerProductOperationSurfaceDenial>
    {
        self.execute(command.into_input())
    }

    pub fn execute_shared_read_batch(
        &self,
        inputs: Vec<ForgeServerProductOperationInput>,
    ) -> Result<ForgeServerExecutedProductReadBatch, ForgeServerProductOperationSurfaceDenial> {
        self.runtime
            .execute_shared_read_batch_from_forge_native(&self.admission, inputs)
    }
}
