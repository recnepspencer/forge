use std::sync::{Arc, RwLock};

use crate::runtime::{RelationalRuntimeConfig, SchemaContractRuntimeSubsystem};

#[derive(Debug, Clone)]
pub(crate) struct RelationalPreparationConfigurationSnapshot {
    pub(crate) config: Arc<RelationalRuntimeConfig>,
    pub(crate) schema_contract_runtime: Arc<SchemaContractRuntimeSubsystem>,
}

#[derive(Debug)]
pub(in crate::runtime) struct RelationalPreparationConfigurationOwner {
    state: Arc<RwLock<RelationalPreparationConfigurationSnapshot>>,
}

#[derive(Debug, Clone)]
pub(crate) struct RelationalPreparationConfigurationBinding {
    state: Arc<RwLock<RelationalPreparationConfigurationSnapshot>>,
}

impl RelationalPreparationConfigurationOwner {
    pub(crate) fn new(
        config: &RelationalRuntimeConfig,
        schema_contract_runtime: &SchemaContractRuntimeSubsystem,
    ) -> Self {
        Self {
            state: Arc::new(RwLock::new(RelationalPreparationConfigurationSnapshot {
                config: Arc::new(config.clone()),
                schema_contract_runtime: Arc::new(schema_contract_runtime.clone()),
            })),
        }
    }

    pub(crate) fn binding(&self) -> RelationalPreparationConfigurationBinding {
        RelationalPreparationConfigurationBinding {
            state: Arc::clone(&self.state),
        }
    }

    pub(crate) fn synchronize(
        &self,
        config: &RelationalRuntimeConfig,
        schema_contract_runtime: &SchemaContractRuntimeSubsystem,
    ) {
        *self
            .state
            .write()
            .expect("preparation configuration lock poisoned") =
            RelationalPreparationConfigurationSnapshot {
                config: Arc::new(config.clone()),
                schema_contract_runtime: Arc::new(schema_contract_runtime.clone()),
            };
    }
}

impl RelationalPreparationConfigurationBinding {
    pub(crate) fn snapshot(&self) -> RelationalPreparationConfigurationSnapshot {
        self.state
            .read()
            .expect("preparation configuration lock poisoned")
            .clone()
    }
}
